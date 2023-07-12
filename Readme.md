# Unseen CLI Tool

Unseen is a simple command-line interface (CLI) tool that allows you to hide running applications from the Dock on macOS. It is written in Rust.

## Features

- List all currently running applications.
- Hide specific applications from the Dock.

## Installation

```
cargo install unseen
```

## Usage

Here are the available options:

- `-l`, `--list`: List all running applications.
- `<applications>`: The names of the applications to hide from the Dock.

For example, to hide the Messages application, you would run:

```
sudo unseen Messages
```

## Dependencies

Unseen depends on the following crates:

- `libc`: For checking whether the program is run with `sudo`.
- `clap`: For parsing command-line arguments.

## Contributing

Contributions to Unseen are welcome. If you have a feature request, bug report, or want to contribute code, please open an issue or pull request on GitHub.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
```