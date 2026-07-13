# 宽字符支持

SWI-Prolog 使用 [Unicode](https://home.unicode.org) 表示字符。Unicode 定义了范围为 `0..0x10FFFF` 的码点（code points）。这些码点几乎可以表示任何语言中的任何字符。此外，Unicode 标准还定义了字符类别（字母、数字、标点等）、大小写转换以及更多内容。Unicode 是 ISO 8859-1（ISO Latin-1）的超集，而 ISO Latin-1 又是 US-ASCII 的超集。

SWI-Prolog 对原子和字符串对象有两种表示（见字符串相关章节）。如果文本可以放入 ISO Latin-1，就表示为 8 位字符数组。否则，文本表示为 `wchar_t` 字符数组。除 MS-Windows 外，几乎所有系统上的 `wchar_t` 都是 32 位无符号整数，因此能够表示所有 Unicode 码点。在 MS-Windows 上，`wchar_t` 是 16 位无符号整数，因此只能表示 `0..0xFFFF` 范围内的码点。从 SWI-Prolog 8.5.14 开始，Windows 上的 `wchar_t` 会被解释为 UTF-16 字符串。UTF-16 编码使用代理对（surrogate pairs），把 `0x10000..0x10FFFF` 范围内的码点表示为 `0xD800..0xDFFF` 范围内的两个码元（code units）。作为 Unicode 码点，这个范围是未分配的。为保持一致性，SWI-Prolog 不接受代理对范围内的整数作为有效码点，例如：

```prolog
?- char_code(X, 0xD800).
ERROR: Type error: `character_code' expected, found `55296' (an integer)
```

内部字符表示对 Prolog 用户完全透明。不过，使用外部语言接口的用户有时需要了解这些问题，相关内容见外部语言接口章节。

当需要从文件读取字符串字符、向文件写入字符串字符，或通过外部语言接口与其他软件组件通信时，字符编码就会显现出来。本节只处理通过流进行的 I/O，包括文件 I/O 以及通过网络套接字进行的 I/O。

## 流上的宽字符编码

虽然内部使用 Unicode 标准唯一地编码字符，但流和文件是面向字节（8 位）的，而在 8 位八位组流中表示较大的 Unicode 码点有多种方式。最流行的一种，尤其是在 Web 语境中，是 UTF-8。字节 `0..127` 直接表示对应的 US-ASCII 字符，而字节 `128..255` 用于对 Unicode 空间中更高位置的字符进行多字节编码。特别是在 MS-Windows 上，以字节对表示的 16 位 UTF-16 标准也很流行。

Prolog I/O 流有一个名为编码（encoding）的属性，它指定所用编码，并影响 `get_code/2`、`put_code/2` 以及所有其他文本 I/O 谓词。

文件的默认编码来自 Prolog 标志 `encoding`。该标志由 `setlocale(LC_CTYPE, NULL)` 初始化为 `text`、`utf8` 或 `iso_latin_1` 之一。如果识别出编码名称，就使用后两者之一；否则默认使用 `text`。使用 `text` 时，转换留给 C 库的宽字符函数处理。Prolog 原生 UTF-8 模式明显快于通用的 `mbrtowc()` 模式。在 MS-Windows 上，默认值无条件为 `utf8`，不受系统代码页影响，因为 UTF-8 是源文件的事实编码，而 Windows C 运行时基于 locale 的宽字符函数提供的 Unicode 覆盖弱于 Prolog 自己的表。

可以在 `load_files/2` 中为使用替代编码载入的 Prolog 源显式指定编码；打开文件时可以在 `open/4` 中指定；也可以在任意已打开流上用 `set_stream/2` 指定。对于 Prolog 源文件，还提供了 `encoding/1` 指令，可用于在与 US-ASCII 兼容的编码之间切换（`ascii`、`iso_latin_1`、`utf8` 以及许多 locale）。编写包含非 US-ASCII 字符的 Prolog 文件见国际化源文件相关章节；语法问题见 Unicode Prolog 源码相关章节。更多信息和 Unicode 资源见 <http://www.unicode.org/>。

SWI-Prolog 当前定义并支持以下编码：

- `octet`：`binary` 流的默认编码。这会使流在读写时完全不做转换。

- `ascii`：8 位字节中的 7 位编码。等价于 `iso_latin_1`，但遇到大于 127 的值时会生成错误和警告。

- `iso_latin_1`：支持许多西方语言的 8 位编码。这会使流在读写时完全不做转换。上面是 SWI-Prolog 的原生名称。该编码也可以使用官方 [IANA](https://www.iana.org) 名称 `ISO-8859-1` 指定。

- `text`：文本文件的 C 库默认 locale 编码。文件使用 C 库函数 `mbrtowc()` 和 `wcrtomb()` 读写。它可能与其他某种编码相同，尤其是在西方语言环境中可能等同于 `iso_latin_1`，在 UTF-8 环境中可能等同于 `utf8`。

- `utf8`：完整 Unicode 的多字节编码，与 `ascii` 兼容。见上文。上面是 SWI-Prolog 的原生名称。该编码也可以使用官方 [IANA](https://www.iana.org) 名称 `UTF-8` 指定。

- `utf16be` / `utf16le`：UTF-16 编码。按字节对读取输入。`utf16be` 是大端（Big Endian），高有效字节在前；`utf16le` 是小端（Little Endian），高有效字节在后。UTF-16 可以通过代理对表示完整 Unicode。上面是 SWI-Prolog 的原生名称。这些编码也可以使用官方 [IANA](https://www.iana.org) 名称 `UTF-16BE` 和 `UTF-16LE` 指定。为了向后兼容，也支持 `unicode_be` 和 `unicode_le`。

注意，并非所有编码都能表示所有字符。这意味着向流写入文本时，可能因为该流无法表示这些字符而出错。流在遇到这些错误时的行为可以用 `set_stream/2` 控制。初始状态下，终端流会使用 Prolog 转义序列写出这些字符，而其他流会生成 I/O 异常。

### BOM：字节顺序标记

读完“流上的宽字符编码”后，你可能已经感觉到文本文件很复杂。本节处理一个相关主题：它通常让用户的生活更轻松，但也给程序员带来另一层顾虑。**BOM**，即字节顺序标记（Byte Order Marker），是一种识别 Unicode 文本文件及其所用编码的技术。这类文件以 Unicode 字符 `0xFEFF` 开头；这是一个不换行、零宽的空格字符。它是一个相当独特的序列，不太可能出现在非 Unicode 文件开头，并能唯一地区分各种 Unicode 文件格式。由于它是零宽空白，甚至不会产生任何输出。这似乎解决了所有问题，或者说……

有些格式以 US-ASCII 开始，并可能包含某种编码标记来切换到 UTF-8，例如 XML 头中的 `encoding="UTF-8"`。这类格式通常明确禁止使用 UTF-8 BOM。另一些情况下，文件中还有额外信息揭示编码，使 BOM 变得多余，甚至非法。

BOM 由 SWI-Prolog 的 `open/4` 谓词处理。默认情况下，读取文本文件时会探测 BOM。如果找到 BOM，就相应设置编码，并且可通过 `stream_property/2` 获得属性 `bom(true)`。以写入方式打开文件时，可以通过 `open/4` 的选项 `bom(true)` 请求写入 BOM。
