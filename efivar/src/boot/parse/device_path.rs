use std::fmt::Display;

use byteorder::{LittleEndian, ReadBytesExt};
use uuid::Uuid;

use crate::boot::boot_entry_parser::read_nt_utf16_string;

use super::consts;

pub enum DevicePath {
    FilePath(std::path::PathBuf),
    HardDrive(EFIHardDrive),
}

pub enum EFIHardDriveType {
    Mbr,
    Gpt,
    Unknown,
}

impl EFIHardDriveType {
    pub fn parse(sig_type: u8) -> EFIHardDriveType {
        match sig_type {
            0x01 => Self::Mbr,
            0x02 => Self::Gpt,
            _ => Self::Unknown,
        }
    }
}

impl Display for EFIHardDriveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EFIHardDriveType::Mbr => f.write_str("MBR"),
            EFIHardDriveType::Gpt => f.write_str("GPT"),
            EFIHardDriveType::Unknown => f.write_str("Unknown"),
        }
    }
}

pub struct EFIHardDrive {
    pub partition_number: u32,
    pub partition_start: u64,
    pub partition_size: u64,
    pub partition_sig: Uuid,
    pub format: u8,
    pub sig_type: EFIHardDriveType,
}

impl Display for EFIHardDrive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HD({},{},{})",
            self.partition_number, self.sig_type, self.partition_sig
        )
    }
}

impl DevicePath {
    pub fn parse(buf: &mut &[u8]) -> Option<DevicePath> {
        let r#type = buf.read_u8().unwrap();
        let subtype = buf.read_u8().unwrap();
        let length = buf.read_u16::<LittleEndian>().unwrap();

        let data_size = length - 1 - 1 - 2;

        let (mut device_path_data, new_buf) = buf.split_at(data_size.into());
        *buf = new_buf;

        #[allow(clippy::single_match)]
        match r#type {
            consts::DEVICE_PATH_TYPE::MEDIA_DEVICE_PATH => match subtype {
                consts::MEDIA_DEVICE_PATH_SUBTYPE::FILE_PATH => {
                    return Some(DevicePath::FilePath(
                        read_nt_utf16_string(&mut device_path_data).into(),
                    ));
                }
                consts::MEDIA_DEVICE_PATH_SUBTYPE::HARD_DRIVE => {
                    return Some(DevicePath::HardDrive(EFIHardDrive {
                        partition_number: device_path_data.read_u32::<LittleEndian>().unwrap(),
                        partition_start: device_path_data.read_u64::<LittleEndian>().unwrap(),
                        partition_size: device_path_data.read_u64::<LittleEndian>().unwrap(),
                        partition_sig: Uuid::from_u128(
                            device_path_data.read_u128::<LittleEndian>().unwrap(),
                        ),
                        format: device_path_data.read_u8().unwrap(),
                        sig_type: EFIHardDriveType::parse(device_path_data.read_u8().unwrap()),
                    }));
                }
                _ => {}
            },
            _ => {}
        };

        None
    }
}
