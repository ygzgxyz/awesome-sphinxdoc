mod build_html;
mod dev_server;
mod link_checker;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "dev" => {
                let port = if args.len() > 2 {
                    args[2].parse::<u16>().unwrap_or(8000)
                } else {
                    8000
                };
                dev_server::run(port);
            }
            "check-links" => {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    if let Err(e) = link_checker::check().await {
                        eprintln!("Error checking links: {}", e);
                        std::process::exit(1);
                    }
                });
            }
            _ => {
                println!("Unknown command: {}", args[1]);
                println!("Usage: cargo run [dev|check-links]");
                std::process::exit(1);
            }
        }
    } else {
        // Default action: build
        if let Err(e) = build_html::compile() {
            eprintln!("Error building HTML: {}", e);
            std::process::exit(1);
        }
    }
}
