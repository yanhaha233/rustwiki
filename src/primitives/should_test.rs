pub fn test_primitives(value: &str) -> i32 {
    let value = value;
    println!("before shadow value is {}", value);
    let values = 5;
    println!("after shadow value is {}", values);
    values
}

pub struct Operator(pub i32, pub i32, pub String, pub String);

pub fn test_literals_and_operators(lsrs: &str, number: Operator) -> String {
    let mut value = 0;
    let mut result = "".to_string();
    match lsrs {
        "+" => {
            value = number.0 + number.1;
        }
        "-" => {}
        "and" => {}
        "or" => {}
        "not" => {}
        _ => {
            result = "nothing happened or should use lower-case".to_string();
        }
    }
    if value != 0 {
        value.to_string()
    } else {
        result
    }
}
