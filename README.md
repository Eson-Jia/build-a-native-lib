# 构建本地库

参考[教程](https://doc.rust-lang.org/cargo/reference/build-script-examples.html#building-a-native-library)

## 硬编码编译选项的缺点

通过调用 gcc 将目标文件转换为静态库这种硬编码方式有许多缺点:

1. 该 gcc 命令本身是无法跨平台移植.例如, windows 平台不太可能有 gcc,甚至并非所有 Unix 平台都可能有 gcc,ar 命令也处于类似的情况.
2. 这些命令 不考虑交叉编译.如果我们为 Android 等平台进行交叉编译,则不太可能 gcc 生成 ARM 可执行文件.