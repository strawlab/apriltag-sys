#[derive(Debug)]
struct Error {}

impl From<glob::PatternError> for Error {
    fn from(_: glob::PatternError) -> Error {
        Error {}
    }
}

impl From<glob::GlobError> for Error {
    fn from(_: glob::GlobError) -> Error {
        Error {}
    }
}

fn main() -> Result<(),Error> {
    match std::env::var_os("APRILTAG_SRC") {
        Some(sdk_path) => {
            // Source code location given as environment variable.

            let sdk_path = std::path::PathBuf::from(sdk_path);
            let inc_dir = sdk_path.clone();

            let mut compiler = cc::Build::new();

            // add files in base SDK dir
            let mut c_files = sdk_path.clone();
            c_files.push("*.c");
            let glob_pattern = c_files.to_str().unwrap();
            let paths = glob::glob_with(glob_pattern, glob::MatchOptions::new())?;
            for path in paths {
                let path = path?;
                let path_str = path.display().to_string();
                if path_str.ends_with("apriltag_pywrap.c") {
                    continue
                }
                compiler.file(&path);
            }

            // add files in base/common SDK dir
            let mut c_files = sdk_path.clone();
            c_files.push("common");
            c_files.push("*.c");
            let glob_pattern = c_files.to_str().unwrap();
            let paths = glob::glob_with(glob_pattern, glob::MatchOptions::new())?;
            for path in paths {
                compiler.file(&path?);
            }

            compiler.include(&inc_dir);
            compiler.extra_warnings(false);
            compiler.compile("apriltags");
        }
        None => {
            // Source code location not given, use pkg-config
            let _libapriltag = pkg_config::probe_library("apriltag").unwrap();
        }
    }
    Ok(())
}
