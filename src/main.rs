use crate::project_dirs::ProjDirs;

mod errors;
mod get;
mod init;
mod install;
mod memory;
mod project_dirs;
mod registry;
mod set;
mod types;
mod uninstall;
mod update;

fn main() {
    ProjDirs::new();
    println!("Hello, world!");
}
