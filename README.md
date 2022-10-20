# Field Notes

A static html server with compiled-in-binary content sources. The goal of this project is to create a singular binary capable of serving blog-like content without the mess of databases, stateful servers, filesystems, and so on. Note that the release binary includes _my_ static html content, and other users will need to fork and recompile.

## Usage:

- Add content to serve in the `content/` directory. The filename will be the slug of the page.
- Update `fieldnotes.json` to include the slug, title, and unix epoch for publishing
- Add templates in `templates/` using Jinja2/Tera syntax
- Add static files in `static/`
- Build with `cargo build --release`
- Open a web browser to `127.0.0.1:3030`

## TODO:

- Allow scheduled publishing by including future epochs
- Transition to standards-compliant HTML/CSS
- Add marketing-related features (analytics, newsletter signups, etc)
