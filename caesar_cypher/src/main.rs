use clap::{App, Arg};

const ABC: &str = "abcdefghijklmnopqrstuvwxyz";
fn main() -> Result<(), &'static str> {
    let matches = App::new("Caesar")
        .arg(
            Arg::with_name("text")
                .index(1)
                .required(true)
                .multiple(true)
                .validator(is_abc)
                .help("Text to encode"),
        )
        .arg(
            Arg::with_name("rot")
                .short("r")
                .long("rotation")
                .help("Rotation number")
                .takes_value(true)
                .default_value("13")
                .validator(is_num),
        )
        .get_matches();
    let mut encoded = String::new();
    for value in matches.values_of("text").ok_or("uh")?.collect::<Vec<_>>() {
        for letter in value.chars() {
            let rot_val = matches
                .value_of("rot")
                .ok_or("val?")?
                .parse::<usize>()
                .map_err(|_| "is_num sucks")?;
            let abc_vec = ABC.chars().collect::<Vec<_>>();
            let index = abc_vec
                .iter()
                .position(|p| p == &letter.to_ascii_lowercase())
                .ok_or("is_abc suck")?;
            let char_encoded_index = {
                if index + rot_val > 26 {
                    (index + rot_val) % 26
                } else {
                    index + rot_val
                }
            };
            if letter.is_uppercase() {
                encoded.push(abc_vec[char_encoded_index].to_ascii_uppercase());
            } else {
                encoded.push(abc_vec[char_encoded_index]);
            }
        }
        encoded.push(' ');
    }
    println!("{}", encoded);
    Ok(())
}

fn is_num(v: String) -> Result<(), String> {
    v.parse::<usize>()
        .map_err(|_| String::from("Value is not a number or too big"))?;
    Ok(())
}

fn is_abc(v: String) -> Result<(), String> {
    let v = v.to_lowercase();
    for letter in v.chars() {
        if ABC.matches(letter).next().is_none() {
            return Err(String::from("Text doesn't match the alphabet"));
        }
    }
    Ok(())
}
