use std::env;
use rand::seq::IndexedRandom;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "convert" => {
            if args.len() < 3 {
                eprintln!("Usage: {} convert <temperature_in_fahrenheit>", args[0]);
                std::process::exit(1);
            }
            let fahrenheit_str = &args[2];
            let fahrenheit: f64 = match fahrenheit_str.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Error: '{}' is not a valid number.", fahrenheit_str);
                    std::process::exit(1);
                }
            };

            let (celsius, kelvin) = convert_fahrenheit(fahrenheit);

            println!("Fahrenheit: {:.2}°F", fahrenheit);
            println!("Celsius: {:.2}°C", celsius);
            println!("Kelvin: {:.2}K", kelvin);
        }
        "choose" => {
            if args.len() < 3 {
                eprintln!("Usage: {} choose <option1> <option2> ...", args[0]);
                std::process::exit(1);
            }
            let options = &args[2..];
            match choose_option(options) {
                Some(choice) => println!("I choose: {}", choice),
                None => println!("No options provided."),
            }
        }
        _ => {
            // Backward compatibility: if the first argument is a number, treat it as a fahrenheit value
            if let Ok(fahrenheit) = command.parse::<f64>() {
                let (celsius, kelvin) = convert_fahrenheit(fahrenheit);
                println!("Fahrenheit: {:.2}°F", fahrenheit);
                println!("Celsius: {:.2}°C", celsius);
                println!("Kelvin: {:.2}K", kelvin);
            } else {
                eprintln!("Unknown command: {}", command);
                print_usage(&args[0]);
                std::process::exit(1);
            }
        }
    }
}

fn print_usage(program_name: &str) {
    eprintln!("Usage:");
    eprintln!("  {} convert <fahrenheit>", program_name);
    eprintln!("  {} choose <option1> <option2> ...", program_name);
}

fn convert_fahrenheit(f: f64) -> (f64, f64) {
    let c = (f - 32.0) * 5.0 / 9.0;
    let k = c + 273.15;
    (c, k)
}

fn choose_option(options: &[String]) -> Option<&String> {
    let mut rng = rand::rng();
    options.choose(&mut rng)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_32() {
        let (c, k) = convert_fahrenheit(32.0);
        assert!((c - 0.0).abs() < 1e-10);
        assert!((k - 273.15).abs() < 1e-10);
    }

    #[test]
    fn test_convert_212() {
        let (c, k) = convert_fahrenheit(212.0);
        assert!((c - 100.0).abs() < 1e-10);
        assert!((k - 373.15).abs() < 1e-10);
    }

    #[test]
    fn test_choose_option() {
        let options = vec!["Pizza".to_string(), "Sushi".to_string()];
        let choice = choose_option(&options);
        assert!(choice.is_some());
        let choice_val = choice.unwrap();
        assert!(options.contains(choice_val));
    }
}
