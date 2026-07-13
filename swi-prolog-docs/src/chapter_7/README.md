# 在线帮助

## 7.1 `library(help)`：基于文本的手册

该模块提供 `help/1` 和 `apropos/1`。前者给出某个主题的帮助，后者在手册中搜索相关主题。

默认情况下，`help/1` 的结果会通过诸如 `less` 这样的分页器发送。该行为由以下内容控制：

- Prolog 标志 `help_pager`，可以设置为下列值之一：

  - `false`

    永不使用分页器。

  - `default`

    使用默认行为。系统会尝试判断 Prolog 是否运行在允许分页器的交互式环境中。如果是，它会检查环境变量 `PAGER`；否则尝试查找 `less` 程序。

  - `Callable`

    `Callable` 项会被解释为 `program_name(Arg, ...)`。例如，`less('-r')` 可以作为默认值。注意，如果使用单引号，程序名可以是绝对路径。

### `help`

### `help(+What)`

显示 `What` 的帮助。`What` 是一个描述帮助主题的项。`What` 的可用记法包括：

- `Atom`

  最常用但也有歧义的形式，会显示所有匹配文档。例如：

  ```prolog
  ?- help(append).
  ```

- `Name/Arity`

  显示匹配 `Name/Arity` 的谓词帮助。`Arity` 可以未绑定。

- `Name//Arity`

  显示匹配的 DCG 规则，也就是非终结符。

- `Module:Name`

  显示 `Module` 中名为 `Name`、任意 arity 的谓词帮助。仅用于已载入代码。

- `Module:Name/Arity`

  显示 `Module` 中名为 `Name` 且 arity 为 `Arity` 的谓词帮助。仅用于已载入代码。

- `f(Name/Arity)`

  显示匹配的 Prolog 算术函数帮助。

- `c(Name)`

  显示匹配的 C 接口函数帮助。

- `section(Label)`

  显示手册中匹配 `Label` 的章节。

`help/1` 会显示手册中的文档，也会显示已载入用户代码中的文档，前提是该代码使用 PlDoc 编写了文档。若只想显示已载入谓词的文档，可以在谓词指示器前加上定义该谓词的模块。

如果精确匹配失败，该谓词会尝试模糊匹配；如果成功，会显示结果，并在开头给出警告，说明这些匹配基于模糊匹配。

如果可能，结果会通过诸如 `less` 的分页器发送。该行为由 Prolog 标志 `help_pager` 控制。参见本节开头的说明。

另请参阅 `apropos/1`，它用于搜索手册名称和摘要。

### `show_html_hook(+HTML:string)`

这是一个半确定的 multifile 钩子，用于显示抽取出的 HTML 文档。如果该钩子失败，HTML 会使用 `html_text/2` 渲染为纯文本并显示在控制台上。

### `apropos(+Query)`

打印手册中名称或摘要与 `Query` 匹配的对象。`Query` 可采用下列形式：

- `Type:Text`

  查找与 `Text` 匹配的对象，并按 `Type` 过滤结果。`Type` 匹配采用大小写不敏感的前缀匹配。已定义的类型包括 `section`、`cfunction`、`function`、`iso_predicate`、`swi_builtin_predicate`、`library_predicate`、`dcg`，以及别名 `chapter`、`arithmetic`、`c_function`、`predicate`、`nonterminal` 和 `non_terminal`。例如：

  ```prolog
  ?- apropos(c:close).
  ?- apropos(f:min).
  ```

- `Text`

  `Text` 会被拆分为多个 token。如果某个主题的名称或摘要中包含所有 token，则认为匹配。匹配不区分大小写。结果会按匹配质量排序。

### `help_apropos(+Query, -Obj, -Summary, -Score)`

在帮助数据库中查找匹配的已文档化对象。`Obj` 是正式对象标识符，`Summary` 是摘要说明，`Score` 是表示匹配质量的数字。

### `help_text(+Predicate:term, -HelpText:string)`

如果 `Predicate` 是形如 `Name/Arity` 的项，且存在对应文档，则 `HelpText` 是从 HTML 帮助解析出的文本格式文档。

## 7.2 `library(explain)`：描述 Prolog 项

`library(explain)` 用于描述 Prolog 项。它最有用的功能是交叉引用能力。

```prolog
?- explain(subset(_,_)).
"subset(_, _)" is a compound term
    from 2-th clause of lists:subset/2
    Referenced from 46-th clause of prolog_xref:imported/3
    Referenced from 68-th clause of prolog_xref:imported/3
lists:subset/2 is a predicate defined in
    /staff/jan/lib/pl-5.6.17/library/lists.pl:307
    Referenced from 2-th clause of lists:subset/2
    Possibly referenced from 2-th clause of lists:subset/2
```

注意，PceEmacs 可以跳转到定义，`gxref/0` 可用于查看依赖概览。

### `explain(@Term)`

解释 `Term`。`Term` 可以是任意 Prolog 数据对象。某些项具有特殊含义：

- 对谓词的完整或部分引用，会给出谓词、它的主要属性以及对该谓词的引用。部分引用包括：

  - `Module:Name/Arity`
  - `Module:Head`
  - `Name/Arity`
  - `Name//Arity`
  - `Name`
  - `Module:Name`

- 某些谓词属性。这会列出具有该属性的谓词，谓词形式如上。该说明可以写成 `Module:Property`，也可以只写 `Property`。带模块限定的版本会把结果限制到 `Module` 中定义的谓词。支持的属性包括：

  - `dynamic`
  - `thread_local`
  - `multifile`
  - `tabled`

### `explain(@Term, -Explanation)`

当 `Explanation` 是 `Term` 的解释时为真。`Explanation` 是一个元素列表，会使用 `print_message(information, explain(Explanation))` 打印。
