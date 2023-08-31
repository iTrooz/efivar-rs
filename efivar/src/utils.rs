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
            0x0000 => {
                return String::from_utf16(&vec).map_err(StringParseError::Parse);
            }
            chr => {
                vec.push(chr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_string() -> Result<(), StringParseError> {
        let data: Vec<u8> = vec![b'a', 0x00, b'b', 0x00, b'c', 0x00, 0x00, 0x00, 0xFF]; // abc + null termination + a byte
        let data_slice = &mut &data[..];

        // verify extracted string is right
        assert_eq!(read_nt_utf16_string(data_slice)?, "abc");

        // verify the byte is left
        assert_eq!(data_slice, &vec![0xFF]);
        Ok(())
    }

    #[test]
    fn read_string_without_nt() {
        let data: Vec<u8> = vec![b'a', 0x00, b'b', 0x00, b'c', 0x00]; // abc
        let data_slice = &mut &data[..];

        let err = read_nt_utf16_string(data_slice).expect_err("Invalid string should return error");

        if let StringParseError::Read(io_err) = err {
            assert_eq!(io_err.kind(), std::io::ErrorKind::UnexpectedEof);
        } else {
            panic!("Error was not read error");
        };
    }

    #[test]
    fn read_string_invalid_utf16() {
        let data: Vec<u8> = vec![0x00, 0xD8, 0x69, 0x00, 0x00, 0x00];
        let data_slice = &mut &data[..];

        let err = read_nt_utf16_string(data_slice).expect_err("Invalid string should return error");

        if let StringParseError::Parse(_) = err {
        } else {
            panic!("Error was not parse error");
        };
    }
}
