// 标量类型
// 有符号整数（signed integers）：i8、i16、i32、i64、i128 和 isize（指针宽度）
// 无符号整数（unsigned integers）： u8、u16、u32、u64、u128 和 usize（指针宽度）
// 浮点数（floating point）： f32、f64
// char（字符）：单个 Unicode 字符，如 'a'，'α' 和 '∞'（每个都是 4 字节）
// bool（布尔型）：只能是 true 或 false
// 单元类型（unit type）：()。其唯一可能的值就是 () 这个空元组

// 复合类型
// 数组（array）：如 [1, 2, 3]
// 元组（tuple）：如 (1, true)

pub fn types() {
    let _logical: bool = true;
    let _a_float: f64 = 1.0; //常规说明
    let _an_integer = 5i32; //后缀说明

    let _default_float = 3.0; // f64默认说明
    let _default_integer = 7; // i32默认说明

    let mut value = 32;
    value = 64i64; // 根据后续赋值推断上面位i64类型

    let mut mutable = 12;
    mutable = 32;
    //mutable = true; // 变量的类型不能更改
    //let mutable = true; // 但是可以shadow前面的变量
}

pub fn literals_and_operators() {
    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 - 2 = {}", 1i32 - 2);

    println!("true AND false is {}", false); // true && false = false

    println!("true OR false is {}", true); // true || false = true

    println!("NOT ture is {}", !true);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);

    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);

    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);

    println!("1 << 5 is {}", 1u32 << 5);

    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("One million is written as {}", 1_000_000u32);
}
