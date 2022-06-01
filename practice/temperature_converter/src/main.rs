use std::io;
use std::cmp::Ordering;

fn to_celcius(temperature){

}

fn to_farenheit(temperature){

}

fn main() {
    loop{ 
        println!("Hello, welcome to temperature unit converter!");
        println!("Press C to start converting from Celcius or F, to start converting from Faranheit:");
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read the line.");
        println!("Option selected: {}", option);

        println!("Insert temperature: ");
        let mut temperature = String::new();
        io::stdin().read_line(&mut temperature).expect("Failed to read line.");
        let temperature:u32= match temperature.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Temperature inserted: {}", temperature);


        match option{

        }
    }
}
