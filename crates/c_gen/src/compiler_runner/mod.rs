use std::path::PathBuf;

pub mod clang;
pub mod gcc;

pub trait CodegenRunner {
    fn name(&self) -> String;
    fn is_available(&self) -> bool;
    fn run(&self, options: RunnerOptions) -> Result<PathBuf, String>;
}

pub struct RunnerOptions {
    pub output_name: String,
    pub output_path: PathBuf,
    pub build_files: Vec<PathBuf>,
    pub optimization_level: OptLevel,
}

pub enum OptLevel {
    Debug,
    Release,
    UnsafeRelease,
}
