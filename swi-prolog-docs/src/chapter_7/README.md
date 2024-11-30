# 在线帮助

## 7.1 library(help)：基于文本的手册

此模块提供 help/1 和 apropos/1，它们提供有关某个主题的帮助或在手册中搜索相关主题。

默认情况下，help/1 的结果通过诸如 less 之类的寻呼机发送。此行为由以下项控制：

Prolog 标志 help_pager，可设置为以下值之一：

false
永远不要使用寻呼机。
default
使用默认行为。这会尝试确定 Prolog 是否在允许寻呼机的环境中以交互方式运行。如果是，它会检查环境变量 PAGER，否则尝试查找 less 程序。
Callable
Callable 术语被解释为 program_name(Arg, ...)。例如，less('-r') 将是默认值。请注意，如果使用单引号，则程序名称可以是绝对路径。