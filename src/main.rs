use std::env;

mod fuckvm;

fn get_arg_one() -> Result<String, &'static str> {
    // Can't just get the args as an array for some reason
    let mut args: Vec<String> = Vec::new();
    for argument in env::args_os() {
        match argument.into_string() {
            Ok(str_arg) => args.push(str_arg),
            Err(e) => println!("{:?}", e),
        }
    }
    if args.len() > 1 {
        Ok(args[1].clone())
    }
    else {
        Err("No code to run")
    }
}

fn main() {
    let instructions = get_arg_one();
    match instructions {
        Ok(ins) => fuckvm::execute(ins),
        Err(e) => println!("Error: {:?}", e),
    }
    print!("\n");
}
