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
	
	// & 借用：
	/*
	The Rules of References
	1. At any given time, u can hv either one mutable reference
	or any number of immutable refernces.
	2. References must always be valid.
	 */
	let mut str1 = String::from("uuuuuuuuu");
	let length = str_length(&mut str1);
	println!("the length of {} is {}", str1, length); // the length of uuuuuuuuuooops is 14

	// 可变引用：借用的值有严格的使用限制
	let mut str2 = String::from("kkkkkkkk");
	let r1 = &str2;
	let r2 = &str2;
	// let r3: = &mut str2; // 错误 ❌
	println!("{}, {}", r1, r2); 
	let r3 = &mut str2; // 正确
	println!("r3 {}", r3); // r3 kkkkkkkk

	// 悬空引用 Dangling References
	// 具体见README.md

	// Slice Type 切片类型 &str
	let str3: String = String::from("hello world");
	let hello3: &str = &str3[..5];
	let world3: &str = &str3[6..];
	
	println!("str3 第一个词 {}", hello3); // str3 第一个词 hello
	println!("str3 第二个词 {}", world3); // str3 第二个词 world

	let str4: &str = "hello rust";
	let hello4: &str = &str4[..5];
	let rust4: &str = &str4[6..];
	let hello_rust: &str = &str4[..];
	println!("str4 第一个词 {}", hello4); // str4 第一个词 hello
	println!("str4 第二个词 {}", rust4); // str4 第二个词 rust
	println!("整个词 {}", hello_rust); // 整个词 hello rust

	let word = first_word(&str3);
	println!("word -> {}", word);

	// array slice
	let a = [1, 2, 3, 4, 5];
	let slice = &a[1..3];
	assert_eq!(slice, &[2, 3]);
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

fn str_length(s: &mut String) -> usize {
	s.push_str("ooops");
	let length = s.len();
	length
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