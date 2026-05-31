mod config;
mod services;
mod symlinks;
mod theme;

fn main() {
    let config_path = config::find_config();
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("usage: ivylink <apply|theme>");
        return;
    }

    match args[1].as_str() {
        "apply" => symlinks::apply(config_path),
        "theme" => {
            if args.len() < 3 {
                println!("usage: ivylink theme apply <pack> <variant>");
                return;
            }
            match args[2].as_str() {
                "apply" => {
                    if args.len() < 5 {
                        println!("usage: ivylink theme apply <pack> <variant>");
                        return;
                    }
                    theme::apply(&args[3], &args[4])
                }
                _ => println!("usage: ivylink theme apply <pack> <variant>"),
            }
        }
        _ => println!("usage: ivylink <apply|theme>"),
    }
}
