use std::{fmt::Display, path::PathBuf};

use super::{DevicePath, EFIHardDrive};

/// holds the potential fields we may get from a packed file path list
pub struct OptFilePathList {
    pub file_path: Option<PathBuf>,
    pub hard_drive: Option<EFIHardDrive>,
}

/// Same structure as OptFilePathList, but we ensure that the file path list
/// is a valid file path overall
pub struct FilePathList {
    pub file_path: PathBuf,
    pub hard_drive: EFIHardDrive,
}

impl Display for FilePathList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\\File({})", self.hard_drive, self.file_path.display())
    }
}

impl FilePathList {
    pub fn parse(full_buf: &mut &[u8]) -> Option<FilePathList> {
        let opt_file_path_list = Self::parse_opt(full_buf);

        Some(FilePathList {
            file_path: opt_file_path_list.file_path?,
            hard_drive: opt_file_path_list.hard_drive?,
        })
    }

    pub fn parse_opt(full_buf: &mut &[u8]) -> OptFilePathList {
        let mut file_path_list = OptFilePathList {
            file_path: None,
            hard_drive: None,
        };

        loop {
            if full_buf.is_empty() {
                break;
            } else {
                match DevicePath::parse(full_buf) {
                    Some(DevicePath::FilePath(inner_path)) => {
                        file_path_list.file_path = Some(inner_path);
                    }
                    Some(DevicePath::HardDrive(inner_hard_drive)) => {
                        file_path_list.hard_drive = Some(inner_hard_drive);
                    }
                    None => {}
                };
            };
        }

        file_path_list
    }
}
