use image::ImageReader;
use sha2::{Digest, Sha512};
use std::path::Path;

use crate::utils::files::file_extension_lowercase;

const RAW_EXTENSIONS: &[&str] = &["nef"];
const JPEG_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png"];

pub fn get_content_hash(path: &Path) -> String {
    let extension = file_extension_lowercase(path);
    let content = match extension {
        Some(ext) if RAW_EXTENSIONS.contains(&ext.as_str()) => get_raw_content(path),
        Some(ext) if JPEG_EXTENSIONS.contains(&ext.as_str()) => get_jpeg_content(path),
        _ => get_file_content(path),
    };
    let hash = hash_bytes(&content);
    hash
}

fn get_file_content(path: &Path) -> Vec<u8> {
    let content = std::fs::read(path).unwrap();
    content
}

fn hash_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha512::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn get_raw_content(path: &Path) -> Vec<u8> {
    let raw = rawloader::decode_file(path).unwrap();
    let raw_bytes = match raw.data {
        rawloader::RawImageData::Integer(data) => convert_integer_to_bytes(&data),
        rawloader::RawImageData::Float(data) => convert_float_to_bytes(&data),
    };
    raw_bytes
}

fn get_jpeg_content(path: &Path) -> Vec<u8> {
    match ImageReader::open(path) {
        Ok(reader) => match reader.decode() {
            Ok(img) => img.as_bytes().to_vec(),
            Err(_) => Vec::new(),
        },
        Err(_) => Vec::new(),
    }
}

fn convert_integer_to_bytes(raw_data: &[u16]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for &value in raw_data {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    bytes
}
fn convert_float_to_bytes(raw_data: &[f32]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for &value in raw_data {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    bytes
}
