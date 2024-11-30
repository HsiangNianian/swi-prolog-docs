# UI 主题

UI（颜色）主题在两个方面发挥作用：写入控制台时以及用于基于 xpce 的开发工具（如 PceEmacs 或图形调试器）。彩色控制台输出基于 ansi_format/3。基于 print_message/2 的中央消息基础结构使用指定角色的 Prolog 术语标记消息（组件）。这通过钩子 prolog:console_color/2 映射到具体颜色。IDE 主题使用 xpce 类变量，这些变量在加载 xpce 时从 Prolog 初始化。

主题在文件搜索路径 library/theme 中作为 Prolog 文件实现。可以使用（例如）用户初始化文件中的以下指令加载主题（参见第 2.2 节）。

```pl
:- use_module(library(theme/dark)).
```

 The theme file library(theme/auto) is provided to automatically choose a reasonable theme based on the environment. The current version detects the background color on xterm compatible terminal emulators (found on most Unix systems) and loads the dark theme if the background is‘darkish’.

The following notes apply to the different platforms on which SWI-Prolog is supported:

Unix/Linux
    If an xterm compatible terminal emulator is used to run Prolog you may wish to load either an explicit theme or library(theme/auto).
Windows
    The swipl-win.exe graphical application can be themed by loading a theme file. The theme file also sets the foreground and background colours for the console. 

## 5.1 主题支持状态

SWI-Prolog 8.1.11 中添加了主题支持。仅涵盖了部分 IDE 工具，并且唯一的附加主题（深色）不太平衡。主题文件与 IDE 组件之间的接口尚未建立。请通过改进深色主题做出贡献。一旦完成并正常运行，我们就可以开始添加新主题。