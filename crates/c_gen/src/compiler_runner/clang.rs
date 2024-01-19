use super::{CodegenRunner, OptLevel, RunnerOptions};
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

#[derive(Default)]
pub struct Clang {}

impl Clang {
    pub fn new() -> Self {
        Self {}
    }
}

impl CodegenRunner for Clang {
    fn name(&self) -> String {
        String::from("clang")
    }

    fn is_available(&self) -> bool {
        Command::new("clang")
            .arg("-v")
            .stderr(Stdio::null())
            .status()
            .is_ok()
    }

    fn run(&self, options: RunnerOptions) -> Result<PathBuf, String> {
        let mut cmd = Command::new("clang");

        let out = options.output_path.join(&options.output_name);
        cmd.arg("-o").arg(&out);

        cmd.args(options.build_files);

        match options.optimization_level {
            OptLevel::Debug => {
                cmd.arg("-g");
            }
            OptLevel::Release => {
                cmd.arg("-O3");
            }
            OptLevel::UnsafeRelease => {
                cmd.arg("-Ofast");
            }
        }

        let output = match cmd.output() {
            Ok(output) => output,
            Err(err) => return Err(err.to_string()),
        };

        let generated_source = if !output.status.success() {
            let stderr = String::from_utf8(output.stderr).unwrap();
            return Err(stderr);
        } else {
            out
        };

        Ok(generated_source)
    }
}
