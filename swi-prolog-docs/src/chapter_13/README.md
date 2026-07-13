# 钩子谓词概览

SWI-Prolog 提供了大量 hook，主要用于控制消息处理、调试、启动、关闭、宏展开等行为。下面汇总了所有已定义的 hook，并标明其可移植性。

- `portray/1`

  接入 `write_term/3`，修改项的打印方式（ISO）。

- `message_hook/3`

  接入 `print_message/2`，修改系统消息的打印方式（Quintus/SICStus）。

- `message_property/2`

  接入 `print_message/2`，定义前缀、输出流、颜色等属性。

- `message_prefix_hook/2`

  接入 `print_message/2`，向消息添加额外前缀，例如时间和线程。

- `library_directory/1`

  接入 `absolute_file_name/3`，定义新的库目录（多数 Prolog 系统）。

- `file_search_path/2`

  接入 `absolute_file_name/3`，定义新的搜索路径（Quintus/SICStus）。

- `term_expansion/2`

  接入 `load_files/2`，在读取到的项被编译前修改它们，也就是进行宏处理（多数 Prolog 系统）。

- `goal_expansion/2`

  类似 `term_expansion/2`，但作用于单个目标（SICStus）。

- `prolog_load_file/2`

  接入 `load_files/2`，从“非文件”资源中把其他数据格式载入为 Prolog 源。谓词 `load_files/2` 是 `consult/1`、`use_module/1` 等谓词的祖先。

- `prolog_edit:locate/3`

  接入 `edit/1`，定位对象（SWI）。

- `prolog_edit:edit_source/1`

  接入 `edit/1`，调用内部编辑器（SWI）。

- `prolog_edit:edit_command/2`

  接入 `edit/1`，定义要使用的外部编辑器（SWI）。

- `prolog_list_goal/1`

  接入 tracer，列出与特定目标关联的代码（SWI）。

- `prolog_trace_interception/4`

  接入 tracer，处理 trace 事件（SWI）。

- `prolog:debug_control_hook/1`

  接入 `spy/1`、`nospy/1`、`nospyall/0` 和 `debugging/0`，把这些控制谓词扩展到更高层的库。

- `prolog:help_hook/1`

  接入 `help/0`、`help/1` 和 `apropos/1`，扩展帮助系统。

- `resource/3`

  定义新的资源。严格来说这不是真正的 hook，但与 hook 类似（SWI）。

- `exception/3`

  早期对通用 hook 机制的一次尝试。它处理未定义谓词（SWI）。

- `attr_unify_hook/2`

  attributed variable 的合一 hook。可以在任何模块中定义。详情参见 attributed variable 章节。
