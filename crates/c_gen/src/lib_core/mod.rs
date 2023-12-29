use std::{
    fs,
    path::{Path, PathBuf},
};

const PATH_CORE_C: &str = "crates/c_gen/src/lib_core/core_c";

pub fn fetch_core_c() -> Vec<PathBuf> {
    let p = Path::new(PATH_CORE_C);
    let c_std = fs::read_dir(p).unwrap();
    c_std
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.is_file() && path.extension().unwrap() == "h" || path.extension().unwrap() == "c"
        })
        .collect()
}

pub fn copy_core_c<P: AsRef<Path>>(copy_to: P) {
    let copy_to = copy_to.as_ref();
    fs::create_dir_all(copy_to).unwrap();
    let file = fetch_core_c();
    for header_file in file {
        let file_name: &std::ffi::OsStr = header_file.file_name().unwrap();
        let copy_to = copy_to.join(file_name);
        fs::copy(header_file, copy_to).unwrap();
    }
}
