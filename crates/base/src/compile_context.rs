use std::path::PathBuf;

use crate::file_cache::FileCache;

#[derive(Debug)]
pub struct CompileContext {
    pub emit_irs: bool,
    pub file_cache: FileCache,
    pub build_pkg: PathBuf,
    pub build_dir: PathBuf,
    pub run_immediately: bool,
}
