// ============================================================================
// src/pdu.rs
// ============================================================================
use crate::crc16::ccitt_16;

const SSN_START: &str = "===ssn1";

#[derive(Debug, Clone)]
pub struct SsnPdu {
    pub dest_obj: u16,
    pub src_obj: u16,
    pub msg_type: u8,
    pub msg_id: Option<u16>,
    pub msg_data: Vec<u8>,
    pub timestamp: i64,
}

impl SsnPdu {
    pub fn new(dest_obj: u16, src_obj: u16, msg_type: u8, msg_data: Vec<u8>) -> Self {
        Self {
            dest_obj,
            src_obj,
            msg_type,
            msg_id: None,
            msg_data,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn get_ssn_pdu(&self) -> String {
        let data_str = String::from_utf8_lossy(&self.msg_data);
        let crc = ccitt_16(data_str.as_bytes());
        
        format!(
            "{}{:04x}{:04x}{:02x}{:04x}{}{:04x}",
            SSN_START,
            self.dest_obj,
            self.src_obj,
            self.msg_type,
            data_str.len(),
            data_str,
            crc
        )
    }

    pub fn process_buffer(buffer: &str) -> Option<(Self, String)> {
        if let Some(start_pos) = buffer.find(SSN_START) {
            let current_pos = start_pos + SSN_START.len();
            
            if buffer.len() - current_pos < 14 {
                return None;
            }

            let header = &buffer[current_pos..current_pos + 14];
            let dest_obj = u16::from_str_radix(&header[0..4], 16).ok()?;
            let src_obj = u16::from_str_radix(&header[4..8], 16).ok()?;
            let msg_type = u8::from_str_radix(&header[8..10], 16).ok()?;
            let packet_len = usize::from_str_radix(&header[10..14], 16).ok()?;

            let data_pos = current_pos + 14;
            if data_pos + packet_len + 4 > buffer.len() {
                return None;
            }

            let pdu_data = &buffer[data_pos..data_pos + packet_len];
            let pdu_crc = u16::from_str_radix(&buffer[data_pos + packet_len..data_pos + packet_len + 4], 16).ok()?;
            let calc_crc = ccitt_16(pdu_data.as_bytes());

            if calc_crc == pdu_crc {
                let pdu = SsnPdu {
                    dest_obj,
                    src_obj,
                    msg_type,
                    msg_id: None,
                    msg_data: pdu_data.as_bytes().to_vec(),
                    timestamp: chrono::Utc::now().timestamp(),
                };
                let tail = buffer[data_pos + packet_len + 4..].to_string();
                Some((pdu, tail))
            } else {
                log::warn!("CRC Error! dest[{}] src[{}]", dest_obj, src_obj);
                None
            }
        } else {
            None
        }
    }
}
