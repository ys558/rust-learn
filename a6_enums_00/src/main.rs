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
	// // 创建变体实例, 用 :: 绑定值
	let four = IpAddrKind::v4;
	let six = IpAddrKind::v6;

	// // 用 struct 存储 实际ip值
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
