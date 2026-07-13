# 调试器概览

像 C++、Python 或 JavaScript 这样的命令式语言，大多执行近似线性的代码，中间带有一些分支和子例程调用。它们的调试器支持逐行单步执行并在每一行暂停，或者运行程序直到命中断点后暂停。暂停时，用户可以检查当前程序状态，或向调试器发出命令。

Prolog 的执行模型是逻辑式的：它会尝试证明逻辑谓词，因此需要不同的调试方式。SWI-Prolog 使用传统 Prolog 的“Byrd 盒模型”（Byrd Box Model）或“四端口模型”（4 Port Model）作为命令行调试器的基础，并在此基础上做了一些扩展。还有另外两个调试器也建立在这套基础设施上：一个是[图形调试器](https://www.swi-prolog.org/gtrace.html)，另一个是 [SWISH](https://swish.swi-prolog.org/) 提供的 Web 界面中的远程调试。

所有可用于操纵调试器的谓词，其参考信息见调试器章节。

## Byrd 盒模型和端口

标准 Prolog 调试工具围绕所谓的“Byrd 盒模型”或“四端口模型”构建。该模型把 Prolog 程序中的每个谓词建模为一个状态机，也就是一个“盒子”；程序求值时，这个盒子会在若干状态，也就是“端口”之间转换。开发者可以要求引擎在到达特定端口或特定谓词时暂停，以便检查程序。

阅读本概览时，请记住：“端口”只是“状态”的另一种说法；每个谓词在求值期间都会穿过这些状态。这个状态机被称为“盒子”，是因为它通常画成这样：

```text
                *--------------------------------------*
         Call   |                                      |   Exit
     ---------> +  descendant(X,Y) :- offspring(X,Y).  + --------->
                |                                      |
                |  descendant(X,Z) :-                  |
     <--------- +     offspring(X,Y), descendant(Y,Z). + <---------
         Fail   |                                      |   Redo
                *--------------------------------------*
```

标准端口包括：`call`、`redo`、`exit` 和 `fail`。SWI-Prolog 在此基础上扩展了两个端口：`unify` 和 `exception`。每条 trace 都发生在谓词解析的某个特定阶段。回忆一下，在解析或“证明”一个谓词时，Prolog 引擎会：

1. 收集所有“可能”匹配的规则，也就是头部具有相同名称和参数数量的规则。
   - 如果有任何规则可能匹配，会 trace 一次 `call`。
   - 当引擎回溯以寻找下一条匹配规则时，也会 trace `redo`。
2. 找到下一条其头部可以与该谓词合一的规则。
   - 如果找到这样的规则，会 trace `unify`，并显示合一结果。
   - 如果没有任何规则头可以合一，则 trace `fail`。
3. 将合一得到的变量赋值应用到规则体中的子句，然后带着更新后的子句继续第 1 步。
4. 在匹配规则的所有规则体子句都成功、失败或抛出异常之后：
   - 如果它们全都成功，会 trace `exit`，表示该规则为真。
   - 如果其中任何一个失败，会 trace `fail`，表示该规则为假。
   - 如果其中任何一个抛出异常，会 trace `exception`。

这意味着，在某个谓词最初的 `call` 和该谓词 trace 结束之间，可能出现大量 trace。

## Trace 模式示例

谓词 `trace/0` 会开启“trace 模式”。默认情况下，它会为每个谓词的每个端口产生 trace，并在每个端口暂停，以便用户检查程序状态。通常这是在 Prolog 控制台窗口中完成的；但对于嵌入式 Prolog 系统，或 Prolog 作为守护进程运行的情况，也可以通过 [libssh](https://www.swi-prolog.org/pack/list?p=libssh) 包获得提示符来完成。

> 注意：如果原生图形插件 XPCE 可用，命令 `gtrace/0` 和 `gspy/1` 会激活图形调试器，而 `tdebug/0` 和 `tspy/1` 允许调试任意线程。

每个目标都使用 Prolog 谓词 `write_term/2` 打印。打印风格由 Prolog 标志 `debugger_write_options` 定义；可以修改该标志，也可以使用 tracer 的 `w`、`p` 和 `d` 命令修改。

下面是一个展示基本流程的调试会话示例。默认情况下，`unify` 端口关闭，因为对命令行调试器来说，它在多数情况下不会提供很多额外信息。

```prolog
is_a(rock1, rock).
is_a(rock2, rock).
color(rock1, red).

noun(X, Type) :- is_a(X, Type).
adjective(X, color, Value) :- color(X, Value).
```

```prolog
?- trace.
true.

[trace]  ?- noun(X, rock), adjective(X, color, red).
   Call: (11) noun(_9774, rock) ? creep
```

谓词 `trace/0` 打开了 trace 模式，现在每个提示符都会显示为 `[trace] ?-`。用户给出的初始查询是 `noun(X, rock), adjective(X, color, red)`，意思是寻找一个“红色的石头”。最后，触发的第一个端口是初始查询中第一个谓词的 `Call`，表示引擎将要寻找第一条匹配 `noun(_9774, rock)` 的规则。

按下空格键、`c` 或回车，会让 tracer 打印 `creep`，随后显示下一条 trace。tracer 还提供了许多其他命令，后文会介绍。

```prolog
is_a(rock1, rock).
is_a(rock2, rock).
color(rock1, red).

noun(X, Type) :- is_a(X, Type).
adjective(X, color, Value) :- color(X, Value).
```

```prolog
[trace]  ?- noun(X, rock), adjective(X, color, red).
...
   Call: (12) is_a(_9774, rock) ? creep
   Exit: (12) is_a(rock1, rock) ? creep
   Exit: (11) noun(rock1, rock) ? creep
...
```

接下来，`noun/2` 第一条子句的 `is_a/2` 调用得到 `call` trace，因为引擎正在寻找下一条匹配 `is_a(_9774, rock)` 的规则。由于确实存在可以合一的事实 `is_a(rock1, rock)`，trace 显示了 `exit`，也就是成功，并显示了该值。因为这是 `noun/2` 规则体中的最后一个谓词，所以 `noun/2` 本身也得到一个 `exit` trace，显示其头部的合一结果：`noun(rock1, rock)`。

```prolog
is_a(rock1, rock).
is_a(rock2, rock).
color(rock1, red).

noun(X, Type) :- is_a(X, Type).
adjective(X, color, Value) :- color(X, Value).
```

```prolog
[trace]  ?- noun(X, rock), adjective(X, color, red).
...
   Call: (11) adjective(rock1, color, red) ? creep
   Call: (12) color(rock1, red) ? creep
   Exit: (12) color(rock1, red) ? creep
   Exit: (11) adjective(rock1, color, red) ? creep
   X = rock1 ;
...
```

随后，Prolog 移动到初始查询中的下一个谓词 `adjective/3`，并以类似方式求解它。由于这是查询中的最后一个谓词，系统返回一个答案。按下 `;` 会请求下一个答案，并开始 Prolog 回溯。

```prolog
is_a(rock1, rock).
is_a(rock2, rock).
color(rock1, red).

noun(X, Type) :- is_a(X, Type).
adjective(X, color, Value) :- color(X, Value).
```

```prolog
[trace]  ?- noun(X, rock), adjective(X, color, red).
...
   Redo: (12) is_a(_9774, rock) ? creep
   Exit: (12) is_a(rock2, rock) ? creep
   Exit: (11) noun(rock2, rock) ? creep
   Call: (11) adjective(rock2, color, red) ? creep
   Call: (12) color(rock2, red) ? creep
   Fail: (12) color(rock2, red) ? creep
   Fail: (11) adjective(rock2, color, red) ? creep
false.
```

唯一需要 `redo` 的选择点，也就是需要回溯的地方，是 `noun/2` 中的 `is_a/2` 子句，因为还有一个潜在匹配 `is_a(rock2, rock)` 可以尝试合一。它能够合一，所以 trace 显示 `exit`；同时也使 `noun(rock2, rock)` 像前面一样以 `exit` 成功。

随着 trace 继续，可以看到 `color(rock2, red)` 激活了 `fail` 端口，因为无法证明该谓词，所以整个查询最终返回 `false`。

在输入 `notrace.` 关闭 trace 模式之前，之后提出的每个查询都会继续被 trace。

## Trace 模式选项：`leash/1` 和 `visible/1`

使用 `trace/0` 启用 trace 模式时，tracer 默认会在每个谓词命中的每个端口暂停，并等待命令。谓词 `leash/1` 可用于修改哪些端口会暂停。这是全局设置，因此修改后会一直保留，直到再次修改或 SWI-Prolog 重启。通过 `notrace/0` 禁用 tracer 不会影响哪些端口被 leash。

`leash/1` 的参数必须以 `+` 开头表示添加，或以 `-` 开头表示移除，后跟端口名称，例如 `call`、`exit` 等。也可以使用特殊项，例如 `all`，从而不必手动添加或移除每个端口。

如果只想在 `fail` 端口停止，可以这样使用 `leash/1`：

```prolog
?- leash(-all).
true.

?- leash(+fail).
true.

?- trace.
true.

[trace]  ?- noun(X, rock), adjective(X, color, red).
   Call: (11) noun(_3794, rock)
   Call: (12) is_a(_3794, rock)
   Exit: (12) is_a(rock1, rock)
   Exit: (11) noun(rock1, rock)
   Call: (11) adjective(rock1, color, red)
   Call: (12) color(rock1, red)
   Exit: (12) color(rock1, red)
   Exit: (11) adjective(rock1, color, red)
X = rock1 ;
   Redo: (12) is_a(_3794, rock)
   Exit: (12) is_a(rock2, rock)
   Exit: (11) noun(rock2, rock)
   Call: (11) adjective(rock2, color, red)
   Call: (12) color(rock2, red)
   Fail: (12) color(rock2, red) ? creep
   Fail: (11) adjective(rock2, color, red) ? creep
false.
```

现在，只有以 `Fail:` 开头的行后面带有 `creep`，因为那是 tracer 唯一暂停等待命令的时候。如果希望永不暂停，只想看到所有 trace，可以使用 `leash(-all)`，并且不要重新打开任何端口。

默认端口仍然会打印出来，因为另一个设置 `visible/1` 控制哪些端口会被打印。`visible/1` 的参数形式与 `leash/1` 相同。如果只想在 `fail` 端口停止并且只显示 `fail` 端口，可以这样使用 `leash/1` 和 `visible/1`：

```prolog
?- leash(-all).
true.

?- leash(+fail).
true.

?- visible(-all).
true.

?- visible(+fail).
true.

?- trace.
true.

[trace]  ?- noun(X, rock), adjective(X, color, red).
X = rock1 ;
   Fail: (12) color(rock2, red) ? creep
   Fail: (11) adjective(rock2, color, red) ? creep
false.
```

## 暂停时的 Trace 模式命令

当 tracer 在某个端口暂停时，除了按空格键之外，还可以做很多事情。所有动作都是单字符命令，并且无需等待回车就会执行，除非命令行选项 `--no-tty` 处于活动状态。暂停时按 `?` 或 `h` 也会打印这些命令的列表。

### 控制流命令

| 命令 | 键 | 说明 |
| --- | --- | --- |
| **Abort** | `a` | 中止 Prolog 执行，参见 `abort/0` |
| **Break** | `b` | 进入 Prolog break 环境，参见 `break/0` |
| **Creep** | `c` | 继续执行，在下一个端口停止；也可以按回车或空格 |
| **Exit** | `e` | 终止 Prolog，参见 `halt/0` |
| **Fail** | `f` | 强制当前目标失败 |
| **Find** | `/` | 搜索某个端口，见下文 Find 命令说明 |
| **Ignore** | `i` | 忽略当前目标，假装它已经成功 |
| **Leap** | `l` | 继续执行，在下一个 spy point 停止 |
| **No debug** | `n` | 以 no debug 模式继续执行 |
| **Repeat find** | `.` | 重复上一次 find 命令 |
| **Retry** | `r` | 撤销当前目标从 `call` 端口之后的所有动作，数据库和 I/O 动作除外，并从该目标的 `call` 端口恢复执行 |
| **Skip** | `s` | 继续执行，在当前目标的下一个端口停止，因此会跳过该目标子调用的所有端口 |
| **Spy** | `+` | 在当前谓词上设置 spy point，参见 `spy/1` |
| **No spy** | `-` | 从当前谓词移除 spy point，参见 `nospy/1` |
| **Up** | `u` | 继续执行，在父目标的下一个端口停止，因此会跳过当前目标及其所有子调用。该选项适合停止 trace 由失败驱动的循环 |

#### Find（`/`）说明和示例

Find（`/`）命令会继续执行，直到找到与 find 模式匹配的端口。输入 `/` 后，用户可以输入一行来指定要搜索的端口。该行由一组表示端口类型的字母组成，后面可以跟一个可选项；该项应与该端口正在运行的目标合一。如果没有指定项，则把它视为变量，也就是搜索指定类型的任意端口。如果给出原子，则任何函子名等于该原子的目标都会匹配。示例：

| 命令 | 说明 |
| --- | --- |
| `/f` | 搜索任意 `fail` 端口 |
| `/fe solve` | 搜索名称为 `solve` 的任意目标上的 `fail` 或 `exit` 端口 |
| `/c solve(a, _)` | 搜索对 `solve/2` 的调用，且第一个参数是变量或原子 `a` |
| `/a member(_, _)` | 搜索 `member/2` 上的任意端口。这等价于在 `member/2` 上设置 spy point |

### 信息命令

| 命令 | 键 | 说明 |
| --- | --- | --- |
| **Alternatives** | `A` | 显示所有还有替代分支的目标 |
| **Goals** | `g` | 显示父目标列表，也就是执行栈。注意，由于尾递归优化，一些父目标可能已经不存在 |
| **Help** | `h` | 显示可用选项；也可以按 `?` |
| **Listing** | `L` | 使用 `listing/1` 列出当前谓词 |

### 格式化命令

| 命令 | 键 | 说明 |
| --- | --- | --- |
| **Context** | `C` | 切换“显示上下文”。如果为 `on`，目标的上下文模块会显示在方括号中。默认值为 `off` |
| **Display** | `d` | 设置 `debugger_write_options` 的 `max_depth(Depth)` 选项，限制打印项的深度。另请参阅 `w` 和 `p` 选项 |
| **Print** | `p` | 将 Prolog 标志 `debugger_write_options` 设置为 `[quoted(true), portray(true), max_depth(10), priority(699)]`。这是默认值 |
| **Write** | `w` | 将 Prolog 标志 `debugger_write_options` 设置为 `[quoted(true), attributes(write), priority(699)]`，绕过 `portray/1` 等 |

## Trace 模式与 Trace Point

这里需要稍微绕开一下，说明几个容易混淆的相关谓词：如果只想 trace 一个谓词或一组选定谓词，可以使用 `trace/1` 或 `trace/2` 谓词设置 trace point。虽然它们使用同一个基础谓词名 `trace`，但这些谓词会忽略 `leash/1` 和 `visible/1` 的全局设置，并且在 trace 端口时不会暂停。它们实际上是另一个功能，只是也会输出 trace。

trace point 设置在某个特定谓词上，并 trace 该谓词的端口；无论当前是否处于 `trace/0` 的 trace 模式，它都会工作。如果使用 `trace/2` 变体，每个 trace point 可以 trace 不同端口。

```prolog
?- trace(is_a/2).
%         is_a/2: [all]
true.

?- noun(X, rock), adjective(X, color, red).
 T Call: is_a(_25702, rock)
 T Exit: is_a(rock1, rock)
X = rock1 ;
 T Redo: is_a(rock1, rock)
 T Exit: is_a(rock2, rock)
false.
```

请注意：这里不需要使用 `trace/0` 打开 trace 模式；它只输出执行 `is_a/2` 时命中的端口；并且程序从未暂停。

实际上，如果在使用 trace point 时又打开 trace 模式，情况会变得非常混乱，因为 trace point 基础设施本身也会被 trace。

```prolog
?- trace(is_a/2).
%         is_a/2: [all]
true.

?- trace.
true.

[trace]  ?- noun(X, rock), adjective(X, color, red).
   Call: (11) noun(_29318, rock) ? creep
   Call: (12) is_a(_29318, rock) ? creep
   Call: (13) print_message(debug, frame(user:is_a(_29318, rock), trace(call))) ? creep
   Call: (18) push_msg(frame(user:is_a(_29318, rock), trace(call))) ? creep
   Call: (21) exception(undefined_global_variable, '$inprint_message', _30046) ? creep
   Fail: (21) exception(undefined_global_variable, '$inprint_message', _30090) ? creep
   Exit: (18) push_msg(frame(user:is_a(_29318, rock), trace(call))) ? creep
   Call: (19) prolog:message(frame(user:is_a(_29318, rock), trace(call)), _30140, _30142) ? creep
   Fail: (19) prolog:message(frame(user:is_a(_29318, rock), trace(call)), _30140, _30142) ? creep
   Call: (19) message_property(debug, stream(_30192)) ? creep
   Fail: (19) message_property(debug, stream(_30192)) ? creep
   Call: (20) message_property(debug, prefix(_30200)) ? creep
   Fail: (20) message_property(debug, prefix(_30200)) ? creep
 T Call: is_a(_29318, rock)
   Call: (17) pop_msg ? creep
   Exit: (17) pop_msg ? creep
   ...Lots more after this...
```

所以，trace point 和 trace mode 是两个名称容易混淆、但彼此独立的功能。

## Spy Point 和 Debug 模式

回到 trace 模式相关功能：由于 Prolog 程序的 trace 输出通常可能非常大，有时希望只在程序深处的某个特定点开始 trace 模式。这正是 spy point 的用途。它指定某个谓词，命中该谓词时应打开 trace 模式。

可以这样启用 spy point：`spy(mypredicate/2)`。执行该命令后，第一次遇到 `mypredicate/2` 时会打开 trace 模式，并像平常一样工作。这包括遵守全局的 `leash/1` 和 `visible/1` 设置。可以使用 `nospy/1` 或 `nospyall/0` 移除 spy point。

```prolog
is_a(rock1, rock).
is_a(rock2, rock).
color(rock1, red).

noun(X, Type) :- is_a(X, Type).
adjective(X, color, Value) :- color(X, Value).
```

```prolog
?- spy(is_a/2).
% Spy point on is_a/2
true.

[debug]  ?- noun(X, rock), adjective(X, color, red).
 * Call: (12) is_a(_1858, rock) ? creep
 * Exit: (12) is_a(rock1, rock) ? creep
   Exit: (11) noun(rock1, rock) ? creep
   Call: (11) adjective(rock1, color, red) ? creep
   Call: (12) color(rock1, red) ? creep
   Exit: (12) color(rock1, red) ? creep
   Exit: (11) adjective(rock1, color, red) ? creep
X = rock1 ;
 * Redo: (12) is_a(_1858, rock) ? creep
 * Exit: (12) is_a(rock2, rock) ? creep
   Exit: (11) noun(rock2, rock) ? creep
   Call: (11) adjective(rock2, color, red) ? creep
   Call: (12) color(rock2, red) ? creep
   Fail: (12) color(rock2, red) ? creep
   Fail: (11) adjective(rock2, color, red) ? creep
false.
```

命中 spy point 后，上面的输出与对初始查询运行 `trace/0` 得到的 trace 相同，只是显然缺少 spy point 之前的所有 trace。

注意，调用 `spy/1` 后，`?-` 前面会出现一个新标签，也就是 `[debug]`：

```prolog
?- spy(is_a/2).
% Spy point on is_a/2
true.

[debug] ?-
```

这表示系统处于“debug 模式”。debug 模式做两件事：告诉系统监视 spy point；关闭一些会让 trace 难以理解的优化。许多 Prolog 书籍中描述的理想四端口模型，在许多 Prolog 实现中不可见，因为代码优化会移除一部分选择点和 exit 点。如果某个目标确定性成功，或其替代分支被 cut 移除，回溯点不会显示。运行在 debug 模式时，选择点只有在被 cut 移除时才会销毁，并且最后调用优化会关闭。注意：这意味着系统在 debug 模式下可能耗尽栈，而非 debug 模式下不会出现问题。

可以使用 `nodebug/0` 再次关闭 debug 模式，但这样 spy point 会被忽略，虽然仍会被记住。通过 `debug/0` 重新打开 debug 模式后，会再次命中 spy point。

```prolog
is_a(rock1, rock).
is_a(rock2, rock).
color(rock1, red).

noun(X, Type) :- is_a(X, Type).
adjective(X, color, Value) :- color(X, Value).
```

```prolog
?-  spy(is_a/2).
% Spy point on is_a/2
true.

[debug]  ?- nodebug.
true.

?- noun(X, rock).
X = rock1 ;
X = rock2.

?- debug.
true.

[debug]  ?- noun(X, rock).
 * Call: (11) is_a(_47826, rock) ? creep
 * Exit: (11) is_a(rock1, rock) ? creep
   Exit: (10) noun(rock1, rock) ? creep
X = rock1 ;
 * Redo: (11) is_a(_47826, rock) ? creep
 * Exit: (11) is_a(rock2, rock) ? creep
   Exit: (10) noun(rock2, rock) ? creep
X = rock2.
```

因此，debug 模式允许 Prolog 监视 spy point，并在命中 spy point 时启用 trace 模式。谓词 `tracing/0` 和 `debugging/0` 会报告系统是否处于这些模式之一。

## 断点

有时候，连 spy point 也不够。某个谓词可能在许多地方使用，而只在某一次特定调用发生时打开 trace 模式会更有帮助。断点允许在命中特定源文件、行号以及该行中的字符位置时打开 trace 模式。使用的谓词是 `set_breakpoint/4` 和 `set_breakpoint/5`。可以同时激活多个断点。

注意，这些谓词提供的接口并不面向最终用户。内置的 PceEmacs 编辑器也嵌入在图形调试器中，它允许根据光标位置设置断点。

现在，`Example.pl` 修改为包含多个对 `noun/2` 的调用：

```prolog
is_a(rock1, rock).
is_a(rock2, rock).
color(rock1, red).

noun(X, Type) :- is_a(X, Type).
adjective(X, color, Value) :- color(X, Value).
test_noun1(X, Type) :- noun(X, Type).
test_noun2(X, Type) :- noun(X, Type).
```

如果只想在 `noun/2` 从 `test_noun2/2` 调用时启用 trace，可以这样使用 `set_breakpoint/4`：

```prolog
?- set_breakpoint('/...path.../Example.pl', 8, 24, ID).
% Breakpoint 1 in 1-st clause of test_noun2/2 at Example.pl:8
ID = 1.

?- debug.
true.

[debug]  ?- noun(X, rock).
X = rock1 .

[debug]  ?- test_noun1(X, rock).
X = rock1 .

[debug]  ?- test_noun2(X, rock).
   Call: (11) noun(_44982, rock) ? creep
   Call: (12) is_a(_44982, rock) ? creep
   Exit: (12) is_a(rock1, rock) ? creep
   Exit: (11) noun(rock1, rock) ? creep
   Exit: (10) test_noun2(rock1, rock) ? creep
X = rock1 .

[trace]  ?- notrace.
true.

[debug]  ?-
```

调用 `set_breakpoint/4` 时必须指定源文件 `Example.pl`、行号 `8` 和该行中的字符位置 `24`，以精确指出哪条子句应该打开 trace 模式。使用图形调试器时这会容易得多，因为它会显示源代码。

如果系统不在 debug 模式下，断点不会触发。与设置 spy point 不同，`set_breakpoint/4` 不会自动进入 debug 模式。因此，示例中使用 `debug/0` 手动打开了 debug 模式。

输出显示，只有对 `test_noun2/2` 的调用，也就是设置断点的位置，实际打开了 trace 模式。注意，结尾处的 `[trace] ?-` 表明 trace 模式在触发后仍保持开启。可以通过 `notrace/0` 再次关闭它，这会让系统留在 debug 模式。调用 `nodebug/0` 可以一次关闭所有调试模式，因为关闭 debug 模式会自动关闭 trace 模式。

此外，SWI-Prolog 支持通过 `set_breakpoint_condition/2` 为每个断点附加任意目标，从而得到条件断点。条件断点与前面讨论的普通断点相同，只是每当断点被触发时，系统会调用给定目标；只有在该目标成功时，trace 模式才会打开。

如果只想在 `noun/2` 从 `test_noun2/2` 调用，并且第一个参数为 `rock2` 时启用 trace，可以如下使用 `set_breakpoint_condition/2`。注意，条件是一个 Prolog 字符串；系统会解析该字符串以获得目标以及变量名。得到的目标会在执行该子句体的模块中调用，参见 `clause_property/2` 的 `module` 属性。

```prolog
?- set_breakpoint('/...path.../Example.pl', 8, 24, ID).
ID = 1.

?- set_breakpoint_condition(1, "X == rock2").
true.

?- debug.
true.

[debug]  ?- test_noun2(X, rock).
X = rock1 ;
X = rock2.

[debug]  ?- test_noun2(rock2, rock).
   Call: (11) noun(rock2, rock) ? creep
   Call: (12) is_a(rock2, rock) ? creep
   Exit: (12) is_a(rock2, rock) ? creep
   Exit: (11) noun(rock2, rock) ? creep
   Exit: (10) test_noun2(rock2, rock) ? creep
true.

[trace]  ?-
```

## 命令行调试器总结

总结来说，实际上存在两个不同的“tracing”功能：trace 模式和 trace point。两者都会使用 Byrd 盒模型向控制台写出 trace，但相似之处到此为止。

### Trace 模式

Trace 模式是主要的 Prolog 命令行调试器。它可以 trace 谓词穿过解析状态的过程，这些状态在 Byrd 盒模型中表示为端口；也可以在命中特定端口时暂停，让用户发出命令。

可以通过 `trace/0` 手动打开 trace 模式。也可以在使用 `debug/0` 进入 debug 模式后，通过 `spy/1` 在遇到某个特定谓词时打开；或者通过 `set_breakpoint/4`、`set_breakpoint/5` 在遇到某个谓词的特定调用时打开。

处于 trace 模式时，`visible/1` 控制哪些端口会写到控制台，`leash/1` 控制哪些端口会导致执行暂停，以便检查程序。

执行暂停时，可以使用许多命令检查程序状态、让目标失败或成功，等等。

Trace 模式通过 `notrace/0` 关闭，debug 模式通过 `nodebug/0` 关闭。

### Trace Point

Trace point 是一个独立于 trace 模式的功能，它允许在某个谓词求值时把指定端口写到控制台。它永远不会暂停程序执行，也不需要处于 trace 或 debug 模式。

Trace point 通过 `trace/1` 和 `trace/2` 打开。

它们不关注 `visible/1`，因为显示哪些端口是在 `trace/2` 中设置的；也不关注 `leash/1`，因为它们不会暂停执行。

可以通过 `trace/2` 关闭它们。
