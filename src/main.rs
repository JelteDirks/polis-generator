use std::io::stdin;
use std::io::stdout;
use std::process::exit;

#[derive(Debug)]
enum CLIErrors {
    CouldNotParse,
}

fn main() {
    let mut input = String::new();
    let mut type_option = OptionPrompt {
        query: "What type is the label".to_string(),
        options: Vec::new(),
    };

    type_option.options.push(String::from("basis"));
    type_option.options.push(String::from("tekst"));
    type_option.options.push(String::from("label"));
    type_option.options.push(String::from("valuta"));

    let prompt_result = prompt(&type_option);

    println!("prompt result {}", prompt_result);

    loop {
        stdin()
            .read_line(&mut input)
            .expect("error while reading input from stdin");

        let as_bool = string_to_boolean(&input);

        if let Ok(b) = as_bool {
            if b {
                println!("{}", input);
            } else {
                println!("na");
            }
        } else {
            // something went wrong in the conversion
            println!("{:?}", as_bool.unwrap_err());
            exit(1);
        }

        input.clear();
    }
}

struct OptionPrompt {
    query: String,
    options: Vec<String>,
}

fn prompt(prompt: &OptionPrompt) -> String {
    loop {
        let mut i = 0;
        for option in &prompt.options {
            println!("({}) {:?}", i, option);
            i += 1;
        }
        println!("{}", prompt.query);

        let mut response = String::new();

        stdin()
            .read_line(&mut response)
            .expect("error reading input from stdin");

        let index = str::parse::<usize>(&response.trim());

        if index.is_err() {
            println!("could not parse that choice, try again");
            continue;
        }

        let result_option = prompt.options.get(index.unwrap());

        match result_option.is_some() {
            true => return result_option.unwrap().to_string(),
            false => {
                println!("something went wrong or that choice is invalid, try again");
            }
        }
    }
}

fn string_to_boolean(value: &String) -> Result<bool, CLIErrors> {
    let lowercased = String::from(value).to_lowercase();

    if lowercased.is_empty() {
        return Ok(false);
    }

    if lowercased.starts_with("y") {
        return Ok(true);
    } else if lowercased.starts_with("n") {
        return Ok(false);
    }

    if lowercased == "1" {
        return Ok(true);
    } else if lowercased == "0" {
        return Ok(false);
    }

    return Err(CLIErrors::CouldNotParse);
}
