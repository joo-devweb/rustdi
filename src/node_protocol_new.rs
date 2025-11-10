use crate::errors::*;
use std::collections::HashMap;

// Node protokol WhatsApp
#[derive(Debug, Clone)]
pub struct Node {
    pub tag: String,
    pub attrs: HashMap<String, String>,
    pub content: Option<NodeContent>,
}

// Jenis konten dalam node
#[derive(Debug, Clone)]
pub enum NodeContent {
    Text(String),
    Binary(Vec<u8>),
    List(Vec<Node>),
}

// Encoder untuk node protokol
pub struct NodeEncoder {
    pub data: Vec<u8>,
}

// Decoder untuk node protokol
pub struct NodeDecoder {
    pub data: Vec<u8>,
    pub index: usize,
}

// Konstanta token
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

impl NodeEncoder {
    pub fn new() -> Self {
        NodeEncoder {
            data: Vec::new(),
        }
    }

    pub fn write_node(&mut self, node: &Node) -> Result<()> {
        let num_attributes = node.attrs.len();
        let has_content = node.content.is_some();

        let total_children = 2 * num_attributes + 1 + if has_content { 1 } else { 0 };
        self.write_list_start(total_children)?;
        self.write_string(&node.tag, false)?;

        // Tulis atribut
        for (key, value) in &node.attrs {
            self.write_string(key, false)?;
            self.write_string(value, false)?;
        }

        // Tulis konten
        if let Some(content) = &node.content {
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
        if !i && s == "c.us" {
            self.write_token(104)?; // token s.whatsapp.net
            return Ok(());
        }

        if let Some(token_index) = SINGLE_BYTE_TOKENS.iter().position(|&t| t == s) {
            if token_index < SINGLE_BYTE_MAX as usize {
                self.write_token(token_index as u8)?;
            } else {
                let overflow = token_index - SINGLE_BYTE_MAX as usize;
                let dictionary_index = (overflow >> 8) as u8;
                if dictionary_index < 4 {
                    self.write_token(DICTIONARY_0 + dictionary_index)?;
                    self.write_token((overflow % 256) as u8)?;
                } else {
                    return Err("Double byte dictionary token out of range".into());
                }
            }
        } else {
            // Penanganan JID
            if let Some(jid_pos) = s.find('@') {
                let (left, right) = s.split_at(jid_pos);
                let right = &right[1..]; // Lewati '@'
                self.write_jid(left, right)?;
            } else {
                self.write_string_raw(s)?;
            }
        }

        Ok(())
    }

    fn write_token(&mut self, token: u8) -> Result<()> {
        if token < SINGLE_BYTE_MAX {
            self.data.push(token);
        } else if token <= 500 {
            return Err("Invalid token".into());
        }
        Ok(())
    }

    fn write_jid(&mut self, left: &str, right: &str) -> Result<()> {
        self.data.push(JID_PAIR);
        if !left.is_empty() {
            self.write_string(left, false)?;
        } else {
            self.write_token(LIST_EMPTY)?;
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
            },
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
        let list_size_tag = self.read_byte()?;
        let list_size = self.read_list_size(list_size_tag)?;
        
        if list_size == 0 {
            return Err("Invalid node".into());
        }

        let tag = self.read_byte()?;
        let description = self.read_string(tag)?;

        let num_attrs = (list_size - 1) >> 1;
        let mut attrs = HashMap::new();
        
        for _ in 0..num_attrs {
            let key_tag = self.read_byte()?;
            let key = self.read_string(key_tag)?;
            let value_tag = self.read_byte()?;
            let value = self.read_string(value_tag)?;
            attrs.insert(key, value);
        }

        let has_content = list_size % 2 == 0;
        let content = if has_content {
            let tag = self.read_byte()?;
            Some(self.read_content(tag)?)
        } else {
            None
        };

        Ok(Node {
            tag: description,
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
            LIST_16 => Ok(u16::from_be_bytes([self.read_byte()?, self.read_byte()?]) as usize),
            _ => Err("Invalid list tag".into()),
        }
    }

    fn read_string(&mut self, tag: u8) -> Result<String> {
        if tag >= 3 && tag <= SINGLE_BYTE_MAX {
            let token = SINGLE_BYTE_TOKENS.get(tag as usize - 3)
                .ok_or("Invalid token index")?;
            if token == &"s.whatsapp.net" {
                Ok("c.us".to_string())
            } else {
                Ok(token.to_string())
            }
        } else {
            match tag {
                DICTIONARY_0..=DICTIONARY_3 => {
                    let index1 = tag - DICTIONARY_0;
                    let index2 = self.read_byte()?;
                    let n = (index1 as u16) * 256 + (index2 as u16);
                    Err("Double token not implemented".into())
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
                    let length = u32::from_be_bytes([self.read_byte()?, self.read_byte()?, self.read_byte()?, self.read_byte()?]);
                    self.read_string_from_chars(length as usize)
                },
                JID_PAIR => {
                    let left_tag = self.read_byte()?;
                    let left = self.read_string(left_tag)?;
                    let right_tag = self.read_byte()?;
                    let right = self.read_string(right_tag)?;
                    if !left.is_empty() && !right.is_empty() {
                        Ok(format!("{}@{}", left, right))
                    } else {
                        Err("Invalid jid pair".into())
                    }
                },
                NIBBLE_8 | HEX_8 => self.read_packed8(tag),
                _ => Err("Invalid string tag".into()),
            }
        }
    }

    fn read_string_from_chars(&mut self, length: usize) -> Result<String> {
        if self.index + length > self.data.len() {
            return Err("End of stream".into());
        }
        let bytes = &self.data[self.index..self.index + length];
        self.index += length;
        Ok(String::from_utf8(bytes.to_vec())?)
    }

    fn read_int20(&mut self) -> Result<u32> {
        if self.index + 3 > self.data.len() {
            return Err("End of stream".into());
        }
        let value = ((self.data[self.index] as u32 & 15) << 16) | 
                    ((self.data[self.index + 1] as u32) << 8) | 
                    (self.data[self.index + 2] as u32);
        self.index += 3;
        Ok(value)
    }

    fn read_packed8(&mut self, _tag: u8) -> Result<String> {
        // Implementasi pembacaan packed bytes
        Err("Packed bytes reading not implemented".into())
    }

    fn read_content(&mut self, tag: u8) -> Result<NodeContent> {
        if self.is_list_tag(tag) {
            let list = self.read_list(tag)?;
            Ok(NodeContent::List(list))
        } else {
            match tag {
                BINARY_8 => {
                    let length = self.read_byte()?;
                    let bytes = &self.data[self.index..self.index + length as usize];
                    self.index += length as usize;
                    Ok(NodeContent::Binary(bytes.to_vec()))
                },
                BINARY_20 => {
                    let length = self.read_int20()?;
                    let bytes = &self.data[self.index..self.index + length as usize];
                    self.index += length as usize;
                    Ok(NodeContent::Binary(bytes.to_vec()))
                },
                BINARY_32 => {
                    let length = u32::from_be_bytes([self.read_byte()?, self.read_byte()?, self.read_byte()?, self.read_byte()?]);
                    let bytes = &self.data[self.index..self.index + length as usize];
                    self.index += length as usize;
                    Ok(NodeContent::Binary(bytes.to_vec()))
                },
                _ => {
                    let s = self.read_string(tag)?;
                    Ok(NodeContent::Text(s))
                }
            }
        }
    }

    fn is_list_tag(&mut self, tag: u8) -> bool {
        tag == LIST_EMPTY || tag == LIST_8 || tag == LIST_16
    }

    fn read_list(&mut self, tag: u8) -> Result<Vec<Node>> {
        let size = self.read_list_size(tag)?;
        let mut list = Vec::with_capacity(size);
        for _ in 0..size {
            list.push(self.read_node()?);
        }
        Ok(list)
    }
}