# 加载和运行项目

大多数 Prolog 程序会拆分到多个文件中，这些文件组织在一个目录里，也可以再分到多个子目录中。通常，所有文件都是 Prolog 模块（module）文件，参见模块章节。这个目录通常包含一个文件，常命名为 `load.pl`，它会使用 `use_module/[1,2]` 载入所有其他文件（模块）；对于不使用模块的项目，则使用 `ensure_loaded/1`。

如果项目是应用程序，而不是库，则有几种启动方式。一种选择是使用命令行选项 `-g goal`。经典 Prolog 方式是使用 `initialization/1` 指令。后一种方式的问题在于，这类指令既用于模块中的运行时初始化，也用于启动应用程序，而它们的执行顺序很难控制。因此，SWI-Prolog 引入了 `initialization/2`，增加一个参数来指定初始化的角色，并间接指定初始化顺序。现在，应用程序入口点可以这样声明：

```prolog
:- initialization(start, main).

start :-
    ...
```

采用这些约定后，可以用下面的命令行运行应用程序。其中 `option ...` 是 Prolog 选项，用来控制例如内存限制等设置；通常不需要指定。`arg ...` 会通过 Prolog 标志 `argv` 提供给程序。

```shell
% swipl [option ...] load.pl [arg ...]
```

如果只想载入代码而不运行应用程序，并且入口点是通过上面所述的 `initialization/2` 指令启动的，则可以使用 `-l`。载入之后，可以调试和/或编辑应用程序。

```shell
% swipl [option ...] -l load.pl [arg ...]
```

应用程序通常不会像上面那样直接使用 `start/0`，而是使用库 `library(main)` 中的 `main/0`。谓词 `main/0` 会为非开发场景做好准备，并用应用程序的 `argv`（命令行参数）调用 `main/1`。这些参数通常会使用同一库中的 `argv_options/2` 处理为位置参数（positional arguments）和选项（options）。

上面的方式在命令行中使用 Prolog 时可以很好工作，但在难以控制 SWI-Prolog 命令行的场景中不太适合，例如使用 `swipl-win`，或在 Emacs 等 IDE 下运行 Prolog。把一个使用上述 `initialization/2` 指令的程序载入顶层：

```prolog
?- [load].
```

**不会**启动入口点。而使用 `swipl-win` 打开 `.pl` 文件则会启动入口点。

## 运行应用程序

如果想让程序可用于真实场景，有多种选择。最佳选择取决于程序是否只在装有 SWI-Prolog 开发系统的机器上使用、程序大小，以及操作系统（Unix 或 Windows）。有四种选择：

- 在类 Unix 系统上，可以使用 shebang 魔法序列把 Prolog 源文件变成可执行文件。参见“使用 PrologScript”。
- 在任何系统上，都可以使用 shell 脚本（Unix 的 `sh` 或 Windows 的 `cmd`）来启动应用程序。参见“创建 shell 脚本”。
- 在任何系统上，都可以创建一个保存状态（saved state），其中包含虚拟机代码和启动序列。保存状态可以是独立的；在采取一些预防措施后，它们也可以在未安装 SWI-Prolog 本身的环境中工作。它们启动很快，但体积较大；如果程序使用原生代码扩展和文件资源，从该程序创建保存状态并不简单，具体细节取决于操作系统和所需资源。参见“创建保存状态”。
- 在任何系统上，都可以把一个 Prolog 文件加入指定目录，并允许用下面的形式启动它：

  ```shell
  swipl name [arg ...]
  ```

  可以通过 Prolog 包（pack）、用户专属目录，或系统级目录，向 Prolog 安装添加新命令。参见“SWI-Prolog 应用脚本”。

### 使用 PrologScript

Prolog 源文件可以用 Unix 的 `#!` 魔法开头直接作为 Unix 程序使用。Unix 的 `#!` 魔法是允许的，因为如果 Prolog 文件的第一个字符是 `#`，第一行会被视为注释。`#` 号也可以合法地作为普通 Prolog 子句的开头。在极少数确实需要这种写法的情况下，请把第一行留空，或添加一个头部注释。要创建 Prolog 脚本，可以使用下面两种形式之一作为第一行。第一种可把脚本绑定到某个特定的 Prolog 安装，后一种则使用 `$PATH` 中默认安装的 Prolog。

```shell
#!/path/to/swipl
#!/usr/bin/env swipl
```

不同 Unix 派生系统对 HashBang 行中可执行程序参数的解释不同。为了可移植性，`#!` 后面必须立即跟一个可执行文件的绝对路径，并且应当没有参数或只有一个参数。可执行文件路径和参数都不能使用引号或空格。以这种方式启动时，Prolog 标志 `argv` 包含脚本调用之后的命令行参数。

从 7.5.8 版本开始，`initialization/2` 支持 `When` 选项 `program` 和 `main`，因此可以如下定义一个 Prolog 脚本，用于求值命令行中的算术表达式。注意，`main/0` 由库 `library(main)` 定义。它会在禁用信号处理之后，用命令行参数调用 `main/1`。

```prolog
#!/usr/bin/env swipl

:- initialization(main, main).

main(Argv) :-
    atomic_list_concat(Argv, ' ', SingleArg),
    term_to_atom(Term, SingleArg),
    Val is Term,
    format('~w~n', [Val]).
```

下面是两个运行示例：

```shell
% ./eval 1+2
3
% ./eval foo
ERROR: is/2: Arithmetic: `foo/0' is not a function
```

出于调试或检查目的，可以使用 `-l` 或 `-t` 启动 Prolog 脚本。例如，`-l` 只会载入脚本，忽略 `main` 和 `program` 初始化。

```prolog
swipl -l eval 1+1
<banner>

?- main.
2
true.

?-
```

也可以使用 `-t prolog` 强制程序在应用程序完成后进入交互式顶层：

```prolog
swipl -t prolog eval 1+1
2
?-
```

Windows 版本会直接忽略 `#!` 行。旧版本曾经从 HashBang 行中提取命令行参数。从 5.9 版本开始，所有相关设置都可以通过指令完成。由于 HashBang 行处理存在兼容性问题，SWI-Prolog 决定完全移除这项处理。

### 创建 shell 脚本

随着 PrologScript 的引入（参见“使用 PrologScript”），本节说明的 shell 脚本方式对于大多数应用程序已经显得多余。

尤其是在 Unix 系统上，对于规模不太大的应用程序，编写一个只负责载入应用程序并调用入口点的 shell 脚本，通常是不错的选择。下面先给出脚本骨架，随后给出获取程序参数的 Prolog 代码。详情参见库 `library(main)` 和 `argv_options/3`。

```shell
#!/bin/sh

base=<absolute-path-to-source>
SWIPL=swipl

exec $SWIPL "$base/load.pl" -- "$@"
```

```prolog
:- use_module(library(main)).
:- initialization(main,main).

main(Argv) :-
    argv_options(Argv, Positional, Options),
    go(Positional, Options).

go(Positional, Options) :-
    ...
```

在 Windows 系统上，可以通过创建指向 Prolog 的快捷方式并传入适当选项，或编写 `.bat` 文件，达成类似行为。

### 创建保存状态

对于较大的程序，以及必须在没有安装 SWI-Prolog 开发系统的机器上运行的程序，创建保存状态是最佳方案。保存状态使用 `qsave_program/[1,2]` 或命令行选项 `-c` 创建。保存状态是一个文件，包含机器无关的中间代码，并采用专为快速载入设计的格式。保存状态不依赖 CPU 指令集或字节序。32 位和 64 位的保存状态不兼容。通常，保存状态只能在创建它的同一 Prolog 版本上运行。也可以把仿真器集成到保存状态中，从而创建一个单文件但机器相关的可执行文件。这个过程会在运行时章节中说明。

### 使用 `-c` 命令行选项进行编译

该机制会载入一系列 Prolog 源文件，然后像 `qsave_program/2` 一样创建保存状态。命令语法如下：

```shell
% swipl [option ...] [-o output] -c file.pl ...
```

参数 `options` 是传给 `qsave_program/2` 的选项，写成下面的格式。选项名称及其值由 `qsave_program/2` 说明。

```text
--option-name=option-value
```

例如，要创建一个独立可执行文件，使其启动时执行 `main/0`，并且通过 `load.pl` 载入源代码，可以使用下面的命令：

```shell
% swipl --goal=main --stand_alone=true -o myprog -c load.pl
```

这与执行下面的内容效果完全相同：

```prolog
% swipl
<banner>

?- [load].
?- qsave_program(myprog,
         [ goal(main),
           stand_alone(true)
         ]).
?- halt.
```

### SWI-Prolog 应用脚本

从 9.1.18 版本开始，SWI-Prolog 允许用下面的命令启动应用程序：

```shell
swipl [option ...] [path:]name [arg ...]
```

这条命令行首先处理“命令行选项”中说明的 Prolog 选项。注意，大多数标准 Prolog 命令行选项在这里并不相关。`-f` 默认值为 `none`，这意味着默认不会载入用户初始化文件。如果应用程序希望载入用户初始化文件，应在该文件存在时载入 `user_app_config(init)`（参见 `exists_source/1`）。

接下来，它会使用 SWI-Prolog 的文件搜索机制（由 `absolute_file_name/3` 定义）定位 `path(name)`。载入这个文件后，它会查找通过 `initialization/2` 为 `main` 注册的最后一个目标，如本章前文所述；如果没有 `main` 的初始化指令，程序会以错误终止。默认情况下，入口点终止后应用程序也会终止。入口点可以通过调用 `cli_enable_development_system/0` 启用交互式 Prolog REPL 循环。除了 `main` 之外，也允许使用其他形式的 `initialization/2` 指令。

`[path:]name` 后面的所有命令行选项，都可以通过 Prolog 标志 `argv` 访问。

可选的 `path` 默认为 `app`。默认情况下，应用会在下面这些目录中搜索。详情参见 `file_search_path/2`。

1. SWI-Prolog 安装目录中的 `app` 目录。
2. 用户和站点配置。在使用 XDG 文件名约定的 POSIX 系统上，这通常是 `~/.local/share/swi-prolog/app/` 和 `/usr/share/swi-prolog/app`。
3. Prolog 包（pack）的 `app` 目录。

安装会提供下列应用：

#### `app`

打印已安装应用的信息。例如，要列出所有可用应用，运行：

```shell
swipl app list
```

#### `pack`

Prolog 包的命令行管理工具。这是 Prolog 库 `library(prolog_pack)` 的前端。例如，要查找与 *type* 相关的包，可以使用下面的命令：

```shell
swipl pack find type
```
