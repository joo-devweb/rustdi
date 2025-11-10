use crate::errors::*;
use std::collections::HashMap;

pub const LIST_EMPTY: u8 = 0;
pub const STREAM_END: u8 = 2;
pub const DICTIONARY_0: u8 = 236;
pub const DICTIONARY_1: u8 = 237;
pub const DICTIONARY_2: u8 = 238;
pub const DICTIONARY_3: u8 = 239;
pub const LIST_8: u8 = 248;
pub const LIST_16: u8 = 249;
pub const JID_PAIR: u8 = 250;
pub const HEX_8: u8 = 251;
pub const BINARY_8: u8 = 252;
pub const BINARY_20: u8 = 253;
pub const BINARY_32: u8 = 254;
pub const NIBBLE_8: u8 = 255;

pub const SINGLE_BYTE_MAX: u8 = 256;
pub const PACKED_MAX: u8 = 254;

pub const SINGLE_BYTE_TOKENS: &[&str] = &[
    "", "", "", "200", "400", "404", "500", "501", "502", "action", "add",
    "after", "archive", "author", "available", "battery", "before", "body",
    "broadcast", "chat", "clear", "code", "composing", "contacts", "count",
    "create", "debug", "delete", "demote", "duplicate", "encoding", "error",
    "false", "filehash", "from", "g.us", "group", "groups_v2", "height", "id",
    "image", "in", "index", "invis", "item", "jid", "kind", "last", "leave",
    "live", "log", "media", "message", "mimetype", "missing", "modify", "name",
    "notification", "notify", "out", "owner", "participant", "paused",
    "picture", "played", "presence", "preview", "promote", "query", "raw",
    "read", "receipt", "received", "recipient", "recording", "relay",
    "remove", "response", "resume", "retry", "s.whatsapp.net", "seconds",
    "set", "size", "status", "subject", "subscribe", "t", "text", "to", "true",
    "type", "unarchive", "unavailable", "url", "user", "value", "web", "width",
    "mute", "read_only", "admin", "creator", "short", "update", "powersave",
    "checksum", "epoch", "block", "previous", "409", "replaced", "reason",
    "spam", "modify_tag", "message_info", "delivery", "emoji", "title",
    "description", "canonical-url", "matched-text", "star", "unstar",
    "media_key", "filename", "identity", "unread", "page", "page_count",
    "search", "media_message", "security", "call_log", "profile", "ciphertext",
    "invite", "gif", "vcard", "frequent", "privacy", "blacklist", "whitelist",
    "verify", "location", "document", "elapsed", "revoke_invite", "expiration",
    "unsubscribe", "disable", "vname", "old_jid", "new_jid", "announcement",
    "locked", "prop", "label", "color", "call", "offer", "call-id",
    "quick_reply", "sticker", "pay_t", "accept", "reject", "sticker_pack",
    "invalid", "canceled", "missed", "connected", "result", "audio",
    "video", "recent"
];

#[derive(Debug, Clone)]
pub struct Node {
    pub tag: String,
    pub attrs: HashMap<String, String>,
    pub content: Option<NodeContent>,
}

#[derive(Debug, Clone)]
pub enum NodeContent {
    Text(String),
    Binary(Vec<u8>),
    List(Vec<Node>),
}

pub struct NodeEncoder {
    pub data: Vec<u8>,
}

pub struct NodeDecoder {
    pub data: Vec<u8>,
    pub index: usize,
}

impl NodeEncoder {
    pub fn new() -> Self {
        NodeEncoder { data: Vec::new() }
    }

    pub fn write_node(&mut self, node: &Node) -> Result<()> {
        // Jumlah atribut = jumlah pasangan (key, value)
        let num_attributes = node.attrs.len();
        let has_content = node.content.is_some();

        // Jumlah anak = 1 (tag) + 2 * jumlah atribut + 1 (jika ada konten)
        let total_children = 1 + 2 * num_attributes + if has_content { 1 } else { 0 };

        // Tulis ukuran list
        self.write_list_start(total_children)?;
        // Tulis tag
        self.write_string(&node.tag, false)?;

        // Tulis pasangan atribut (key, value)
        for (key, value) in &node.attrs {
            self.write_string(key, false)?;
            self.write_string(value, false)?;
        }

        // Tulis konten jika ada
        if let Some(ref content) = node.content {
            self.write_content(content)?;
        }

        Ok(())
    }

    fn write_list_start(&mut self, size: usize) -> Result<()> {
        if size == 0 {
            self.data.push(LIST_EMPTY);
        } else if size < 256 {
            self.data.push(LIST_8);
            self.data.push(size as u8);
        } else {
            self.data.push(LIST_16);
            self.data.extend_from_slice(&(size as u16).to_be_bytes());
        }
        Ok(())
    }

    fn write_string(&mut self, s: &str, i: bool) -> Result<()> {
        if !i && s == "s.whatsapp.net" {
            // Ganti s.whatsapp.net menjadi c.us
            self.write_token(SINGLE_BYTE_TOKENS.iter().position(|&t| t == "s.whatsapp.net").unwrap() as u8)?;
        } else if let Some(token_index) = SINGLE_BYTE_TOKENS.iter().position(|&t| t == s) {
            if token_index < SINGLE_BYTE_MAX as usize {
                self.write_token(token_index as u8)?;
            } else {
                // Gunakan kamus ganda-byte
                let overflow = token_index - SINGLE_BYTE_MAX as usize;
                let dictionary_index = (overflow >> 8) as u8;
                if dictionary_index < 4 {
                    self.write_token(DICTIONARY_0 + dictionary_index)?;
                    self.write_token((overflow % 256) as u8)?;
                } else {
                    return Err("Dictionary token out of range".into());
                }
            }
        } else {
            // Periksa apakah ini JID
            if let Some(pos) = s.find('@') {
                let (left, right) = s.split_at(pos);
                let right = &right[1..]; // Hilangkan '@'
                self.write_jid(left, right)?;
            } else {
                self.write_string_raw(s)?;
            }
        }
        Ok(())
    }

    fn write_token(&mut self, token: u8) -> Result<()> {
        if token < SINGLE_BYTE_MAX as u8 {
            self.data.push(token);
        } else {
            return Err("Invalid token".into());
        }
        Ok(())
    }

    fn write_jid(&mut self, left: &str, right: &str) -> Result<()> {
        self.data.push(JID_PAIR);
        if left.is_empty() {
            self.write_token(LIST_EMPTY)?;
        } else {
            self.write_string(left, false)?;
        }
        self.write_string(right, false)?;
        Ok(())
    }

    fn write_string_raw(&mut self, s: &str) -> Result<()> {
        let bytes = s.as_bytes();
        self.write_byte_length(bytes.len())?;
        self.data.extend_from_slice(bytes);
        Ok(())
    }

    fn write_byte_length(&mut self, length: usize) -> Result<()> {
        if length >= 4294967296 {
            return Err("String too large to encode".into());
        }

        if length >= (1 << 20) {
            self.data.push(BINARY_32);
            self.data.extend_from_slice(&(length as u32).to_be_bytes());
        } else if length >= 256 {
            self.data.push(BINARY_20);
            self.write_int20(length as u32)?;
        } else {
            self.data.push(BINARY_8);
            self.data.push(length as u8);
        }
        Ok(())
    }

    fn write_int20(&mut self, value: u32) -> Result<()> {
        self.data.push(((value >> 16) & 0x0F) as u8);
        self.data.push(((value >> 8) & 0xFF) as u8);
        self.data.push((value & 0xFF) as u8);
        Ok(())
    }

    fn write_content(&mut self, content: &NodeContent) -> Result<()> {
        match content {
            NodeContent::Text(s) => self.write_string(s, true),
            NodeContent::Binary(bytes) => {
                self.write_byte_length(bytes.len())?;
                self.data.extend_from_slice(bytes);
                Ok(())
            }
            NodeContent::List(nodes) => {
                self.write_list_start(nodes.len())?;
                for node in nodes {
                    self.write_node(node)?;
                }
                Ok(())
            }
        }
    }
}

impl NodeDecoder {
    pub fn new(data: &[u8]) -> Self {
        NodeDecoder {
            data: data.to_vec(),
            index: 0,
        }
    }

    pub fn read_node(&mut self) -> Result<Node> {
        // Baca ukuran list
        let list_size_tag = self.read_byte()?;
        let list_size = self.read_list_size(list_size_tag)?;
        
        if list_size == 0 {
            return Err("Invalid node".into());
        }

        // Baca tag
        let tag_token = self.read_byte()?;
        let tag = self.read_string(tag_token)?;

        // Hitung jumlah atribut
        let num_attrs = (list_size - 1) >> 1;
        let mut attrs = HashMap::new();

        // Baca pasangan (key, value) atribut
        for _ in 0..num_attrs {
            let key_token = self.read_byte()?;
            let key = self.read_string(key_token)?;
            let value_token = self.read_byte()?;
            let value = self.read_string(value_token)?;
            attrs.insert(key, value);
        }

        // Cek apakah ada konten
        let has_content = list_size % 2 == 0;
        let content = if has_content {
            let content_token = self.read_byte()?;
            Some(self.read_content(content_token)?)
        } else {
            None
        };

        Ok(Node {
            tag,
            attrs,
            content,
        })
    }

    fn read_byte(&mut self) -> Result<u8> {
        if self.index >= self.data.len() {
            return Err("End of stream".into());
        }
        let byte = self.data[self.index];
        self.index += 1;
        Ok(byte)
    }

    fn read_list_size(&mut self, tag: u8) -> Result<usize> {
        match tag {
            LIST_EMPTY => Ok(0),
            LIST_8 => Ok(self.read_byte()? as usize),
            LIST_16 => {
                let high = self.read_byte()? as usize;
                let low = self.read_byte()? as usize;
                Ok((high << 8) | low)
            }
            _ => Err("Invalid list tag".into()),
        }
    }

    fn read_string(&mut self, tag: u8) -> Result<String> {
        if tag >= 3 && tag < SINGLE_BYTE_MAX as u8 {
            let token_idx = (tag - 3) as usize;
            if token_idx < SINGLE_BYTE_TOKENS.len() {
                let token = SINGLE_BYTE_TOKENS[token_idx];
                if token == "s.whatsapp.net" {
                    Ok("c.us".to_string()) // Ganti kembali ke c.us
                } else {
                    Ok(token.to_string())
                }
            } else {
                Err("Invalid token index".into())
            }
        } else {
            match tag {
                DICTIONARY_0..=DICTIONARY_3 => {
                    let dict_index = tag - DICTIONARY_0;
                    let next_byte = self.read_byte()?;
                    let token_idx = (dict_index as usize) * 256 + next_byte as usize;
                    if token_idx < SINGLE_BYTE_TOKENS.len() {
                        Ok(SINGLE_BYTE_TOKENS[token_idx].to_string())
                    } else {
                        Err("Invalid dictionary token".into())
                    }
                },
                LIST_EMPTY => Ok(String::new()),
                BINARY_8 => {
                    let length = self.read_byte()?;
                    self.read_string_from_chars(length as usize)
                },
                BINARY_20 => {
                    let length = self.read_int20()?;
                    self.read_string_from_chars(length as usize)
                },
                BINARY_32 => {
                    let length = u32::from_be_bytes([
                        self.read_byte()?, 
                        self.read_byte()?, 
                        self.read_byte()?, 
                        self.read_byte()?
                    ]);
                    self.read_string_from_chars(length as usize)
                },
                JID_PAIR => {
                    let left_token = self.read_byte()?;
                    let left = self.read_string(left_token)?;
                    let right_token = self.read_byte()?;
                    let right = self.read_string(right_token)?;
                    if left.is_empty() && right.is_empty() {
                        Err("Invalid JID pair".into())
                    } else {
                        Ok(format!("{}@{}", left, right))
                    }
                },
                HEX_8 | NIBBLE_8 => self.read_packed_string(tag),
                _ => Err("Invalid string tag".into()),
            }
        }
    }

    fn read_string_from_chars(&mut self, length: usize) -> Result<String> {
        if self.index + length > self.data.len() {
            return Err("End of stream".into());
        }
        let string_bytes = &self.data[self.index..self.index + length];
        self.index += length;
        Ok(String::from_utf8(string_bytes.to_vec()).map_err(|_| "Invalid UTF8")?)
    }

    fn read_int20(&mut self) -> Result<u32> {
        if self.index + 3 > self.data.len() {
            return Err("End of stream".into());
        }
        let value = ((self.data[self.index] as u32 & 0x0F) << 16) |
                    ((self.data[self.index + 1] as u32) << 8) |
                    (self.data[self.index + 2] as u32);
        self.index += 3;
        Ok(value)
    }

    fn read_packed_string(&mut self, tag: u8) -> Result<String> {
        // Baca panjang paket
        let length_byte = self.read_byte()?;
        let length = (length_byte & 0x7F) as usize; // Hapus MSB

        let mut result = String::new();
        let mut is_odd_length = (length_byte & 0x80) != 0; // LSB menunjukkan ganjil

        for i in 0..length {
            if self.index >= self.data.len() {
                break; // Perlindungan dari buffer overrun
            }
            let byte = self.data[self.index];
            self.index += 1;

            // Ambil nibble atas dan bawah
            let high_nibble = (byte >> 4) & 0x0F;
            let low_nibble = byte & 0x0F;

            // Interpretasikan nibble sebagai karakter
            result.push(self.unpack_nibble(high_nibble, tag)?);
            
            if i + 1 < length || !is_odd_length {
                result.push(self.unpack_nibble(low_nibble, tag)?);
            }
        }

        Ok(result)
    }

    fn unpack_nibble(&self, nibble: u8, tag: u8) -> Result<char> {
        match tag {
            NIBBLE_8 => {
                match nibble {
                    0..=9 => Ok(std::char::from_digit(nibble as u32, 10).unwrap()),
                    10 => Ok('-'),
                    11 => Ok('.'),
                    15 => Ok('\0'), // EOF
                    _ => Err("Invalid nibble".into()),
                }
            },
            HEX_8 => {
                match nibble {
                    0..=9 => Ok(std::char::from_digit(nibble as u32, 10).unwrap()),
                    10..=15 => Ok(std::char::from_digit(nibble as u32, 16).unwrap().to_ascii_uppercase()),
                    _ => Err("Invalid hex nibble".into()),
                }
            },
            _ => Err("Invalid packed string tag".into()),
        }
    }

    fn read_content(&mut self, tag: u8) -> Result<NodeContent> {
        if self.is_list_tag(tag) {
            let nodes = self.read_list_nodes(tag)?;
            Ok(NodeContent::List(nodes))
        } else {
            match tag {
                BINARY_8 => {
                    let length = self.read_byte()?;
                    self.read_binary_content(length as usize)
                },
                BINARY_20 => {
                    let length = self.read_int20()?;
                    self.read_binary_content(length as usize)
                },
                BINARY_32 => {
                    let length = u32::from_be_bytes([
                        self.read_byte()?, 
                        self.read_byte()?, 
                        self.read_byte()?, 
                        self.read_byte()?
                    ]);
                    self.read_binary_content(length as usize)
                },
                _ => {
                    let string_content = self.read_string(tag)?;
                    Ok(NodeContent::Text(string_content))
                }
            }
        }
    }

    fn is_list_tag(&self, tag: u8) -> bool {
        tag == LIST_EMPTY || tag == LIST_8 || tag == LIST_16
    }

    fn read_list_nodes(&mut self, tag: u8) -> Result<Vec<Node>> {
        let size = self.read_list_size(tag)?;
        let mut nodes = Vec::with_capacity(size);
        
        for _ in 0..size {
            nodes.push(self.read_node()?);
        }
        
        Ok(nodes)
    }

    fn read_binary_content(&mut self, length: usize) -> Result<NodeContent> {
        if self.index + length > self.data.len() {
            return Err("End of stream".into());
        }
        
        let content = self.data[self.index..self.index + length].to_vec();
        self.index += length;
        Ok(NodeContent::Binary(content))
    }
}