//! This module contains parsing code for the device path list component of a boot entry

use std::fmt::Display;

use super::{
    device_path::{self, FilePath},
    DevicePath, EFIHardDrive,
};

/// holds the potential fields we may get from a packed file path list
/// TODO remove ?
#[derive(Debug)]
pub struct OptFilePathList {
    pub file_path: Option<FilePath>,
    pub hard_drive: Option<EFIHardDrive>,
}

/// Same structure as OptFilePathList, but we ensure that the file path list
/// is a valid file path overall
#[derive(Debug, PartialEq, Clone)]
pub struct FilePathList {
    pub file_path: FilePath,
    pub hard_drive: EFIHardDrive,
}

impl From<OptFilePathList> for Option<FilePathList> {
    fn from(value: OptFilePathList) -> Self {
        Some(FilePathList {
            file_path: value.file_path?,
            hard_drive: value.hard_drive?,
        })
    }
}

impl Display for FilePathList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/File({})", self.hard_drive, self.file_path.path)
    }
}

impl FilePathList {
    pub fn parse(full_buf: &mut &[u8]) -> crate::Result<OptFilePathList> {
        let mut file_path_list = OptFilePathList {
            file_path: None,
            hard_drive: None,
        };

        loop {
            if full_buf.is_empty() {
                break;
            } else {
                match DevicePath::parse(full_buf)? {
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

        Ok(file_path_list)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        bytes.append(&mut self.hard_drive.to_bytes_encap());
        bytes.append(&mut self.file_path.to_bytes_encap());
        bytes.append(&mut device_path::get_end_device_path_bytes());

        bytes
    }
}
