# 构建链接本地库

参考[教程](https://doc.rust-lang.org/cargo/reference/build-script-examples.html#building-a-native-library)

## 硬编码编译选项的缺点

通过调用 gcc 将目标文件转换为静态库这种硬编码方式有许多缺点:

1. 该 gcc 命令本身是无法跨平台移植.例如, windows 平台不太可能有 gcc,甚至并非所有 Unix 平台都可能有 gcc,ar 命令也处于类似的情况.
2. 这些命令 不考虑交叉编译.如果我们为 Android 等平台进行交叉编译,则不太可能 gcc 生成 ARM 可执行文件.

## 使用 crates.io 中的 cc 优化编译脚本

使用 cc crate 抽象一定范围的 C 代码生成脚本要求:

- 它调用适当的编译器(windows 的 MSVC,MinGW 的 gcc,Unix 的 cc)等
- 它通过将适当的标志传递给正在使用的编译器来应用 TARGET 变量.
- 其他的环境变量,如 OPT_LEVEL,DEBUG 等,都自动处理.
- stdout 输出和 OUT_DIR 位置也由 cc 库来接管.

在这里,我们可以开始看到将尽可能多的功能用于常见的构建依赖项而不是在所有构建脚本中复制逻辑的一些好处.
我们还看到构建脚本将`crate`纯粹用于构建过程的依赖项,而不是在运行时用于本 crate 本身.