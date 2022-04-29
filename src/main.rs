use std::env;
use std::error::Error;

#[derive(Debug)]
enum Flag {
    Bootstrap,
    Empty,
    Validation
}

fn main() -> Result<(), Box<dyn Error>> {
    
    let args: Vec<String> = env::args().collect();

    let (_password, _flag) = parser(args)?;

    Ok(())

}

fn parser(args: Vec<String>) -> Result<(String, Flag), Box<dyn Error>> {

    match args.len() {

        3 => {

            let flag = match &args[2][..] {
                "-b" => Flag::Bootstrap,
                "-v" => Flag::Validation,
                _ => Err("Unknown Flag!")?
            };

            Ok((args[1].clone(), flag))

        },

        2 => Ok((args[1].clone(), Flag::Empty)),

        _ => Err("Enter Password!")?

    }

}
