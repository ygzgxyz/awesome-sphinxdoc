use notify::{RecursiveMode, Watcher};
use std::fs;
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use tiny_http::{Header, Response, Server};

fn rebuild() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rebuilding...");
    match crate::build_html::compile() {
        Ok(_) => {
            println!("Rebuild successful.");
            Ok(())
        }
        Err(e) => {
            eprintln!("Error during rebuild: {}", e);
            Err(e)
        }
    }
}

pub fn run(port: u16) {
    // Initial build
    if rebuild().is_err() {
        eprintln!("Initial build failed. Please fix the errors before starting the server.");
        return;
    }

    // Start the server in a new thread
    let _server_thread = thread::spawn(move || {
        let addr = format!("127.0.0.1:{}", port);
        let server = Server::http(addr).unwrap();
        println!("Server started at http://127.0.0.1:{}", port);

        for request in server.incoming_requests() {
            let path = request.url().split('?').next().unwrap_or("/");

            let file_path = if path == "/" {
                "dist/index.html".to_string()
            } else {
                format!("dist{}", path)
            };

            match fs::read(&file_path) {
                Ok(content) => {
                    let mut response = Response::from_data(content);
                    if file_path.ends_with(".css") {
                        response.add_header("Content-Type: text/css".parse::<Header>().unwrap());
                    } else if file_path.ends_with(".js") {
                        response.add_header(
                            "Content-Type: application/javascript"
                                .parse::<Header>()
                                .unwrap(),
                        );
                    } else if file_path.ends_with(".html") {
                        response.add_header("Content-Type: text/html".parse::<Header>().unwrap());
                    }
                    println!(
                        "{} {} - {}",
                        request.method(),
                        request.url(),
                        response.status_code().0
                    );
                    request.respond(response).unwrap();
                }
                Err(_) => {
                    let response = Response::from_string("404 Not Found").with_status_code(404);
                    println!("{} {} - 404", request.method(), request.url());
                    request.respond(response).unwrap();
                }
            }
        }
    });

    // Set up the file watcher
    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(tx).unwrap();

    watcher
        .watch(Path::new("links.toml"), RecursiveMode::NonRecursive)
        .unwrap();
    watcher
        .watch(
            Path::new("templates/template.html"),
            RecursiveMode::NonRecursive,
        )
        .unwrap();

    println!("Watching for file changes...");

    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(Ok(_)) => {
                println!("Change detected.");
                let _ = rebuild();
            }
            Ok(Err(e)) => println!("watch error: {:?}", e),
            Err(_) => {
                // Timeout, do nothing
            }
        }
    }
}
