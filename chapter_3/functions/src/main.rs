fn f(x: i32) -> i32 {
    x + 1
}

// ' -> '  return function
// ' {...} ' An expression and syntatic scope
fn main() {
    println!(
        "{}",
        f({
            let y = 1;
            y + 1
        })
    );

    // let diego: &str = "diego";

    // println!("su nombre es, {diego}");
}
