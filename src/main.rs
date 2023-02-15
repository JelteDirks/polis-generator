fn main() {
    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).expect("error while reading input from stdin");

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
    }
}

#[derive(Debug)]
enum CLIErrors {
    CouldNotParse,
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

