pub fn run(args: &[String]) {
    if args.len() == 0 {
        println!();
        std::process::exit(0);
    }
    if args.len() == 1 {
        if &args[0] == "-h" {
            usage();
            std::process::exit(0);
        }
    }
    if &args[0] == "-n" {
        let args = args
            .iter()
            .skip(1)
            .map(|e| e.to_string())
            .collect::<Vec<_>>();
        print_without_newline(&parse_args(&args));
    } else {
        print_with_newline(&parse_args(args));
    }
}

fn usage() {
    println!("\tUsage: echor [-n] [string ...]");
    println!("\t\t-n: do not print newline");
}

fn parse_args(args: &[String]) -> Vec<String> {
    args.iter().map(|e| e.trim().to_string()).collect()
}

fn print_without_newline(args: &[String]) {
    print!("{}", args.join(" "))
}

fn print_with_newline(args: &[String]) {
    println!("{}", args.join(" "))
}
