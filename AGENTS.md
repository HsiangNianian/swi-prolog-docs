# AGENTS.md

Guidance for Coding Agents working in this repository.

## Project Shape

This repository maintains unofficial Chinese documentation for SWI-Prolog.

There are two documentation outputs:

- `swi-prolog-docs/` is the mdBook project.
- The root Rust crate exposes the same Markdown through rustdoc.

`swi-prolog-docs/src` is the single source of truth for translated document text.
Do not copy chapter bodies into `src/`.

## Source Layout

- `swi-prolog-docs/book.toml`: mdBook configuration.
- `swi-prolog-docs/src/SUMMARY.md`: mdBook table of contents.
- `swi-prolog-docs/src/chapter_*/README.md`: translated chapter Markdown.
- `docs/SOURCE_MAP.md`: upstream SWI-Prolog manual section map.
- `src/lib.rs`: thin Rust-facing wrapper using `include_str!`.
- `src/main.rs`: informational binary entry point.

Generated output is not source. Do not edit or commit:

- `target/`
- `swi-prolog-docs/book/`
- local `.omx/` runtime state

`Cargo.lock` is intentionally ignored because this crate is treated as a library.

## Single-Source Documentation Rule

The chapter Markdown files under `swi-prolog-docs/src` must remain the only
maintained copy of documentation text.

When adding or renaming a chapter:

1. Add or update `swi-prolog-docs/src/chapter_N/README.md`.
2. Add or update its entry in `swi-prolog-docs/src/SUMMARY.md`.
3. Add or update the upstream mapping in `docs/SOURCE_MAP.md`.
4. Add or update the matching `include_chapter!` entry in `src/lib.rs`.
5. Add or update the corresponding `Chapter` entry in `CHAPTERS`.
6. Keep the order in `CHAPTERS` the same as `SUMMARY.md`.

Do not create `src/chapter_N/mod.rs` files for copied docs.

## Markdown Compatibility

For the current Overview translation, use upstream
`SWI-Prolog/swipl-devel:man/overview.plx` and the online section links recorded
in `docs/SOURCE_MAP.md`.

The same Markdown is parsed by mdBook and rustdoc. Prefer Markdown that is valid
in both:

- Wrap literal placeholders such as `<value>` or `<script>` in backticks.
- Wrap predicate arity forms containing square brackets, such as
  `qsave_program/[1,2]`, in backticks.
- Write bare URLs as autolinks: `<https://example.com/>`.
- Keep command examples in fenced code blocks.
- Use non-Rust code fence languages such as `shell`, `text`, `prolog`, or `pl`
  for examples that rustdoc should not treat as Rust doctests.

## Dependencies

The root crate should not depend on `mdbook` or `mdbook-typst-pdf` as Rust
dependencies. mdBook is an external build tool for the book project, not a crate
API dependency.

The PDF backend in `book.toml` is optional so local HTML builds work without
`mdbook-typst-pdf`. CI may still install the PDF backend and publish both HTML
and PDF.

Do not add new dependencies unless they are clearly needed for a source-level
feature, not just for invoking a documentation build tool.

## Validation

For Rust-side changes, run:

```shell
cargo fmt --check
cargo check
cargo doc --no-deps
cargo test
```

`cargo test` includes an integration check that verifies `SUMMARY.md`,
`src/lib.rs::CHAPTERS`, chapter source files, and `docs/SOURCE_MAP.md` stay in
sync.

For mdBook-side changes, run when `mdbook` is available:

```shell
mdbook build swi-prolog-docs
```

If `mdbook-typst-pdf` is not installed locally, `mdbook build` should still
complete the HTML build and skip the optional PDF backend with a warning.

Before reporting completion, also run:

```shell
git diff --check
```

## CI And Publishing

`.github/workflows/deploy.yml` builds the mdBook project and publishes
`swi-prolog-docs/book` to `gh-pages` on `main`.

`.github/workflows/nightly.yml` publishes a nightly archive release.

Keep workflow changes narrow. If changing `book.toml`, verify whether the deploy
workflow still installs every required backend and whether output paths still
match the workflow.

## Working Style

- Preserve the single-source design before optimizing anything else.
- Prefer small, reviewable changes.
- Fix Markdown/rustdoc warnings at the Markdown source, not by duplicating text.
- Keep Chinese translation content in Markdown files; keep Rust files as wrappers
  and metadata only.
