# UI 主题

UI（颜色）主题在两个部分发挥作用：写入控制台时，以及用于基于 xpce 的开发工具时，例如 PceEmacs 或图形调试器。彩色控制台输出基于 `ansi_format/3`。基于 `print_message/2` 的中央消息基础设施会用一个 Prolog 项标记消息或消息组件，以说明其角色。钩子 `prolog:console_color/2` 会把这些角色映射到具体颜色。IDE 主题则使用 xpce 类变量；这些变量会在载入 xpce 时由 Prolog 初始化。

主题实现为 Prolog 文件，位于文件搜索路径 `library/theme` 中。可以在用户初始化文件中使用下面这样的指令载入主题：

```prolog
:- use_module(library(theme/dark)).
```

系统提供了主题文件 `library(theme/auto)`，用于根据环境自动选择一个合理的主题。当前版本会在兼容 `xterm` 的终端模拟器上检测背景色；这类终端常见于多数 Unix 系统。如果背景色偏暗，则载入 `dark` 主题。

下面是 SWI-Prolog 支持平台上的一些注意事项：

- Unix/Linux

  如果使用兼容 xterm 的终端模拟器运行 Prolog，可能需要显式载入某个主题，或载入 `library(theme/auto)`。

- Epilog Prolog 控制台

  图形应用程序 `swipl-win` 可以通过载入主题文件来设置主题。主题文件也会设置 Epilog 控制台的前景色和背景色。

## 5.1 主题支持状态

主题支持是在 SWI-Prolog 8.1.11 中加入的。目前只覆盖了一部分 IDE 工具，唯一额外提供的主题 `dark` 也还没有很好地平衡。主题文件与 IDE 组件之间的接口，尤其是相关组件接口，还没有很好地稳定下来。欢迎通过改进 `dark` 主题做出贡献。等它完整并正常工作后，就可以开始添加新的主题。
