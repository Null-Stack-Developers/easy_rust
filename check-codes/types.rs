// So those are two reasons for all the different number types in Rust. Here is another reason: usize is the size that Rust uses for indexing. (Indexing means "which item is first", "which item is second", etc.) usize is the best size for indexing because:
//
// An index can't be negative, so it needs to be a number with a u
// It should be big, because sometimes you need to index many things, but
// It can't be a u64 because 32-bit computers can't use u64.
// So Rust uses usize so that your computer can get the biggest number for indexing that it can read.
//
// Let's learn some more about char. You saw that a char is always one character, and uses '' instead of "".
//
// All chars use 4 bytes of memory, since 4 bytes are enough to hold any kind of character:
//
// Basic letters and symbols usually need 1 out of 4 bytes: a b 1 2 + - = $ @
// Other letters like German Umlauts or accents need 2 out of 4 bytes: ä ö ü ß è é à ñ
// Korean, Japanese or Chinese characters need 3 or 4 bytes: 国 안 녕
//
fn main() {
    //let my_number = 100;
    // println!("{}", my_number as u8 as char);

    // let my_number: u8 = 100;
    //
    // println!("{}", my_number);
    //
    // println!("{}", my_number as char)

    println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
    println!("Size of string containing 'a': {}", "a".len()); // .len() gives the size of the string in bytes
    println!("Size of string containing 'ß': {}", "ß".len());
    println!("Size of string containing '国': {}", "国".len());
    println!("Size of string containing '𓅱': {}", "𓅱".len());
}
