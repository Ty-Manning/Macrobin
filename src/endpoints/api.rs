use crate::pasta::PastaFile;
use crate::util::animalnumbers::to_animal_names;
use crate::util::db::insert;
use crate::util::hashids::to_hashids;
use crate::util::misc::{encrypt, encrypt_file, is_valid_url};
use crate::{AppState, Pasta, ARGS};
use actix_web::{error, post, web, Error, HttpResponse, Responder};
use bytesize::ByteSize;
use log::warn;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
pub struct CreatePastaRequest {
    pub content: Option<String>,
    pub file_name: Option<String>, // For file uploads, base64 encoded
    pub file_content: Option<String>, // For file uploads, base64 encoded
    pub extension: Option<String>,
    pub private: Option<bool>,
    pub readonly: Option<bool>,
    pub editable: Option<bool>,
    pub encrypt_server: Option<bool>,
    pub encrypt_client: Option<bool>,
    pub encrypted_key: Option<String>,
    pub expiration: Option<String>,
    pub burn_after: Option<u16>,
    pub uploader_password: Option<String>,
    pub plain_key: Option<String>,
    pub random_key: Option<String>, // For client-side encryption
}

#[derive(Debug, Serialize)]
pub struct CreatePastaResponse {
    pub url: String,
}

fn expiration_to_timestamp(expiration: &str, timenow: i64) -> i64 {
    match expiration {
        "1min" => timenow + 60,
        "10min" => timenow + 60 * 10,
        "1hour" => timenow + 60 * 60,
        "24hour" => timenow + 60 * 60 * 24,
        "3days" => timenow + 60 * 60 * 24 * 3,
        "1week" => timenow + 60 * 60 * 24 * 7,
        "never" => {
            if ARGS.eternal_pasta {
                0
            } else {
                timenow + 60 * 60 * 24 * 7
            }
        }
        _ => {
            log::error!("Unexpected expiration time: {}", expiration);
            expiration_to_timestamp(&ARGS.default_expiry, timenow)
        }
    }
}

#[post("/api/create")]
pub async fn create_api(
    data: web::Data<AppState>,
    req: web::Json<CreatePastaRequest>,
) -> Result<impl Responder, Error> {
    let mut pastas = data.pastas.lock().unwrap();

    let timenow: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs() as i64,
        Err(_) => {
            log::error!("SystemTime before UNIX EPOCH!");
            0
        }
    };

    let mut new_pasta = Pasta {
        id: rand::thread_rng().gen::<u16>() as u64,
        content: String::from(""),
        file: None,
        extension: req.extension.clone().unwrap_or_else(|| String::from("")),
        private: req.private.unwrap_or(false),
        readonly: req.readonly.unwrap_or(false),
        editable: req.editable.unwrap_or(ARGS.editable),
        encrypt_server: req.encrypt_server.unwrap_or(false),
        encrypt_client: req.encrypt_client.unwrap_or(false),
        encrypted_key: req.encrypted_key.clone(),
        created: timenow,
        read_count: 0,
        burn_after_reads: req.burn_after.unwrap_or(0) as u64,
        last_read: timenow,
        pasta_type: String::from(""), // Will be determined later
        expiration: expiration_to_timestamp(
            &req.expiration.clone().unwrap_or_else(|| ARGS.default_expiry.clone()),
            timenow,
        ),
    };

    // Handle uploader password if readonly mode is enabled
    if ARGS.readonly && ARGS.uploader_password.is_some() {
        if req.uploader_password.as_deref() != ARGS.uploader_password.as_deref() {
            return Ok(HttpResponse::Unauthorized().json("Incorrect uploader password"));
        }
    }

    // Handle content
    if let Some(content) = &req.content {
        new_pasta.content = content.clone();
        new_pasta.pasta_type = if is_valid_url(content.as_str()) {
            String::from("url")
        } else {
            String::from("text")
        };
    }

    // Handle file upload
    if let (Some(file_name), Some(file_content)) = (&req.file_name, &req.file_content) {
        if ARGS.no_file_upload {
            return Ok(HttpResponse::Forbidden().json("File uploads are disabled"));
        }

        match PastaFile::from_unsanitized(file_name) {
            Ok(mut file_obj) => {
                let file_bytes = match base64::decode(file_content) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        log::error!("Failed to decode base64 file content: {}", e);
                        return Ok(HttpResponse::BadRequest().json("Invalid base64 file content"));
                    }
                };

                if (new_pasta.encrypt_server
                    && file_bytes.len() > ARGS.max_file_size_encrypted_mb * 1024 * 1024)
                    || file_bytes.len() > ARGS.max_file_size_unencrypted_mb * 1024 * 1024
                {
                    return Ok(HttpResponse::BadRequest().json("File exceeded size limit."));
                }

                std::fs::create_dir_all(format!(
                    "{}/attachments/{}",
                    ARGS.data_dir,
                    &new_pasta.id_as_animals()
                ))
                .map_err(|e| {
                    error::ErrorInternalServerError(format!("Failed to create attachment directory: {}", e))
                })?;

                let filepath = format!(
                    "{}/attachments/{}/{}",
                    ARGS.data_dir,
                    &new_pasta.id_as_animals(),
                    &file_obj.name()
                );

                let mut f = std::fs::File::create(&filepath).map_err(|e| {
                    error::ErrorInternalServerError(format!("Failed to create file: {}", e))
                })?;
                f.write_all(&file_bytes).map_err(|e| {
                    error::ErrorInternalServerError(format!("Failed to write file: {}", e))
                })?;

                file_obj.size = ByteSize::b(file_bytes.len() as u64);
                new_pasta.file = Some(file_obj);
                new_pasta.pasta_type = String::from("file"); // Or "text" if it's the only content
            }
            Err(e) => {
                warn!("Unsafe file name: {:?} for API upload", e);
                return Ok(HttpResponse::BadRequest().json("Unsafe file name"));
            }
        }
    }

    // Handle encryption keys and content encryption
    let plain_key = req.plain_key.clone().unwrap_or_default();
    let random_key = req.random_key.clone().unwrap_or_default();

    if !plain_key.is_empty() && new_pasta.readonly {
        new_pasta.encrypted_key = Some(encrypt(new_pasta.id.to_string().as_str(), &plain_key));
    }

    if new_pasta.encrypt_server && !new_pasta.readonly && !new_pasta.content.is_empty() {
        if new_pasta.encrypt_client {
            new_pasta.content = encrypt(&new_pasta.content, &random_key);
        } else {
            new_pasta.content = encrypt(&new_pasta.content, &plain_key);
        }
    }

    if new_pasta.file.is_some() && new_pasta.encrypt_server && !new_pasta.readonly {
        let filepath = format!(
            "{}/attachments/{}/{}",
            ARGS.data_dir,
            &new_pasta.id_as_animals(),
            &new_pasta.file.as_ref().unwrap().name()
        );
        if new_pasta.encrypt_client {
            encrypt_file(&random_key, &filepath).map_err(|e| {
                error::ErrorInternalServerError(format!("Failed to encrypt file: {}", e))
            })?;
        } else {
            encrypt_file(&plain_key, &filepath).map_err(|e| {
                error::ErrorInternalServerError(format!("Failed to encrypt file: {}", e))
            })?;
        }
    }

    let id = new_pasta.id;
    let encrypt_server = new_pasta.encrypt_server;

    pastas.push(new_pasta);

    // Persist to database
    for (_, pasta) in pastas.iter().enumerate() {
        if pasta.id == id {
            insert(Some(&pastas), Some(pasta));
        }
    }

    let slug = if ARGS.hash_ids {
        to_hashids(id)
    } else {
        to_animal_names(id)
    };

    let url = if encrypt_server {
        format!("{}/auth/{}/success", ARGS.public_path_as_str(), slug)
    } else {
        format!("{}/upload/{}", ARGS.public_path_as_str(), slug)
    };

    Ok(HttpResponse::Ok().json(CreatePastaResponse { url }))
}
