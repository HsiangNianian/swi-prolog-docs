fn main() {
    println!(
        "SWI-Prolog docs: {} chapters. Run `cargo doc --no-deps` for rustdoc or `mdbook build swi-prolog-docs` for mdBook.",
        swi_prolog_docs::CHAPTERS.len()
    );
}
