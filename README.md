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
`rustc` 只能试用在一些比较简单的程序，复杂的程序需用到rust自带的项目管理工具 `cargo`，
本仓库使用讲的每个章节均用一个项目的方式, 每个项目均能独立跑起, 下面是几个常用命令：

```bash
# 新建项目：
cargo new <ProjectName>

# 编译构建项目，构建完成的项目会放在 target/debug 目录
cd <ProjectName>
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

## 变量


```rust
fn main() {
  let x = 88;
  // 报错：
  // x = 99;
  
  // 如要使其可变，则应加上 mut 关键字，immutable缩写：
  let mut y = 99;
  y = 100;
  
  println!("x is {}, y is {}", x, y)
  
  // 定义一个空字符串, 
  // 中间两个冒号:表明String的关联函数，针对类型本身而实现的，而非针对字符串某特定实例实现的
  let mut guess = String::new();
}
```

## crate 库的使用

- rust的库叫 `crate`, 需要用的lib包可在 [`crate 官网`](https://crates.io/) 里找到。这次我们用随机生成数字的库, 在项目目录里添加：

```toml
...

# 前面的 ^ 表示任何与0.8.4兼容的版本均可以下载
[dependencies]
rand = "^0.3.23"
```

- 添加完成后，返回终端，输入 `cargo check` 或 `cargo build`，则可以下载该lib包, 如：

```powershell
ziyouzhiyi@zyzy: ~/Documents/code/rust-learn/test-project main ⚡
$ cargo check                                                                                                                                                                                                                                                      [11:57:13]
Updating crates.io index
Downloaded rand v0.4.6
Downloaded rand v0.3.23
Downloaded 2 crates (87.7 KB) in 1.38s
Compiling libc v0.2.126
Checking rand v0.4.6
Checking rand v0.3.23
Checking test-project v0.1.0 (/Users/ziyouzhiyi/Documents/code/rust-learn/test-project)
Finished dev [unoptimized + debuginfo] target(s) in 5.74s
```

- `Cargo.lock` 文件：当第一次安装lib包成功后，会将该包 `rand v0.3.23` 所依赖包所有lib包版本号放入 `Cargo.lock` 文件中，以后每次重新构建，均会按照里面列举的版本进行安装，除非自己手动进行 `rand v0.8.4` 版本升级。

- `cargo update` 命令：如要更新b版本以上的版本，须手动在 `toml` 文件里重新写上版本号再执行update命令，此时，cargo 会忽略 `Cargo.lock` 文件直接更新
