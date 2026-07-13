# swi-prolog-docs
Unofficial swi-prolog docs in multi language edition

## Documentation source

`swi-prolog-docs/src` is the single source of truth for the translated Markdown.
mdBook reads those files directly. The Rust crate exposes the same chapter files
through `include_str!`, so `cargo doc --no-deps` and the mdBook build do not need
separate copies of the text.

Build the Rust documentation with:

```shell
cargo doc --no-deps
```

Build the mdBook output with mdBook installed:

```shell
mdbook build swi-prolog-docs
```

The PDF backend is optional for local builds. Install `mdbook-typst-pdf` when you
need the PDF output as well as HTML.

See `docs/SOURCE_MAP.md` for the upstream SWI-Prolog manual sections mapped to
local chapter files.
