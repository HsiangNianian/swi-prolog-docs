# 二进制兼容性

SWI-Prolog 首先尝试维护版本之间的源代码兼容性。数据和程序通常也可以用二进制形式表示。这涉及多个接口，而这些接口具有不同程度的兼容性。相关版本号和签名可通过 `PL_version_info()`、`--abi-version` 以及 Prolog 标志 `abi_version` 获得。

- **外部扩展**

  可动态载入的外部扩展通常依赖体系结构、C 编译器的 ABI 模型、动态链接库格式等。它们还依赖 `libswipl` 提供的 `PL_*` API 函数的向后兼容性。

  兼容的 API 允许以二进制形式分发外部扩展，尤其适用于编译较复杂的平台（例如 Windows）。因此，这种兼容性在优先级列表中很靠前，但偶尔仍必须妥协。

  对应版本信息：`PL_version_info()` 中的 `PL_VERSION_FLI`；`abi_version` 键：`foreign_interface`。

- **二进制项**

  项可以使用 `PL_record_external()` 和 `fast_write/2` 表示为二进制格式。由于这些格式用于在数据库中存储二进制项，或在 Prolog 进程之间以二进制形式传递项，因此会非常谨慎地维护兼容性。

  对应版本信息：`PL_version_info()` 中的 `PL_VERSION_REC`；`abi_version` 键：`record`。

- **QLF 文件**

  QLF 文件（见 `qcompile/1`）是 Prolog 文件或模块的二进制表示。它们把子句表示为虚拟机（VM）指令序列。其兼容性依赖 QLF 文件格式以及 VM 的 ABI。系统会以一定程度的谨慎来维护兼容性。

  对应版本信息：`PL_version_info()` 中的 `PL_VERSION_QLF`、`PL_VERSION_QLF_LOAD` 和 `PL_VERSION_VM`；`abi_version` 键：`qlf`、`qlf_min_load`、`vmi`。

- **保存状态**

  保存状态（见 `-c` 和 `qsave_program/2`）是一个 zip 文件，使用与 QLF 文件相同的表示包含整个 Prolog 数据库。保存状态可以包含额外资源，例如外部扩展、数据文件等。除 QLF 文件的依赖性问题外，内置谓词和核心库谓词还可能调用**内部**外部谓词。公共内置谓词和内部外部谓词之间的接口经常变化。稳定分支中的补丁级发布会尽可能维持兼容性。

  相关 ABI 版本键与 QLF 文件相同，另加一项：`PL_version_info()` 中的 `PL_VERSION_BUILT_IN`；`abi_version` 键：`built_in`。
