# Awesome Sphinxdoc

## About the Project

Awesome Sphinxdoc is a curated collection of valuable resources for Sphinx, the powerful and flexible documentation generator. This project aims to provide a centralized and easily navigable list of extensions, themes, tutorials, and other tools that enhance the Sphinx documentation ecosystem. It features multi-language support and tag-based filtering to help users quickly find the resources they need. You can view the content at https://ygzgxyz.github.io/awesome-sphinxdoc.

## Technology Stack

This project is built using **Rust** and managed with **Cargo**. The HTML generation is powered by the `minijinja` templating engine.


## How to Add a Link

We welcome contributions! To add a new link to the Awesome Sphinxdoc list:

1.  **Start the development server:**
    Run the following command to start the development server. You can optionally provide a port number.

    ```bash
    cargo run -- dev [PORT]
    ```
2.  **Edit `links.toml`:** Open the `links.toml` file in the project root and add a new `[[links]]` entry with the following structure:

    ```toml
    [[links]]
    name = "Your Link Name"
    link = "https://example.com/your-link"
    description = "A brief description of what this link offers."
    tags = ["tag1", "tag2"]
    ```

    *   `name`: The title of the resource.
    *   `link`: The URL to the resource.
    *   `description`: A concise explanation of the resource.
    *   `tags`: A list of relevant tags.

3.  **Verify your changes:** The development server will automatically rebuild the page. Open `http://127.0.0.1:<PORT>` in your browser (e.g., `http://127.0.0.1:8000`) to see your changes and make sure they look correct.

4.  **Submit a Pull Request:** Once you are happy with your changes, commit them and create a pull request on the GitHub repository.

## Contributing

This project includes a development server that simplifies local development by automatically rebuilding the HTML file when you make changes.

### Development Workflow

1.  **Start the Dev Server:**
    Run the following command to start the development server. By default, it runs on port 8000.

    ```bash
    cargo run -- dev
    ```

    You can also specify a custom port:

    ```bash
    cargo run -- dev 8080
    ```

    This will:
    *   Perform an initial build of the `dist/index.html` file.
    *   Start a local web server at the specified (or default) port.
    *   Watch for changes in `links.toml` and `templates/template.html` and automatically rebuild the HTML file.

2.  **Make Changes:**
    Edit files like `links.toml` or `templates/template.html`. The server will detect the changes and rebuild automatically.

3.  **View in Browser:**
    Open `http://127.0.0.1:<PORT>` in your browser (e.g., `http://127.0.0.1:8000`) to see your changes.

### Checking for Broken Links

Before submitting a pull request, it's a good practice to check for any broken links. You can do this with the `check-links` command:

```bash
cargo run -- check-links
```

## Features

*   Multi-language support (EN, KO, JA, ZH) with URL persistence (`?lang=`).
*   Tag-based filtering with URL persistence (`?tag=`).
*   Case-insensitive alphabetical sorting of links.