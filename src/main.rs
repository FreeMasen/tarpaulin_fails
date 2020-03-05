use cargo::util::{Config, homedir};
use cargo::core::{Workspace, Shell};
use std::path::{Path, PathBuf};

fn main() {
    let cwd = PathBuf::from("");
    let hd = homedir(Path::new(&cwd)).unwrap();
    let mut cfg = Config::new(Shell::new(), cwd, hd);
    cfg.configure(
        0u32,
        false,
        None,
        false,
        false,
        false,
        &None,
        &[],
        &[]
    ).unwrap();
    let _workspace = Workspace::new(&Path::new("Cargo.toml"), &cfg);
}
