use std::{convert::TryInto, fmt::Display, io::Read};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{efi::Variable, utils::read_nt_utf16_string, Error, VarReader};

use super::FilePathList;

bitflags! {
    /// Possible attributes of a boot entry as a bitfield
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct BootEntryAttributes : u32 {
        const LOAD_OPTION_ACTIVE = 0x1;
        const LOAD_OPTION_FORCE_RECONNECT = 0x2;
        const LOAD_OPTION_HIDDEN = 0x8;
        const LOAD_OPTION_CATEGORY_APP = 0x100;
    }
}

impl Display for BootEntryAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

pub struct BootEntry {
    pub attributes: BootEntryAttributes,
    pub description: String,
    pub file_path_list: Option<FilePathList>,
    pub optional_data: Vec<u8>,
}

impl BootEntry {
    pub fn parse(manager: &(impl ?Sized + VarReader), variable: &Variable) -> crate::Result<Self> {
        let (value, _flags) = manager.read(variable)?;

        // slice of the buffer
        // Used so we can move the offset in it with ReadBytesExt functions
        let mut buf = &value[..];

        let attributes = buf
            .read_u32::<LittleEndian>()
            .map_err(|_| Error::VarParseError)?;

        let file_path_list_length = buf
            .read_u16::<LittleEndian>()
            .map_err(|_| Error::VarParseError)?;

        let description = read_nt_utf16_string(&mut buf).map_err(crate::Error::StringParseError)?;

        let mut file_path_list_buf = vec![0u8; file_path_list_length.into()];
        buf.read_exact(&mut file_path_list_buf)
            .map_err(|_| Error::VarParseError)?;

        let file_path_list = FilePathList::parse(&mut &file_path_list_buf[..])?.into();

        Ok(BootEntry {
            attributes: BootEntryAttributes::from_bits(attributes).ok_or(Error::VarParseError)?,
            description,
            file_path_list,
            optional_data: buf.to_vec(),
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        // append attribute bytes
        bytes.append(&mut self.attributes.bits().to_le_bytes().to_vec());

        // append file path list length
        let mut fpl_bytes: Vec<u8> = if let Some(fpl) = &self.file_path_list {
            fpl.to_bytes()
        } else {
            vec![]
        };
        bytes
            .write_u16::<LittleEndian>(fpl_bytes.len().try_into().unwrap())
            .unwrap();

        // append description bytes
        let mut desc_bytes = self
            .description
            .encode_utf16()
            .flat_map(|var| var.to_le_bytes())
            .collect();
        bytes.append(&mut desc_bytes);
        // write description null termination
        bytes.write_u16::<LittleEndian>(0x0000).unwrap();

        // append file path list bytes
        bytes.append(&mut fpl_bytes);

        // append optional data
        bytes.append(&mut self.optional_data.clone());

        bytes
    }
}
