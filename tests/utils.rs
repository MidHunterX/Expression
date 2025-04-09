use std::{fs, io, path::Path};

pub fn setup_test_dir(root: &Path, files: &[&str], dirs: &[&str]) -> io::Result<()> {
    fs::create_dir_all(root)?;
    for file in files {
        let file_path = root.join(file);
        fs::write(file_path, "test")?;
    }
    for dir in dirs {
        let dir_path = root.join(dir);
        fs::create_dir_all(dir_path)?;
    }
    Ok(())
}

pub fn cleanup_test_dir(root: &Path) {
    if root.exists() {
        fs::remove_dir_all(root).ok();
    }
}
