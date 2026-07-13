//! Rust-facing view of the SWI-Prolog documentation.
//!
//! The Markdown files under `swi-prolog-docs/src` are the single source of
//! truth. mdBook reads them directly, and this crate exposes the same files to
//! rustdoc and to Rust callers through `include_str!`.

/// A documentation chapter backed by one mdBook Markdown file.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Chapter {
    /// Stable module and directory name.
    pub slug: &'static str,
    /// Human-readable chapter title.
    pub title: &'static str,
    /// Markdown source shared with mdBook.
    pub markdown: &'static str,
}

macro_rules! include_chapter {
    ($module:ident, $title:literal, $path:literal) => {
        #[doc = include_str!($path)]
        pub mod $module {
            /// Chapter title as shown in the mdBook table of contents.
            pub const TITLE: &str = $title;

            /// Markdown source shared by rustdoc and mdBook.
            pub const MARKDOWN: &str = include_str!($path);
        }
    };
}

include_chapter!(
    chapter_1,
    "快速入门",
    "../swi-prolog-docs/src/chapter_1/README.md"
);
include_chapter!(
    chapter_2,
    "用户的初始化文件",
    "../swi-prolog-docs/src/chapter_2/README.md"
);
include_chapter!(
    chapter_3,
    "初始化文件和目标",
    "../swi-prolog-docs/src/chapter_3/README.md"
);
include_chapter!(
    chapter_4,
    "命令行选项",
    "../swi-prolog-docs/src/chapter_4/README.md"
);
include_chapter!(
    chapter_5,
    "UI 主题",
    "../swi-prolog-docs/src/chapter_5/README.md"
);
include_chapter!(
    chapter_6,
    "GNU Emacs 接口",
    "../swi-prolog-docs/src/chapter_6/README.md"
);
include_chapter!(
    chapter_7,
    "在线帮助",
    "../swi-prolog-docs/src/chapter_7/README.md"
);
include_chapter!(
    chapter_8,
    "命令行历史",
    "../swi-prolog-docs/src/chapter_8/README.md"
);
include_chapter!(
    chapter_9,
    "复用顶层绑定",
    "../swi-prolog-docs/src/chapter_9/README.md"
);
include_chapter!(
    chapter_10,
    "调试器概览",
    "../swi-prolog-docs/src/chapter_10/README.md"
);
include_chapter!(
    chapter_11,
    "加载和运行项目",
    "../swi-prolog-docs/src/chapter_11/README.md"
);

/// Chapters exposed in the same order as the mdBook summary.
pub const CHAPTERS: &[Chapter] = &[
    Chapter {
        slug: "chapter_1",
        title: chapter_1::TITLE,
        markdown: chapter_1::MARKDOWN,
    },
    Chapter {
        slug: "chapter_2",
        title: chapter_2::TITLE,
        markdown: chapter_2::MARKDOWN,
    },
    Chapter {
        slug: "chapter_3",
        title: chapter_3::TITLE,
        markdown: chapter_3::MARKDOWN,
    },
    Chapter {
        slug: "chapter_4",
        title: chapter_4::TITLE,
        markdown: chapter_4::MARKDOWN,
    },
    Chapter {
        slug: "chapter_5",
        title: chapter_5::TITLE,
        markdown: chapter_5::MARKDOWN,
    },
    Chapter {
        slug: "chapter_6",
        title: chapter_6::TITLE,
        markdown: chapter_6::MARKDOWN,
    },
    Chapter {
        slug: "chapter_7",
        title: chapter_7::TITLE,
        markdown: chapter_7::MARKDOWN,
    },
    Chapter {
        slug: "chapter_8",
        title: chapter_8::TITLE,
        markdown: chapter_8::MARKDOWN,
    },
    Chapter {
        slug: "chapter_9",
        title: chapter_9::TITLE,
        markdown: chapter_9::MARKDOWN,
    },
    Chapter {
        slug: "chapter_10",
        title: chapter_10::TITLE,
        markdown: chapter_10::MARKDOWN,
    },
    Chapter {
        slug: "chapter_11",
        title: chapter_11::TITLE,
        markdown: chapter_11::MARKDOWN,
    },
];
