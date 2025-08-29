// --- Global Setup for marked.js ---
// This configuration and extension definition runs as soon as the script is loaded.
if (typeof marked !== 'undefined') {
    marked.setOptions({
        gfm: true, // GitHub Flavored Markdown
        breaks: true // Convert \n to <br>
    });

    // Define custom extensions for marked
    marked.use({
        extensions: [
            {
                name: 'coloredText',
                level: 'inline',
                start(src) { return src.indexOf('%'); },
                tokenizer(src) {
                    const rule = /^%([a-zA-Z]+|#(?:[0-9a-fA-F]{3}){1,2})%(.*?)%%/;
                    const match = rule.exec(src);
                    if (match) {
                        return {
                            type: 'coloredText',
                            raw: match[0],
                            color: match[1],
                            text: match[2].trim()
                        };
                    }
                    return false; // Use false if no match
                },
                renderer(token) {
                    return `<span style="color: ${token.color};">${token.text}</span>`;
                }
            },
            {
                name: 'highlight',
                level: 'inline',
                start(src) { return src.indexOf('=='); },
                tokenizer(src) {
                    const rule = /^==(.*?)==/;
                    const match = rule.exec(src);
                    if (match) {
                        return {
                            type: 'highlight',
                            raw: match[0],
                            text: match[1].trim()
                        };
                    }
                    return false;
                },
                renderer(token) {
                    return `<span style="background-color: yellow;">${token.text}</span>`;
                }
            },
            {
                name: 'spoiler',
                level: 'inline',
                start(src) { return src.indexOf('!>'); },
                tokenizer(src) {
                    const rule = /^!>(.*)/;
                    const match = rule.exec(src);
                    if (match) {
                        return {
                            type: 'spoiler',
                            raw: match[0],
                            text: match[1].trim()
                        };
                    }
                    return false;
                },
                renderer(token) {
                    const innerHtml = marked.parseInline(token.text);
                    return `<details><summary>Spoiler</summary>${innerHtml}</details>`;
                }
            },
            {
                name: 'admonition',
                level: 'block',
                start(src) { return src.indexOf('!!!'); },
                tokenizer(src, tokens) {
                    const rule = /^!!!\s*(note|info|warning|danger|greentext)(?:\s+(.*?))?\n([\s\S]*?)(?=\n\n|\n!!!|$)/;
                    const match = rule.exec(src);
                    if (match) {
                        return {
                            type: 'admonition',
                            raw: match[0],
                            admonitionType: match[1],
                            title: match[2] ? match[2].trim() : null,
                            text: match[3].trim()
                        };
                    }
                    return false;
                },
                renderer(token) {
                    const titleText = token.title || token.admonitionType.charAt(0).toUpperCase() + token.admonitionType.slice(1);
                    let icon = '';
                    let className = `admonition admonition-${token.admonitionType}`;
                    switch (token.admonitionType) {
                        case 'note': icon = '📝'; break;
                        case 'info': icon = 'ℹ️'; break;
                        case 'warning': icon = '⚠️'; break;
                        case 'danger': icon = '🚨'; break;
                        case 'greentext': icon = '💬'; className = 'admonition admonition-greentext'; break;
                    }
                    const titleHtml = marked.parseInline(titleText);
                    const bodyHtml = marked.parseInline(token.text);
                    return `<div class="${className}">
                               <p class="admonition-title">${icon} ${titleHtml}</p>
                               <p>${bodyHtml}</p>
                             </div>`;
                }
            }
        ]
    });
} else {
    console.error("marked.js is not loaded. Custom markdown extensions will not be applied.");
}


// --- Page-Specific Logic for index.html ---
// This logic only runs when the DOM is fully loaded on the page where this script is included.
document.addEventListener('DOMContentLoaded', function() {
    // This logic is specific to the index.html page (the creation page)
    const renderMarkdownCheckbox = document.getElementById("render_markdown");
    const contentInput = document.getElementById("content-input");
    const markdownPreview = document.getElementById("markdown-preview");

    // Only proceed if the elements for the index page live preview exist
    if (renderMarkdownCheckbox && contentInput && markdownPreview) {
        function updateMarkdownPreview() {
            if (renderMarkdownCheckbox.checked) {
                const markdownText = contentInput.value;
                try {
                    const htmlContent = marked.parse(markdownText);
                    markdownPreview.innerHTML = htmlContent;
                    contentInput.style.display = "none";
                    markdownPreview.style.display = "block";
                } catch (e) {
                    console.error("Error parsing markdown:", e);
                    markdownPreview.innerHTML = "<p style='color: red;'>Error rendering markdown. Check console for details.</p>";
                    contentInput.style.display = "none";
                    markdownPreview.style.display = "block";
                }
            } else {
                contentInput.style.display = "block";
                markdownPreview.style.display = "none";
            }
        }

        renderMarkdownCheckbox.addEventListener("change", updateMarkdownPreview);
        contentInput.addEventListener("input", updateMarkdownPreview);

        // Initial render state for index.html
        updateMarkdownPreview();
    }
});
