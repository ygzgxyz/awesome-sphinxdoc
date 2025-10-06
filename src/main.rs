
mod build_html;

fn main() {
    if let Err(e) = build_html::compile() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
