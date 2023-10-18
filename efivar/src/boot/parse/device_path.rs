//! This module contains parsing code for a device path, part of a device path list

use std::{convert::TryInto, fmt::Display, io::Write, path::PathBuf};

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use uuid::Uuid;

use crate::{utils::read_nt_utf16_string, Error};

use super::consts;

pub enum DevicePath {
    FilePath(FilePath),
    HardDrive(EFIHardDrive),
}

#[derive(Debug, PartialEq, Clone)]
pub enum EFIHardDriveType {
    Mbr,
    Gpt,
    Unknown, // TODO: remove ?
}

impl EFIHardDriveType {
    pub fn parse(sig_type: u8) -> EFIHardDriveType {
        match sig_type {
            0x01 => Self::Mbr,
            0x02 => Self::Gpt,
            _ => Self::Unknown,
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            EFIHardDriveType::Mbr => 0x01,
            EFIHardDriveType::Gpt => 0x02,
            EFIHardDriveType::Unknown => panic!(),
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

#[derive(Debug, PartialEq, Clone)]
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
    pub fn parse(buf: &mut &[u8]) -> crate::Result<Option<DevicePath>> {
        let r#type = buf.read_u8().map_err(|_| Error::VarParseError)?;
        let subtype = buf.read_u8().map_err(|_| Error::VarParseError)?;
        let length = buf
            .read_u16::<LittleEndian>()
            .map_err(|_| Error::VarParseError)?;

        let data_size = length - 1 - 1 - 2;

        let (mut device_path_data, new_buf) = buf.split_at(data_size.into());
        *buf = new_buf;

        #[allow(clippy::single_match)]
        match r#type {
            consts::DEVICE_PATH_TYPE::MEDIA_DEVICE_PATH => match subtype {
                consts::MEDIA_DEVICE_PATH_SUBTYPE::FILE_PATH => {
                    return Ok(Some(DevicePath::FilePath(FilePath {
                        path: read_nt_utf16_string(&mut device_path_data)
                            .map_err(crate::Error::StringParseError)?
                            .into(),
                    })));
                }
                consts::MEDIA_DEVICE_PATH_SUBTYPE::HARD_DRIVE => {
                    return Ok(Some(DevicePath::HardDrive(EFIHardDrive {
                        partition_number: device_path_data
                            .read_u32::<LittleEndian>()
                            .map_err(|_| Error::VarParseError)?,
                        partition_start: device_path_data
                            .read_u64::<LittleEndian>()
                            .map_err(|_| Error::VarParseError)?,
                        partition_size: device_path_data
                            .read_u64::<LittleEndian>()
                            .map_err(|_| Error::VarParseError)?,
                        partition_sig: Uuid::from_u128(
                            device_path_data
                                .read_u128::<BigEndian>()
                                .map_err(|_| Error::VarParseError)?,
                        ),
                        format: device_path_data
                            .read_u8()
                            .map_err(|_| Error::VarParseError)?,
                        sig_type: EFIHardDriveType::parse(
                            device_path_data
                                .read_u8()
                                .map_err(|_| Error::VarParseError)?,
                        ),
                    })));
                }
                _ => {}
            },
            _ => {}
        };

        Ok(None)
    }
}

fn encap_as_device_path(r#type: u8, r#subtype: u8, mut raw_data: Vec<u8>) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];

    bytes.write_u8(r#type).unwrap();
    bytes.write_u8(r#subtype).unwrap();

    let raw_data_size: u16 = raw_data.len().try_into().unwrap();
    bytes
        .write_u16::<LittleEndian>(raw_data_size + 1 + 1 + 2)
        .unwrap();

    bytes.append(&mut raw_data);

    bytes
}

impl EFIHardDrive {
    /// get bytes representation for a EFIHardDrive, without encapsulating them in a DevicePath structure
    fn to_bytes_raw(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        bytes
            .write_u32::<LittleEndian>(self.partition_number)
            .unwrap();
        bytes
            .write_u64::<LittleEndian>(self.partition_start)
            .unwrap();
        bytes
            .write_u64::<LittleEndian>(self.partition_size)
            .unwrap();
        bytes.write_all(self.partition_sig.as_bytes()).unwrap();
        bytes.write_u8(self.format).unwrap();
        bytes.write_u8(self.sig_type.as_u8()).unwrap();

        bytes
    }

    /// get bytes representation for a EFIHardDrive, as a DevicePath (EFI_DEVICE_PATH_PROTOCOL) structure
    pub fn to_bytes_encap(&self) -> Vec<u8> {
        encap_as_device_path(
            consts::DEVICE_PATH_TYPE::MEDIA_DEVICE_PATH,
            consts::MEDIA_DEVICE_PATH_SUBTYPE::HARD_DRIVE,
            self.to_bytes_raw(),
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct FilePath {
    pub path: PathBuf, // TODO: do not use PathBuf, because it is a OS-specific type ?
}

impl FilePath {
    /// get bytes representation for a FilePath, without encapsulating them in a DevicePath structure
    fn to_bytes_raw(&self) -> Vec<u8> {
        let utf16_bytes = self.path.to_str().unwrap().encode_utf16();
        let mut utf8_bytes: Vec<u8> = utf16_bytes
            .into_iter()
            .flat_map(|var| var.to_le_bytes())
            .collect();

        // write null termination
        utf8_bytes.write_u16::<LittleEndian>(0x0000).unwrap();

        utf8_bytes
    }

    /// get bytes representation for a FilePath, as a DevicePath (EFI_DEVICE_PATH_PROTOCOL) structure
    pub fn to_bytes_encap(&self) -> Vec<u8> {
        encap_as_device_path(
            consts::DEVICE_PATH_TYPE::MEDIA_DEVICE_PATH,
            consts::MEDIA_DEVICE_PATH_SUBTYPE::FILE_PATH,
            self.to_bytes_raw(),
        )
    }
}

pub fn get_end_device_path_bytes() -> Vec<u8> {
    encap_as_device_path(
        consts::DEVICE_PATH_TYPE::END_OF_HARDWARE_DEVICE_PATH,
        consts::END_OF_HARDWARE_DEVICE_PATH_SUBTYPE::END_ENTIRE_DEVICE_PATH,
        vec![],
    )
}
