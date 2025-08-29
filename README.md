# MacroBin

**MacroBin** is a fork of the excellent [MicroBin](https://github.com/szabodanika/microbin) by Dániel Szabó. It retains all the core features of MicroBin while introducing several enhancements and changes.

This version adds support for:

* Markdown styling for pastas.
* 4 animal names per pasta instead of 3 for increased variety.
* `/api` endpoints for programmatic generation of pastas.

Check out MacroBin at macrobin.co

## Features (Inherited from MicroBin)

* Entirely self-contained executable
* Server-side and client-side encryption
* File uploads (e.g. `server.com/file/pig-dog-cat-emu`)
* Raw text serving (e.g. `server.com/raw/pig-dog-cat-emu`)
* Animal names instead of random numbers for upload identifiers (64 animals)
* SQLite and JSON database support
* Private and public, editable and uneditable, automatically and never expiring uploads
* Automatic dark mode and custom styling support with very little CSS and only vanilla JS (see [`water.css`](https://github.com/kognise/water.css))
* And much more!

## What is an upload?

In MicroBin (and thus MacroBin), an upload can be:

* A text that you want to paste from one machine to another, e.g. some code,
* A file that you want to share, e.g. a video that is too large for Discord, a zip with a code project in it or an image,
* A URL redirection.

## When is MicroBin/MacroBin useful?

You can use MicroBin/MacroBin:

* To send long texts to other people,
* To send large files to other people,
* To share secrets or sensitive documents securely,
* As a URL shortener/redirect service,
* To serve content on the web, eg . configuration files for testing, images, or any other file content using the Raw functionality,
* To move files between your desktop and a server you access from the console,
* As a "postbox" service where people can upload their files or texts, but they cannot see or remove what others sent you,
* Or even to take quick notes.

...and many other things, why not get creative?

## License

The original MicroBin project is licensed under the BSD 3-Clause License. MacroBin is licensed under the GNU General Public License v3.0 (GPLv3).

### Original MicroBin BSD 3-Clause License

Copyright (c) 2022-2023, Dániel Szabó
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

### MacroBin is licensed under the GPLv3 License. See LICENSE file for more details
