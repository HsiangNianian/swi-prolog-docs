# Source Map

This file maps local translated chapters to the upstream SWI-Prolog manual.

Upstream manual entry points:

- Online manual: <https://www.swi-prolog.org/pldoc/doc_for?object=manual>
- Overview page: <https://www.swi-prolog.org/pldoc/man?section=overview>
- Source repository: <https://github.com/SWI-Prolog/swipl-devel>
- Overview source file: `man/overview.plx`
- Source snapshot checked for this map:
  `SWI-Prolog/swipl-devel@9bacc37144a3fc7203224376f50ea51e5bb5a8e1`

Use this map when translating, re-translating, or checking whether
`swi-prolog-docs/src/SUMMARY.md` and `src/lib.rs` are complete.

## Current Local Chapters

| Local chapter | Chinese title | Upstream title | Upstream label | Online section |
| --- | --- | --- | --- | --- |
| `chapter_1` | 快速入门 | Getting started quickly | `quickstart` | <https://www.swi-prolog.org/pldoc/man?section=quickstart> |
| `chapter_2` | 用户的初始化文件 | The user's initialisation file | `initfile` | <https://www.swi-prolog.org/pldoc/man?section=initfile> |
| `chapter_3` | 初始化文件和目标 | Initialisation files and goals | `initgoal` | <https://www.swi-prolog.org/pldoc/man?section=initgoal> |
| `chapter_4` | 命令行选项 | Command line options | `cmdline` | <https://www.swi-prolog.org/pldoc/man?section=cmdline> |
| `chapter_5` | UI 主题 | UI Themes | `theme` | <https://www.swi-prolog.org/pldoc/man?section=theme> |
| `chapter_6` | GNU Emacs 接口 | GNU Emacs Interface | `gemacs` | <https://www.swi-prolog.org/pldoc/man?section=gemacs> |
| `chapter_7` | 在线帮助 | Online Help | `online-help` | <https://www.swi-prolog.org/pldoc/man?section=online-help> |
| `chapter_8` | 命令行历史 | Command line history | `history` | <https://www.swi-prolog.org/pldoc/man?section=history> |
| `chapter_9` | 复用顶层绑定 | Reuse of top-level bindings | `topvars` | <https://www.swi-prolog.org/pldoc/man?section=topvars> |
| `chapter_10` | 调试器概览 | Overview of the Debugger | `debugoverview` | <https://www.swi-prolog.org/pldoc/man?section=debugoverview> |
| `chapter_11` | 加载和运行项目 | Loading and running projects | `compilation` | <https://www.swi-prolog.org/pldoc/man?section=compilation> |
| `chapter_12` | 环境控制（Prolog 标志） | Environment Control (Prolog flags) | `flags` | <https://www.swi-prolog.org/pldoc/man?section=flags> |

## Overview Translation Queue

Continue the Overview chapter in this upstream order:

| Proposed local chapter | Chinese title | Upstream title | Upstream label | Online section |
| --- | --- | --- | --- | --- |
| `chapter_13` | 钩子谓词概览 | An overview of hook predicates | `hooks` | <https://www.swi-prolog.org/pldoc/man?section=hooks> |
| `chapter_14` | 库的自动加载 | Automatic loading of libraries | `autoload` | <https://www.swi-prolog.org/pldoc/man?section=autoload> |
| `chapter_15` | SWI-Prolog 语法 | The SWI-Prolog syntax | `syntax` | <https://www.swi-prolog.org/pldoc/man?section=syntax> |
| `chapter_16` | 有理树（循环项） | Rational trees (cyclic terms) | `cyclic` | <https://www.swi-prolog.org/pldoc/man?section=cyclic> |
| `chapter_17` | 即时子句索引 | Just-in-time clause indexing | `jitindex` | <https://www.swi-prolog.org/pldoc/man?section=jitindex> |
| `chapter_18` | 宽字符支持 | Wide character support | `widechars` | <https://www.swi-prolog.org/pldoc/man?section=widechars> |
| `chapter_19` | 系统限制 | System limits | `limits` | <https://www.swi-prolog.org/pldoc/man?section=limits> |
| `chapter_20` | 二进制兼容性 | Binary compatibility | `abi-versions` | <https://www.swi-prolog.org/pldoc/man?section=abi-versions> |

## Translation Notes

- Treat the upstream `.plx` source as authoritative when the online HTML and
  rendered text differ.
- Keep local Markdown compatible with both mdBook and rustdoc.
- Use local chapter numbers for repository structure only. Preserve upstream
  section names and labels in this map.
- When an upstream section is split into multiple local files, document the
  split in this file before changing `SUMMARY.md`.
