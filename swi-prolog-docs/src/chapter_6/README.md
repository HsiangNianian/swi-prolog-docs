# GNU Emacs 接口

SWI-Prolog 通过 `sweep` 包与 GNU Emacs 紧密集成。该包把 SWI-Prolog 嵌入为一个动态 Emacs 模块，使得 Prolog 查询可以直接从 Emacs Lisp 执行。配套的 Emacs 包 `sweeprolog.el` 可以通过标准 Emacs 包管理器 `package.el` 安装；它构建在这种嵌入能力之上，为 GNU Emacs 中的 SWI-Prolog 提供完整集成的开发环境。

GNU Emacs 默认附带一个名为 `prolog.el` 的 Prolog 模式。与 `sweeprolog.el` 相比，该模式因缺少合适的 Prolog 解析器而存在一些问题。Masanobu Umeda 编写的原始 `prolog.el` 自 1989 年起就包含在 GNU Emacs 中；2006 年，Stefan Monnier 为 `prolog.el` 添加了对 SWI-Prolog 的显式支持。2011 年，原始实现的大部分被新的 Prolog 模式替换；该模式最初由 Stefan Bruda 为 XEmacs 移植编写，后来由 Stefan Monnier 改编到 GNU Emacs。此后，Stefan Monnier 与其他 GNU Emacs 贡献者一起维护它。该模式的用户可以在 <https://www.metalevel.at/pceprolog/> 找到有用的配置建议。

其他可用于配合 SWI-Prolog 工作的 Emacs 包包括：

- <https://www.metalevel.at/ediprolog/>

  直接在 Emacs buffer 中与 SWI-Prolog 交互。

- <https://www.metalevel.at/etrace/>

  使用 Emacs 跟踪 Prolog 代码。

- <https://emacs-lsp.github.io/dap-mode/page/configuration/#swi-prolog>

  通过 `dap-mode` 以及 <https://github.com/eshelyaron/debug_adapter> 中的 `debug_adapter` 包，在 Emacs 中为 SWI-Prolog 提供调试适配器协议（DAP）支持。

- <https://emacs-lsp.github.io/lsp-mode/page/lsp-prolog/>

  通过 `lsp-mode` 以及 <https://github.com/jamesnvc/lsp_server> 中的 `lsp_server` 包，在 Emacs 中为 SWI-Prolog 提供语言服务器协议（LSP）支持。
