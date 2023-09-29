

fn main() {
	// variables
	// mut key word
	let mut x:i32 = 5;
	println!("x value is {}", x);
	x = 6;
	println!("x value is changed: {}", x);

	// variable shadowing:
	let y: u32 = 7;
	println!("y value is {}", y);
	let y: &str = "eight";
	println!("y value is changed: {}", y);

	// constant:
	const COUNT: u32 = 1_000_000;
	println!("COUNT value is {}", COUNT);

	/*
	4 base data type:
	Integers
	Floating
	Booleans
	Character
	*/
	// Integers
	let a: i32 = 423241234; // default Integers is i32
	println!("a value is {}", a);

	// the min and max u128, is 2**
	println!("{}, {}", std::u128::MIN, std::u128::MAX); // i128: 340282366920938463463374607431768211455
	// 可储存最大值为i128，有符号：
	println!("{}, {}", std::i128::MIN, std::i128::MAX); // i128-170141183460469231731687303715884105728
	

	// 进制：
	let decimal = 02_551; // decimal -> thousand seperator
	println!("{}", decimal); //2551

	let hex = 0xff;
	println!("{}", hex); //255

	let octal = 0o77;
	println!("{}", octal); // 63

	let binary = 0b1111_1111;
	println!("{}", binary); // 255

	let byte = b'A'; // 65
	println!("{}", byte);

	// Floating
	let x1 = 2.45; // f64
	println!("{}", x1);

	// Boolean
	let t = false;
	println!("{}", t);

	// chcaractor
	let ch = 'z';
	println!("char z: {}", ch); // char z: z
	let z_char: char = 'ℤ'; // with explicit type annotation
	println!("char z: {}", z_char); // char z: ℤ
	let heart_eyed_cat = '😻';
	println!("heart_eyed_cat: {}", heart_eyed_cat); // heart_eyed_cat: 😻


	// Compound Types:
  // tuple
	let tup0 = (11,22);
  let tup: (&str, i32, f32) = ("let's get Rusty!", 1_000_000, 0.45);
  // destructure tuple
  let (channel, sub_count, float_num) = tup;
	println!("{} {} {}", channel, sub_count, float_num);
  // index tuple
  let sub_count: i32 = tup.1;
	println!("sub_count: {}", sub_count);

	// array
	let arr = [1, 2, 3, 4, 5];
	// type defined: [datatype, length]
	let arr1: [i32; 5] = [1, 2, 3, 4, 5];

	// function:
	let result = my_function(12, 34);
	println!("result: {}", result);

	// Control Flow
	// if-else
	let number: i32 = 5;
	if number < 10 {
		println!("1 true");
	}else if number < 22 {
		println!("2 true");
	}else{
		println!("false");
	}

	let condition: bool = true;
	// if-else in one line:
	let num:i32 = if condition { 1 } else { 2 };

	// loop:
	loop {
		println!("again!");
		break
	}

	let mut counter = 0;
	let loop_result = loop {
		counter += 1;
		if counter <= 3 {
			break counter;
		}
	};
	println!("loop_result: {}", counter);

	// while
	let mut n = 3;
	while n != 0 {
		println!("{}!", n);
		n -= 1;
	}

	// for
	let arr_for = [11,22,33];
	for item in arr_for.iter() {
		println!("the arr value is: {}", item);
	}
	// range use in for
	for value in 1..4 {
		println!("{}!!", value);
	}
}


fn my_function (x: i32, y: i32) -> i32 {
	x + y
}