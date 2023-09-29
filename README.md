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

## 变量
- 参见项目 `a3_common_programming_concept` 代码, 定义变量后，如果赋值后要改变，必须加上 `mut` 关键字，否则则不能改变其值。同样的，定义了 `const` 之后的常量，则不能加上 `mut`

- 数字较大时，可以用下划线替代千分号分隔数字

```rust
// variables and constance
let mut x:i32 = 5;
println!("x value is {}", x);
x = 6;
println!("x value is changed: {}", x);

const COUNT: u32 = 1_000_000;
println!("COUNT value is {}", COUNT);
```

- 变量遮蔽(variable shadowing)，同一个作用域内，允许对同一变量名进行二次定义, 这样做的好处是方便，坏处是后期代码量大会使得代码难以维护，所以使用时应谨慎

```rust
// variable shadowing:
let y: u32 = 7;
println!("y value is {}", y);
let y: &str = "eight";
println!("y value is changed: {}", y);
```

## 数据类型

### Intergers

分为有符号（i）和无符号（u）：

| Length | Signed | Unsigned |  
| :--: | :--: | :--: |  
| 8-bit | i8 | u8 |  
| 16-bit | i16 | u16 |  
| 32-bit | i32 | u32 |  
| 64-bit | i64 | u64 |  
| 128-bit | i128 | u128 |  
| arch | isize | usize |

其取值范围分别是：`2`

| Type | Range |  
| :--: | :--: |  
| i8 | -128~127 |  
| u8 | 0~255 |  
| i16 | -32768~32767 |  
| u16 | 0~65535 |  
| i32 | -2147483648~2147483647 |  
| u32 | 0~4294967295 |  
| i64 | -9223372036854775808~9223372036854775807 |  
| u64 | 0~18446744073709551615 |
| i128 | -170141183460469231731687303715884105728~170141183460469231731687303715884105727 |
| u128 | 0~340282366920938463463374607431768211455 |

进制：
| Number literals | Example |  
| :--: | :--: |  
| Decimal | 98_222 |  
| Hex | 0xff |  
| Octal | 0o77 |  
| Binary | 0b1111_0000 |  
| Byte (u8 only) | b'A' |

### Floating

```rust
let x1 = 2.45; // f64
println!("{}", x1);
```

### Boolean

```rust
let t = false;
println!("{}", t);
```

### Charactor (字符型)
```rust
let ch = 'z';
println!("char z: {}", ch); // char z: z
let z_char: char = 'ℤ'; // with explicit type annotation
println!("char z: {}", z_char); // char z: ℤ
let heart_eyed_cat = '😻';
println!("heart_eyed_cat: {}", heart_eyed_cat); // heart_eyed_cat: 😻
```

### tuple （元组）

```rust
let tup0 = (11,22);
```

```rust
// 可对元组里的每个元素单独定义：
let tup1: (&str, i32, f32) = ("let's get Rusty!", 1_000_000, 0.45);
// 解构：
let (channel, sub_count, float_num) = tup;
println!("{} {} {}", channel, sub_count, float_num); // let's get Rusty! 1000000 0.45

// 索引：
let sub_count: i32 = tup.1;
println!("sub_count: {}", sub_count);
```

### array

```rust
let arr = [1, 2, 3, 4, 5];

// 定义类型：[数据类型，数组长度]
let arr1: [i32; 5] = [1, 2, 3, 4, 5];
```

### 函数

- 定义与调用：
```rust
fn main() {
  my_function(12, 34);
}

fn my_function (x: i32, y: i32) -> i32 {
  println!("my function: {}, y: {}", x, y);
}
```

- 函数返回值，可省 `return` 关键字，且返回的语句或变量无需加 `;`
- 函数如有返回值，则须用 `->` 定义返回值类型
```rust
fn main() {
  let result = my_function(12, 34);
	println!("result: {}", result);
}

fn my_function (x: i32, y: i32) -> i32 {
	x + y
}
```

### Control Flow

#### `if-else`

- `if-else`分支的条件无需套括号 `()`
```rust
let number: i32 = 5;
if number < 10 {
  println!("1 true");
}else if number < 22 {
  println!("2 true");
}else{
  println!("false");
}
```
- `if-else` 条件可写在一行：
```rust
let condition: bool = true;
let num:i32 = if condition { 1 } else { 2 };
```

### `while`
```rust
let mut n = 3;
while n != 0 {
  println!("{}!", n);
  n -= 1;
}
// 3!
// 2!
// 1!
```
#### `loop`
和while循环有点类似：
```rust
loop {
  println!("again!");
  break
}

let mut counter = 0;
let loop_result = loop {
  counter += 1;
  if counter == 10 {
    break counter;
  }
}; 
println!("loop_result: {}", counter); // loop_result: 10
```

#### `for`

```rust
let arr_for = [11,22,33,44,55];
for item in arr_for.iter() {
  println!("the arr value is: {}", item);
}
// the arr value is: 11
// the arr value is: 22
// the arr value is: 33
```

- 类似 `python` 的 `range`  和  `scala`的 `Range` ，在`rust`里使用`..` 表示数值范围：
- `..` 的取值范围包括开始值，不包括终止值：
```rust
for value in 1..4 {
  println!("{}!!", value);
}
// 1!!
// 2!!
// 3!!
```