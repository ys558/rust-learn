# Rust 语言初探

## 安装

见[文档](https://www.rust-lang.org/zh-CN/tools/install)，十分简单，这里不赘述


## hello world

`touch main.rs` 创建文件，并写下：

```rust
fn main() {
    println!("Hello world");
}
```

rust是需要静态编译的语言，所以先执行 `rustc main.rs` 编译为二进制码后再执行，可看到控制台打印 `hello world`:

```bash
# mac
./main

# windows
.\main.exe
```

## `cargo` 的使用
`rustc` 只能试用在一些比较简单的程序，复杂的程序需用到rust自带的项目管理工具 `cargo`，下面是几个常用命令：

```bash
# 新建项目：
cargo new <ProjectName>

# 编译构建项目，构建完成的项目会放在 target/debug 目录
cargo build

# 为发布的编译，须加上 --release，会对编译的代码进行优化，构建完的项目会放在 target/release 目录
cargo build --release

# 编译构建并运行项目：
cargo run

# 检查是否能通过编译：效率比 cargo build 快很多
cargo check
```

## 注释

```rust
// 单行注释
/*
  多行注释
*/
```


