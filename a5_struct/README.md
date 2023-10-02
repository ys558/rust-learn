## 定义及实例化结构 Struct

`struct` 关键字，中文可以理解为结构体, 是一种自定义数据类型。

```rust
struct User {
  username: String,
  active: bool,
  email: String,
  sign_in_count: u64
}

fn main() {
  let mut user1 = User {
    username: String::from("user1"),
    email: String::from("user1@mail.com"),
    active: true,
    sign_in_count: 1
  };
}
```
- 改变实例中的值，可以直接用点 `.` 进行访问及赋值，如：

```rust
let name = user1.username;
user1.username = String::from("John123");
println!(
  "username before: {}, username after: {}", 
  name, user1.username
); // username before: user1, username after: John123
```

### `Field Init` 简写

用函数生成实例：封装一个函数用于生成 User 对象，再生成一个 `user2`的实例

当生成 `user3` 时，如果其他属性不想列出，可以直接用 `..` 的语法拿取 `user2` 中的参数直接使用：

```rust
fn main() {
  let user2 = build_user(
    String::from("kyle@mail.com"), 
    String::from("kyle123")
  );
  println!("user2.username => {}, user3.email after => {}", user2.username, user2.email); // user2.username => kyle123, user3.email after => kyle@mail.com

  let user3 = User {
    email: String::from("james@mail.com"),
    username: String::from("james234"),
    ..user2
  };
  println!("user3.username {}, user3.email {}", user3.username, user3.email); // user3.username james234, user3.email james@mail.com
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1
  }
}
```

### `tuple struct` 使用元组结构创建不同类型

```rust
// tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```

以下的代码是一个普通的计算长方形面积的函数：
```rust
fn main() {
  let width1 = 30;
  let height1 = 50;
  println!("area of the reactangle is {}", area(width1, height1)); // area of the reactangle is 1500

}

fn area(width: u32, height: u32) -> u32 {
  width * height
}
```

将其传参改为元组，则可以改写成：

```rust
fn main() {
  let rect = (30, 50);
  println!("area of the reactangle is {}", area_by_tuple(rect)); // area of the reactangle is 1500
}

fn area_by_tuple(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}
```

将元组的部分提炼成 `struct`, 改写成：
```rust
struct Rectangle {
  width: u32,
  height: u32
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50
  };
  println!("rect1 {:?}", rect1); // rect1 Rectangle { width: 30, height: 50 }
  println!("the area of the reactangle is  {}", area_by_struct(&rect1)); // the area of the reactangle is  1500
}

fn area_by_struct(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}
```

tips：在我们定义了 `let rect1` 后我们是无法直接打印出来的，会报错，这时，我们需要在 `struct Rectangle` 上加一个修饰符 `#[derive(Debug)]`，而且打印里的{}也需要加上，才能正确打印出 `rect1`，如：

```rust
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32
}

fn main() {
  let rect1 = Rectangle {
    width: 30,
    height: 50
  };
  println!("rect1 {:?}", rect1); // rect1 Rectangle { width: 30, height: 50 }
  println!("rect1 {:#?}", rect1); 
  // rect1 Rectangle {
  //   width: 30,
  //   height: 50,
  // }
}
```
以上涉及到 `trait` 特征的概念，我们后面的章节会讲到

### 方法 Method Syntax

#### 定义方法

上面的 `struct` 由于有固定的公式，即width*height 
- 我们可以把他简化为 `impl`, 将area的计算公式定义在其内部，少写了一个函数
- 

```rust
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32
}

impl Rectangle {
  fn area_by_impl(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

fn main() {
  let rect2 = Rectangle {
    width: 33,
    height: 55
  };
  println!("the area of the reactangle is  {}", rect2.area_by_impl()); // the area of the reactangle is  1815

  let rect3 = Rectangle {
    width: 22,
    height: 33
  };
  let rect4 = Rectangle {
    width: 66,
    height: 77
  };
  println!("rect2 can hold rect3 {}", rect2.can_hold(&rect3)); // rect2 can hold rect3 true
  println!("rect2 can hold rect4 {}", rect2.can_hold(&rect4)); // rect2 can hold rect4 false
  
}
```

### 关联函数

尽管官方文档有很多解释，但简单来说就是 `impl` 能直接像普通函数一样传入参数，我们来看一个例子

```rust
impl Rectangle {
  fn square(size: u32) -> Self {
    Self {
      width: size,
      height: size
    }
  }
}

fn main() {
  let rect5 = Rectangle::square(22);
  println!("rect5 {:?}", rect5); // rect5 Rectangle { width: 22, height: 22 }
}

```
需要注意，关联函数需要用 `::` 进行函数传值，而不是点。