# 命令行选项

SWI-Prolog 可以在以下模式之一中执行：

swipl --help
swipl --version
swipl --arch
swipl --dump-runtime-variables
这些选项必须作为唯一选项出现。它们使 Prolog 打印一条信息消息并退出。请参阅第 2.4.1 节。
swipl [option ...] script-file [arg ...]
如果执行以 #!/path/to/executable [option ...] 开头的文件，则在 Unix 系统上传递这些参数。脚本文件后的参数在 Prolog 标志 argv 中可用。
swipl [option ...] prolog-file ... [[--] arg ...]
这是启动 Prolog 的正常方式。选项在第 2.4.2 节、第 2.4.3 节和第 2.4.4 节中进行了描述。 Prolog 标志 argv 提供对 arg 的访问... 如果选项后面跟着一个或多个 Prolog 文件名（即扩展名为 .pl、.prolog 或（在 Windows 上）在安装期间注册的用户首选扩展名的名称），则会加载这些文件。第一个文件在 Prolog 标志 related_file 中注册。此外，pl-win[.exe] 使用 working_directory/2 切换到此主要源文件所在的目录。
swipl -o output -c prolog-file ...
-c 选项用于将一组 Prolog 文件编译成可执行文件。请参阅第 2.4.5 节。
swipl -o output -b bootfile prolog-file ...
引导编译。请参阅第 2.4.6 节。

## 4.1 信息性命令行选项

--arch
当作为唯一选项给出时，它会打印体系结构标识符（请参阅 Prolog 标志 arch）并退出。另请参阅 --dump-runtime-variables。
--dump-runtime-variables [=format]
当作为唯一选项给出时，它会打印一系列变量设置，这些变量设置可用于 shell 脚本来处理 Prolog 参数。swipl-ld 也使用此功能（请参阅第 12.5 节）。下面是使用此功能的典型示例。

```pl
eval `swipl --dump-runtime-variables`
cc -I$PLBASE/include -L$PLBASE/lib/$PLARCH ...
```

该选项后面可以跟 =sh，以 POSIX shell 格式转储（默认）或 =cmd，以 MS-Windows cmd.exe 兼容格式转储。
--help
当作为唯一选项给出时，它会总结最重要的选项。
--version
当作为唯一选项给出时，它会总结版本和体系结构标识符。
--abi-version
打印一个键（字符串），表示多个方面的二进制兼容性。请参阅第 2.21 节。

## 4.2 运行 Prolog 的命令行选项

请注意，布尔选项可以写为 --name (true)、--noname 或 --no-name (false)。它们在下面写为 --no-name，因为默认值为“true”。

-D name[=value]
将 Prolog 标志名称设置为值。加载初始保存状态后立即设置标志。如果标志已定义，则值将转换为标志的类型。如果标志未定义，则将其设置为一个数字，否则值表示数字，否则表示原子。如果没有给出 =value，则使用布尔值。如果 name 为无标志，则标志设置为 false。否则，标志名称设置为 true。name[=value] 可以紧跟在 -D 后面，也可以作为下一个命令行参数出现。

请注意，许多命令行选项都由 Prolog 标志反映。我们打算将它们作为同义词处理。目前，一些命令行标志在加载已保存状态完成之前会影响 Prolog 初始化，而其他一些标志在 Prolog 初始化之后可能不会更改。例如，未来版本将支持 -Dhome=dir 来更改 Prolog 安装目录的概念。

--debug-on-interrupt
立即启用中断信号 (Control-C、SIGINT) 上的调试。通常在进入交互式顶层时启用中断调试。此标志可用于在执行来自 -g 或初始化/[1,2] 的目标时在中断上启动调试器。另请参阅 Prolog 标志 debug_on_interrupt。

--home[=DIR]
使用 DIR 作为主目录。详情请参阅第 12.6 节。如果省略 DIR，则打印找到的位置并退出进程。如果找不到位置，则打印错误并以状态 1 退出进程。

--quiet
将 Prolog 标志 verbose 设置为 silent，抑制信息和横幅消息。也可用作 -q。

--no-debug
禁用调试。详情请参阅 current_prolog_flag/2 标志 generate_debug_info。

--no-signals
禁止 Prolog 处理任何信号，该属性有时对于嵌入式应用程序来说是理想的。此选项将标志 signals 设置为 false。详情请参阅第 12.4.25.1 节。请注意，仍安装了用于解除系统调用阻止的处理程序。可以使用 
--sigalert=0 额外阻止此操作。请参阅 --sigalert。

--no-threads
在运行时禁用多线程版本的线程。另请参阅标志threads 和 gc_thread。

--no-packs
不附加扩展包（附加组件）。另请参阅attach_packs/0 和 Prolog 标志包。

--no-pce
启用/禁用 xpce GUI 子系统。默认情况下，如果已安装并且系统可以访问图形，则将其作为自动加载组件提供。使用 --pce 在用户空间中加载 xpce 系统，而 --no-pce 使其在会话中不可用。

--on-error =style
如何处理错误。有关详细信息，请参阅 Prolog 标志 on_error。

--on-warning =style
如何处理警告。有关详细信息，请参阅 Prolog 标志 on_warning。

--pldoc [=port]
在空闲网络端口上启动 PlDoc 文档系统，并在 http://localhost:port 上启动用户的浏览器。如果指定了端口，则服务器将在给定端口启动，并且不会启动浏览器。

--sigalert=NUM
使用信号 NUM（1 ... 31）来提醒线程。这是使thread_signal/2和派生的Prolog信号处理在目标线程被可中断的系统调用（例如，sleep/1，对大多数设备的读/写）阻塞时立即起作用所必需的。默认使用SIGUSR2。如果NUM为0（零），则未安装此处理程序。请参阅prolog_alert_signal/2以在运行时查询或修改此值。

--no-tty
仅限Unix。控制终端的开关允许向跟踪器和get_single_char/1发送单字符命令。默认情况下，除非系统检测到它未连接到终端或它正在作为GNU-Emacs下级进程运行，否则将启用对终端的操作。另请参阅tty_control。

--win-app
此选项仅在swipl-win.exe中可用，用于开始菜单项。如果导致plwin在文件夹... \ My Documents \ Prolog或其本地等效文件夹中启动（请参阅win_folder/2）。如果不存在 Prolog 子目录，则创建该子目录。

-O
优化编译。有关详细信息，请参阅 current_prolog_flag/2 标志优化。

-l 文件
加载文件。此标志提供与其他一些 Prolog 系统的兼容性。10 它在 SWI-Prolog 中用于跳过使用初始化/2 指令指定的程序初始化。另请参阅第 2.11.1.1 节和初始化/0。

-s 文件
将文件用作脚本文件。在使用 -f 文件选项指定的初始化文件之后加载脚本文件。与 -f 文件不同，使用 -s 不会阻止 Prolog 加载个人初始化文件。

-f file
使用 file 作为初始化文件，而不是默认的 init.pl。‘-f none’ 停止 SWI-Prolog 搜索启动文件。此选项可用作 -s file 的替代，可阻止 Prolog 加载个人初始化文件。另请参阅第 2.2 节。

-F script
从 SWI-Prolog 主目录中选择一个启动脚本。脚本文件名为 <script>.rc。默认脚本名称从可执行文件推导而来，从程序名称中获取前导字母数字字符（字母、数字和下划线）。-F none 停止查找脚本。旨在简单管理略有不同的版本。例如，可以编写脚本 iso.rc，然后使用 pl -F iso 选择 ISO 兼容模式，或者从 iso-pl 到 pl 建立链接。

-x bootfile
从 bootfile 而不是系统的默认启动文件启动。引导文件是使用 -b 或 -c 选项进行 Prolog 编译后生成的文件，或者是使用 qsave_program/[1,2] 保存的程序。

-p alias=path1[:path2 ...
为 file_search_path 定义路径别名。alias 是别名的名称，arg path1 ... 是别名的值列表。在 Windows 上，列表分隔符为 ;。在其他系统上，它是 :。值可以是别名 (值) 或路径名形式的术语。计算出的别名使用 asserta/1 添加到 file_search_path/2，因此它们位于别名的预定义值之前。有关使用此文件定位机制的详细信息，请参阅 file_search_path/2。

--traditional
此标志禁用 SWI-Prolog 版本 7 的最重要的扩展（请参阅第 5 节），这些扩展会导致与早期版本不兼容。具体来说，列表以传统方式表示，双引号文本由字符代码列表表示，不支持字典上的函数符号。如果存在此标志，则字典作为语法实体，以及作用于它们的谓词仍然受支持。

--

停止扫描更多参数，因此您可以在此之后为应用程序传递参数。请参阅 current_prolog_flag/2，使用标志 argv 获取命令行参数。

## 4.3 控制堆栈大小

从版本 7.7.14 开始，堆栈不再单独受限。相反，只限制组合大小。请注意，32 位系统仍存在 128Mb 的限制。请参阅第 2.19.1 节。默认情况下，64 位计算机上的组合限制为 1Gb，32 位计算机上的组合限制为 512Mb。

例如，要将堆栈限制为 32Gb，请使用以下命令。请注意，堆栈限制适用于每个线程。可以使用 thread_create 的 stack_limit(+Bytes) 选项控制单个线程。任何线程都可以调用 set_prolog_flag(stack_limit, Limit)（请参阅 stack_limit）来调整堆栈限制。此限制由从此线程创建的线程继承。

```shell
$ swipl --stack-limit=32g
```

--stack-limit=size[bkmg]
将 Prolog 堆栈的总大小限制为指定的大小。后缀将值指定为字节、千字节、兆字节或千兆字节。
--table-space=size[bkmg]
表空间的限制。这是用于保存 memoized11 答案以进行制表的尝试的存储位置。在 64 位计算机上默认为 1Gb，在 32 位计算机上默认为 512Mb。请参阅 Prolog 标志 table_space。
--shared-table-space=size[bkmg]
共享表的表空间限制。请参阅第 7.9 节。

## 4.4 从命令行运行目标

-g goal
在进入顶层之前执行目标。此选项可能出现多次。有关详细信息，请参阅第 2.3 节。如果没有初始化目标，系统将调用 version/0 来打印欢迎消息。可以使用 --quiet 或 -g true 来抑制欢迎消息。目标可能是一个复杂的术语。在这种情况下，通常需要使用引号来保护它不被 shell 扩展。以下是一种非交互运行目标的安全方法。如果 go/0 成功，-g halt 会导致进程停止并返回退出代码 0。如果失败，退出代码为 1；如果引发异常，退出代码为 2。
-t goal
使用目标作为交互式顶层，而不是默认目标 prolog/0。目标可能是一个复杂的术语。如果顶层目标成功，SWI-Prolog 将以状态 0 退出。如果失败，退出状态为 1。如果顶层引发异常，则将其打印为未捕获的错误，并重新启动顶层。此标志还确定由 break/0 和 abort/0 启动的目标。如果要阻止用户进入交互模式，请使用“-g goal -t halt”启动应用程序。

## 4.5 编译选项

-c file ...
将文件编译为“中间代码文件”。请参阅第 2.11 节。
-o output
与 -c 或 -b 结合使用以确定编译的输出文件。

## 4.6 维护选项

以下选项用于系统维护。仅供参考。

-b initfile ...-c file ...
引导编译。initfile ...由 C 编写的引导编译器编译，file ...由普通 Prolog 编译器编译。仅用于系统维护。
-d token1,token2,...
打印标有指示标记之一的 DEBUG 语句的调试消息。仅当使用 -DO_DEBUG 标志编译系统时才有效。仅用于系统维护。