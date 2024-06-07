# maud-magic-rs

maud-magic-rs is a lightweight example of a Website utilizing `light-magic` as its database and powered by the `HTMX` library for seamless server-client interactions.

## Features

- **Website:** Built with `daisy-ui`. No extra js required, it just uses HTMX!
- **light-magic:** Utilizes an `in-memory` database for data storage, providing a simple and self-contained solution.
- **HTMX Integration:** Enhances server-client interactions through HTMX, allowing for dynamic updates and a smoother user experience.

## Usage

- **Install Perquisites:** You have to have [rust](https://www.rust-lang.org/), [air](https://github.com/cosmtrek/air) and [bun](https://bun.sh/) installed.
- **Install Dependencies:** Install dependencies of bun (in [content](/assets/static/content/)).
- **Run Dev:** Finally, You have to use the `air` command, it's pre-configured in the [air-toml](.air.toml).
- **Build:** To build the project you have to run the following command, **make sure to include in your export the static files**:
```sh
cd ./assets/static/content && bunx tailwindcss -i ./../../input.css -o ./dist/output.css && cd ./../../../ && cargo build -r
```
