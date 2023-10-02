struct User {
  username: String,
  active: bool,
  email: String,
  sign_in_count: u64
}

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

impl Rectangle {
  fn square(size: u32) -> Self {
    Self {
      width: size,
      height: size
    }
  }
}

fn main() {
  let mut user1 = User {
    username: String::from("user1"),
    email: String::from("user1@mail.com"),
    active: true,
    sign_in_count: 1
  };

  // change username fr "user1" to "John123"
  let name = user1.username;
  user1.username = String::from("John123");
  println!(
    "username before: {}, username after: {}", 
    name, user1.username
  ); // username before: user1, username after: John123

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

  // tuple struct
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);

  let width1 = 30;
  let height1 = 50;
  println!("area of the reactangle is {}", area(width1, height1)); // area of the reactangle is 1500

  let rect = (30, 50);
  println!("area of the reactangle is {}", area_by_tuple(rect)); // area of the reactangle is 1500

  let rect1 = Rectangle {
    width: 30,
    height: 50
  };
  println!("the area of the reactangle is  {}", area_by_struct(&rect1)); // the area of the reactangle is  1500
  
  // 利用 trait 进行 debug：#[derive(Debug)] 
  println!("rect1 {:?}", rect1); // rect1 Rectangle { width: 30, height: 50 }
  println!("rect1 {:#?}", rect1); 
  // rect1 Rectangle {
    //   width: 30,
    //   height: 50,
    // }
    
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

  let rect5 = Rectangle::square(22); 
  println!("rect5 {:?}", rect5); // rect5 Rectangle { width: 22, height: 22 }
  
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1
  }
}

fn area(width: u32, height: u32) -> u32 {
  width * height
}

fn area_by_tuple(dimensions: (u32, u32)) -> u32 {
  dimensions.0 * dimensions.1
}

fn area_by_struct(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}