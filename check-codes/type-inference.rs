// Type inference means that if you don't tell the compiler the type,
// but it can decide by itself, it will decide.
// The compiler always needs to know the type of the variables,
// but you don’t always need to tell it.
// Actually, usually you don't need to tell it. For example,
//
//
//!!!!!!! for let my_number = 8, my_number will be an i32.
// That is because the compiler chooses i32 for integers if you don't tell it.
// But if you say let my_number: u8 = 8,
// it will make my_number a u8, because you told it u8.
//
//
// So usually the compiler can guess. But sometimes you need to tell it, for two reasons:
//
// You are doing something very complex and the compiler doesn't know the type you want.
// You want a different type (for example, you want an i128, not an i32).
//
//
//For numbers, you can say the type after the number. You don't need a space - just type it right after the number.

// fn main() {
//     let small_number = 10u8; // 10u8 = 10 of type u8
// }
// You can also add _ if you want to make the number easy to read.
//
// fn main() {
//     let small_number = 10_u8; // This is easier to read
//     let big_number = 100_000_000_i32; // 100 million is easy to read with _
// }
// The _ does not change the number. It is only to make it easy for you to read. And it doesn't matter how many _ you use:
//
// fn main() {
//     let number = 0________u8;
//     let number2 = 1___6______2____4______i32;
//     println!("{}, {}", number, number2);
// }
// This prints 0, 1624.
//
// Floats
//
fn main() {
  let my_float: f64 = 5.0;
  let my_other_float: f32 = 8.5;

  let third_float = my_float + my_other_float as f64; // my_other_float as f64 = use my_other_float like an f64
  println!("{}", third_float)
}
