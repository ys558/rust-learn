fn main() {
    // * rust是强类型语言，但可省略类型，他会自动推断数据类型：
    let str = "string";
    println!("{}", str);

    let integer = 9999;
    println!("{}", integer);

    // 有符号类型signed可以存储正数或负数，无符号类型unsigned只能存储正数。
    // 按存储控件来说，划分为1字节，2字节，4字节，8字节，16字节，1字节 = 8位

    // i8 : range is range is `-128..=127`
    let x8: i8 = 127;
    println!("x8: {}", x8);
    // u8 ： range is `0..=255`
    let y8: u8 = 255;
    println!("y8: {}", y8);

    // i16 :  range is `-32768..=32767`
    let x16: i16 = 32767;
    println!("x16: {}", x16);
    // u16 : range is `0..=65535`
    let y16: u16 = 65535;
    println!("y16: {}", y16);

    // i32 ： range is `-2147483648..=2147483647`
    let x32: i32 = 2147483647;
    println!("x32: {}", x32);
    // u32 ： range is `0..=4294967295`
    let y32: u32 = 4294967295;
    println!("y32: {}", y32);

    // i64 : range is `-9223372036854775808..=9223372036854775807`
    let x64: i64 = 9223372036854775807;
    println!("x64: {}", x64);
    // u64 : range is `0..=18446744073709551615`
    let y64: u64 = 18446744073709551615;
    println!("x64: {}", y64);

    // 可储存最大值为u128，无符号：
    println!("{}", std::u128::MAX); // 340282366920938463463374607431768211455
    // 可储存最大值为i128，有符号：
    println!("{}", std::i128::MIN); // -170141183460469231731687303715884105728
    
    // 进制：
    let decimal = 02_551;
    println!("{}", decimal); //2551

    let hex = 0xff;
    println!("{}", hex); //255

    let octal = 0o77;
    println!("{}", octal); // 63

    let binary = 0b1111_1111;
    println!("{}", binary); // 255
}
