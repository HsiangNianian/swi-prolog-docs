/// # 快速入门
///
/// ## 1.1 在 Unix 上开始 SWI-Prolog
///
/// 默认情况下，SWI-Prolog 安装为“swipl”。SWI-Prolog 本身及其实用程序的命令行参数使用标准 Unix 手册页记录。SWI-Prolog 通常作为交互式应用程序运行，只需启动程序即可：
///
/// ```pl
/// $ swipl
/// Welcome to SWI-Prolog ...
/// ...
///
/// 1 ?-
/// ```
///
/// 启动 Prolog 后，通常使用 consult/1 将程序加载到其中，可以通过将程序文件的名称放在方括号中来缩写。以下目标加载文件 likes.pl，其中包含谓词 likes/2 的子句：
///
/// ```pl
/// ?- [likes].
/// true.
///
/// ?-
/// ```
///
/// 或者，源文件也可以作为命令行参数给出：
///
/// ```shell
/// $ swipl likes.pl
/// Welcome to SWI-Prolog ...
/// ...
///
/// 1 ?-
/// ```
///
/// > 以上两个假设 likes.pl 位于您的工作目录中。如果您使用命令行版本 swipl，则工作目录与您启动 SWI-Prolog 的 shell 相同。如果您启动的是 GUI 版本 (swipl-win)，则这在很大程度上取决于操作系统。您可以使用 pwd/0 和 cd/0 来查找和更改工作目录。实用程序 ls/0 列出了工作目录的内容。
/// >
/// > ```shell
/// > ?- pwd.
/// > % /home/janw/src/swipl-devel/linux/
/// > true.
/// > ?- cd('~/tmp').
/// > true.
/// >
/// > ?- pwd.
/// > % /home/janw/tmp/
/// > true.
/// > ```
/// >
/// > 文件 likes.pl 也安装在 SWI-Prolog 安装目录内的子目录 demo 中，可以使用以下命令加载，而不管工作目录如何。有关 SWI-Prolog 如何指定文件位置的详细信息，请参阅 absolute_file_name/3 和 file_search_path/2。
/// >
/// > ```shell
/// > ?- [swi(demo/likes)].
/// > true.
/// > ```
///
/// 此后，Unix 和 Windows 用户统一起来，因此如果您使用 Unix，请继续阅读第 1.2 节。
///
/// ## 1.2 在 Windows 上启动 SWI-Prolog
///
/// 在 Windows 系统上安装 SWI-Prolog 后，用户可以使用以下重要的新功能：
///
/// 一个名为 swipl 的文件夹（本文档的其余部分称为目录），其中包含系统的可执行文件、库等。此目录之外不安装任何文件。
/// 程序 swipl-win.exe，提供与 Prolog 交互的窗口。程序 swipl.exe 是在控制台窗口中运行的 SWI-Prolog 版本。
/// 文件扩展名 .pl 与程序 swipl-win.exe 相关联。打开 .pl 文件将导致 swipl-win.exe 启动，将目录更改为要打开的文件所在的目录，并加载此文件。
///
/// 启动第 2.1.1.1 节中提到的 likes.pl 文件的正常方法是简单地在 Windows 资源管理器中双击此文件。
///
/// ## 1.2 从控制台添加规则
/// 尽管我们强烈建议将程序放在文件中，也可以选择编辑它并使用 make/0 重新加载它（参见第 2.1.4 节），但可以从终端管理事实和规则。添加一些子句的最方便的方法是咨询伪文件用户。输入以系统文件结束字符结束。
///
/// ```shell
/// ?- [user].
/// |: hello :- format('Hello world~n').
/// |: ^D
/// true.
///
/// ?- hello.
/// Hello world
/// true.
/// ```
///
/// 谓词 assertz/1 和 retract/1 是添加和删除规则和事实的替代方法。
///
/// ## 1.3 执行查询
///
/// 加载程序后，可以向 Prolog 询问有关该程序的查询。下面的查询询问 Prolog 食物‘sam’喜欢什么。如果系统可以证明某个 X 的目标，则系统会响应 X = <value>。如果用户想要另一个解决方案，可以键入分号 (;) 或空格键 7。如果不想看到更多答案，请使用回车键。如果用户使用回车键或 Prolog 知道没有更多答案，则 Prolog 会用句号 (.) 完成输出。如果 Prolog 找不到（更多）答案，它会写入 false。最后，Prolog 使用错误消息来回答，以表明查询或程序包含错误。
///
/// ```shell
/// ?- likes(sam, X).
/// X = dahl ;
/// X = tandoori ;
/// ...
/// X = chips.
///
/// ?-
/// ```
///
/// 请注意，Prolog 编写的答案是一个有效的 Prolog 程序，执行时会产生与原始程序相同的答案集。
///
/// ## 1.4 检查和修改程序
///
/// 如果配置正确，谓词 edit/1 会根据参数启动内置或用户配置的编辑器。参数可以是任何可以链接到位置的内容：文件名、谓词名称、模块名称等。如果参数仅解析为一个位置，则编辑器将在此位置启动，否则将向用户提供选择。
///
/// 如果有图形用户界面，编辑器通常会创建一个新窗口，系统会提示下一个命令。用户可以编辑源文件，保存它并运行 make/0 来更新任何已修改的源文件。如果无法在窗口中打开编辑器，它将在同一控制台中打开，并让编辑器运行 make/0 来重新加载已修改的任何源文件。
///
/// ```shell
/// ?- edit(likes).
///
/// true.
/// ?- make.
/// % /home/jan/src/pl-devel/linux/likes compiled 0.00 sec, 0 clauses
///
/// ?- likes(sam, X).
/// ...
/// ```
///
/// 程序也可以使用 listing/1 进行反编译，如下所示。listing/1 的参数只是一个谓词名称，一个形式为 Name/Arity 的谓词指示符，例如 ?- listing(mild/1). 或一个头，例如 ?- listing(likes(sam, _)).，列出所有匹配子句。谓词 listing/0，即不带参数的谓词，列出了整个程序。9
///
/// ```shell
/// ?- listing(mild).
/// mild(dahl).
/// mild(tandoori).
/// mild(kurma).
///
/// true.
/// ```
///
/// ## 1.5 停止 Prolog
///
/// 交互式顶层可以通过两种方式停止：输入系统文件结束符（通常是 Control-D）或执行 halt/0 谓词：
///
/// ```shell
/// ?- halt.
/// $
/// ```
pub fn chapter_1() {}
