
fn main() {
    match divide(10, 2) {
    Ok(val) => println!("Result: {}", val),
    Err(err) => println!("Error: {}", err),
    }
    let result = divide(90 , 2).map(|val| val*2);
    println!("{:?}", result);
}

fn divide(a: i32, b: i32)-> Result<i32, String>{
    if b == 0{
        return Err("Divided by zero not possible".to_string());
    }

    return Ok(a/b);
}