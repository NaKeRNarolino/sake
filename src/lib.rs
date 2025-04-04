pub mod src {
    use std::ffi::OsStr;
    use std::path::{Path, PathBuf};

    /// Returns the contents of the specific file in the BP folder
    pub fn read_bp(path: impl AsRef<OsStr>) -> std::io::Result<String> {
        let buf= PathBuf::from(&path);
        let mut res = PathBuf::new();
        res.push("BP");
        res.push(buf);

        std::fs::read_to_string(&res)
    }

    /// Returns the contents of the specific file in the RP folder
    pub fn read_rp(path: impl AsRef<OsStr>) -> std::io::Result<String> {
        let buf= PathBuf::from(&path);
        let mut res = PathBuf::new();
        res.push("RP");
        res.push(buf);

        std::fs::read_to_string(&res)
    }

    /// Writes to some path in the BP folder
    pub fn write_bp(path: impl AsRef<OsStr>, contents: impl AsRef<[u8]>) -> std::io::Result<()> {
        let buf= PathBuf::from(&path);
        let mut res = PathBuf::new();
        res.push("BP");
        res.push(buf);

        std::fs::write(&res, contents)
    }

    /// Writes to some path in the RP folder
    pub fn write_rp(path: impl AsRef<OsStr>, contents: impl AsRef<[u8]>) -> std::io::Result<()> {
        let buf= PathBuf::from(&path);
        let mut res = PathBuf::new();
        res.push("RP");
        res.push(buf);

        std::fs::write(&res, contents)
    }
}

pub mod dev {
    use colog::format::{CologStyle, DefaultCologStyle};
    use log::Level;

    pub fn is_dev_env() -> bool {
        std::env::var("SAKE_DEV").unwrap_or("false".into()) == "true"
    }

    pub struct ActionCologStyle(pub String);
    impl CologStyle for ActionCologStyle {
        fn prefix_token(&self, level: &Level) -> String {
            format!("[{}] -> {}", self.0, DefaultCologStyle.prefix_token(level))
        }
    }

    pub fn setup_colog(name: impl Into<String>) {
        colog::default_builder().format(colog::formatter(ActionCologStyle(name.into()))).init();
    }
}