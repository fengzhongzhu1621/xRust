use super::Result;
use super::{PathAbs, PathDir, PathFile};
use super::{PathMut, PathOps};
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Borrow;
use std::fmt;
use std::path::{Path, PathBuf};
use std::string::ToString;
use std::sync::Arc;
use stfu8;

use std::ffi::{OsStr, OsString};
#[cfg(unix)]
use std::os::unix::ffi::{OsStrExt, OsStringExt};
#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::{OsStrExt, OsStringExt};
#[cfg(windows)]
use std::os::windows::ffi::{OsStrExt, OsStringExt};

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct PathSer(Arc<PathBuf>);

impl PathSer {
    pub fn new<P: Into<Arc<PathBuf>>>(path: P) -> Self {
        PathSer(path.into())
    }

    pub fn as_path(&self) -> &Path {
        self.as_ref()
    }
}

impl fmt::Debug for PathSer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<OsStr> for PathSer {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.0.as_ref().as_ref()
    }
}

impl AsRef<Path> for PathSer {
    fn as_ref(&self) -> &Path {
        self.0.as_ref()
    }
}

impl AsRef<PathBuf> for PathSer {
    fn as_ref(&self) -> &PathBuf {
        self.0.as_ref()
    }
}

impl Borrow<Path> for PathSer {
    fn borrow(&self) -> &Path {
        self.as_ref()
    }
}

impl Borrow<PathBuf> for PathSer {
    fn borrow(&self) -> &PathBuf {
        self.as_ref()
    }
}

impl<'a> Borrow<Path> for &'a PathSer {
    fn borrow(&self) -> &Path {
        self.as_ref()
    }
}

impl<'a> Borrow<PathBuf> for &'a PathSer {
    fn borrow(&self) -> &PathBuf {
        self.as_ref()
    }
}

impl<P: Into<PathBuf>> From<P> for PathSer {
    fn from(path: P) -> PathSer {
        PathSer::new(path.into())
    }
}

impl From<PathSer> for Arc<PathBuf> {
    fn from(path: PathSer) -> Arc<PathBuf> {
        path.0
    }
}

impl PathMut for PathSer {
    fn append<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.0.append(path)
    }
    fn pop_up(&mut self) -> Result<()> {
        self.0.pop_up()
    }
    fn truncate_to_root(&mut self) {
        self.0.truncate_to_root()
    }
    fn set_file_name<S: AsRef<OsStr>>(&mut self, file_name: S) {
        self.0.set_file_name(file_name)
    }
    fn set_extension<S: AsRef<OsStr>>(&mut self, extension: S) -> bool {
        self.0.set_extension(extension)
    }
}

impl PathOps for PathSer {
    type Output = PathSer;

    fn concat<P: AsRef<Path>>(&self, path: P) -> Result<Self::Output> {
        Ok(PathSer(self.0.concat(path)?))
    }

    fn join<P: AsRef<Path>>(&self, path: P) -> Self::Output {
        let buf = Path::join(self.as_path(), path);
        Self::Output::new(buf)
    }

    fn with_file_name<S: AsRef<OsStr>>(&self, file_name: S) -> Self::Output {
        PathSer(self.0.with_file_name(file_name))
    }

    fn with_extension<S: AsRef<OsStr>>(&self, extension: S) -> Self::Output {
        PathSer(self.0.with_extension(extension))
    }
}

pub trait ToStfu8 {
    fn to_stfu8(&self) -> String;
}

impl<T> ToStfu8 for T
where
    T: Borrow<PathBuf>,
{
    #[cfg(any(target_os = "wasi", unix))]
    fn to_stfu8(&self) -> String {
        let bytes = self.borrow().as_os_str().as_bytes();
        stfu8::encode_u8(bytes)
    }

    #[cfg(windows)]
    fn to_stfu8(&self) -> String {
        let wide: Vec<u16> = self.borrow().as_os_str().encode_wide().collect();
        stfu8::encode_u16(&wide)
    }
}

pub trait FromStfu8: Sized {
    fn from_stfu8(s: &str) -> ::std::result::Result<Self, stfu8::DecodeError>;
}

impl<T> FromStfu8 for T
where
    T: From<PathBuf>,
{
    #[cfg(any(target_os = "wasi", unix))]
    fn from_stfu8(s: &str) -> ::std::result::Result<T, stfu8::DecodeError> {
        let raw_path = stfu8::decode_u8(s)?;
        let os_str = OsString::from_vec(raw_path);
        let pathbuf: PathBuf = os_str.into();
        Ok(pathbuf.into())
    }

    #[cfg(windows)]
    fn from_stfu8(s: &str) -> ::std::result::Result<T, stfu8::DecodeError> {
        let raw_path = stfu8::decode_u16(&s)?;
        let os_str = OsString::from_wide(&raw_path);
        let pathbuf: PathBuf = os_str.into();
        Ok(pathbuf.into())
    }
}

macro_rules! stfu8_serialize {
    ($name:ident) => {
        impl Serialize for $name {
            fn serialize<S>(
                &self,
                serializer: S,
            ) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(&self.to_stfu8())
            }
        }
    };
}

stfu8_serialize!(PathSer);
stfu8_serialize!(PathAbs);
stfu8_serialize!(PathFile);
stfu8_serialize!(PathDir);

impl<'de> Deserialize<'de> for PathSer {
    fn deserialize<D>(
        deserializer: D,
    ) -> ::std::result::Result<PathSer, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let path = PathBuf::from_stfu8(&s)
            .map_err(|err| serde::de::Error::custom(&err.to_string()))?;
        Ok(PathSer(Arc::new(path)))
    }
}

impl<'de> Deserialize<'de> for PathAbs {
    fn deserialize<D>(
        deserializer: D,
    ) -> ::std::result::Result<PathAbs, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let path = PathBuf::from_stfu8(&s)
            .map_err(|err| serde::de::Error::custom(&err.to_string()))?;
        Ok(PathAbs(Arc::new(path)))
    }
}

impl<'de> Deserialize<'de> for PathFile {
    fn deserialize<D>(
        deserializer: D,
    ) -> ::std::result::Result<PathFile, D::Error>
    where
        D: Deserializer<'de>,
    {
        let abs = PathAbs::deserialize(deserializer)?;
        PathFile::try_from(abs)
            .map_err(|err| serde::de::Error::custom(&err.to_string()))
    }
}

impl<'de> Deserialize<'de> for PathDir {
    fn deserialize<D>(
        deserializer: D,
    ) -> ::std::result::Result<PathDir, D::Error>
    where
        D: Deserializer<'de>,
    {
        let abs = PathAbs::deserialize(deserializer)?;
        PathDir::try_from(abs)
            .map_err(|err| serde::de::Error::custom(&err.to_string()))
    }
}
