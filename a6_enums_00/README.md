## 枚举 enum

### 基本概念

枚举为您提供了一种表示某个值是一组可能的值之一的方法。

- 下面的代码定义了枚举 `enum` 类型，`IpAddrKind` v4和v6
- 绑定枚举值时，须用 `::` 绑定

```rust
#[derive(Debug)]
enum IpAddrKind {
  v4,
	v6
}

fn main() {
	// 创建变体实例, 用 :: 绑定值
	let four = IpAddrKind::v4;
	let six = IpAddrKind::v6;

	route(four); // ip_kind -> v4
	route(six); // ip_kind -> v6
}

fn route(ip_kind: IpAddrKind) {
	println!("ip_kind -> {:?}", ip_kind); 
}
```
我们创建一个上一节提到的 `struct` 结构体，对ip的实际地址用字符串进行存储：
```rust
#[derive(Debug)]
enum IpAddrKind {
  v4,
	v6
}

#[derive(Debug)]
struct IpAddr {
	kind: IpAddrKind,
	address: String
}

fn main() {
	let home = IpAddr {
		kind: IpAddrKind::v4,
		address: String::from("127.0.0.1"),
	};

	let loopback = IpAddr {
		kind: IpAddrKind::v6,
		address: String::from("::1"),
	};

	route(home); // ip_kind -> IpAddr { kind: v4, address: "127.0.0.1" }
	route(loopback); // ip_kind -> IpAddr { kind: v6, address: "::1" }
}

fn route(ip_kind: IpAddr) {
	println!("ip_kind -> {:?}", ip_kind);
}
```

#### `enum` 可独自定义每个成员的数据类型

但上面的代码可以让 `enum` 直接存储类型为 `String`, 省去更多代码，可以改写如下：

```rust
#[derive(Debug)]
enum IpAddrKind {
  v4(String),
	v6(String)
}

fn main() {
	let home = IpAddrKind::v4(String::from("127.0.0.1"));
	let loopback = IpAddrKind::v6(String::from("::1"));

	println!("home {:?}", home); // home v4("127.0.0.1")
	println!("loopback {:?}", loopback); // loopback v6("::1")
}
```

甚至可以再细化, 做到结构体无法完成的任务，例如ipv4一般是4个8 bit的 数值，定义类型时可和ipv6区分开来：

```rust
#[derive(Debug)]
enum IpAddrKind {
  v4(u8, u8, u8, u8),
	v6(String)
}

fn main() {
	let home = IpAddrKind::v4(String::from(127, 0, 0, 1));
	let loopback = IpAddrKind::v6(String::from("::1"));

	println!("home {:?}", home); // home v4("127.0.0.1")
	println!("loopback {:?}", loopback); // loopback v6("::1")
}
```

其实我们在[标准库](https://doc.rust-lang.org/std/net/enum.IpAddr.html)里，可以看到一个专门定义ip的结构体，可以用来这样表达`enum`：

```rust
struct Ipv4Addr {
	// --snip--
}

struct Ipv6Addr {
	// --snip--
}

enum IpAddr {
	V4(Ipv4Addr),
	V6(Ipv6Addr),
}
```
