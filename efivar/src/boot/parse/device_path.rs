//! This module contains parsing code for a device path, part of a device path list

use std::{convert::TryInto, fmt::Display};

use byteorder::{LittleEndian, ReadBytesExt};
use uuid::Uuid;

use crate::{push::PushVecU8, utils::read_nt_utf16_string, Error};

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
// See spec, 10.3.6.1
pub struct EFIHardDrive {
    pub partition_number: u32,
    pub partition_start: u64,
    pub partition_size: u64,
    pub partition_sig: Uuid,
    pub format: u8,
    pub sig_type: EFIHardDriveType,
}

impl EFIHardDrive {
    pub fn parse(buf: &mut &[u8]) -> crate::Result<EFIHardDrive> {
        Ok(EFIHardDrive {
            partition_number: buf
                .read_u32::<LittleEndian>()
                .map_err(|_| Error::VarParseError)?,
            partition_start: buf
                .read_u64::<LittleEndian>()
                .map_err(|_| Error::VarParseError)?,
            partition_size: buf
                .read_u64::<LittleEndian>()
                .map_err(|_| Error::VarParseError)?,
            partition_sig: Uuid::from_fields(
                buf.read_u32::<LittleEndian>()
                    .map_err(|_| Error::VarParseError)?,
                buf.read_u16::<LittleEndian>()
                    .map_err(|_| Error::VarParseError)?,
                buf.read_u16::<LittleEndian>()
                    .map_err(|_| Error::VarParseError)?,
                &buf.read_u64::<LittleEndian>()
                    .map_err(|_| Error::VarParseError)?
                    .to_le_bytes(),
            ),
            format: buf.read_u8().map_err(|_| Error::VarParseError)?,
            sig_type: EFIHardDriveType::parse(buf.read_u8().map_err(|_| Error::VarParseError)?),
        })
    }
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

        if data_size as usize > buf.len() {
            return Err(Error::VarParseError);
        }
        
        let (mut device_path_data, new_buf) = buf.split_at(data_size.into());
        *buf = new_buf;

        #[allow(clippy::single_match)]
        match r#type {
            consts::DEVICE_PATH_TYPE::MEDIA_DEVICE_PATH => match subtype {
                consts::MEDIA_DEVICE_PATH_SUBTYPE::FILE_PATH => {
                    return Ok(Some(DevicePath::FilePath(FilePath {
                        path: read_nt_utf16_string(&mut device_path_data)
                            .map_err(crate::Error::StringParseError)?,
                    })));
                }
                consts::MEDIA_DEVICE_PATH_SUBTYPE::HARD_DRIVE => {
                    return Ok(Some(DevicePath::HardDrive(EFIHardDrive::parse(
                        &mut device_path_data,
                    )?)));
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

    bytes.push_u8(r#type);
    bytes.push_u8(r#subtype);

    let raw_data_size: u16 = raw_data.len().try_into().expect("length should fit in u16");
    bytes.push_u16(raw_data_size + 1 + 1 + 2);

    bytes.append(&mut raw_data);

    bytes
}

impl EFIHardDrive {
    /// get bytes representation for a EFIHardDrive, without encapsulating them in a DevicePath structure
    fn to_bytes_raw(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        bytes.push_u32(self.partition_number);
        bytes.push_u64(self.partition_start);
        bytes.push_u64(self.partition_size);

        let (f1, f2, f3, f4) = self.partition_sig.as_fields();
        bytes.push_u32(f1);
        bytes.push_u16(f2);
        bytes.push_u16(f3);
        bytes.append(&mut f4.to_vec());
        bytes.push_u8(self.format);
        bytes.push_u8(self.sig_type.as_u8());

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

#[derive(Debug, PartialEq, Clone)]
pub struct FilePath {
    /// the UEFI standard seem to use UCS-2 strings (a subset of UTF-16), so Rust UTF8 strings should be able to represent them
    pub path: String,
}

impl FilePath {
    /// get bytes representation for a FilePath, without encapsulating them in a DevicePath structure
    fn to_bytes_raw(&self) -> Vec<u8> {
        let utf16_bytes = self.path.encode_utf16();
        let mut utf8_bytes: Vec<u8> = utf16_bytes
            .into_iter()
            .flat_map(|var| var.to_le_bytes())
            .collect();

        // write null termination
        utf8_bytes.push_u16(0x0000);

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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use uuid::Uuid;

    use super::{EFIHardDrive, EFIHardDriveType};

    #[test]
    fn efi_hard_drive_type_parse() {
        assert_eq!(EFIHardDriveType::parse(0x01), EFIHardDriveType::Mbr);
        assert_eq!(EFIHardDriveType::parse(0x02), EFIHardDriveType::Gpt);
        assert_eq!(EFIHardDriveType::parse(0x03), EFIHardDriveType::Unknown);
        assert_eq!(EFIHardDriveType::parse(0xFF), EFIHardDriveType::Unknown);
    }

    #[test]
    fn efi_hard_drive_type_dump() {
        assert_eq!(EFIHardDriveType::Mbr.as_u8(), 0x01);
        assert_eq!(EFIHardDriveType::Gpt.as_u8(), 0x02);
    }

    #[test]
    fn efi_hard_drive_type_print() {
        assert_eq!(format!("{}", EFIHardDriveType::Mbr), "MBR");
        assert_eq!(format!("{}", EFIHardDriveType::Gpt), "GPT");
        assert_eq!(format!("{}", EFIHardDriveType::Unknown), "Unknown");
    }

    #[test]
    #[should_panic]
    fn efi_hard_drive_type_dump_invalid() {
        EFIHardDriveType::Unknown.as_u8();
    }

    #[test]
    fn print_hard_drive() {
        assert_eq!(
            "HD(1,GPT,90364bbd-1000-47fc-8c05-8707e01b4593)",
            format!(
                "{}",
                EFIHardDrive {
                    partition_number: 1,
                    partition_start: 2,
                    partition_size: 3,
                    partition_sig: Uuid::from_str("90364bbd-1000-47fc-8c05-8707e01b4593").unwrap(),
                    format: 5,
                    sig_type: EFIHardDriveType::Gpt,
                }
            )
        );
    }

    #[test]
    fn to_from_bytes() {
        let drive = EFIHardDrive {
            partition_number: 1,
            partition_start: 2,
            partition_size: 3,
            partition_sig: Uuid::from_str("90364bbd-1000-47fc-8c05-8707e01b4593").unwrap(),
            format: 5,
            sig_type: EFIHardDriveType::Gpt,
        };
        let bytes = drive.to_bytes_raw();
        let mut x = bytes.as_slice();
        let test_parse = EFIHardDrive::parse(&mut x).unwrap();
        assert_eq!(drive, test_parse)
    }
}
