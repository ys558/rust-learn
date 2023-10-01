## 所有权（Ownership）

Rust的所有权（Ownership）是一个核心概念，它帮助Rust在没有垃圾回收机制的情况下进行内存管理。在Rust中，每个值都有一个与之关联的变量，这个变量被称为该值的所有者（owner）。

以下是关于Rust所有权的一些关键规则：

> - 一个值在任何时刻只能有一个所有者。
> - 当值的所有者超出其作用域时，该值将被销毁。

这意味着Rust在运行时内存管理方面与许多其他编程语言不同。在某些语言中（如C），开发人员需要显式地分配（allocate）和回收（deallocate）内存。而在其他具有垃圾回收功能的语言（如Java）中，垃圾回收器会在程序运行时不断寻找不再使用的内存。但Rust的所有权系统允许它无需这些额外的机制就能保证内存安全。

> 1. 垃圾回收器：在其他一些具有垃圾回收机制的语言中，由于垃圾回收器自动管理内存，可能会导致程序运行速度更快但性能表现更加不可预测。此外，大型程序会更大，手动管理内存可能会增加错误处理和管理的成本。然而，Rust没有使用垃圾回收器，因此不会出现这些问题。
>2. 内存控制：使用Rust可以更好地控制内存的使用，从而避免内存泄漏或内存溢出等问题。此外，由于没有垃圾回收器，写入时间会更快，但大型程序的大小可能会导致更高的运行时延迟。
>3. 所有权模型：Rust拥有所有权模型，这有助于减少内存错误并提高代码的可靠性。然而，这也会导致学习曲线变得更加陡峭，因为需要更多地了解内存管理和内存分配策略。
>4. 内存管理：Rust具有更好的内存管理能力，这有助于优化性能。然而，这也会增加错误处理和维护成本。
>5. 学习曲线：由于Rust的内存管理和内存控制的复杂性，需要更多的学习和练习才能掌握它们。


所有权代码示例：
```rust
fn main() {
  // ------ 所有权规则 --------
  // 1. 每个值在Rust中都有一个被称为其所有者的变量。
  // 2. 在任何时候，一个值只能有一个所有者。
  // 3. 当所有者超出其作用域时，该值将被丢弃（drop）。
  {
    // s 还未被声明。
    let s: &str = "hello"; // s 被定义，开始生效
    println!("x value is {}", x); 
    // 当包含's'的作用域结束时（即大括号 '{}' 结束时），'s'将不再有效。因为在这个时刻，'s'的所有者已经超出了其作用域，所以Rust会安全地删除's'所引用的内存中的数据。
  }
}
```

### 移动（Move）操作的数据交互

Rust的移动（Move）概念，类似别的语言里的复制（copy），其原理在引用值复制上有着本质区别，这也是Rust语言独特的地方。
#### 原始值复制
这种和其他语言的copy区别不大
```rust
// 原始值的复制
// 将值绑定5到x；然后复制 中的值x并将其绑定到y
let x = 5;
let y = x; // copy
```

#### 引用值复制

先看一段rust代码：
```rust
let s1 = String::from("hello");
let s2 = s1; // 移动（Move），而非浅拷贝（shallow copy）
```

如果按照其他语言的copy操作
- s1被创建后，其指针、长度和容量存在于栈（Stack）中，其值"hello"存在于堆（Heap）中。
- s2复制s1，s2创造出一块新的栈（Stack），存储其指针、长度和容量等属性，但共用堆中的"hello" 值。
![引用值复制](https://doc.rust-lang.org/book/img/trpl04-02.svg)
- rust 对以上的操作做了优化，正如前面所说的，当s2被绑定了s1的堆（Heap）引用值后，s1已超出范围（scope），rust认为s1的值不再有效，直接丢弃（drop）了，示例图：
![引用值复制](https://doc.rust-lang.org/book/img/trpl04-03.svg)

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1); // 报错❌
```
以上的print会报错如下：因为s1的值已经移动（Move）到s2
```shell
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++

For more information about this error, try `rustc --explain E0382`.
error: could not compile `ownership` due to previous error

```

按照以上的报错，如果还需要用到`s1`, rust也保留了copy操作，我们必须用`.clone()`进行拷贝，代码修改如下：

```rust
let s1 = String::from("hello");
--- let s2 = s1;
+++ let s2 = s1.clone();
println!("{}", s1); // hello
```

#### Move 在函数中的应用
```rust
fn main() {
  let s3: String = String::from("xxxxx");
	takes_ownership(s3);
	// println!("s3 {}", s3); // 报错❌
}

fn takes_ownership(some_str: String) {
  println!("some_str {}", some_str); 
	// 变量 some_str 的作用域move到这里为止
}

```
上述代码中，
- 因s3值已经move 到函数`takes_ownership(s3);`里，其值已经不存在s3上，而跳到了函数里
- 函数里的 `some_str` 传进来后，被限制在函数的作用域里后被销毁（drop）
- 所以 `println!("s3 {}", s3);` 则会报错，因为 `s3`的值已被销毁在函数 `takes_ownership`的作用域里

而原始值则没有这个问题：
```rust
fn main() {
	let x = 5;
	makes_copy(x);
	println!("x -> {}", x); // x -> 5
}

fn makes_copy(some_int: i32) {
	println!("some_int {}", some_int); 
}
```

回到引用值的移动（move）的问题，如果要避免move后带来的影响，则须将引用值直接放函数里作为返回值即可，如：

```rust
fn main() {
	// 引用值改为可用状态，将引用值放函数里作为返回值即可：
	let s4 = gives_overship();
	println!("s4 {}", s4); // s4 hello
}

fn gives_overship() -> String {
	String::from("hello")
}
```
或如下，变量 `ss2` 传入函数 `takes_and_gives_back` 后，仍要使用 `ss2`，则须再写一个函数 `takes_and_gives_back` 将其值困
```rust
fn main() {
	let ss1 = gives_overship();
	let ss2 = String::from("ss2 pass in string");
	println!("ss1 = {}, takes_and_gives_back = {}", ss1, takes_and_gives_back(ss2)); // ss1 = hello, ss3 = ss2 pass in string
}

fn gives_overship() -> String {
	String::from("hello")
}

fn takes_and_gives_back(a_string: String) -> String {
	a_string
}
```

### `&` 引用及借用 (References and Borrowing)

rust 允许使用 返回元组的解构

例如以下例子，`x1`传入了函数 `calculate_str_length(x1)` 里，已经被移动了，

在外面如果要再获取x1，则需返回其原值并将其解构出来，但这样返回值就复杂了，传入的同时还需返回他
```rust
fn main() {
	let x1 = String::from("kkkkkk");
	let (y, len) = calculate_str_length(x1);
	println!("the length of {} is {}",  y, len);
}

fn calculate_str_length(s: String) -> (String, usize) {
	let length = s.len();
	(s, length)
}
```

如果传进的函数的值 `x1` 要在外面获取，但又不需要将其传回作为返回值，就需要用到引用：

```rust
fn main() {
	let str1 = String::from("uuuuuuuuu");
	let length = str_length(&str1);
	println!("the length of {} is {}", str1, length); // the length of uuuuuuuuu is 9
}

fn str_length(s: &String) -> usize {
	// s.push_str("ooops"); // err ❌
	let length = s.len();
	length
}
```

上面的代码中，调用str1时，加入了 `&` 符号，即引用
- 函数里的引用变量会自行寻找值，而不是获取他的所有权，从而让进入函数的变量，在后续能被继续使用。
- 同样的，借用的变量 `s` 则不能进行其他操作，因为仅仅是借用。

### 可变引用（Mutable References）

上面代码解决的办法就是将 `str1` 定义为可迭代操作 `mut`，以下是修改后的代码：

```rust
fn main() {
	let mut str1 = String::from("uuuuuuuuu");
	let length = str_length(&mut str1);
	println!("the length of {} is {}", str1, length); // the length of uuuuuuuuuooops is 14
}

fn str_length(s: &mut String) -> usize {
	s.push_str("ooops");
	let length = s.len();
	length
}
```

而引用有严格的限制, 例如以下代码：
```rust
let mut str2 = String::from("kkkkkkkk");
let r1 = &str2;
let r2 = &str2;
// let r3: = &mut str2; // 错误 ❌
println!("{}, {}", r1, r2); 
```

上面定义的 `r3` 会报错，是因为：
- `r3` 前定义的 `r1` 和 `r2` 已定义为非 `mut`变量，他们还未被使用
- 要解决以上报错，则须将其移动到 print 之后即可，因 `r1`, `r2` 定义后已被使用过

修改后：

```rust
let mut str2 = String::from("kkkkkkkk");
let r1 = &str2;
let r2 = &str2;
println!("{}, {}", r1, r2); 
let r3 = &mut str2; // 正确 r1, r2定义后已被使用过，out of scope
println!("r3 {}", r3); // r3 kkkkkkkk
```

### 悬空引用（Dangling References）

在函数内部，引用值不能作为返回值！这是因为引用值 `s` 已超出范围, `s` 值已被丢弃，以下是错误❌代码示范：

```rust
fn dangle() -> &String { // dangle returns a reference to a String

	let s = String::from("hello"); // s is a new String

	&s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger! ❌
```

### 字符串切片类型（The Slice Type）

Rust 的切片是一种比较特殊的形式，这里我们单独讨论。以下是官方文档对切片所有权的总结：

> Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

切片允许您引用集合中连续的元素序列，而不是整个集合。切片是一种引用，因此它没有所有权。

让我们看一个切片的例子，
```rust
let str3: String = String::from("hello world");
let hello3: &str = &str3[..5];
let world3: &str = &str3[6..];

println!("str3 第一个词 {}", hello); // str3 第一个词 hello
```
上面的而切片有着一个特殊的类型 `&str`, 同样的，定义字符串时，也可将其定义为 `&str`，就像下面的例子：
```rust
let str4: &str = "hello rust";
let hello4: &str = &str4[..5];
let rust4: &str = &str4[6..];
let hello_rust = &str[..];
println!("str4 第一个词 {}", hello4); // str4 第一个词 hello
println!("str4 第二个词 {}", rust4); // str4 第二个词 rust
println!("整个词 {}", hello_rust); // 整个词 hello rust
```
封装寻找第一个词的函数：
```rust
fn main() {
	let str3: String = String::from("hello world");

	let word = first_word(&str3);
	println!("word -> {}", word);
}

fn first_word (s: &str) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			// 如果该句子有空格，则返回第一个元素
			return &s[0..i];
		}
	}
	// 如果没有空格则返回整个句子
	&s[..]
}
```
可以看出，我们传入和返回的类型均为 `&str`