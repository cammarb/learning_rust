use std::io;

fn to_celcius(initial_temp: f32) {
    let final_temp: f32 = (initial_temp - 32.0) * 0.5556;
    println!(
        "The temperature {} Farenheit is equal to {} Celcius.",
        initial_temp, final_temp
    );
}

fn to_farenheit(initial_temp: f32) {
    let final_temp: f32 = (1.8 * initial_temp) + 32.0;
    println!(
        "The temperature {} Celcius is equal to {} Farenheit.",
        initial_temp, final_temp
    );
}

fn main() {
    loop {
        println!("Hello, welcome to temperature unit converter!");
        println!(
            "Press C to start converting from Celcius or F, to start converting from Faranheit:"
        );
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read the line.");
        println!("Option selected: {}", option);

        println!("Insert temperature: ");
        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line.");
        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Temperature inserted: {}", temperature);

        if option.contains('c') {
            to_farenheit(temperature);
        }
        if option.contains('f') {
            to_celcius(temperature);
        }
    }
}
