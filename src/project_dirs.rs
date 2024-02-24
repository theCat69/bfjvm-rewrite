use std::{
    fs,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use once_cell::sync::Lazy;

#[derive(Clone)]
pub struct ProjDirs {
    proj_dir: ProjectDirs,
    data_local_dir: PathBuf,
    config_dir: PathBuf,
    candidates_dir: PathBuf,
    current_dir: PathBuf,
    memory_dir: PathBuf,
    registry_dir: PathBuf,
}

unsafe impl Sync for ProjDirs {}

impl ProjDirs {
    fn new() -> Self {
        let proj_dir: ProjectDirs = init_proj_dir();
        let data_local_dir: PathBuf = lazy_init_dirs(proj_dir.data_local_dir());
        let config_dir: PathBuf = lazy_init_dirs(proj_dir.config_local_dir());
        let candidates_dir: PathBuf = lazy_init_dirs(Path::join(&data_local_dir, "candidates"));
        let current_dir: PathBuf = lazy_init_dirs(Path::join(&data_local_dir, "current"));
        let memory_dir: PathBuf = lazy_init_dirs(Path::join(&data_local_dir, "memory"));
        let registry_dir: PathBuf = lazy_init_dirs(Path::join(&data_local_dir, "registry"));
        ProjDirs {
            proj_dir,
            data_local_dir,
            config_dir,
            candidates_dir,
            current_dir,
            memory_dir,
            registry_dir,
        }
    }
}

pub fn get_proj_dirs_lazy() -> ProjDirs {
    static PROJ_DIRS: Lazy<ProjDirs> = Lazy::new(ProjDirs::new);
    PROJ_DIRS.clone()
}

fn init_proj_dir() -> ProjectDirs {
    match ProjectDirs::from("rs", "", "bf-j-vm-rewrite") {
        Some(proj_dirs) => proj_dirs,
        None => panic!("Error creating config dir"),
    }
}

fn lazy_init_dirs<P: AsRef<Path>>(dir: P) -> PathBuf {
    let ret_dir = dir.as_ref().to_path_buf();
    match fs::create_dir_all(dir) {
        Ok(()) => ret_dir,
        Err(err) => panic!("Error creating config dir : {err}"),
    }
}
