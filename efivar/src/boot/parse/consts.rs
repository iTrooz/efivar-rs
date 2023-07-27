/// Holds magic numbers of the different subtypes for the device path type MEDIA_DEVICE_PATH
#[allow(unused)]
#[allow(non_snake_case)] // module only holds constants
pub mod MEDIA_DEVICE_PATH_SUBTYPE {
    pub const HARD_DRIVE: u8 = 0x01;
    pub const CD_ROM: u8 = 0x02;
    pub const VENDOR: u8 = 0x03;
    pub const FILE_PATH: u8 = 0x04;
    pub const MEDIA_PROTOCOL: u8 = 0x05;
    pub const PIWG_FIRMWARE_FILE: u8 = 0x06;
    pub const PIWG_FIRMWARE_VOLUME: u8 = 0x07;
    pub const RELATIVE_OFFSET_RANGE: u8 = 0x08;
    pub const RAM_DISK_DEVICE_PATH: u8 = 0x09;
}

/// Holds magic numbers of the different types of device path types
#[allow(unused)]
#[allow(non_snake_case)] // module only holds constants
pub mod DEVICE_PATH_TYPE {
    pub const HARDWARE_DEVICE_PATH: u8 = 0x01;
    pub const ACPI_DEVICE_PATH: u8 = 0x02;
    pub const MESSAGING_DEVICE_PATH: u8 = 0x03;
    pub const MEDIA_DEVICE_PATH: u8 = 0x04;
    pub const BIOS_BOOT_SPECIFICATION_DEVICE_PATH: u8 = 0x05;
    pub const END_OF_HARDWARE_DEVICE_PATH: u8 = 0x7F;
}
