use env_plus::EnvLoader;
use rust_server;

fn main() {
    EnvLoader::new()
    .change_file(String::from("./.env"))
    .change_comment(String::from("#"))
    .activate();

    rust_server::start();
}
