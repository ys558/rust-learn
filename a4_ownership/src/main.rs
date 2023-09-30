fn main() {
  {
    let s: &str = "hello"; // s 被定义，开始生效
    println!("s value is {}", s); 
    // 当包含's'的作用域结束时（即大括号 '{}' 结束时），'s'将不再有效。因为在这个时刻，'s'的所有者已经超出了其作用域，所以Rust会安全地删除's'所引用的内存中的数据。
  }

	// Move:
  let s1 = String::from("hello");
	// let s2 = s1; // s1 is moved, s1's value is droped
	// println!("{}, world!", s1); // 报错❌

	let ss2 = s1.clone();
	println!("s1 = {}, s2 = {}", s1, ss2); // s1 = hello, s2 = hello


	let s3: String = String::from("xxxxx");
	takes_ownership(s3);
	// println!("s3 {}", s3); // 报错❌ 

	// 原始值则不会有上述的问题，能直接copy，在函数调用后，仍可以使用x：
	let x = 5;
	makes_copy(x);
	println!("x -> {}", x); // x -> 5

	// 引用值改为可用状态，将引用值放函数里作为返回值即可：
	let s4 = gives_overship();
	println!("s4 {}", s4); // s4 hello

	let ss1 = gives_overship();
	let ss2 = String::from("ss2 pass in string"); // 
	println!("ss1 = {}, takes_and_gives_back = {}", ss1, takes_and_gives_back(ss2)); // ss1 = hello, ss3 = ss2 pass in string

	let x1 = String::from("kkkkkk"); // the length of kkkkkk is 6
	let (y, len) = calculate_str_length(x1);
	println!("the length of {} is {}",  y, len);
	
	let str1 = String::from("uuuuuuuuu");
	let length = str_length(&str1);
	println!("the length of {} is {}", str1, length); // the length of uuuuuuuuu is 9
}

fn takes_ownership(some_str: String) {
	println!("some_str {}", some_str); 
	// after this scope , some_str is done, it droped
}

fn makes_copy(some_int: i32) {
	println!("some_int {}", some_int); 
}

fn gives_overship() -> String {
	String::from("hello")
}

fn takes_and_gives_back(a_string: String) -> String {
	a_string
}

fn calculate_str_length(s: String) -> (String, usize) {
	let length = s.len();
	(s, length)
}

fn str_length(s: &String) -> usize {
	let length = s.len();
	length
}