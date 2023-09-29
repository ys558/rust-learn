# 编程基础概念在 `Rust` 中的体现

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

### Array

```rust
let arr = [1, 2, 3, 4, 5];

// 定义类型：[数据类型，数组长度]
let arr1: [i32; 5] = [1, 2, 3, 4, 5];
```

### Funtion

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