use std::fmt;
use std::fs;
use std::io;
use std::path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileType {
    File,
    Dir,
    Symlink,
}

impl FileType {
    /// 根据路径获得文件的类型
    pub fn from_path(path: &path::Path, follow: bool) -> io::Result<FileType> {
        let file_type =
            if follow { path.metadata() } else { path.symlink_metadata() }?
                .file_type();
        if file_type.is_dir() {
            return Ok(FileType::Dir);
        }
        if file_type.is_file() {
            return Ok(FileType::File);
        }
        Ok(FileType::Symlink)
    }

    pub fn eval(self, ft: fs::FileType) -> bool {
        match self {
            FileType::File => ft.is_file(),
            FileType::Dir => ft.is_dir(),
            FileType::Symlink => ft.is_symlink(),
        }
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let t = match *self {
            FileType::File => "file",
            FileType::Dir => "dir",
            FileType::Symlink => "symlink",
        };
        write!(f, "{}", t)
    }
}
