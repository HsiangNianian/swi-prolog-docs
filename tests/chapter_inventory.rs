use std::fs;
use std::path::Path;

use swi_prolog_docs::CHAPTERS;

#[derive(Debug, Eq, PartialEq)]
struct SummaryEntry {
    title: String,
    slug: String,
}

#[test]
fn summary_chapters_match_rust_inventory_and_source_files() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let entries = parse_summary(&root.join("swi-prolog-docs/src/SUMMARY.md"));

    let summary_slugs: Vec<&str> = entries.iter().map(|entry| entry.slug.as_str()).collect();
    let rust_slugs: Vec<&str> = CHAPTERS.iter().map(|chapter| chapter.slug).collect();
    assert_eq!(
        summary_slugs, rust_slugs,
        "SUMMARY.md order must match src/lib.rs CHAPTERS order"
    );

    for (entry, chapter) in entries.iter().zip(CHAPTERS) {
        assert_eq!(
            entry.title, chapter.title,
            "SUMMARY.md title for {} must match CHAPTERS",
            entry.slug
        );

        let chapter_path = root
            .join("swi-prolog-docs/src")
            .join(&entry.slug)
            .join("README.md");
        assert!(
            chapter_path.is_file(),
            "missing source file for {} at {}",
            entry.slug,
            chapter_path.display()
        );

        let markdown = fs::read_to_string(&chapter_path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", chapter_path.display()));
        assert_eq!(
            markdown, chapter.markdown,
            "{} must be exposed through include_str!",
            entry.slug
        );
    }
}

#[test]
fn current_source_map_covers_all_summary_chapters() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let entries = parse_summary(&root.join("swi-prolog-docs/src/SUMMARY.md"));
    let source_map = fs::read_to_string(root.join("docs/SOURCE_MAP.md"))
        .expect("failed to read docs/SOURCE_MAP.md");

    for entry in entries {
        let needle = format!("| `{}` |", entry.slug);
        assert!(
            source_map.contains(&needle),
            "docs/SOURCE_MAP.md must include current chapter {}",
            entry.slug
        );
    }
}

fn parse_summary(path: &Path) -> Vec<SummaryEntry> {
    let summary = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));

    summary.lines().filter_map(parse_summary_line).collect()
}

fn parse_summary_line(line: &str) -> Option<SummaryEntry> {
    let line = line.trim();
    if !line.starts_with("- [") {
        return None;
    }

    let title_end = line.find("](")?;
    let title = line[3..title_end].to_owned();
    let path_start = title_end + 2;
    let path_end = line[path_start..].find(')')? + path_start;
    let path = &line[path_start..path_end];
    let slug = path
        .trim_start_matches("./")
        .strip_suffix("/README.md")?
        .to_owned();

    Some(SummaryEntry { title, slug })
}
