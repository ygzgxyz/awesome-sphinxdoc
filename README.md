# Awesome Sphinxdoc

## About the Project

Awesome Sphinxdoc is a curated collection of valuable resources for Sphinx, the powerful and flexible documentation generator. This project aims to provide a centralized and easily navigable list of extensions, themes, tutorials, and other tools that enhance the Sphinx documentation ecosystem. It features multi-language support and tag-based filtering to help users quickly find the resources they need.

## Technology Stack

This project is built using **Rust** and managed with **Cargo**. The HTML generation is powered by the `minijinja` templating engine.


## How to Add a Link

We welcome contributions! To add a new link to the Awesome Sphinxdoc list:

1.  **Edit `links.toml`:** Open the `links.toml` file in the project root.
2.  **Add a new entry:** Append a new `[[links]]` entry with the following structure:

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
    *   `tags`: A list of relevant tags (e.g., `"extensions"`, `"themes"`, `"tutorial"`, `"miscellaneous"`, `"publication"`, `"internationalizations"`).

3.  **Generate HTML:** After adding your entry, run the builder to update `dist/index.html`:

    ```bash
    cargo run
    ```

4.  **Submit a Pull Request:** Create a pull request on the GitHub repository with your changes.

For local development and testing, refer to the [Contributing](#contributing) section.

## Contributing

To work on this project (build, generate HTML, and serve locally for development):

1.  **Build Project:** Compile the Rust executable without running it. This creates the executable in `target/debug/`.

    ```bash
    cargo build
    ```

2.  **Generate HTML:** Compile (if necessary) and run the project to create `dist/index.html`.

    ```bash
    cargo run
    ```

3.  **Serve Locally:** Run a local HTTP server.

    ```bash
    python3 -m http.server 8000 --directory dist &
    ```

4.  **View:** Access in browser.

    ```
    http://localhost:8000/index.html
    ```

## Features

*   Multi-language support (EN, KO, JA, ZH) with URL persistence (`?lang=`).
*   Tag-based filtering with URL persistence (`?tag=`).
*   Case-insensitive alphabetical sorting of links.