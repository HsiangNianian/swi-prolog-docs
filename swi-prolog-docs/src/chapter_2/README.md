# 用户的初始化文件

系统初始化完成后，会 consult 用户的初始化文件。该文件通过 `absolute_file_name/3` 查找，并使用路径别名 `app_config`。`app_config` 指向操作系统默认的应用配置数据目录下名为 `swi-prolog` 的目录：

- 在 Windows 上，它是 CSIDL 文件夹 `CSIDL_APPDATA`，通常类似 `C:\Documents and Settings\username\Application Data`。
- 如果设置了环境变量 `XDG_DATA_HOME`，则使用该变量。这遵循 [freedesktop](https://standards.freedesktop.org) 标准。
- 否则使用 `~/.config` 的展开结果。

可以用下面的调用找到该目录：

```prolog
?- absolute_file_name(app_config(.), Dir, [file_type(directory)]).
Dir = '/home/jan/.config/swi-prolog'.
```

找到第一个启动文件后，系统会载入它，并停止继续寻找其他启动文件。启动文件名可以用 `-f file` 选项修改。如果 `File` 是绝对路径，则直接载入该文件；否则使用与默认启动文件相同的约定查找。最后，如果 `file` 为 `none`，则不载入任何文件。

安装目录提供了 `customize/init.pl` 文件，其中包含一些常用于定制 Prolog 行为的命令，这些命令默认被注释掉，例如编辑器集成、颜色选择或历史记录参数。许多开发工具也提供菜单项，用于编辑启动文件，或从系统骨架创建新的启动文件。

另请参阅命令行选项中的 `-s`（脚本）和 `-F`（系统范围初始化），以及“初始化文件和目标”。
