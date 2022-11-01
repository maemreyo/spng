use args::{matches};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

// fn main() -> Result<()> {
fn main() {
    env_logger::init();
    matches();
}
