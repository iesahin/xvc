use std::ffi::OsString;
use std::fmt::Display;
use std::ops::Deref;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AbsolutePath(PathBuf);

impl From<PathBuf> for AbsolutePath {
    fn from(p: PathBuf) -> Self {
        if p.is_absolute() {
            Self(p)
        } else {
            let current_dir = std::env::current_dir().expect("Cannot determine current dir");
            let joined = current_dir.join(p);
            Self(
                joined
                    .canonicalize()
                    .unwrap_or_else(|_| panic!("Cannot canonicalize {:?}", joined)),
            )
        }
    }
}

impl Display for AbsolutePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string_lossy())
    }
}

impl AbsolutePath {
    pub fn into_os_string(&self) -> OsString {
        self.0.clone().as_os_str().to_os_string()
    }

    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

impl From<&PathBuf> for AbsolutePath {
    fn from(p: &PathBuf) -> Self {
        Self::from(p.to_owned())
    }
}

impl From<&Path> for AbsolutePath {
    fn from(p: &Path) -> Self {
        Self::from(p.to_path_buf())
    }
}

impl From<AbsolutePath> for PathBuf {
    fn from(a: AbsolutePath) -> Self {
        a.0
    }
}

impl From<&str> for AbsolutePath {
    fn from(s: &str) -> Self {
        Self::from(PathBuf::from(s.to_string()))
    }
}

impl From<String> for AbsolutePath {
    fn from(s: String) -> Self {
        Self::from(PathBuf::from(s))
    }
}

impl Deref for AbsolutePath {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Path> for AbsolutePath {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}
