# GNU Emacs 接口

SWI-Prolog 通过 sweep 包与 GNU Emacs 紧密集成。此包将 SWI-Prolog 嵌入为动态 Emacs 模块，允许直接从 Emacs Lisp 执行 Prolog 查询。随附的 Emacs 包 sweeprolog.el 可通过标准 Emacs 包管理器 package.el 安装，它基于此嵌入构建，为 GNU Emacs 中的 SWI-Prolog 提供完全集成的开发环境。

GNU Emacs 默认附带一个名为 prolog.el 的 Prolog 模式。与 sweeprolog.el 相比，此模式由于缺少合适的 Prolog 解析器而存在一些问题。Masanobu Umeda 编写的原始 prolog.el 自 1989 年以来一直包含在 GNU Emacs 中，2006 年 Stefan Monnier 在 prolog.el 中添加了对 SWI-Prolog 的明确支持。 2011 年，大部分原始实现已被 Stefan Bruda 最初为 XEmacs 端口编写的新 Prolog 模式所取代。Stefan Monnier 将 Bruda 的模式改编为 GNU Emacs，从那时起，他一直与其他 GNU Emacs 贡献者一起维护该模式。此模式的用户可以访问 https://www.metalevel.at/pceprolog/ 找到有用的配置建议。

其他可用于与 SWI-Prolog 配合使用的 Emacs 软件包包括：

https://www.metalevel.at/ediprolog/
直接在 Emacs 缓冲区中与 SWI-Prolog 交互。
https://www.metalevel.at/etrace/
使用 Emacs 跟踪 Prolog 代码。
https://emacs-lsp.github.io/dap-mode/page/configuration/#swi-prolog
通过 dap-mode 和 https://github.com/eshelyaron/debug_adapter 中的 debug_adapter 包，Emacs 中的 SWI-Prolog 支持调试适配器协议 (DAP)
https://emacs-lsp.github.io/lsp-mode/page/lsp-prolog/
通过 lsp-mode 和 https://github.com/jamesnvc/lsp_server 中的 lsp_server 包，Emacs 中的 SWI-Prolog 支持语言服务器协议 (LSP)