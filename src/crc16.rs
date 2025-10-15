// ============================================================================
// src/crc16.rs
// ============================================================================
pub fn ccitt_16(data: &[u8]) -> u16 {
    const POLY: u16 = 0x1021;
    let mut crc: u16 = 0xFFFF;

    for &byte in data {
        for i in 0..8 {
            let lsb = (byte >> (7 - i)) & 1;
            let msb = (crc >> 15) & 1;
            crc <<= 1;
            if lsb ^ msb == 1 {
                crc ^= POLY;
            }
        }
    }

    crc & 0xFFFF
}

pub fn crc_modbus(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;

    for &byte in data {
        crc ^= byte as u16;
        for _ in 0..8 {
            if crc & 1 > 0 {
                crc >>= 1;
                crc ^= 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }

    crc
}
