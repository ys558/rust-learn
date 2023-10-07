#[derive(Debug)]
enum IpAddrKind {
	v4(u8, u8, u8, u8),
	v6(String)
}

#[derive(Debug)]
enum Message {
	Quit,
	Move { x: i32, y: i32 },
	Write(String),
	ChangeColor(i32, i32, i32),
}

/*
	upper enum Message should be same as below structs:
*/
// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
	fn call(&self) {
		println!("{:?}", self); 
	}
}


fn main() {
	let home = IpAddrKind::v4(127, 0,0,1);
	let loopback = IpAddrKind::v6(String::from("::1"));

	println!("home {:?}", home); // home v4(127, 0, 0, 1)
	println!("loopback {:?}", loopback); // loopback v6("::1")

	let message_write = Message::Write(String::from("hello"));
	message_write.call(); // Write("hello")

	// Option:
	let some_number: Option<i32> = Some(1);
	let some_string: Option<&str> = Some("hello");
	let absent_num: Option<i32> = None;

	let x: i8 = 5;
	let y: Option<i8> = Some(5);
	let sum = x * y.unwrap_or(1);
	println!("sum: {}", sum); // sum: 25
}