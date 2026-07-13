# 命令行选项

SWI-Prolog 可以以下列模式之一执行：

- `swipl --help`
- `swipl --version`
- `swipl --arch`
- `swipl --dump-runtime-variables`

  这些选项必须作为唯一选项出现。它们会让 Prolog 打印信息性消息并退出。参见“信息性命令行选项”。

- `swipl [option ...] script-file [arg ...]`

  在 Unix 系统中，如果执行的文件以 `#!/path/to/executable [option ...]` 开头，则会使用这种参数形式。脚本文件之后的参数可以通过 Prolog 标志 `argv` 访问。

- `swipl [option ...] prolog-file ... [[--] arg ...]`

  这是启动 Prolog 的常规方式。选项见“运行 Prolog 的命令行选项”、“控制堆栈大小”和“从命令行运行目标”。Prolog 标志 `argv` 提供对 `arg ...` 的访问。如果选项后跟一个或多个 Prolog 文件名，也就是扩展名为 `.pl`、`.prolog`，或在 Windows 上安装期间注册的用户首选扩展名的文件，则这些文件会被载入。第一个文件会记录在 Prolog 标志 `associated_file` 中。此外，`pl-win[.exe]` 会使用 `working_directory/2` 切换到这个主源文件所在的目录。

- `swipl -o output -c prolog-file ...`

  选项 `-c` 用于把一组 Prolog 文件编译成可执行文件。参见“编译选项”。

- `swipl -o output -b bootfile prolog-file ...`

  引导编译。参见“维护选项”。

## 4.1 信息性命令行选项

### `--arch`

作为唯一选项给出时，打印体系结构标识符并退出。该标识符也可通过 Prolog 标志 `arch` 查看。另请参阅 `--dump-runtime-variables`。

### `--dump-runtime-variables[=format]`

作为唯一选项给出时，打印一组变量设置，可在 shell 脚本中用于处理 Prolog 参数。`swipl-ld` 也使用这个功能。下面是一个典型用法：

```shell
eval `swipl --dump-runtime-variables`
cc -I$PLBASE/include -L$PLBASE/lib/$PLARCH ...
```

该选项可以跟 `=sh`，以 POSIX shell 格式输出；这是默认值。也可以跟 `=cmd`，以兼容 MS-Windows `cmd.exe` 的格式输出。

### `--help`

作为唯一选项给出时，概述最重要的选项。

### `--version`

作为唯一选项给出时，概述版本和体系结构标识符。

### `--abi-version`

打印一个表示多方面二进制兼容性的键，也就是字符串。参见“二进制兼容性”。

## 4.2 运行 Prolog 的命令行选项

注意，布尔选项可以写成 `--name` 表示 true，也可以写成 `--noname` 或 `--no-name` 表示 false。下文采用 `--no-name` 的形式，因为默认值为 true。

### `-D name[=value]`

将 Prolog 标志 `name` 设置为 `value`。这些标志会在载入初始保存状态后立即设置。如果标志已经定义，`value` 会被转换为该标志的类型。如果标志尚未定义，且 `value` 表示数字，则将其设置为数字，否则设置为原子。如果没有给出 `=value`，则使用布尔值。如果 `name` 形如 `no-flag`，则把 `flag` 设置为 `false`；否则把名为 `name` 的标志设置为 `true`。`name[=value]` 可以紧跟在 `-D` 后面，也可以作为下一个命令行参数出现。

许多命令行选项也会反映为 Prolog 标志。系统意图把这些形式作为同义项处理。目前，一些命令行标志会在保存状态载入完成前影响 Prolog 初始化，而另一些标志在 Prolog 初始化后可能无法更改。例如，未来版本将支持 `-Dhome=dir`，用于修改 Prolog 安装目录的概念。

### `--debug-on-interrupt`

立即启用在中断信号上调试，也就是 `Control-C` 或 `SIGINT`。通常，中断调试会在进入交互式顶层时启用。该标志可用于在执行来自 `-g` 或 `initialization/[1,2]` 的目标时，通过中断启动调试器。另请参阅 Prolog 标志 `debug_on_interrupt`。

### `--home[=DIR]`

如果带 `DIR`，则把 SWI-Prolog 的 home 目录设置为 `DIR`，并向进程添加环境变量 `SWI_HOME_DIR`，其值为 `DIR`。如果不带参数，则报告找到的 home 目录并退出；如果找不到位置，则打印错误并以状态 `1` 退出。关于 home 目录的定位方式，参见相关章节；关于 SWI-Prolog 如何使用该目录，参见 Prolog 标志 `home`。

### `--quiet`

将 Prolog 标志 `verbose` 设置为 `silent`，抑制信息性消息和横幅消息。也可以写作 `-q`。

### `--no-debug`

禁用调试。详情参见 `current_prolog_flag/2` 标志 `generate_debug_info`。

### `--no-signals`

禁止 Prolog 处理任何信号。这一属性有时适合嵌入式应用程序。该选项会把标志 `signals` 设置为 `false`。注意，用于解除系统调用阻塞的处理器仍会安装。可以额外使用 `--sigalert=0` 阻止这一点。另请参阅 `--sigalert`。

### `--no-threads`

在运行时禁用多线程版本的线程功能。另请参阅标志 `threads` 和 `gc_thread`。

### `--no-packs`

不附加扩展包，也就是 add-ons。另请参阅 `attach_packs/0` 和 Prolog 标志 `packs`。

### `--no-pce`

启用或禁用 xpce GUI 子系统。默认情况下，如果已安装 xpce 且系统可以访问图形环境，它会作为可自动载入组件提供。使用 `--pce` 会在用户空间中载入 xpce 系统；使用 `--no-pce` 会使其在本会话中不可用。

### `--on-error=style`

指定如何处理错误。详情参见 Prolog 标志 `on_error`。

### `--on-warning=style`

指定如何处理警告。详情参见 Prolog 标志 `on_warning`。

### `--pldoc[=port]`

在一个空闲网络端口启动 PlDoc 文档系统，并在 `http://localhost:port` 打开用户浏览器。如果指定了 `port`，服务器会在给定端口启动，并且不会启动浏览器。

### `--sigalert=NUM`

使用信号 `NUM`，范围为 `1` 到 `31`，提醒线程。这是为了让 `thread_signal/2` 以及派生的 Prolog 信号处理在目标线程阻塞于可中断系统调用时立即生效，例如 `sleep/1` 或对多数设备的读写。默认使用 `SIGUSR2`。如果 `NUM` 为 `0`，则不安装该处理器。可以用 `prolog_alert_signal/2` 在运行时查询或修改该值。

### `--no-tty`

仅限 Unix。该开关控制终端，以允许向跟踪器和 `get_single_char/1` 发送单字符命令。默认情况下，除非系统检测到自己没有连接到终端，或正作为 GNU Emacs 下级进程运行，否则会启用终端操作。另请参阅 Prolog 标志 `tty_control`。

### `--win-app`

该选项只在 `swipl-win.exe` 中可用，用于开始菜单项。它会让 `plwin` 在 `...\My Documents\Prolog` 或其本地等效文件夹中启动。若 `Prolog` 子目录不存在，则创建它。另请参阅 `win_folder/2`。

### `-O`

优化编译。详情参见 `current_prolog_flag/2` 标志 `optimise`。

### `-l file`

载入 `file`。该标志提供与其他一些 Prolog 系统的兼容性。在 SWI-Prolog 中，它用于跳过通过 `initialization/2` 指令指定的程序初始化。另请参阅脚本相关章节和 `initialize/0`。

### `-s file`

将 `file` 用作脚本文件。脚本文件会在通过 `-f file` 选项指定的初始化文件之后载入。不同于 `-f file`，使用 `-s` 不会阻止 Prolog 载入个人初始化文件。

### `-f file`

使用 `file` 作为初始化文件，而不是默认的 `init.pl`。`-f none` 会停止 SWI-Prolog 搜索启动文件。该选项可以替代 `-s file`，用于阻止 Prolog 载入个人初始化文件。另请参阅“用户的初始化文件”。

### `-F script`

从 SWI-Prolog home 目录选择启动脚本。脚本文件名为 `<script>.rc`。默认的 `script` 名称从可执行文件推导而来，取程序名开头的字母数字字符，包括字母、数字和下划线。`-F none` 会停止查找脚本。该机制用于简单管理稍有差异的版本。例如，可以编写脚本 `iso.rc`，再使用 `pl -F iso` 选择 ISO 兼容模式；或者从 `iso-pl` 到 `pl` 建立链接。

### `-x bootfile`

从 `bootfile` 启动，而不是从系统默认引导文件启动。引导文件可以是使用 `-b` 或 `-c` 选项进行 Prolog 编译得到的文件，也可以是使用 `qsave_program/[1,2]` 保存的程序。

### `-p alias=path1[:path2 ...]`

为 `file_search_path` 定义路径别名。`alias` 是别名名称，`path1 ...` 是该别名的值列表。在 Windows 上，列表分隔符是 `;`；在其他系统上是 `:`。值可以是形如 `alias(value)` 的项，也可以是路径名。计算出的别名会使用 `asserta/1` 添加到 `file_search_path/2`，因此它们排在该别名的预定义值之前。关于这种文件定位机制的细节，参见 `file_search_path/2`。

### `--traditional`

该标志禁用 SWI-Prolog 7 中引入的最重要扩展，这些扩展会造成与早期版本的不兼容。具体来说，列表会以传统方式表示，双引号文本表示为字符代码列表，并且不支持字典上的函数记法。如果存在该标志，字典作为语法实体以及作用于字典的谓词仍然可用。

### `--`

停止扫描更多参数。这样可以在它之后向应用程序传递参数。可以通过 `current_prolog_flag/2` 读取标志 `argv`，以获得命令行参数。

## 4.3 控制堆栈大小

从 7.7.14 版本起，堆栈不再分别受限，而只限制组合大小。注意，32 位系统仍然有 128Mb 限制。默认情况下，64 位机器上的组合限制为 1Gb，32 位机器上为 512Mb。

例如，要把堆栈限制为 32Gb，可以使用下面的命令。注意，堆栈限制按线程应用。单个线程可以通过 `thread_create` 的 `stack_limit(+Bytes)` 选项控制。任意线程都可以调用 `set_prolog_flag(stack_limit, Limit)` 来调整堆栈限制。该限制会被从此线程创建的线程继承。

```shell
$ swipl --stack-limit=32g
```

### `--stack-limit=size[bkmg]`

将 Prolog 堆栈的组合大小限制为指定的 `size`。后缀把值指定为字节、Kbytes、Mbytes 或 Gbytes。

### `--table-space=size[bkmg]`

限制表空间。表空间用于保存记忆化答案的 trie，这些答案来自制表。默认值在 64 位机器上为 1Gb，在 32 位机器上为 512Mb。另请参阅 Prolog 标志 `table_space`。

### `--shared-table-space=size[bkmg]`

限制共享表使用的表空间。参见共享制表相关章节。

## 4.4 从命令行运行目标

### `-g goal`

`Goal` 会在进入顶层之前执行。该选项可以出现多次。详情见“初始化文件和目标”。如果没有初始化目标，系统会调用 `version/0` 打印欢迎消息。可以用 `--quiet` 抑制欢迎消息，也可以用 `-g true` 达到同样效果。`goal` 可以是复杂项。在这种情况下，通常需要使用引号，避免被 shell 展开。下面是非交互式运行目标的一种安全方式。如果 `go/0` 成功，`-g halt` 会让进程以退出代码 `0` 停止；如果失败，退出代码为 `1`；如果引发异常，退出代码为 `2`。

```shell
% swipl <options> -g go -g halt
```

### `-t goal`

使用 `goal` 作为交互式顶层，而不是默认目标 `prolog/0`。`goal` 可以是复杂项。如果顶层目标成功，SWI-Prolog 以状态 `0` 退出。如果失败，退出状态为 `1`。如果顶层引发异常，该异常会作为未捕获错误打印，并且顶层会重新启动。该标志也决定 `break/0` 和 `abort/0` 启动的目标。如果想阻止用户进入交互模式，可以用 `-g goal -t halt` 启动应用程序。

## 4.5 编译选项

### `-c file ...`

把文件编译为“中间代码文件”。参见“加载和运行项目”。

### `-o output`

与 `-c` 或 `-b` 结合使用，用于确定编译输出文件。

## 4.6 维护选项

下列选项用于系统维护，仅作为参考列出。

### `-b initfile ... -c file ...`

引导编译。`initfile ...` 由 C 编写的 bootstrap 编译器编译，`file ...` 由普通 Prolog 编译器编译。仅用于系统维护。

### `-d token1,token2,...`

为带有指定 token 的 `DEBUG` 语句打印调试消息。只有在系统以 `-DO_DEBUG` 标志编译时才有效。仅用于系统维护。
