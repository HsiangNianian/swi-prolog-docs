# 快速入门

## 1.1 启动 SWI-Prolog

### 1.1.1 在 Unix 上启动 SWI-Prolog

默认情况下，SWI-Prolog 安装后的可执行程序名为 `swipl`。SWI-Prolog 本身及其实用程序的命令行参数使用标准 Unix `man` 手册页记录。SWI-Prolog 通常作为交互式应用程序运行，直接启动程序即可：

```shell
$ swipl
Welcome to SWI-Prolog ...
...

1 ?-
```

启动 Prolog 后，通常使用 `consult/1` 将程序载入系统。也可以把程序文件名放在方括号中作为简写。下面的目标会载入文件 [`likes.pl`](https://raw.githubusercontent.com/SWI-Prolog/swipl-devel/master/demo/likes.pl)，其中包含谓词 `likes/2` 的子句：

```prolog
?- [likes].
true.

?-
```

也可以把源文件作为命令行参数传给 `swipl`：

```shell
$ swipl likes.pl
Welcome to SWI-Prolog ...
...

1 ?-
```

> 上面两个例子都假设 `likes.pl` 位于当前工作目录。如果使用命令行版本 `swipl`，工作目录就是启动 SWI-Prolog 的 shell 所在目录。如果启动的是 GUI 版本 `swipl-win`，工作目录则主要取决于操作系统。可以使用 `pwd/0` 和 `cd/0` 查看和修改工作目录。实用程序 `ls/0` 会列出工作目录内容。
>
> ```prolog
> ?- pwd.
> % /home/janw/src/swipl-devel/linux/
> true.
> ?- cd('~/tmp').
> true.
>
> ?- pwd.
> % /home/janw/tmp/
> true.
> ```
>
> 文件 `likes.pl` 也安装在 SWI-Prolog 安装目录的 `demo` 子目录中，因此可以不依赖当前工作目录，用下面的命令载入。关于 SWI-Prolog 如何指定文件位置，请参阅 `absolute_file_name/3` 和 `file_search_path/2`。
>
> ```prolog
> ?- [swi(demo/likes)].
> true.
> ```

从这里开始，Unix 和 Windows 用户面对的内容相同。如果你使用 Unix，可以继续阅读“从控制台添加规则”。

### 1.1.2 在 Windows 上启动 SWI-Prolog

在 Windows 系统上安装 SWI-Prolog 后，用户会得到以下重要内容：

- 一个名为 `swipl` 的文件夹，本文档后续称为目录；其中包含系统的可执行文件、库等。不会在该目录之外安装文件。
- 程序 `swipl-win.exe`，提供一个用于与 Prolog 交互的窗口。程序 `swipl.exe` 是运行在控制台窗口中的 SWI-Prolog 版本。
- 文件扩展名 `.pl` 会关联到程序 `swipl-win.exe`。打开 `.pl` 文件会启动 `swipl-win.exe`，切换到被打开文件所在目录，并载入该文件。

启动前文提到的 `likes.pl` 文件，通常只需在 Windows 资源管理器中双击该文件。

## 1.2 从控制台添加规则

虽然强烈建议把程序放在文件中，并在需要时编辑文件再使用 `make/0` 重新载入，但也可以直接从终端管理事实和规则。添加少量子句最方便的方法是 consult 伪文件 `user`。输入用系统的文件结束字符结束。

```prolog
?- [user].
|: hello :- format('Hello world~n').
|: ^D
true.

?- hello.
Hello world
true.
```

谓词 `assertz/1` 和 `retract/1` 也可以用来添加和删除规则或事实。

## 1.3 执行查询

载入程序后，就可以向 Prolog 询问关于该程序的问题。下面的查询询问 Prolog：`sam` 喜欢什么食物。如果系统可以证明某个 `X` 满足该目标，它会以 `X = <value>` 的形式回答。若用户想要另一个解，可以输入分号 `;` 或空格键。若不想查看更多答案，按回车即可。如果用户按回车，或者 Prolog 知道已经没有更多答案，Prolog 会用句号 `.` 结束输出。如果 Prolog 找不到更多答案，会写出 `false.`。最后，如果查询或程序包含错误，Prolog 会用错误消息回答。

```prolog
?- likes(sam, X).
X = dahl ;
X = tandoori ;
...
X = chips.

?-
```

注意，Prolog 写出的答案本身也是有效的 Prolog 程序；执行它可以得到与原程序相同的一组答案。

## 1.4 检查和修改程序

如果配置正确，谓词 `edit/1` 会根据参数启动内置编辑器或用户配置的编辑器。参数可以是任何能够关联到位置的对象：文件名、谓词名、模块名等。如果参数只解析到一个位置，编辑器会在该位置打开；否则系统会让用户选择。

如果存在图形用户界面，编辑器通常会创建新窗口，而系统会继续提示下一个命令。用户可以编辑源文件、保存文件，然后运行 `make/0` 来更新所有修改过的源文件。如果编辑器无法在窗口中打开，它会在同一控制台中打开；退出编辑器时会运行 `make/0`，重新载入任何已修改的源文件。

```prolog
?- edit(likes).

true.
?- make.
% /home/jan/src/pl-devel/linux/likes compiled 0.00 sec, 0 clauses

?- likes(sam, X).
...
```

也可以使用 `listing/1` 反编译程序，如下所示。`listing/1` 的参数可以只是谓词名，也可以是形如 `Name/Arity` 的谓词指示器，例如 `?- listing(mild/1).`；还可以是一个头，例如 `?- listing(likes(sam, _)).`，用于列出所有匹配的子句。不带参数的 `listing/0` 会列出整个程序。

```prolog
?- listing(mild).
mild(dahl).
mild(tandoori).
mild(kurma).

true.
```

## 1.5 停止 Prolog

交互式顶层可以通过两种方式停止：输入系统文件结束字符，通常是 `Control-D`；或者执行谓词 `halt/0`：

```prolog
?- halt.
$
```
