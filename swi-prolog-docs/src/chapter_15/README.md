# SWI-Prolog 语法

SWI-Prolog 的语法接近 ISO Prolog 标准语法，而 ISO Prolog 又基于 Edinburgh Prolog 语法。形式化描述可以在 ISO 标准文档中找到。若需要非正式介绍，可参考 Prolog 教科书以及[在线教程](http://www.swi-prolog.org/Links.html)。除了这里记录的 ISO 标准差异之外，SWI-Prolog 还提供了若干扩展，其中一部分也扩展了语法。更多信息见扩展相关章节。

## ISO 语法支持

本节列出 SWI-Prolog 相对于 ISO Prolog 语法的若干扩展。

### 处理器字符集

处理器字符集规定解析 Prolog 源文本时每个字符所属的类别。字符分类固定为 [Unicode](http://www.unicode.org/)。另见宽字符支持相关章节。

### 嵌套注释

SWI-Prolog 允许嵌套 `/* ... */` 注释。ISO 标准会把 `/* ... /* ... */` 接受为一段注释，而 SWI-Prolog 会继续寻找终止用的 `*/`。如果要把一段本身已经包含 `/* ... */` 注释的代码整体注释掉，这会很有用。这个修改也避免了下面示例中的意外注释问题：第一段注释的结束 `*/` 被忘记了。

```prolog
/* comment

code

/* second comment */

code
```

### 字符转义语法

在带引号的原子（使用单引号：`'<atom>'`）中，特殊字符用转义序列表示。转义序列由反斜杠（`\`）引入。转义序列列表兼容 ISO 标准，但包含一些扩展；为提升兼容性，数值指定字符的解释也略微更灵活。未定义的转义字符会抛出 `syntax_error` 异常。SWI-Prolog 6.1.9 及以前版本会逐字复制未定义转义字符，即去掉反斜杠。

| 转义 | 含义 |
| --- | --- |
| `\a` | 警告字符。通常是 ASCII 字符 7（响铃）。 |
| `\b` | 退格字符。 |
| `\c` | 不产生输出。跳过输入中直到第一个非布局字符之前的所有字符。这可用于写出更美观的长行。ISO 不支持。 |
| `\` 后接换行 | 在 ISO 模式中（见 Prolog 标志 `iso`），只跳过这个序列。在原生模式中，换行后的空白也会被跳过，并打印警告，指出该结构已弃用并建议使用 `\c`。 |
| `\e` | 转义字符（ASCII 27）。非 ISO，但被广泛支持。 |
| `\f` | 换页字符。 |
| `\n` | 下一行字符。 |
| `\r` | 仅回车，即回到行首。 |
| `\s` | 空格字符。用于允许写出 `0'\s` 来取得空格字符的字符码。非 ISO。 |
| `\t` | 水平制表符。 |
| `\v` | 垂直制表符（ASCII 11）。 |
| `\xXX..\` | 字符的十六进制表示。按 ISO 标准，结尾的 `\` 是必需的；但 SWI-Prolog 中它是可选的，以增强与较早 Edinburgh 标准的兼容性。代码 `\xa\3` 产生字符 10（十六进制 `a`）后接 `3`。这种方式指定的字符按 Unicode 字符解释。另见 `\u`。 |
| `\uXXXX` | Unicode 字符表示，其中字符用**恰好** 4 个十六进制数字指定。这是 ISO 标准的扩展，修复了两个问题：首先，`\x` 定义的是数字字符码，但没有指定该字符码应在哪个字符集中解释；其次，不再需要 ISO Prolog 中特别的结尾反斜杠语法。 |
| `\UXXXXXXXX` | 与 `\uXXXX` 相同，但使用 8 个数字，以覆盖整个 Unicode 集。 |
| `\40` | 八进制字符表示。十六进制表示的规则和说明同样适用于八进制表示。 |
| `\\` | 转义反斜杠本身。因此，`'\\'` 是一个只包含单个 `\` 的原子。 |
| `\'` | 单引号。注意，`'\''` 和 `''''` 都描述只包含单个 `'` 的原子，也就是说 `'\'' == ''''` 为真。 |
| `\"` | 双引号。 |
| `` \` `` | 反引号。 |

`\c` 的示例：

```prolog
format('This is a long line that looks better if it was \c
       split across multiple physical lines in the input')
```

对于反斜杠后接换行的长行写法，建议使用 `\c`，或者把布局字符放在 `\` 之前，如下所示。`\c` 被多个其他 Prolog 实现支持，并将继续被 SWI-Prolog 支持。下面这种风格是兼容性最好的方案。

```prolog
format('This is a long line that looks better if it was \
split across multiple physical lines in the input')
```

而不是：

```prolog
format('This is a long line that looks better if it was\
 split across multiple physical lines in the input')
```

注意，SWI-Prolog 也允许未转义的换行出现在带引号的材料中。ISO 标准不允许这样做，但过去这曾是常见实践。

只有在 `current_prolog_flag(character_escapes, true)` 生效时，字符转义才可用（默认如此）。见 `current_prolog_flag/2`。字符转义会以两种方式与 `writef/2` 冲突：`\40` 会被 `writef/2` 解释为十进制 40，但会被 `read` 解释为八进制 40（十进制 32）。此外，`writef/2` 序列 `\l` 是非法的。建议改用支持更广泛的 `format/[2,3]` 谓词。如果坚持使用 `writef/2`，可以把 Prolog 标志 `character_escapes` 切换为 `false`，也可以使用双重 `\\`，例如 `writef('\\l')`。

### 非十进制数的语法

SWI-Prolog 同时实现 Edinburgh 和 ISO 两种非十进制数表示。按照 Edinburgh 语法，这类数写作 `<radix>'<number>`，其中 `<radix>` 是 2 到 36 之间的数字。ISO 使用 `0[bxo]<number>` 定义二进制、八进制和十六进制数。例如，下面是一个合法表达式：

```prolog
A is 0b100 \/ 0xf00
```

这类数总是无符号的。

### 在大整数中使用数字分组

SWI-Prolog 支持把长整数拆分成数字分组（digit groups）。数字分组可以用“下划线 + 可选空白”的序列分隔。如果基数小于或等于 10，也可以用恰好一个空格分隔。下面几种写法都表示整数 100 万：

```prolog
1_000_000
1 000 000
1_000_/*more*/000
```

可以用 `format/2` 和格式说明符 `~I` 按这种记法打印整数。例如：

```prolog
?- format('~I', [1000000]).
1_000_000
```

当前语法由 Ulrich Neumerkel 在 SWI-Prolog 邮件列表上提出。

### 有理数语法

从 8.1.22 版开始，如果 SWI-Prolog 在编译时启用了 GMP 库，它会把有理数作为一等原子数据类型支持。这可以用 Prolog 标志 `bounded` 测试。原子类型也需要一种语法。遗憾的是，在不破坏 ISO 标准的前提下加入有理数，选择并不多。ECLiPSe 使用 `numerator_denominator`。该语法与 SWI-Prolog 的数字分组冲突（见上一节），而且没有被广泛认可为有理数写法。`1/3r` 和 `1/3R` 也曾被提出；`1/3r` 与 Ruby 兼容，但因为需要向前看而难以解析，也不太自然。另见 <https://en.wikipedia.org/wiki/Rational_data_type>。

ECLiPSe 和 SWI-Prolog 已同意把有理数的规范语法定义为类似 `1r3` 的形式。此外，ECLiPSe 接受 `1_3`；SWI-Prolog 可通过模块敏感的 Prolog 标志 `rational_syntax` 接受 `1/3`。该标志的取值如下。注意，`write_canonical/1` 始终使用兼容的 `1r3` 语法。

- `natural`：默认模式。在该模式下，系统忽略歧义问题，并采用最自然的 `<integer>/<nonneg>` 形式。这里，`<integer>` 遵循 Prolog 十进制整数的常规规则；`<nonneg>` 也遵循同样规则，但不允许符号。解析器会把有理数转换为规范形式，这意味着结果的分子和分母没有公因子。有理数示例：

  | 输入 | 规范形式 |
  | --- | --- |
  | `1/2` | `1/2` |
  | `2/4` | `1/2` |
  | `1 000 000/33 000` | `1000/33` |
  | `-3/5` | `-3/5` |

  我们预计极少程序会在本来期望项的位置解析出有理数。注意，对于出现在算术表达式中的有理数，唯一差异是求值从运行时移动到了编译时。工具 `list_rationals/0` 可用于检查已载入程序是否在子句中包含有理数，从而可能受到兼容性影响。如果本意是写项，可以写成 `/(1,2)`、`(1)/2`、`1 / 2` 或类似变体。

- `compatibility`：以类似 `1r3` 的形式读写有理数。换句话说，它遵循上面 `natural` 的同样规则，但使用 `r` 而不是 `/`。注意，这可能与传统 Prolog 冲突，因为 `r` 可以被定义为中缀运算符。ISO 标准中作为数字语法一部分的 `0x23` 等写法也有同样问题。

有理数语法由标志 `rational_syntax` 控制；整数除法和乘方的行为由标志 `prefer_rationals` 控制。有理数算术见相关章节。

### NaN、Infinity 浮点数及其语法

SWI-Prolog 支持按 Joachim Schimpf 提出的、ECLiPSe Prolog 中可用的 Prolog 标准核心浮点算术更新提案，读取和打印“特殊”浮点值。具体来说：

- Infinity 打印为 `1.0Inf` 或 `-1.0Inf`。任何匹配正则表达式 `[+-]?\sd+[.]\sd+Inf` 的序列都会映射为正无穷或负无穷。

- `NaN`（Not a Number，非数）打印为 `1.xxxNaN`，其中 `1.xxx` 是把指数替换为 `1` 后的浮点数。这些数可以被读取，并产生同一个 `NaN`。`NaN` 常量也可以用函数 `nan/0` 生成，例如：

```prolog
?- A is nan.
A = 1.5NaN.
```

默认情况下，SWI-Prolog 算术遵循 ISO 标准：浮点运算要么产生普通浮点数，要么抛出异常。IEEE 浮点相关章节描述了可用于支持 IEEE 特殊浮点值的 Prolog 标志。创建、读取和写入这些值的能力，有助于和支持完整 IEEE 双精度范围的语言交换数据。

### 强制只有下划线引入变量

按照 ISO 标准和大多数 Prolog 系统，以大写字母或下划线开头的标识符都是变量。过去，Prolog by BIM 提供过一种替代语法：只有下划线（`_`）引入变量。从 SWI-Prolog 7.3.27 开始，SWI-Prolog 支持这种替代语法，由 Prolog 标志 `var_prefix` 控制。与 `character_escapes` 标志一样，该标志按模块维护；默认值为 `false`，即支持标准语法。

如果代码包含大小写敏感外部语言的标识符，那么只有下划线引入变量会特别有用。例如 RDF 库中，代码常常指定属性名和类名；又如 R 接口需要指定以大写字符开头的函数或变量。词汇数据库中也常有部分项以大写字母开头，使用该选项能提高这类代码的可读性。

### Unicode Prolog 源码

ISO 标准用 ASCII 字符规定 Prolog 语法。由于 SWI-Prolog 支持在源文件中使用 Unicode，我们必须扩展语法。本节描述这对源文件的影响；编写国际化源文件则在相关章节中描述。

SWI-Prolog 的 Unicode 字符分类遵循只读 Prolog 标志 `unicode_syntax_version` 报告的 Unicode 版本。注意，`char_type/2` 及相关谓词用于处理任意文本而非 Prolog 源码，它们基于 C 库的区域设置分类例程；`library(unicode)` 中的谓词会报告随附 `utf8proc` 数据的版本，该版本可能不同于语法分类器的版本（见 `unicode_version/1`）。

- **带引号的原子和字符串**：任何文字系统中的任何字符都可以用于带引号的原子和字符串。转义序列 `\uXXXX` 和 `\UXXXXXXXX`（见“字符转义语法”）被引入，用于在 ASCII 文件中指定 Unicode 码点。

- **原子和变量**：二者关系密切，因此合在一起说明。Unicode 标准为计算机语言中的标识符定义了一种语法（见 <http://www.unicode.org/reports/tr31/>）。SWI-Prolog 使用 `XID_Start` 和 `XID_Continue` 集合：标识符由一个 `XID_Start` 码点开头，后接一串 `XID_Continue` 码点。作为配置扩展，上标数字（²、³、¹，以及 ⁰--⁹，即 U+00B2、U+00B3、U+00B9、U+2070、U+2074..U+2079）和下标数字（₀--₉，U+2080..U+2089）也被接受为 `XID_Continue`，从而允许 `X²` 和 `X₁` 这样的变量。这类序列作为一个 token 处理。只有当 token 以下划线（`_`）开头，或以 Unicode 通用类别 `Lu`（大写字母）中的码点开头时，它才是变量；否则它是原子。注意，标题式大写字母（通用类别 `Lt`，例如 ǅ）会开启原子，而不是变量；这不同于较早版本使用更宽泛派生属性 `Uppercase` 的行为。许多语言没有字符大小写概念；在这类语言中，变量**必须**写成 `_name` 这样的形式。

- **数字**：在源码中（`read_term/2`），数字字面量只使用 ASCII 数字 `0` 到 `9`。通过 `atom_number/2`、`number_codes/2` 和 `number_chars/2` 转换时，对于整数、有理数（见“有理数语法”）和浮点数，还额外接受任意 Unicode `Nd` 数字块；同一个数字中的所有数字必须来自同一个块，也就是说，如果有理数的分子使用印度文字，分母也必须如此。符号、有理数分隔符、浮点数的 `.` 和浮点指数始终是 ASCII。

- **空白**：布局字符恰好是 UAX #31 定义的 Unicode `Pattern_White_Space` 集合：U+0009..U+000D、U+0020、U+0085、U+200E、U+200F、U+2028 和 U+2029。NBSP（U+00A0）有意被排除在 `Pattern_White_Space` 之外；如果它出现在带引号材料之外，会抛出杂散字符语法错误。从文字处理器粘贴的程序偶尔会在错误位置带入 NBSP，明确报告它比静默当作分隔符更好。

- **行终止**：`Pattern_White_Space` 中有七个码点会结束一行：U+000A（LF）、U+000B（VT）、U+000C（FF）、U+000D（CR）、U+0085（NEL）、U+2028（LINE SEPARATOR）和 U+2029（PARAGRAPH SEPARATOR）。它们会终止 `%` 行注释，推动源码位置的行计数器，并在带引号字符串中作为反斜杠-换行续行的换行符（`\` 后接行尾，再后接零个或多个空白会被消耗）。同一集合通过 `code_type/2` 和 `char_type/2` 中的 `prolog_end_of_line` 暴露给用户代码；不带前缀的 `end_of_line` 仍限制为四个 ISO/POSIX 控制码（LF、VT、FF、CR），而 11 个成员的 `Pattern_White_Space` 集合本身是 `prolog_layout`。

- **源文本中的杂散字符**：在 token 起始位置（也就是允许布局字符的位置），如果某个码点不属于任何已识别语法类别，即布局（见上文）、十进制数字、标识符起始、标识符延续、单独字符、括号开符号或引号开符号，则抛出 `syntax_error(illegal_character)`。这包括 C0 和 C1 控制范围、未分配码点和非字符码点、代理码点、不在 `Pattern_White_Space` 中的 `Zs` / `Zl` / `Zp` 分隔符类别（NBSP、OGHAM SPACE MARK、NARROW NO-BREAK SPACE、IDEOGRAPHIC SPACE 等）、既不在 `Pattern_White_Space` 中也不在 `Other_ID_Continue` 中的 `Cf` 格式字符（SOFT HYPHEN、ZERO WIDTH SPACE 等）、包围组合标记（`Me`），以及不属于显式上标或下标数字配置的其他数字字符（`No`，例如常用分数和罗马数字形式 U+00BC..U+00BE）。

  非间距组合标记（`Mn`、`Mc`）同样会在 token 起始位置被拒绝，因为它们不能开启标识符；但它们属于 `XID_Continue`，因此会并入前面的标识符（U+0061 后接 U+0300 COMBINING GRAVE 会读作一个由两个码点组成的单 token 标识符）。

- **带引号材料内部**：在单引号原子（`'...'`）、双引号字符串（`"..."`）、反引号文本（`` `...` ``）、Unicode 引号对（见上文）、`%` 注释以及 `/* ... */` 注释内部，**任何** Unicode 标量值（U+0000 到 U+10FFFF，不包括 UTF-8 无法编码的代理码点）都会被逐字接受。转义序列 `\uXXXX` 和 `\UXXXXXXXX`（见“字符转义语法”）可用于可移植性和显式清晰性，而不是作为准入门槛。唯一例外是双向文本覆盖/隔离范围（U+202A..U+202E 和 U+2066..U+2069），它们会作为 Trojan-source 防御被拒绝（见 `unicode_atoms`）。对包含控制码点或零宽码点的原子或字符串进行带引号写出（例如 `writeq/1`）时，该原子或字符串会被加引号，问题码点会用转义序列写出；见“字符转义语法”。

- **其他字符**：前 128 个字符遵循 ISO Prolog 标准。特别是，ASCII 符号字符会粘连成复合原子，产生熟悉的运算符 token，如 `==`、`=..`、`:-` 等。ASCII 之外，所有 Unicode 符号字符（通用类别 `Sm`、`Sc`、`Sk`、`So`）以及连接符、短横线和其他标点类别（`Pc`、`Pd`、`Po`）都被视为单独字符（solo）：每个字符各自形成一个原子，不会和相邻符号粘连。这是相对于早期版本的有意改变；早期版本中 Unicode 符号会像 ASCII 符号一样粘连成复合原子。该改变确保 ≤、€、· 等字符保持逐字符意义。由 Unicode 符号构成的运算符必须用 `op/3` 显式声明。其他类型的数字字符（通用类别 `No`，例如分数和带圈数字）不属于标识符集合；只有显式列出的上标和下标数字扩展了标识符。

- **括号（成对分隔符）**：开标点/闭标点类别 `Ps` 和 `Pe` 形成括号对（bracket pairs）：一个开字符后接一个 Prolog 项，再后接匹配的闭字符，会被读作一元复合项，其函子是两个分隔符字符拼接而成。这与 `{Term}` 变成 `'{}'(Term)` 的形态相同，只是推广到了完整 Unicode `Ps`/`Pe` 集合（64 对，来源于按通用类别过滤的 Unicode `BidiMirroring.txt`）。括号内的运算符会被遵守；嵌套也按预期工作。闭字符不匹配或游离闭字符会抛出 `syntax_error`。与 `{}` 的类比是完整的：一个空对（可只包含布局）会读作双字符原子，而不是复合项；该原子后接 `(` 时是函子；输出时，`'<open><close>'(X)` 会写作 `<open>X<close>`，裸原子也不加引号，二者都受写选项 `brace_terms(true)` 约束。

- **引号（成对字面文本分隔符）**：起始/结束引号类别 `Pi` 和 `Pf` 形成引号对（quote pairs）：一个开字符后接字面文本，再后接匹配的闭字符，会被读作一元复合项，其函子是两个分隔符字符拼接而成，参数则是其中的文本，形式由 `double_quotes` 选择（默认为字符串，也可以是原子、codes 或 chars）。其中的文本**不会**按 Prolog 项解析；转义序列（`\n`、`\uXXXX` 等）会像 ASCII 带引号字符串一样处理。例如，在 `double_quotes` 设置为 `string` 时，源码文本 `«hello, world»` 会读作一个复合项，其函子是双字符原子 `«»`，唯一参数是字符串 `"hello, world"`。引号对来自 `BidiMirroring.txt` 中的 `Pi`/`Pf` 条目（8 对），再加上标准左右弯引号对 U+2018/U+2019 和 U+201C/U+201D；后两对不在 `BidiMirroring.txt` 中。闭字符不匹配和开字符无匹配都会抛出 `syntax_error`。

上述特性让源文本可以不经转义地包含 Unicode：

```prolog
p(X⁰, X) :-              % Superscript variable profile for
    q(X⁰, X¹),           % threaded variables.
    r(X¹, X).

?- atom_number('१२३', N).     % Devanagari Nd via atom_number/2
N = 123.

?- atom_codes(≤, Cs).          % Unicode symbol stays solo
Cs = [8804].

?- term_string(T, "⟨a, b⟩"),  % bracket pair (Ps/Pe)
   display(T).
⟨⟩(','(a,b))
T = ⟨a, b⟩.

?- term_string(T, "⟨ ⟩").     % empty pair is the atom
T = ⟨⟩.

?- term_string(T, "«hello").  % quote pair (Pi/Pf)
T = '«»'("hello").
```

### 单例变量检查

单例变量（singleton variable）是在一个子句中只出现一次的变量。它总是可以替换为匿名变量 `_`。不过，在某些情况下，人们更愿意给这个变量一个名字。由于变量拼写错误是常见错误，如果某个变量只使用一次，Prolog 系统一般会给出警告（由 `style_check/1` 控制）。如果变量本来就应当只出现一次，可以通过让它以 `_` 开头来通知系统，例如 `_Name`。请注意，除单独的 `_` 之外，任何变量都会与同名变量共享。项 `t(_X, _X)` 等价于 `t(X, X)`，这与 `t(_, _)` **不同**。

由于许多语言中的变量都必须以下划线开头，这套方案已经被扩展。首先定义两类命名变量。

- **命名单例变量**：命名单例以双下划线（`__`）开头，或以单下划线后接大写字母开头，例如 `__var` 或 `_Var`。

- **普通变量**：所有其他变量都是“普通”变量。注意，这使 `_var` 成为普通变量。一些 Prolog 方言会这样书写变量。

任何在子句中只出现一次的普通变量，以及任何出现多于一次的命名单例变量，都会被报告。下面是一些示例，右列给出警告。单例消息可用 `style_check/1` 指令抑制。

最后，名为 `_<digit>` 的变量永远不会接受风格检查。这些变量名由 `write/1` 等谓词生成。这个例外也可用于把单例传给 `debug/3`。把单例传给 `debug/3` 在其他情况下会有问题：使用普通变量时，如果优化移除了 `debug/3` 语句，就会产生单例警告；使用命名匿名变量时，又会产生多例（multiton）警告。例如：

```prolog
p(X) :-
    q(X,_0Y),
    debug(demo, 'q/2 says ~p', [_0Y]).
```

| 示例 | 警告 |
| --- | --- |
| `test(_).` |  |
| `test(_a).` | `Singleton variables: [_a]` |
| `test(A).` | `Singleton variables: [A]` |
| `test(_12).` |  |
| `test(_A).` |  |
| `test(__a).` |  |
| `test(_, _).` |  |
| `test(_a, _a).` |  |
| `test(__a, __a).` | `Singleton-marked variables appearing more than once: [__a]` |
| `test(_A, _A).` | `Singleton-marked variables appearing more than once: [_A]` |
| `test(A, A).` |  |

**语义单例变量**

从 6.5.1 版开始，SWI-Prolog 区分语法单例变量（syntactic singletons）和语义单例变量（semantic singletons）。前者由 `read_clause/3` 检查，也可由带有选项 `singletons(warning)` 的 `read_term/3` 检查。后者由编译器针对在某个分支中单独出现的变量生成。例如，在下面的代码中，变量 `X` 不是**语法**单例变量，但变量 `X` 不传递任何绑定，把 `X` 替换为 `_` 不会改变语义。

```prolog
test :-
    (   test_1(X)
    ;   test_2(X)
    ).
```
