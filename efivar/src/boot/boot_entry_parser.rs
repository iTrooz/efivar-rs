use std::{fmt::Display, io::Read};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{efi::VariableName, Error, VarReader};

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

pub fn read_nt_utf16_string(cursor: &mut &[u8]) -> crate::Result<String> {
    let mut vec: Vec<u16> = vec![];
    loop {
        match cursor
            .read_u16::<LittleEndian>()
            .map_err(|_| Error::VarParseError)?
        {
            0 => {
                return String::from_utf16(&vec).map_err(|_| Error::VarParseError);
            }
            chr => {
                vec.push(chr);
            }
        }
    }
}

pub struct BootEntry {
    pub attributes: BootEntryAttributes,
    pub description: String,
    pub file_path_list: Option<FilePathList>,
    pub optional_data: Vec<u8>,
}

impl BootEntry {
    pub fn parse(
        manager: &(impl ?Sized + VarReader),
        variable: &VariableName,
    ) -> crate::Result<Self> {
        let mut conrete_buf = vec![0u8; 512];

        let (written_size, _flags) = manager.read(variable, &mut conrete_buf)?;

        conrete_buf.resize(written_size, 0);
        // slice of the buffer
        // Used so we can move the offset in it with ReadBytesExt functions
        let mut buf = &conrete_buf[..];

        let attributes = buf
            .read_u32::<LittleEndian>()
            .map_err(|_| Error::VarParseError)?;

        let file_path_list_length = buf
            .read_u16::<LittleEndian>()
            .map_err(|_| Error::VarParseError)?;

        let description = read_nt_utf16_string(&mut buf)?;

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
}
