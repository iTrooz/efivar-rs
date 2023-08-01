use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug, Fail)]
pub enum StringParseError {
    /// occurs when you get an error while reading the data
    #[fail(display = "Buffer read error: {}", 0)]
    Read(std::io::Error),
    /// occurs when the bytes are not a valid UTF-16 string
    #[fail(display = "Buffer parse error: {}", 0)]
    Parse(std::string::FromUtf16Error),
}

pub fn read_nt_utf16_string(cursor: &mut &[u8]) -> Result<String, StringParseError> {
    let mut vec: Vec<u16> = vec![];
    loop {
        match cursor
            .read_u16::<LittleEndian>()
            .map_err(StringParseError::Read)?
        {
            0 => {
                return String::from_utf16(&vec).map_err(StringParseError::Parse);
            }
            chr => {
                vec.push(chr);
            }
        }
    }
}
