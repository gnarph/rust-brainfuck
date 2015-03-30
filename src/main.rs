use std::env;

fn get_arg_one() -> String {
    // Can't just get the args as an array for some reason
    let mut args: Vec<String> = Vec::new();
    for argument in env::args_os() {
        match argument.into_string() {
            Ok(str_arg) => args.push(str_arg),
            Err(e) => println!("{:?}", e),
        }
    }
    args[1].clone()
}

fn main() {
    let instructions = get_arg_one();
    println!("{}", instructions);
}
