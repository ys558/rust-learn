#### `enum` 和 `struct` 对比

再举一个复杂点的枚举的例子，以下的例子的枚举，分别定义了4种不同的类型，

```rust
enum Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColor(i32, i32, i32),
}
```

如果换成结构体 `struct`, 则需定义为4种，才能达到和以上相同的效果：

```rust
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```

#### `impl` 可以给枚举定义方法：

枚举和结构体一样，均能定义方法 `impl` :

```rust
#[derive(Debug)]
enum Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColor(i32, i32, i32),
}

impl Message {
	fn call(&self) {
		println!("{:?}", self); 
	}
}

fn main() {
	let message_write = Message::Write(String::from("hello"));
	message_write.call(); // Write("hello")
}
```

#### `Option`

`null` 值发明者 Tony Hoare 在2009年曾表示 ` “Null References: The Billion Dollar Mistake,” `,  `null` 值是他为了方便实现而发明的，

> I call it my billion-dollar mistake. ... This has led to innumerable errors, vulnerabilities, and system crashes, which have probably caused a billion dollars of pain and damage in the last forty years.


> The problem with null values is that if you try to use a null value as a not-null value, you’ll get an error of some kind. 

以上是Rust官方文档对于 `null` 值存在的最大问题的描述，我理解的是：
1. 当我们将一个变量赋值时，要么该变量不存在，要么给他附上 `null` 值表示没有值
2. 但编译时，系统不能保证你使用了值，而他是非 `null` 值。
3. 使用值和没有值 `null` 发生了根本上的逻辑矛盾

在许多编程语言中，`null` 指针是造成内存安全问题的主要原因。尝试访问或解引用一个空指针，通常会导致程序崩溃或未定义行为。Rust语言为了避免这种问题，引入了 `Option` 和 `Result` 这两个特质（`trait`），它们提供了安全的替代方法，以处理可能的空值情况。


我们可以从rust 手册的 [`Option` 定义](https://doc.rust-lang.org/std/option/enum.Option.html)看出， `Option` 被规定为泛型的 `Some(T)`值，或者什么也没有 `None`

Rust的`None`和`null`指针有一点重要的不同：Rust的`None`是类型系统的一部分，而`null`指针是运行时的一个概念。

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

根据上面的Option枚举定义，如果类型定义为 `Option`，那么必须加上 `Some()` 或者直接给 `None`。
```rust
let some_number: Option<i32> = Some(1);
let some_string: Option<&str> = Some("hello");
let absent_num: Option<i32> = None;
```

下面的例子中：
- y被赋值了 `Some(5)` 表明 5 的赋值有可能是不存在的。
- y值有可能不存在，所以用于运算时，则应用上默认值的api： `unwrap_or(1)`
```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);
let sum = x * y.unwrap_or(1);
println!("sum: {}", sum); // sum: 25
```
