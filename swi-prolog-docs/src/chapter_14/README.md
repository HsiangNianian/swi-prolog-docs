# 库的自动加载

如果在运行时捕获到未定义谓词，系统会先尝试从该模块的默认模块导入该谓词（参见 import module 相关章节）。如果失败，自动加载器（auto loader）会被激活。实际过程是先调用 hook `user:exception/3`；只有该 hook 失败时，才会调用自动加载器。

第一次激活时，系统会把所有库目录中所有库文件的索引载入内存（参见 `library_directory/1`、`file_search_path/2` 和 `reload_library_index/0`）。如果能在某个库中找到该未定义谓词，对应库文件会被自动载入，并重新启动对这个此前未定义谓词的调用。默认情况下，该机制会静默载入文件。`current_prolog_flag/2` 的键 `verbose_autoload` 可用于获得详细载入信息。Prolog 标志 `autoload` 可用于启用或禁用自动加载系统。`autoload/[1,2]` 提供了更受控的自动加载形式，也支持应用模块的惰性加载。

自动加载只处理使用模块机制的库源文件。文件会用 `use_module/2` 载入，并且只有被捕获的未定义谓词会被导入到调用该未定义谓词的模块中。每个库目录都必须包含一个 `INDEX.pl` 文件，其中包含该目录所有库文件的索引。该文件由如下格式的行组成：

```prolog
index(Name, Arity, Module, File).
```

谓词 `make/0` 会更新自动加载索引。它会搜索所有库目录（参见 `library_directory/1` 和 `file_search_path/2`），查找包含 `MKINDEX.pl` 或 `INDEX.pl` 的目录。如果当前用户可以写入或创建 `INDEX.pl`，并且该文件不存在、或早于该目录或其中某个文件，则更新该目录的索引。如果 `MKINDEX.pl` 存在，则通过载入该文件来更新索引；该文件通常包含一个调用 `make_library_index/2` 的指令。否则会调用 `make_library_index/1`，为所有包含模块的 `*.pl` 文件创建索引。

下面是一个创建已索引库目录的示例：

```shell
% mkdir ~/${XDG_DATA_HOME-.config}/swi-prolog/lib
% cd ~/${XDG_DATA_HOME-.config}/swi-prolog/lib
% swipl -g 'make_library_index(.)' -t halt
```

如果有多个库文件包含所需谓词，则按下面的搜索方案查找：

1. 如果某个库文件定义了捕获未定义谓词的模块，则使用该文件。
2. 否则，按 `library_directory/1` 谓词中出现的顺序考虑库文件；在同一目录内按字母顺序考虑。

## `autoload_path/1`

`autoload_path(+DirAlias)` 把 `DirAlias` 添加到自动加载器使用的库中。这会扩展搜索路径 `autoload`，并重新载入库索引。例如：

```prolog
:- autoload_path(library(http)).
```

如果该调用作为指令出现，它会被项展开为一个 `user:file_search_path/2` 子句，以及一个调用 `reload_library_index/0` 的指令。这样可以保留源代码信息，并允许移除该指令。

## `make_library_index/1`

`make_library_index(+Directory)` 为该目录创建索引。索引会写入指定目录中的 `INDEX.pl` 文件。如果目录不存在或写保护，则失败并给出警告。

## `make_library_index/2`

`make_library_index(+Directory, +ListOfPatterns)` 通常在 `MKINDEX.pl` 中使用。该谓词会为 `Directory` 创建 `INDEX.pl`，为匹配 `ListOfPatterns` 中任一文件模式的所有文件建立索引。

有时库包包含一个公共载入文件，以及若干由该载入文件使用的文件；这些内部文件导出的谓词不应由最终用户直接使用。这样的库可以放在库的子目录中，而包含公共功能的文件可以加入该库的索引。下面以 XPCE 库的 `MKINDEX.pl` 为例，它把 `trace/browse.pl` 的公共功能加入 XPCE 包可自动加载的谓词中。

```prolog
:- prolog_load_context(directory, Dir),
   make_library_index(Dir,
                      [ '*.pl',
                        'trace/browse.pl',
                        'swi/*.pl'
                      ]).
```

## `reload_library_index/0`

修改库目录集合后，可用 `reload_library_index/0` 强制重新载入索引。修改库目录集合的方式包括：更改 `library_directory/1`、`file_search_path/2` 的规则，添加或删除 `INDEX.pl` 文件。该谓词**不会**更新 `INDEX.pl` 文件。若要更新索引文件，请查看 `make_library_index/[1,2]` 和 `make/0`。

通常，如果某个谓词无法在索引中找到，并且库目录集合已经改变，索引会自动重新载入。如果目录被移除，或库目录顺序发生改变，则必须使用 `reload_library_index/0`。

使用 `qsave_program/2` 或命令行选项 `-c` 创建可执行文件时，必须显式载入所有通常会自动加载的谓词。这一点在运行时章节中讨论。另请参阅 `autoload_all/0`。
