## Rust Server-Watcher 
Watch your server/computer and get notified when  resource limits are above a specified % or simply be notified
about their metrics on an configurable interval.

## Tested on
- Windows 10
- Linux (Ubuntu 20.04 LTS)

<br />

## Should be able to run on:
- Windows 7+
- macOS
- Linux

<br />

## Configuring the .env file
Please visit the root of this project, where 
you can find this project's `env_setup.md` file.
If there are any questions, feel free to open an
issue.

<br />

## Usage
* Have rustc 1.45 or above
* Downloaded this project (through git/github)
* Enter the /rust-server folder and setup the `.env` file
* Run the program with `cargo run`


### Compile with --release
If you want to use it as an executable in release mode, go until step 3 and instead of `cargo run`,
type `cargo build --release`. **The `.env` file is found based on your current working directory.**

