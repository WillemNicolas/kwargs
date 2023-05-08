use kwargs::utils::arg_parser::{Parser,Names,Argument};

#[derive(Debug)]
enum Arg{
   Add(usize)
}
#[derive(Debug)]
enum ErrorArg{
    Add
}

fn main() {
    let mut args:Parser<Arg, ErrorArg> = Parser::build("description");
    args.arg("add", "a", "add the given value",
    |s| {
        match s.parse::<usize>() {
            Ok(num) => Ok(Arg::Add(num)),
            Err(_) => Err(ErrorArg::Add)
        }
    });
    args.flag("test", "t", "test flag",false);
    args.print_help();
    let args = args.parse();
    dbg!(args);
}
