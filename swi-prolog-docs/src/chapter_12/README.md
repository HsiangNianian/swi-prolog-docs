# 环境控制（Prolog 标志）

谓词 `current_prolog_flag/2` 和 `set_prolog_flag/2` 允许用户检查和修改执行环境。它们提供对安装特性、可选功能、操作系统、外部代码环境、命令行参数、版本信息的访问，也提供一些运行时标志，用于控制特定谓词的运行时行为，从而兼容其他 Prolog 环境。

## `current_prolog_flag/2`

`current_prolog_flag(?Key, -Value)` 定义了访问安装特性的接口，包括编译进系统的选项、版本、home 目录等。如果两个参数都未绑定，它会生成所有已定义的 Prolog 标志。如果 `Key` 已实例化，它会把 `Value` 与该 Prolog 标志的值合一；如果 `Key` 不是 Prolog 标志，则失败。

标记为可变（changeable）的标志可以由用户使用 `set_prolog_flag/2` 修改。标志值有类型。标记为 `bool` 的标志可以取值 `true` 或 `false`。谓词 `create_prolog_flag/3` 可用于创建描述或控制库和应用程序行为的标志。库 `library(settings)` 为管理应用程序参数提供了另一种接口。

有些 Prolog 标志并不是在所有版本中都定义。下文通常会用“如果存在且为 true”来说明这种情况。一个布尔 Prolog 标志为 true，当且仅当该 Prolog 标志存在，并且 `Value` 是原子 `true`。对这类标志的测试应写成下面这样：

```prolog
(   current_prolog_flag(windows, true)
->  <Do MS-Windows things>
;   <Do normal things>
)
```

有些 Prolog 标志的作用域限定在一个源文件内。这意味着，如果在文件内部用指令设置它们，载入该文件开始时遇到的标志值会在载入完成后恢复。目前，下列标志具有源文件作用域：`generate_debug_info` 和 `optimise`。

新线程会复制创建它的线程（即父线程）的所有标志。这使用 copy-on-write 技术实现。因此，在线程内部修改某个标志不会影响其他线程。

### `abi_version` (`dict`, `r`)

该标志的值是一个 dict，其中的键描述各种应用程序二进制接口（Application Binary Interface，ABI）组件的版本。详情见“二进制兼容性”。

### `access_level` (`atom`, `rw`)

该标志定义普通“用户”视图（`user`，默认值）或“系统”视图。在系统视图中，所有系统代码都可以像普通用户代码一样完全访问。在用户视图中，某些操作不被允许，某些细节会保持不可见。具体后果没有完全定义；例如，在系统访问级别下可以跟踪系统代码，也可以重定义系统谓词。

### `address_bits` (`integer`, `r`)

宿主机器的地址大小，通常为 32 或 64。除了最大堆栈限制之外，这对用户几乎没有影响。另请参阅 Prolog 标志 `arch`。

### `agc_close_streams` (`boolean`, `rw`)

当为 `true` 时，原子垃圾回收器会关闭那些仍处于打开状态但已被垃圾回收的流，并打印警告。默认值为 `false`，未来版本可能改为 `true`。下面是这类警告示例：

```text
WARNING: AGC: closed <stream>(0x560e29014400)
```

注意，不应把关闭 I/O 流的工作留给原子垃圾回收器。原因是原子垃圾回收器可能要过很久才运行，而且它是保守的（conservative），并不保证所有垃圾原子都能被回收。使用 I/O 流的代码应使用 `setup_call_cleanup/3`，结构如下，其中 `process/1` 是从 `Stream` 读取或写入的谓词。

```prolog
setup_call_cleanup(
    open(..., Stream),
    process(Stream),
    close(Stream)),
...
```

注意，该标志在 `main` 线程中的设置会生效。

### `agc_margin` (`integer`, `rw`)

如果可能成为垃圾的原子数量达到这个值，则在第一个机会执行原子垃圾回收。初始值为 10,000，可以修改。值为 0 会禁用原子垃圾回收。另请参阅 `PL_register_atom()`。由于 SWI-Prolog 对原子长度没有限制，10,000 个原子仍可能占用大量内存。使用超大原子的应用程序可能希望显式调用 `garbage_collect_atoms/0`，或降低这个阈值。

### `allow_dot_in_atom` (`bool`, `rw`)

如果为 `true`（默认值为 `false`），未加引号且以字母开头的原子中可以嵌入点号。嵌入的点号后面必须跟一个标识符继续字符，也就是字母、数字或下划线。许多语言允许在标识符中使用点号，因此这个标志对定义 DSL 可能有用。注意，这会与级联函数式记法冲突。例如，如果该标志设置为 `true`，`Post.meta.author` 会被读取为 `.(Post, 'meta.author')`。

### `allow_variable_name_as_functor` (`bool`, `rw`)

如果为 true（默认值为 false），`Functor(arg)` 会被读取为 `'Functor'(arg)`。有些应用程序使用 Prolog 的 `read/1` 谓词读取应用程序自定义的脚本语言。在这些场景中，常常很难向非 Prolog 用户解释常量和函数只能以小写字母开头。可以通过带 `variable_names` 选项调用 `read_term/2`，并把变量绑定到它们的名称，把变量转换为以大写字母开头的原子。借助此功能，`F(x)` 可以变成这类脚本语言中的合法语法。该特性由 Robert van Engelen 建议，属于 SWI-Prolog 特有功能。

### `android` (`bool`, `r`)

如果存在且为 true，表示正在 Android 操作系统上运行。其他操作系统上不存在该标志。

### `android_api` (`integer`, `r`)

如果在 Android 上运行，该标志表示 C 宏 `__ANDROID_API__` 定义的编译期 API Level。在其他操作系统上未定义。这个 API level 不一定与运行设备的 API level 匹配，因为它表示编译时的 API level。

### `answer_write_options` (`term`, `rw`)

交互式顶层使用该标志打印绑定（答案）的值。打印查询答案时，标志值会传给 `write_term/2`。默认值为 `[quoted(true), portray(true), max_depth(10), attributes(portray)]`。

### `atom_normalize_hook` (`bool`, `r`)

当内核原子规范化 hook 处于活动状态时为 `true`。该 hook 由 `PL_atom_normalize_hook()` 注册，并由 `library(unicode)` 使用。读取器会用它在精确 NFC 检查和基于 `wcwidth` 的后备 NFC 检查之间做选择。另请参阅 `unicode_atoms`。

### `apple` (`bool`, `r`)

如果存在且为 `true`，操作系统是 MacOSX。若用于编译该 SWI-Prolog 版本的 C 编译器定义了 `__APPLE__`，则会定义该标志。注意，在 MacOSX 上也会定义 `unix` 标志。

### `apple_universal_binary` (`bool`, `r`)

如果存在且为 `true`，SWI-Prolog 构建为通用二进制（universal binary）。通用二进制包含多个体系结构的原生可执行代码。目前支持的体系结构是 `x86_64` 和 `arm64`。组件的体系结构前缀是 `fat-darwin`，而 `arch` 取决于实际 CPU 类型。

### `arch` (`atom`, `r`)

SWI-Prolog 正在运行的硬件和操作系统标识符。它用于为正确体系结构选择外部文件。另请参阅共享库相关章节和 `file_search_path/2`。在 Apple 平台上，另请参阅 `apple_universal_binary`。

### `argv` (`list`, `rw`)

一个原子列表，表示应用程序命令行参数。应用程序命令行参数是 Prolog 初始化期间未处理的那些参数。注意，Prolog 的参数处理会在 `--` 或第一个非选项参数处停止。另请参阅 `os_argv`。在 6.5.2 之前，`argv` 的定义等同于现在的 `os_argv`。出于兼容性和实用性的原因，后来修改为当前定义。

### `associated_file` (`atom`, `r`)

如果 Prolog 启动时以某个 Prolog 文件作为参数，则设置该标志。例如，`edit/0` 会使用它编辑初始文件。

### `autoload` (`atom`, `rw`)

该标志控制基于 `autoload/1`、`autoload/2` 以及自动加载库（autoload libraries）的谓词自动加载。它有下列取值：

- `false`：谓词永不自动加载。如果谓词之前通过 `autoload/[1,2]` 导入，则立即用 `use_module/[1,2]` 载入引用的文件。注意，很多开发工具（例如 `listing/1`）必须先显式导入，才能在顶层使用。
- `explicit`：不从自动加载库中自动加载，但对通过 `autoload/[1,2]` 导入的谓词使用惰性加载。
- `user`：类似 `false`，但会把库谓词自动加载到全局 `user` 模块。这会让开发工具和库隐式可用于顶层，但不适用于模块。
- `user_or_explicit`：组合 `explicit` 和 `user`，既为通过 `autoload/[1,2]` 导入的谓词提供惰性加载，又让整个库对顶层隐式可访问。
- `true`：在所有地方提供完整自动加载。这是默认值。

### `back_quotes` (`codes,chars,string,symbol_char`, `rw`)

定义反引号内容的项表示形式。默认值为 `codes`。如果给出 `--traditional`，默认值为 `symbol_char`，这允许在由符号组成的运算符中使用反引号。旧版本有一个布尔标志 `backquoted_strings`，用于在 `string` 和 `symbol_char` 之间切换。另请参阅字符串章节。

### `backtrace` (`bool`, `rw`)

如果为 `true`（默认值），在未捕获异常上打印回溯。

### `backtrace_depth` (`integer`, `rw`)

如果启用了错误回溯，该标志定义打印的最大栈帧数。默认值为 20。

### `backtrace_goal_depth` (`integer`, `rw`)

回溯栈帧会在对目标做浅拷贝后打印。该标志决定目标项被拷贝的深度。默认值为 `3`。

### `backtrace_show_lines` (`bool`, `rw`)

如果为 `true`（默认值），尝试重建异常发生处的行号。

### `bounded` (`bool`, `r`)

ISO Prolog 标志。如果为 `true`，整数表示受 `min_integer` 和 `max_integer` 约束。如果为 `false`，整数可以任意大，且 `min_integer` 和 `max_integer` 不存在。标志 `max_integer_size` 可用于强制任意限制，而不是耗尽内存。参见算术类型相关章节。

### `break_level` (`integer`, `r`)

当前 break level。通过 `-t` 启动的初始顶层值为 0。参见 `break/0`。未运行顶层循环的线程没有该标志。

### `build_type` (`atom`, `r`)

该标志表示构建此 SWI-Prolog 实例时的 CMake `CMAKE_BUILD_TYPE`。可能值取决于平台。一些常见值包括 `Debug`、`Release`、`MinSizeRel`、`RelWithDebInfo`、`Sanitize`、`DEB` 或 `PGO`。

### `bundle` (`bool`, `r`)

当 SWI-Prolog 作为独立 bundle 安装时为 true。SWI-Prolog 下载页分发的 Windows 和 MacOS 二进制包都会设置该标志。它用于调整文件搜索配置。

### `c_cc` (`atom`, `rw`)

用于编译 SWI-Prolog 的 C 编译器名称，通常是 `gcc`、`clang` 或 `cc`。参见 `swipl-ld` 相关章节。

### `c_cflags` (`atom`, `rw`)

用于编译 SWI-Prolog 的 CFLAGS。参见 `swipl-ld` 相关章节。

### `c_cxx` (`atom`, `rw`)

用于测试 SWI-Prolog C++ 绑定的 C++ 编译器名称。这也是 `swipl-ld` 使用的默认 C++ 编译器，并用于通过默认设置编译 pack。注意，SWI-Prolog 本身不包含 C++ 代码，C++ 绑定只由头文件组成。因此不会出现 C++ ABI 兼容性问题。

### `c_ldflags` (`atom`, `rw`)

用于链接 SWI-Prolog 的 LDFLAGS。参见 `swipl-ld` 相关章节。

### `c_libplso` (`atom`, `rw`)

把扩展（共享对象或 DLL）链接到 SWI-Prolog 所需的库。在 ELF 系统上通常为空，在基于 COFF 的系统上通常为 `-lswipl`。参见 `swipl-ld` 相关章节。

### `c_libs` (`atom`, `rw`)

链接嵌入 SWI-Prolog 的可执行文件所需的库。如果 SWI-Prolog 内核是共享库（DLL），通常为 `-lswipl`。如果 SWI-Prolog 内核位于静态库中，该标志还包含其依赖项。

### `char_conversion` (`bool`, `rw`)

决定读取项时是否执行字符转换。另请参阅 `char_conversion/2`。

### `character_escapes` (`bool`, `rw`)

如果为 `true`（默认值），`read/1` 会在带引号的原子和字符串中解释 `\` 转义序列。可以修改。该标志局部于修改它的模块。参见字符转义语法章节。

### `character_escapes_unicode` (`bool`, `rw`)

如果为 `true`（默认值），`write/1` 及相关谓词会使用 `\uXXXX` 或 `\UXXXXXXXX` 语法写出转义字符，而不是 ISO Prolog 的 `\x<hex>\` 语法。SWI-Prolog 可以读取两种形式。

### `ci_speedup` (`float`, `rw`)

如果某个哈希用于子句索引时至少达到该标志指定的加速比，则考虑生成该哈希。默认值为 1.5。

### `ci_max_var_fraction` (`float`, `rw`)

如果某个参数在超过该比例的子句中未绑定，则不为该参数创建子句索引哈希表。默认值为 0.1。

### `ci_min_speedup_ratio` (`float`, `rw`)

如果多参数哈希至少达到该值指定的效率，则考虑添加它。默认值为 3.0。

### `ci_max_lookahead` (`integer`, `rw`)

如果找到一个子句，则在子句列表中最多向前扫描该数量的子句，以寻找可能的替代匹配。默认值为 100。

### `ci_min_clauses` (`integer`, `rw`)

如果主索引参数（第一个参数）已实例化，当谓词拥有超过该数量的子句时，仍考虑使用哈希。默认值为 10。

### `cmake_build_type` (`atom`, `ro`)

提供构建此 SWI-Prolog 版本时使用的 CMake build type。

### `colon_sets_calling_context` (`bool`, `ro`)

使用结构 `Module:Goal` 会为执行 `Goal` 设置调用上下文。该标志由 ISO/IEC 13211-2（Prolog 模块标准）定义。参见模块章节。

### `color_term` (`bool`, `rw`)

该标志由库 `library(ansi_term)` 管理。如果下面两个条件都为真，该库会在启动时载入。注意，这意味着从系统或个人初始化文件中把该标志设置为 `false` 会禁用彩色输出。谓词 `message_property/2` 可用于按传给 `print_message/2` 的消息类型控制实际配色方案。

- `stream_property(current_output, tty(true))`
- `\+ current_prolog_flag(color_term, false)`

### `compile_meta_arguments` (`atom`, `rw`)

该标志控制传给标记为 `0` 或 `^` 的元调用的参数如何编译（参见 `meta_predicate/1`）。支持下列取值：

- `false`：默认值。元参数原样传递。如果参数是控制结构（例如 `(A,B)`、`(A;B)`、`(A->B;C)` 等），则在调用元谓词时将其编译为分配在环境栈上的临时子句。
- `control`：把包含控制结构的元参数编译为辅助谓词。这通常能改善性能和调试体验。
- `always`：总是创建中间子句，即使对系统谓词也是如此。将来这可用于把生成谓词的普通头替换为特殊引用，类似 `assert/2` 使用的数据库引用，从而直接访问可执行代码，避免元调用的运行时谓词查找。

### `compiled_at` (`atom`, `r`)

描述系统的编译时间。只有用于编译 SWI-Prolog 的 C 编译器提供 `__DATE__` 和 `__TIME__` 宏时才可用。

### `conda` (`bool`, `r`)

在 Conda 环境中构建时设置为 `true`。

### `console_menu` (`bool`, `r`)

当 I/O 绑定到 Epilog（`swipl-win`）Prolog 控制台时设置为 `true`，表示该控制台支持菜单。另请参阅 Epilog API 章节。

### `cpu_count` (`integer`, `rw`)

系统中的物理 CPU 或核心数量。该标志标记为读写，是为了允许假装系统拥有更多或更少处理器。另请参阅 `thread_setconcurrency/2` 和库 `library(thread)`。如果系统上无法获取 CPU 数量，则该标志不可用。该标志不会包含在保存状态中（参见 `qsave_program/1`）。

### `dde` (`bool`, `r`)

如果该 Prolog 实例支持 DDE，则设置为 `true`。

### `debug` (`bool`, `rw`)

打开或关闭调试模式。如果调试模式已激活，系统会捕捉遇到的 spy point（参见 `spy/1`）和断点。此外，最后调用优化会被禁用，系统在销毁选择点时也会更保守，以简化调试。

禁用这些优化可能导致在关闭调试模式时行为正确的程序耗尽内存。

### `debug_on_error` (`bool`, `rw`)

如果为 `true`，检测到错误后启动 tracer。否则继续执行。引发错误的目标通常会失败。另请参阅 Prolog 标志 `report_error`。默认值为 `true`。

### `debug_on_interrupt` (`bool`, `rw`)

如果为 `true`，在 `Control-C` 上启动调试器。更精确地说，是在收到 `SIGINT` 时启动。初始值为 `false`，进入交互式顶层时会设为 `true`。若要立即开始处理中断，请参阅命令行选项 `--debug-on-interrupt`。

### `debugger_show_context` (`bool`, `rw`)

如果为 `true`，tracer 打印栈帧时显示上下文模块。通常通过 tracer 的 `C` 选项控制。

### `debugger_write_options` (`term`, `rw`)

该参数作为选项列表传给 `write_term/2`，用于调试器打印目标。调试器的 `w`、`p` 和 `<N> d` 命令会修改它。默认值为 `[quoted(true), portray(true), max_depth(10), attributes(portray)]`。

### `determinism_error` (`atom`, `rw`)

该标志定义谓词的确定性与声明不一致时的行为。参见 `det/1`。可能值为 `error`（默认值）、`warning` 和 `silent`。

### `dialect` (`atom`, `r`)

固定为 `swi`。下面的代码是检测 SWI-Prolog 的可靠且可移植方法：

```prolog
is_dialect(swi) :-
    catch(current_prolog_flag(dialect, swi), _, fail).
```

### `dir_sep` (`atom`, `r`)

操作系统文件名中的目录分隔符。通常为 `/`，但 Windows 上为 `\`。

### `double_quotes` (`codes,chars,atom,string`, `rw`)

该标志决定 Prolog 如何读取双引号字符串。和 `character_escapes`、`back_quotes` 一样，它按模块维护。默认值为 `string`，会生成字符串。如果给出 `--traditional`，默认值为 `codes`，会生成字符代码列表，即表示 Unicode 码点的整数。取值 `chars` 会生成单字符原子列表；取值 `atom` 会让双引号与单引号相同，从而创建原子。另请参阅扩展章节。

### `editor` (`atom`, `rw`)

决定 `edit/1` 使用的编辑器。关于选择编辑器的细节，参见自定义编辑器章节。

### `emacs_inferior_process` (`bool`, `r`)

如果为 true，SWI-Prolog 正作为 GNU/X-Emacs 的下级进程运行。如果环境变量 `EMACS` 为 `t` 且 `INFERIOR` 为 `yes`，SWI-Prolog 会假定这一点成立。

### `encoding` (`atom`, `rw`)

以 `text` 模式打开文件时使用的默认编码。初始值从环境推导。详情参见编码章节。

### `executable` (`atom`, `r`)

正在运行的可执行文件路径名。`qsave_program/2` 会把它作为默认仿真器。

### `executable_format` (`atom`, `r`)

SWI-Prolog 可执行文件的格式，例如当 `swipl` 是 ELF 二进制文件时为 `elf`。

### `engines` (`bool`, `r`)

如果支持 engines，则为 true。在多线程版本中总是如此。单线程版本的 SWI-Prolog 也可能启用 engines。

### `exit_status` (`integer`, `r`)

由 `halt/1` 设置为其参数，使注册到 `at_halt/1` 的 hook 可以访问退出状态。

### `file_name_case_handling` (`atom`, `rw`)

该标志定义 Prolog 如何处理文件名大小写。它用于大小写规范化，并用于判断两个名称是否指向同一文件。注意，文件名大小写处理通常是文件系统属性，而 Prolog 只有一个全局标志来决定其文件处理方式。该标志有下列取值：

- `case_sensitive`：文件系统完全区分大小写。Prolog 不执行任何大小写修改或不区分大小写的匹配。这是 Unix 系统上的默认值。
- `case_preserving`：文件系统不区分大小写，但保留用户创建文件时使用的大小写。这是 Windows 系统上的默认值。
- `case_insensitive`：文件系统既不存储也不匹配大小写。在这种情况下，Prolog 会把所有文件名映射为小写。

### `file_name_variables` (`bool`, `rw`)

如果为 `true`（默认值为 `false`），会在接受文件名的内建谓词参数中展开 `$varname` 和 `~`，例如 `open/3`、`exists_file/1`、`access_file/2` 等。谓词 `expand_file_name/2` 可用于展开环境变量和通配符模式。该 Prolog 标志旨在兼容旧版本 SWI-Prolog。

### `file_search_cache_time` (`number`, `rw`)

`absolute_file_name/3` 搜索结果的缓存时间，单位为秒。在该时间限制内，系统会先检查旧搜索结果是否仍满足条件。默认值为 10 秒，通常可以避免编译期间对库文件等进行大多数重复搜索。将该值设为 0 会禁用缓存。

### `float_max` (`float`, `r`)

可表示的最大浮点数。

### `float_max_integer` (`float`, `r`)

能用浮点数精确表示的最大整数。

### `float_min` (`float`, `r`)

大于 0.0 的最小可表示浮点数。另请参阅函数 `nexttoward/2`。

### `float_overflow` (`atom`, `rw`)

取值为 `error`（默认值）或 `infinity`。前者符合 ISO。使用 `infinity` 时，浮点溢出会映射为正或负 `Inf`。参见 IEEE 浮点章节。该标志也影响 `read_term/3` 及相关谓词，使它们把过大的浮点数读为 infinity。

### `float_rounding` (`atom`, `rw`)

定义算术如何舍入为浮点数。已定义取值为 `to_nearest`（默认值）、`to_positive`、`to_negative` 或 `to_zero`。对大多数场景，函数 `roundtoward/2` 是更安全也更快的替代方案。

### `float_undefined` (`atom`, `rw`)

取值为 `error`（默认值）或 `nan`。前者符合 ISO。使用 `nan` 时，未定义操作（例如 `sqrt(-2.0)`）会映射为 `NaN`。参见 IEEE 浮点章节。

### `float_underflow` (`atom`, `rw`)

取值为 `error` 或 `ignore`（默认值）。后者符合 ISO，会把结果绑定为 0.0。

### `float_zero_div` (`atom`, `rw`)

取值为 `error`（默认值）或 `infinity`。前者符合 ISO。使用 `infinity` 时，除以 0.0 会映射为正或负 `Inf`。参见 IEEE 浮点章节。

### `gc` (`bool`, `rw`)

如果为 true（默认值），垃圾回收器处于活动状态。如果为 false，则既不会执行垃圾回收，也不会执行栈移动，即使显式请求也不会执行。可以修改。

### `gc_thread` (`bool`, `r`)

如果为 `true`（启用线程时默认如此），原子和子句垃圾回收会在一个别名为 `gc` 的独立线程中执行。否则，由检测到足够垃圾的线程执行垃圾回收。由于运行这些全局回收器可能耗时较长，使用独立线程可以改善实时行为。可以使用 `set_prolog_gc_thread/1` 控制 `gc` 线程，该谓词要么启用 gc 线程，要么杀死 gc 线程并等待它结束。

### `generate_debug_info` (`bool`, `rw`)

如果为 `true`（默认值），生成可以用 `trace/0`、`spy/1` 等调试的代码。可以用 `--no-debug` 设为 `false`。该标志在源文件内有作用域。很多库使用 `:- set_prolog_flag(generate_debug_info, false)` 来在普通 trace 中隐藏其内部细节。在当前实现中，这只会在谓词上设置一个标志，使子调用对调试器隐藏；该名称预示编译器未来可能进一步变化。

### `gmp_version` (`integer`, `r`)

如果 Prolog 链接了 GMP，该标志给出所用 GMP 库的主版本。另请参阅 GMP 外部接口章节。链接到 LibBF 时不存在该标志。可通过 Prolog 标志 `bounded` 不存在来测试大整数和有理数支持。

### `gui` (`bool`, `r`)

如果 XPCE 存在且可用于图形界面，则设置为 `true`。

### `halt_grace_time` (`float`, `rw`)

`halt/1` 等待其他线程优雅结束的时间。默认值为 1 秒。

### `heartbeat` (`integer`, `rw`)

如果非零，则每 `N` 次推理调用一次 `prolog:heartbeat/0`。`N` 会四舍五入到 16 的倍数。

### `home` (`atom`, `r`)

SWI-Prolog 所认为的 home 目录。SWI-Prolog 使用 home 目录查找启动文件 `<home>/boot.prc`，并查找库目录 `<home>/library`。有些安装可能把体系结构无关文件放在共享 home 中，并同时定义 `shared_home`。系统文件可通过 `absolute_file_name/3` 以 `swi(file)` 的形式找到。参见 `file_search_path/2`。关于该位置如何确定，参见 home 查找章节；关于从命令行设置或报告它，参见 `--home`。

### `integer_rounding_function` (`down,toward_zero`, `r`)

ISO Prolog 标志，描述算术函数 `//` 和 `rem` 的舍入方式。取值取决于所用 C 编译器。

### `iso` (`bool`, `rw`)

启用一些奇特的 ISO 兼容行为，这些行为与正常 SWI-Prolog 行为不兼容。目前它有下列影响：

- 函子 `//2`（浮点除法）总是返回浮点数，即使应用于可以整除的整数。
- 在项的标准序中，所有浮点数都排在所有整数之前。
- 如果 `atom_length/2` 的第一个参数是数字，则产生类型错误。
- 访问静态谓词时，`clause/[2,3]` 会引发权限错误。
- 访问静态谓词时，`abolish/[1,2]` 会引发权限错误。
- 语法更接近 ISO 标准：
  - 在函数式记法和列表记法中，项的优先级必须低于 1000。这意味着作为参数出现的规则和控制结构需要加括号。像 `[a :- b, c].` 这样的项现在必须消歧为 `[(a :- b), c].` 或 `[(a :- b, c)].`。
  - 作为操作数出现的运算符必须加括号。应写作 `X == (-), true.`，而不是 `X == -, true.`。目前这一点还没有完全强制执行。
  - 反斜线转义的换行会按 ISO 标准解释。参见字符转义语法章节。

### `large_files` (`bool`, `r`)

如果存在且为 `true`，表示 SWI-Prolog 编译时启用了大文件支持（large file support，LFS），可以访问大于 2GB 的文件。该标志在 64 位硬件上总是 `true`；在 32 位硬件上，如果配置检测到 LFS 支持，则为 true。注意，特定文件所在的文件系统仍可能限制文件大小。

### `last_call_optimisation` (`bool`, `rw`)

决定是否启用最后调用优化。通常，该标志的值是 `debug` 标志的否定。由于省略最后调用优化可能让程序耗尽栈空间，有时需要在调试期间启用它。

### `libswipl` (`atom`, `rw`)

SWI-Prolog 共享库 `libswipl` 所在路径，即提供 Prolog 的 SWI-Prolog 共享对象。在某些系统上，可以从正在运行的系统可靠确定该路径，这些系统上的标志是只读的。在其他系统上，它是配置的目标安装位置；如果安装被移动，该值可能错误。由于没有跨平台的可靠方式计算该路径，这些平台上的标志为读写。当前，该标志在 Windows 以及提供 `dladdr()` 函数的 POSIX 系统上可靠；Linux 和 MacOS 都提供该函数。

### `linux` (`bool`, `r`)

如果存在且为 `true`，操作系统是某种 Linux。另请参阅 `unix`。

### `malloc` (`atom`, `r`)

在成功识别所用 `malloc()` 实现后设置。当前可能取值为 `tcmalloc` 和 `ptmalloc`。详情参见内存分配章节。

### `max_answers_for_subgoal` (`integer`, `rw`)

限制表中答案数量。原子 `infinite` 会清除该标志。默认情况下，该标志未定义。详情参见制表限制章节。

### `max_answers_for_subgoal_action` (`atom`, `rw`)

当表达到 `max_answers_for_subgoal` 指定的答案数量时采取的动作。支持的取值为 `bounded_rationality`、`error`（默认值）或 `suspend`。

### `max_arity` (`unbounded`, `r`)

ISO Prolog 标志，表示复合项没有最大元数限制。

### `max_char_code` (`integer`, `r`)

支持的最高 Unicode 码点。SWI-Prolog 支持从 0 到该标志值（含）的所有 Unicode 码点。该值遵循 Unicode 标准，当前为 `0x10ffff`。

### `unicode_syntax_version` (`atom`, `r`)

用于构建 SWI-Prolog 源语法分类器的数据的 Unicode 版本，例如 `'17.0.0'`。它驱动 Unicode Prolog 源语法章节中描述的 identifier、layout 和 solo 类。另请参阅 `library(unicode)` 中的 `unicode_version/1`，它报告绑定的 `utf8proc` 数据版本；该版本可能不同，用于规范化、字素分割和 `unicode_property/2`。

### `max_integer` (`integer`, `r`)

如果整数是有界的，这是最大整数值。另请参阅标志 `bounded` 和算术类型章节。

### `max_integer_size` (`integer`, `rw`)

设置这个 tripwire 后，为大整数和有理数分配内存时会限制为给定字节数。最小值为 1,000。未设置时，分配限制由栈限制决定，因为系统无法表示更大的数，也无法表示 `malloc()` 失败。特别是那些可能代表客户端处理任意算术表达式的服务，可以设置该限制以避免资源耗尽。

### `max_procedure_arity` (`integer`, `r`)

谓词的最大元数。尝试定义或调用这样的谓词会产生 `representation_error(max_procedure_arity)` 异常。当前设为 1024。

### `max_rational_size` (`integer`, `rw`)

限制有理数大小，单位为字节。这个 tripwire 可用于发现把 Prolog 标志 `prefer_rationals` 设为 `true` 后产生过大有理数的情况；如果不需要精度，应使用浮点算术。注意，有理数也会被 Prolog 标志 `max_integer_size` 隐式限制。

### `max_rational_size_action` (`atom`, `rw`)

超过 `max_rational_size` tripwire 时采取的动作。可能值为 `error`（默认值），即抛出 tripwire 资源错误；以及 `float`，即把有理数转换为浮点数。注意，有理数可能超过浮点数范围。

### `max_table_answer_size` (`integer`, `rw`)

限制制表中答案替换的大小。原子 `infinite` 会清除该标志。默认情况下，该标志未定义。详情参见制表限制章节。

### `max_table_answer_size_action` (`atom`, `rw`)

如果向表中添加大于 `max_table_answer_size` 的答案替换，系统采取的动作。支持的取值为 `error`（默认值）、`bounded_rationality`、`suspend` 和 `fail`。

### `max_table_subgoal_size` (`integer`, `rw`)

限制访问表的目标项大小。原子 `infinite` 会清除该标志。默认情况下，该标志未定义。详情参见制表限制章节。

### `max_table_subgoal_size_action` (`atom`, `rw`)

如果制表目标超过 `max_table_subgoal_size`，系统采取的动作。支持的取值为 `error`（默认值）、`abstract` 和 `suspend`。

### `max_tagged_integer` (`integer`, `r`)

可表示为 tagged 值的最大整数。Tagged 整数需要一个字的存储空间。更大的整数表示为间接数据（indirect data），需要显著更多空间。

### `message_context` (`list(atom)`, `rw`)

要添加到 `error` 和 `warning` 级别消息中的上下文信息。该列表可以包含元素 `thread`，用于把生成消息的线程加入消息；也可以包含 `time` 或 `time(Format)`，用于添加时间戳。默认时间格式为 `%T.%3f`。默认值为 `[thread]`。另请参阅 `format_time/3` 和 `print_message/2`。

### `min_integer` (`integer`, `r`)

如果整数是有界的，这是最小整数值。另请参阅标志 `bounded` 和算术类型章节。

### `min_tagged_integer` (`integer`, `r`)

Tagged integer 值范围的起点。

### `mitigate_spectre` (`bool`, `rw`)

当为 `true`（默认值为 `false`）时，强制缓解基于时间的 Spectre 安全漏洞。基于 Spectre 的攻击可以从进程拥有但本应保持不可见的内存中提取信息，例如密码或 Web 服务器的私钥。这类攻击通过导致对敏感数据的推测性访问，并通过连续指令耗时差异等旁路泄漏数据。一个可能受影响的应用示例是 SWISH：它允许用户运行 Prolog 代码，而 SWISH 服务器必须同时保护其他用户的隐私以及 HTTPS 私钥、cookie 和密码。

目前，启用该标志会把 `get_time/1` 和 `statistics/2` CPU 时间的分辨率降低到 20 微秒。

**警告**：虽然更粗粒度的计时器会让这类攻击更难成功，但通常不能可靠防止此类攻击。完整缓解可能需要编译器支持，以禁用对敏感数据的推测性访问。

### `msys2` (`bool`, `r`)

如果存在，表示 SWI-Prolog 是在 MSYS2 shell 下运行的 MS-Windows 版本。

### `occurs_check` (`atom`, `rw`)

该标志控制会创建无限树（也称循环项）的合一，并可取三个值。使用 `false`（默认值）时，合一成功并创建无限树。使用 `true` 时，合一行为类似 `unify_with_occurs_check/2`，静默失败。使用 `error` 时，尝试创建循环项会导致 `occurs_check` 异常。后者用于调试无意创建循环项的情况。注意，这是一个全局标志，会修改 Prolog 的基础行为。把该标志从默认值改掉，可能导致库无法正常工作。

### `on_error` (`atom`, `rw`)

决定如何处理用 `print_message/2` 打印的错误，即报告给用户的错误。可能值为 `print`（默认值）、`status` 和 `halt`。使用 `halt` 时，进程立即以状态 1 停止。否则继续执行。使用 `status` 时，如果进程打印过一个或多个错误，`halt/0` 会以状态 1 退出。在编译模式（参见 `-c`）中，默认值为 `status`。该标志可通过命令行选项 `--on-error` 设置。另请参阅编译消息章节。

### `on_warning` (`atom`, `rw`)

类似 `on_error`，但用于警告。默认值始终为 `print`。对应命令行选项为 `--on-warning`。

### `open_shared_object` (`bool`, `r`)

如果为 true，`open_shared_object/2` 及相关谓词已实现，可访问共享库（`.so` 文件）或动态链接库（`.DLL` 文件）。

### `optimise` (`bool`, `rw`)

如果为 `true`，以优化模式编译。若 Prolog 以命令行选项 `-O` 启动，初始值为 `true`。`optimise` 标志具有源文件作用域。

当前，优化编译意味着编译算术表达式，并删除可能由 `expand_goal/2` 产生的冗余 `true/0`。

未来版本可能包含其他优化，例如把小谓词集成到调用方、消除常量表达式和其他可预测结构。源代码优化从不应用于声明为动态的谓词（参见 `dynamic/1`）。

### `optimise_unify` (`bool`, `rw`)

如果为 `true`（默认值），允许编译器移动或移除显式合一调用（`=/2`）。虽然这能显著提升性能，但源级调试器尚不能正确处理这种行为。参见函数体索引章节。

### `os_argv` (`list`, `rw`)

一个原子列表，表示用于调用 SWI-Prolog 的命令行参数。注意，返回的列表包含所有参数。若要获取应用程序选项，请参阅 `argv`。

### `packs` (`bool`, `r`)

如果为 `true`，扩展包（add-ons）已附加。可以使用 `--no-packs` 设为 `false`。

### `path_max` (`integer`, `r`)

操作系统报告的文件路径名最大长度。这个长度通常不直接定义文件名字符数。实际限制可能因编码而更短；例如 POSIX 系统上，它通常定义经常为 UTF-8 编码的名称的长度限制。底层文件系统也可能施加额外限制。

### `path_sep` (`atom`, `r`)

操作系统中文件搜索路径的分隔符，例如环境变量 `PATH` 使用的分隔符。通常为 `:`，但 Windows 上为 `;`。

### `pid` (`int`, `r`)

正在运行的 Prolog 进程的进程标识符。该标志是否存在由实现定义。

### `pipe` (`bool`, `rw`)

如果为 true，支持 `open(pipe(command), mode, Stream)` 等形式。可以修改，以便在测试该特性的应用程序中禁用 pipe。不推荐这样做。

### `portable_vmi` (`bool`, `rw`)

如果为 `true`（默认值），生成可同时在 32 位和 64 位硬件上运行的 `.qlf` 文件和保存状态。如果为 `false`，某些优化的虚拟机指令只有在整数参数位于 32 位机器 tagged integer 范围内时才使用。

### `posix_shell` (`atom`, `rw`)

POSIX 兼容 shell 的路径。默认通常为 `/bin/sh`。该标志由 `shell/1` 和 `qsave_program/2` 使用。

### `prefer_rationals` (`bool`, `rw`)

只有在系统编译时支持无界和有理数算术时才提供（参见 `bounded`）。如果为 `true`，算术会优先产生有理数而非浮点数。这意味着：

- 两个整数相除（函数 `/2`）会产生有理数。
- 两个整数求幂（函数 `^/2`）会产生有理数，即使第二个操作数为负。例如，`2^(-2)` 求值得到 `1/4`。

使用 `true` 可能创建过大的有理数。Prolog 标志 `max_rational_size` 可用于检测并处理这个 tripwire。

如果为 `false`，有理数只能通过函数 `rational/1`、`rationalize/1`、`rdiv/2` 创建，或通过读取创建。另请参阅 `rational_syntax`、有理数语法章节和有理数章节。

当前默认值为 `false`。未来可能改为 `true`。强烈建议用户把该标志设为 `true`，并报告因此产生的问题。

### `print_write_options` (`term`, `rw`)

指定 `print/1` 和 `print/2` 使用的 `write_term/2` 选项。

### `prompt_alternatives_on` (`atom`, `rw`)

决定 Prolog 顶层如何提示替代答案。默认值为 `determinism`，表示如果目标成功但留下选择点，系统会提示替代答案。许多经典 Prolog 系统表现为 `groundness`：当且仅当查询包含变量时提示替代答案。

### `protect_static_code` (`bool`, `rw`)

如果为 `true`（默认值为 `false`），`clause/2` 不操作静态代码，从而对想列出 Prolog 程序静态代码的攻击者提供一些基本防护。一旦该标志为 `true`，就不能改回 `false`。ISO 模式默认启用保护（参见 Prolog 标志 `iso`）。注意，开发环境的很多部分要求 `clause/2` 能操作静态代码，因此启用该标志应只用于生产代码。

### `qcompile` (`atom`, `rw`)

该选项为 `load_files/2` 的 `qcompile(+Atom)` 选项提供默认值。

### `rational_syntax` (`atom`, `rw`)

决定有理数的读写语法。可能值为 `natural`（例如 `1/3`）或 `compatibility`（例如 `1r3`）。`compatibility` 语法总是被接受。该标志对模块敏感。

当前默认值为 `compatibility`，它会把有理数读写为例如 `1r3`。关于分隔字符仍有一些讨论，参见有理数语法章节。未来可能考虑把默认值改为 `natural`。强烈建议用户把该标志设为 `natural`，并报告因此产生的问题。

### `rationals` (`atom`, `r`)

如果系统支持有理数，则该标志存在且值为 `true`。对 SWI-Prolog 来说，如果标志 `bounded` 为 `false`，该标志总会设置。

### `readline` (`atom`, `rw`)

指定提供哪种命令行编辑形式。可能值如下：

- `false`：没有可用的命令行编辑。
- `editline`：载入库 `library(editline)`，提供基于 BSD libedit 的行编辑。如果 `library(editline)` 可用，这是默认值。

### `report_error` (`bool`, `rw`)

如果为 `true`，打印错误消息；否则抑制它们。可以修改。另请参阅 Prolog 标志 `debug_on_error`。默认值为 `true`，运行时版本除外。

### `resource_database` (`atom`, `r`)

设置为附加状态的绝对文件名。通常是文件 `boot32.prc`、通过 `-x` 指定的文件，或正在运行的可执行文件。另请参阅 `resource/3`。

### `runtime` (`bool`, `r`)

如果存在且为 `true`，SWI-Prolog 以 `-DO_RUNTIME` 编译，会禁用各种有用的开发功能（目前包括 tracer 和 profiler）。

### `sandboxed_load` (`bool`, `rw`)

如果为 `true`（默认值为 `false`），`load_files/2` 会调用 hook，使 `library(sandbox)` 能验证指令的安全性。

### `saved_program` (`bool`, `r`)

如果存在且为 `true`，表示 Prolog 是从使用 `qsave_program/[1,2]` 保存的状态启动的。

### `shared_home` (`atom`, `r`)

表示部分 SWI-Prolog 系统文件安装在 `<prefix>/share/swipl`，而不是 home 下的 `<prefix>/lib/swipl`。该标志表示这个共享 home 的位置，并且该目录会加入文件搜索路径 `swi`。参见 `file_search_path/2` 和标志 `home`。

### `shared_object_extension` (`atom`, `r`)

操作系统用于共享对象的扩展名。多数 Unix 系统为 `.so`，Windows 为 `.dll`。它用于通过 `file_type` `executable` 定位文件。另请参阅 `absolute_file_name/3`。

### `shared_object_search_path` (`atom`, `r`)

系统搜索共享对象时使用的环境变量名称。

### `shared_table_space` (`integer`, `rw`)

为存储共享答案表保留的空间。参见共享制表章节和 Prolog 标志 `table_space`。

### `shift_check` (`bool`, `rw`)

当为 `true`（默认值为 `false`）时，检查由 `shift_for_copy/1` 捕获的可疑定界延续。

### `signals` (`bool`, `r`)

决定 Prolog 是否处理信号（软件中断）。如果宿主操作系统不支持信号处理，或命令行选项 `--no-signals` 处于活动状态，该标志为 `false`。详情参见嵌入式信号章节。

### `source` (`bool`, `rw`)

如果为 `true`，在存在对应 `.pl` 文件时忽略 `.qlf` 文件。库中提供的 `.qlf` 文件是在启用优化（参见 `optimise`）、启用宏展开（参见 `library(apply_macros)`）并移除 `debug/3` 和 `assertion/1` 语句的情况下编译的。使用该标志会载入源代码，从而更好地支持调试。如果某个调试会话能从更好地访问库调试设施中受益，可以在程序载入文件开头设置该 Prolog 标志，或这样启动 Prolog：

```shell
swipl -Dsource [option ...] myfile.pl ...
```

### `source_search_working_directory` (`bool`, `rw`)

如果设为 `true`，从源代码载入相对文件名时，会同时相对源文件所在位置和工作目录搜索。相对工作目录搜索已弃用；如果文件以这种方式找到，会打印警告。未来版本可能把默认值改为 `false`。搜索工作目录一直支持到 9.3.8。9.3.9 禁用了该行为，9.3.10 又带着警告重新启用了它。

### `stack_limit` (`int`, `rw`)

限制当前线程的 Prolog 栈组合大小。另请参阅 `--stack-limit` 和内存限制章节。

### `stream_type_check` (`atom`, `rw`)

定义系统是否以及多严格地验证：字节 I/O 不应作用于文本流，文本 I/O 不应作用于二进制流。取值为 `false`（不检查）、`true`（完整检查）和 `loose`。使用 `loose`（默认值）检查模式时，系统接受从使用 ISO Latin-1 编码的文本流进行字节 I/O，也接受向二进制流写入文本。

### `string_stack_tripwire` (`int`, `rw`)

用于外部语言字符串管理的维护标志。如果字符串栈深度达到 tripwire 值，则打印警告。详情参见外部接口字符串章节。

### `system_thread_id` (`int`, `r`)

在多线程版本中可用，前提是操作系统提供系统范围的整数线程标识符。该整数是操作系统用于调用线程的线程标识符。在 Linux 系统上，这是该线程的 PID。

### `table_incremental` (`bool`, `rw`)

设置是否使用增量制表的默认值。初始值为 `false`。参见 `table/1`。

### `table_shared` (`bool`, `rw`)

设置是否使用共享制表的默认值。初始值为 `false`。参见 `table/1`。

### `table_space` (`integer`, `rw`)

为存储制表谓词的答案表保留的空间（参见 `table/1`）。目前只计算答案 trie 中节点占用的空间。超过该空间时，会引发 `resource_error(table_space)` 异常。

### `table_subsumptive` (`bool`, `rw`)

设置 variant 制表和 subsumptive 制表之间选择的默认值。初始值为 `false`。参见 `table/1`。

### `threads` (`bool`, `rw`)

当支持线程时为 true。如果系统编译时没有线程支持，值为 `false` 且只读。否则，除非系统以 `--no-threads` 启动，值为 `true`。只有在线程尚未运行时才能禁用线程。另请参阅 `gc_thread` 标志。

### `timezone` (`integer`, `r`)

当前时区相对 GMT 向西偏移的秒数。初始化时从与 POSIX `tzset()` 函数关联的 `timezone` 变量设置。另请参阅 `format_time/3`。

### `tmp_dir` (`atom`, `rw`)

临时目录路径。从环境变量 `TMP` 或 `TEMP` 初始化。在 Windows 上使用这些变量；如果未定义，则使用默认值。默认值通常是 `/tmp`，Windows 上通常是 `c:/temp`。

### `toplevel_goal` (`term`, `rw`)

定义运行初始化目标和入口点后执行的目标（参见 `-g`、`initialization/2` 和 PrologScript 章节）。初始值为 `default`，表示启动普通交互式会话。该值可以用命令行选项 `-t` 修改。显式值 `prolog` 等价于 `default`。如果使用 `initialization(Goal,main)` 且顶层为 `default`，顶层会设为 `halt`（参见 `halt/0`）。

### `toplevel_list_wfs_residual_program` (`bool`, `rw`)

如果为 `true`（默认值），且答案根据良基语义（Well Founded Semantics，WFS）为 undefined，则在答案前列出 residual program。否则答案以 **undefined** 终止。另请参阅 `undefined/0`。

### `toplevel_mode` (`atom`, `rw`)

如果为 `backtracking`（默认值），顶层会在完成查询后回溯。如果为 `recursive`，顶层实现为递归循环。这意味着使用 `b_setval/2` 设置的全局变量会在查询之间保持。在 `recursive` 模式下，顶层变量的答案（参见“复用顶层绑定”）保存在可回溯全局变量中，因此**不会被复制**。在 `backtracking` 模式下，顶层变量答案保存在 recorded database 中。

递归模式是为交互式使用 CHR 添加的，因为 CHR 会把全局约束存储保存在可回溯全局变量中。该建议来自 Falco Nogatz。

### `toplevel_name_variables` (`bool`, `rw`)

如果为 `true`（默认值），在顶层为变量命名，而不是把它们打印为 `_NNN`。变量会命名为 `_A`、`_B` 等。只出现一次的变量（singleton）会打印为 `_`。

### `toplevel_print_anon` (`bool`, `rw`)

如果为 `true`，以下划线（`_`）开头的顶层变量会正常打印。如果为 `false`（默认值），这类变量的绑定会从答案中省略。可用于在复杂查询中从顶层隐藏绑定。例如，下面 `_List` 的绑定不会打印：

```prolog
?- numlist(1,1 000 000,_List), sum_list(_List, Sum).
Sum = 500000500000.
```

### `toplevel_print_factorized` (`bool`, `rw`)

如果为 `true`（默认值为 `false`），显示答案替换中子项的内部共享。下面的示例揭示了由 `library(rbtrees)` 谓词 `rb_new/1` 实现的红黑树中叶节点的内部共享：

```prolog
?- set_prolog_flag(toplevel_print_factorized, true).
?- rb_new(X).
X = t(_S1, _S1), % where
    _S1 = black('', _G387, _G388, '').
```

如果该标志为 `false`，`% where` 记法仍用于表示循环，如下例所示。该示例还显示，实现揭示的是内部循环长度，而不是最小循环长度。在 Prolog 中，不同长度的循环无法区分，`S == R` 就说明了这一点。

```prolog
?- S = s(S), R = s(s(R)), S == R.
S = s(S),
R = s(s(R)).
```

### `toplevel_prompt` (`atom`, `rw`)

定义交互式顶层使用的提示符。下面的 `~`（tilde）序列会被替换：

| 序列 | 替换内容 |
| --- | --- |
| `~m` | 如果不是 `user`，替换为输入模块（type-in module，参见 `module/1`） |
| `~l` | 如果不为 0，替换为 break level（参见 `break/0`） |
| `~d` | 如果不是普通执行，替换为调试状态（参见 `debug/0`、`trace/0`） |
| `~!` | 如果启用了历史，替换为历史事件（参见标志 `history`） |

### `toplevel_residue_vars` (`bool`, `rw`)

当为 `true`（默认值为 `false`）时，打印由 `call_residue_vars/2` 检测到、但未出现在目标返回绑定中的 residual variables。

### `toplevel_thread` (`bool`, `rw`)

当为 `true` 时，该线程正在运行顶层 REPL 循环。参见 `prolog/0`。

### `toplevel_var_size` (`int`, `rw`)

在顶层查询中作为变量绑定返回、并保存下来以便通过 `$` 变量引用复用的项，其最大大小（按 literal 计）。当为 0 时，变量记录和复用被禁用。参见“复用顶层绑定”。

### `trace_gc` (`bool`, `rw`)

如果为 `true`（默认值为 `false`），垃圾回收和栈移动会在终端上报告。可以修改。值以字节为单位报告为 `G+T`，其中 `G` 是全局栈值，`T` 是 trail 栈值。`Gained` 描述回收的字节数。`used` 是 GC 后栈上使用的字节数；`free` 是已分配但未使用的字节数。下面是输出示例：

```text
% GC: gained 236,416+163,424 in 0.00 sec;
      used 13,448+5,808; free 72,568+47,440
```

### `traditional` (`bool`, `r`)

在 SWI-Prolog 7 中可用。如果为 `true`，表示使用 `--traditional` 选择了 traditional 模式。注意，某些 SWI7 特性（例如 dict 上的函数式记法）在该模式下不可用。另请参阅扩展章节。

### `tty_control` (`bool`, `rw`)

决定终端是否切换到 raw 模式以支持 `get_single_char/1`，该谓词也读取 trace 中的用户操作。可以设置。如果该标志在启动时为 `false`，命令行编辑会被禁用。另请参阅命令行选项 `--no-tty`。

### `unicode_atoms` (`atom`, `rw`)

新打开文本流的默认原子内容策略；可通过 `set_stream/2` 按流覆盖，也可通过 `open/4` 的 `unicode_atoms` 选项覆盖，还可通过 `read_term/2,3` 和 `read_clause/2,3` 的 `unicode_atoms` 选项按调用覆盖。四个取值为 `accept`、`nfc`、`error`、`reject`，其效果见 `read_term/2,3`。默认值为 `accept`。

把该标志设为 `nfc` 时，如果尚未注册内核规范化 hook，会自动载入 `library(unicode)`；如果该库不可用，`set_prolog_flag/2` 调用会传播 `use_module/1` 引发的 `existence_error(source_sink, library(unicode))`。模式 `error` 不需要该 hook：当 `atom_normalize_hook` 为 `false` 时，它会回退到基于 wcwidth 的检查，把任何 wcwidth 小于 1 的码点（组合标记、零宽和不可打印字符）视为非 NFC。这可能过度拒绝泰语等在 NFC 中使用组合标记的文字系统。

不管该标志如何，SWI-Prolog 读取器都会无条件拒绝 Unicode bidi override 和 isolate 码点（U+202A 到 U+202E，以及 U+2066 到 U+2069），如果它们作为原始字节出现在未加引号的原子、运算符、变量、带引号的原子、字符串或注释中。这缓解了 “Trojan source” 攻击（CVE-2021-42574）。如果程序需要在字面量原子或字符串中使用这类码点，必须使用相应的转义序列，例如 `\u202E`。

### `unix` (`bool`, `r`)

如果存在且为 `true`，操作系统是某种 Unix。如果用于编译该 SWI-Prolog 版本的 C 编译器定义了 `__unix__` 或 `unix`，则定义该标志。其他系统上不可用。另请参阅 `linux`、`apple` 和 `windows`。

### `unknown` (`fail,warning,error`, `rw`)

决定遇到未定义过程时的行为。如果为 `fail`，谓词静默失败。如果为 `warn`，打印警告，并像谓词未定义一样继续执行。如果为 `error`（默认值），引发 `existence_error` 异常。该标志局部于每个模块，并从模块的 import-module 继承。使用默认设置时，这意味着普通模块从 `user` 继承该标志，而 `user` 又从 `system` 继承值 `error`。用户可以修改模块 `user` 的标志，从而改变所有应用程序模块的默认值，也可以修改特定模块的值。强烈建议保留 `error` 默认值，并使用 `dynamic/1` 和/或 `multifile/1` 指定谓词可能不存在。

### `unknown_option` (`ignore,warning,error`, `rw`)

决定处理选项列表的谓词收到不认识的选项时的行为。ISO 标准要求引发 `domain_error` 异常。但这被认为不实用：如果不同 Prolog 系统支持不同选项，它会让编写可移植代码变得困难；也会让那些处理选项并把一部分选项传给一个谓词、另一部分选项传给另一个谓词的谓词难以编写。例如，一个把文件读入项列表的谓词必须把选项分发给 `open/4` 和 `read_term/3`。除 ISO 模式外，SWI-Prolog 一直忽略未知选项（参见 `iso` 标志）。该标志提供对选项处理方式的完整控制。

### `unload_foreign_libraries` (`bool`, `rw`)

如果为 `true`（默认值为 `false`），卸载所有已载入的外部库。默认值为 `false`，因为现代操作系统无论如何都会回收资源，而且卸载外部代码可能导致已注册的 hook 指向不再存在的数据或代码。

### `user_flags` (`Atom`, `rw`)

定义 `set_prolog_flag/2` 在标志未知时的行为。取值为 `silent`、`warning` 和 `error`。前两个取值会即时创建标志，其中 `warning` 会打印消息。取值 `error` 与 ISO 一致：它引发存在性错误，并且不创建标志。另请参阅 `create_prolog_flag/3`。默认值为 `silent`，但未来版本可能改变这一点。鼓励开发者使用其他值，并确保为库使用 `create_prolog_flag/3` 正确创建标志。

### `var_prefix` (`bool`, `rw`)

如果为 `true`（默认值为 `false`），变量必须以下划线（`_`）开头。可以修改。该标志局部于修改它的模块。参见变量前缀章节。

### `var_tag` (`Atom`, `rw`)

该标志控制当 `Tag{...}` 中的 `Tag` 未绑定时如何解释。可能值为：

- `dict`：读作带未绑定 tag 的 dict（参见双向 dict 章节）。这是当前默认值。使用未绑定 tag 已弃用。适当时候默认值会改变，最终改为 `attvar`。参见 dict 兼容性章节。
- `attvar`：把 `Var{...}` 读作 attributed variable。这很可能成为未来默认值。
- `#`：把 `Var{...}` 读作 `#{...}`。该标志允许快速评估使用 `#{...}` 而不是带未绑定 tag 的 dict 会对代码造成什么影响。
- `warning`：类似 `#`，但打印警告。
- `error`：把 `Var{...}` 视为语法错误。

### `verbose` (`atom`, `rw`)

该标志由 `print_message/2` 使用。如果值为 `silent`，类型为 `informational` 和 `banner` 的消息会被抑制。命令行选项 `-q` 会把初始值 `normal` 切换为 `silent`。

### `verbose_autoload` (`bool`, `rw`)

如果为 `true`，自动加载库时会打印普通 consult 消息。默认会抑制该消息。该标志用于调试。

### `verbose_file_search` (`bool`, `rw`)

如果为 `true`（默认值为 `false`），打印消息说明 `absolute_file_name/[2,3]` 定位文件的进展。用于调试复杂文件搜索路径。另请参阅 `file_search_path/2`。

### `verbose_load` (`atom`, `rw`)

决定载入（编译）Prolog 文件时打印哪些消息。当前取值为 `full`（每个文件载入开始和结束时打印消息）、`normal`（每个文件载入结束时打印消息）、`brief`（顶层文件载入结束时打印消息）和 `silent`（不打印消息，默认值）。该标志的值通常由 `load_files/2` 提供的 `silent(Bool)` 选项控制。

### `version` (`integer`, `r`)

版本标识符是一个整数，其值为：

```text
10000 * Major + 100 * Minor + Patch
```

### `version_data` (`swi(Major, Minor, Patch, Extra)`, `r`)

方言兼容层的一部分；另请参阅 Prolog 标志 `dialect` 和方言章节。`Extra` 以列表形式提供平台特定版本信息。`Extra` 用于 “7.4.0-rc1” 这样的 tagged version，在这种情况下，`Extra` 包含项 `tag(rc1)`。

### `version_git` (`atom`, `r`)

如果系统是从 git 仓库创建的，则可用。详情参见 `git-describe`。

### `vmi_builtin` (`bool`, `rw`)

决定 `true/0`、`atom/1` 等知名内建谓词是否通过转换为虚拟机代码来处理。除非启用调试模式，该标志默认值为 `true`。把它设为 `false` 可能改善其他运行时 instrumentation 的结果。注意，优化算术（`-O`，参见 Prolog 标志 `optimise`）目前不会转换为普通谓词调用。

### `warn_autoload` (`bool`, `rw`)

如果为 `true`（默认值为 `false`），当从定义全局项展开或目标展开规则的文件自动加载谓词时发出警告。这些规则通常能提升性能或提供更清晰语义，因此不建议自动加载。未来版本将默认启用该标志。

### `warn_override_implicit_import` (`bool`, `rw`)

如果为 `true`（默认值），当隐式导入的谓词被本地定义覆盖时打印警告。详情参见 `use_module/1`。

### `win_file_access_check` (`atom`, `rw`)

控制 Windows 下 `access_file/2` 的行为。在 Windows 上没有可靠方法检查文件和目录访问权限。该标志允许在三种近似方案之间切换：

- `access`：使用 Windows `_waccess()` 函数。它会忽略 ACL（Access Control List），因此可能在实际不允许访问时指示访问被允许。
- `getfilesecurity`：使用 Windows `GetFileSecurity()` 函数。它并非在所有文件系统上都可用，但在支持它的文件系统上可能是最佳选择，尤其是本地 NTFS 卷。
- `openclose`：尝试打开并关闭文件。这对文件可靠，但对目录不可靠。目前目录用 `_waccess()` 检查。这是默认值。

### `windows` (`bool`, `r`)

如果存在且为 `true`，操作系统是 Microsoft Windows 的某种实现。该标志只在基于 MS-Windows 的版本上可用。另请参阅 `unix`。

### `wine_version` (`atom`, `r`)

如果存在，表示 SWI-Prolog 是在 Wine 仿真器下运行的 MS-Windows 版本。

### `write_attributes` (`atom`, `rw`)

定义 `write/1` 及相关谓词如何写出 attributed variables。选项值随 `write_term/2` 的 `attributes` 选项一起说明。默认值为 `ignore`。

### `write_help_with_overstrike` (`bool`, `r`)

`help/1` 写入终端时使用的内部标志。如果存在且为 `true`，它会使用 overstrike 打印粗体和下划线文本。

### `xdg` (`bool`, `r(w)`)

该标志定义是否遵循 Free Desktop 标准来处理应用程序数据和配置文件。非 Windows 系统上，该标志为 `true` 且只读。在 Windows 上，如果是在 Conda 或 MSYS2 下编译，该标志为 `true` 但可读写；否则未定义。在 Windows 上，搜索顺序如下：

- 标志未定义：先搜索 Windows 目录，再搜索 XDG 目录。这是 Windows 二进制包的默认值。
- 标志为 `true`：只搜索 XDG 目录。
- 标志为 `false`：只搜索 Windows 目录。

### `xpce` (`bool`, `r`)

如果 XPCE 图形系统已载入，则可用并设置为 `true`。

### `xpce_version` (`atom`, `r`)

如果 XPCE 系统已载入，则可用并设置为其版本。

### `xref` (`bool`, `rw`)

如果为 `true`，源代码是为了分析目的读取，例如交叉引用。否则（默认值）源代码是为了编译而读取。该标志在若干地方由 `term_expansion/2` 和 `goal_expansion/2` hook 使用，尤其是这些 hook 带有副作用时。另请参阅库 `library(prolog_source)` 和 `library(prolog_xref)`。

## `set_prolog_flag/2`

`set_prolog_flag(:Key, +Value)` 设置一个 Prolog 标志的值。`Key` 是原子。如果该标志是系统定义的标志，且未在上文标记为可变，尝试修改它会产生 `permission_error`。如果提供的 `Value` 与该标志类型不匹配，会引发 `type_error`。

有些标志（例如 `unknown`）按模块维护。目标模块由 `Key` 这个 meta 参数决定。

除了 ISO 规定的行为外，SWI-Prolog 允许用户定义 Prolog 标志。新的 Prolog 标志应使用 `create_prolog_flag/3` 创建。出于历史原因，如果 Prolog 标志 `user_flags` 为 `true`（默认值），`set_prolog_flag/2` 会静默创建 Prolog 标志；也就是说，`set_prolog_flag/2` 的行为类似下面这样：

```prolog
set_prolog_flag(Key, Value) :-
    current_prolog_flag(Key, _),
    !,
    <set the flag>.
set_prolog_flag(Key, Value) :-
    current_prolog_flag(user_flags, true),
    !,
    create_prolog_flag(Key, Value, []).
set_prolog_flag(Key, _) :-
    existence_error(prolog_flag, Key).
```

## `create_prolog_flag/3`

`create_prolog_flag(+Key, +Value, +Options)` 创建新的 Prolog 标志。ISO 标准没有预见新标志的创建，但许多库都会引入新标志。在多线程环境中，Prolog 标志特别适合管理会变化的全局设置。谓词要么局部于某个线程，要么在所有线程之间共享；而线程会从创建它的线程继承标志（参见 `thread_create/3`），之后的修改则局部于调用线程。

SWI-Prolog 标志有类型。如果没有用 `type(Type)` 选项显式定义类型，类型会从初始值确定。已定义类型包括：`boolean`（如果初始值是 `false`、`true`、`on` 或 `off` 之一）、`atom`（如果初始值是其他原子）、`integer`（如果该值是可表示为 64 位有符号值的整数）。任何其他初始值都会产生无类型标志，可以表示任何有效 Prolog 项。

默认情况下，新标志会加入全局标志表，使尚未设置该标志的所有线程都能访问该值。如果该标志已在调用线程中局部定义，则调用线程和全局标志表中的值都会更新。参见 `local(+Boolean)` 选项。

`Options` 是下列选项的列表。另请参阅 Prolog 标志 `user_flags`。

- `access(+Access)`：定义该标志的访问权限。取值为 `read_write` 和 `read_only`。默认值为 `read_write`。
- `type(+Atom)`：定义类型限制。可能值为 `boolean`、`atom`、`oneof(ListOfAtoms)`、`integer`、`float` 和 `term`。默认值由初始值决定。注意，`term` 会把项限制为 ground。
- `keep(+Boolean)`：如果为 `true`，当标志已存在时不修改它。否则（默认值），如果标志已存在，该谓词行为类似 `set_prolog_flag/2`。

  如果标志已有值，但该值与指定类型不兼容，系统会打印警告，并把标志设置为本次 `create_prolog_flag/3` 调用指定的值和类型。

- `local(+Boolean)`：如果为 `true`（默认值为 `false`），并且该标志不存在，则只在调用线程中创建它。该标志只对调用线程以及从调用线程继承的线程可见。
- `warn_not_accessed(+Boolean)`：如果为 `true`，且该标志从未用 `current_prolog_flag/2` 读取，则打印警告。该选项用于通过命令行选项 `-D<flag>[=<value>]` 设置的选项。

## `push_prolog_flag/2`

`push_prolog_flag(:Key, +Value)` 保存当前线程局部的 `Key` 值，并把它设置为 `Value`。如果 `Key` 不存在，则创建它（仅在调用线程中），并且被压入的状态会记录其原本不存在。该操作可嵌套，并与 `pop_prolog_flag/1` 配对。

这些谓词主要面向两个用例。第一个是使用不同标志进行有作用域的编译。例如，下面的代码会保留子句的书写形式。如果不局部修改该标志，`X = 42` 通常会被移入头部。

```prolog
:- push_prolog_flag(optimise_unify, false).
p(X) :-
    X = 42,
    format('The answer to the ultimate question~n').
:- pop_prolog_flag(optimise_unify).
```

第二个用例是运行时作用域。例如，执行一个带 occurs check 的目标：

```prolog
call_with_occurs_check(Goal) :-
    setup_call_cleanup(
        push_prolog_flag(occurs_check, true),
        Goal,
        pop_prolog_flag(occurs_check)).
```

注意，`pop` 会在 `Goal` 完成时发生。尤其是当 `Goal` 成功但留下选择点时，它不会立即执行。

## `pop_prolog_flag/1`

`pop_prolog_flag(:Key)` 恢复与之匹配的 `push_prolog_flag/2` 保存的状态。如果在匹配的 push 时 `Key` 不存在，则再次移除它。如果当前线程上不存在匹配的 push，则引发 `existence_error(pushed_flag, Key)`。
