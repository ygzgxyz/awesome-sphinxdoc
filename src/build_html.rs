use minijinja::{context, Environment};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Link {
    pub name: String,
    pub link: String,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub links: Vec<Link>,
}

pub fn compile() -> Result<(), Box<dyn std::error::Error>> {
    // Read the TOML file
    let toml_str = fs::read_to_string("links.toml")?;

    // Deserialize the TOML into the Data struct
    let data: Data = toml::from_str(&toml_str)?;

    let mut links = data.links;
    links.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    // Create a new minijinja environment
    let mut env = Environment::new();

    // Read the template file
    let template_str = fs::read_to_string("templates/template.html")?;

    // Add the template to the environment
    env.add_template("index.html", &template_str)?;

    // Get the template
    let tmpl = env.get_template("index.html")?;

    // Render the template with the data
    let html = tmpl.render(context!(links => links))?;

    // Create dist directory if it doesn't exist
    fs::create_dir_all("dist")?;

    // Write the rendered HTML to a new file
    fs::write("dist/index.html", html)?;

    println!("Successfully generated dist/index.html");

    Ok(())
}
