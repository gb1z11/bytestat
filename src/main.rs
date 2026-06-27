use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    match config {
        Ok(config) => {
            println!("File path: {}", config.file_path);

            let contents = fs::read_to_string(config.file_path)
                .expect("Something went wrong reading the file");
            println!("Content of the file: {}", contents);
        }
        Err(error) => {
            println!("{}", error);
        }
    }


}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Result<Config, &'static str> {
    if args.len() >= 2 {
        let query = args[0].clone();
        let file_path = args[1].clone();
        Ok(Config {query, file_path})
    } else {
        Err("Недостаточно аргументов")
    }

}
