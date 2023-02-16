use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::process::exit;

#[derive(Debug)]
enum CLIErrors {
    CouldNotParse,
}

fn main() {
    let mut input = String::new();
    let mut type_option = OptionPrompt {
        query: "What type is the label\n".to_string(),
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
    let stdo = stdout().lock();
    let mut write_handle = std::io::BufWriter::new(stdo);

    loop {
        let mut i = 0;
        for option in &prompt.options {
            write!(write_handle, "({}) {:?}\n", i, option);
            i += 1;
        }
        write!(write_handle, "{}", prompt.query);
        write_handle.flush().unwrap();

        let mut response = String::new();

        stdin()
            .read_line(&mut response)
            .expect("error reading input from stdin");

        let index = str::parse::<usize>(&response.trim());

        if index.is_err() {
            write!(write_handle, "could not parse input, try again\n");
            continue;
        }

        let result_option = prompt.options.get(index.unwrap());

        match result_option.is_some() {
            true => return result_option.unwrap().to_string(),
            false => {
                write!(
                    write_handle,
                    "something went wrong or that choice is invalid, try again\n"
                );
                write_handle.flush();
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
