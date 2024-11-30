# 用户的初始化文件

系统初始化后，系统会查阅（参见 consult/1）用户的初始化文件。使用路径别名 app_config（参见 file_search_path/2）absolute_file_name/3 搜索此文件。这是一个名为 swi-prolog 的目录，位于操作系统默认名称下，用于放置应用程序配置数据：

在 Windows 上，CSIDL 文件夹 CSIDL_APPDATA，通常为 C:\Documents and Settings\username\Application Data。

如果设置了环境变量 XDG_DATA_HOME，请使用此变量。这遵循自由桌面标准。
/.config 的扩展。

可以使用以下调用找到该目录：

```pl
?- absolute_file_name(app_config(.), Dir, [file_type(directory)]).
Dir = '/home/jan/.config/swi-prolog'.
```

找到第一个启动文件后，它会被加载，Prolog 会停止寻找其他启动文件。可以使用“-f file”选项更改启动文件的名称。如果 File 表示绝对路径，则加载此文件，否则使用与默认启动文件相同的约定来搜索文件。最后，如果 file 为 none，则不加载任何文件。

安装程序提供了一个文件 customize/init.pl，其中包含通常用于自定义 Prolog 行为的（注释）命令，例如与编辑器交互、颜色选择或历史参数。许多开发工具都提供了菜单项，用于编辑启动文件并从系统框架启动新的启动文件。

另请参阅第 2.4 节和第 2.3 节中的 -s（脚本）和 -F（系统范围初始化）。