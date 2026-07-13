# 有理树（循环项）

SWI-Prolog 支持有理树（rational trees），也称为循环项（cyclic terms）。这里的“支持”定义为：面对有理树时，大多数相关内置谓词都能终止。SWI-Prolog 几乎所有内置项操作谓词，都能以与该项在栈上表示所用内存量成线性关系的时间处理项。下面这些谓词可以安全处理有理树：

`=../2`、`==/2`、`=@=/2`、`=/2`、`@</2`、`@=</2`、`@>=/2`、`@>/2`、`\==/2`、`\=@=/2`、`\=/2`、`acyclic_term/1`、`bagof/3`、`compare/3`、`copy_term/2`、`cyclic_term/1`、`dif/2`、`duplicate_term/2`、`findall/3`、`ground/1`、`term_hash/2`、`numbervars/3`、`numbervars/4`、`recorda/3`、`recordz/3`、`setof/3`、`subsumes_term/2`、`term_variables/2`、`throw/1`、`unify_with_occurs_check/2`、`unifiable/3`、`when/2`、`write/1`（以及相关谓词）。

此外，一些内置谓词会识别有理树并抛出合适的异常。算术求值属于这一类。编译器（`asserta/1` 等）也会抛出异常。未来版本可能支持有理树。能够对有理树提供有意义处理的谓词会抛出 `representation_error`。对于没有有意义解释的有理树，相关谓词会抛出 `type_error`。例如：

```prolog
1 ?- A = f(A), asserta(a(A)).
ERROR: asserta/1: Cannot represent due to `cyclic_term'
2 ?- A = 1+A, B is A.
ERROR: is/2: Type error: `expression' expected, found
             `@(S_1,[S_1=1+S_1])' (cyclic term)
```
