use eframe::egui;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
include!(concat!(env!("OUT_DIR"), "/assets.rs"));
const REGISTRY: &str = include_str!("../../../data/brand-icons.json");

pub struct Brands {
    registry: Value,
    textures: HashMap<String, egui::TextureHandle>,
}
impl Default for Brands {
    fn default() -> Self {
        Self {
            registry: serde_json::from_str(REGISTRY).unwrap(),
            textures: HashMap::new(),
        }
    }
}
impl Brands {
    fn brand(&self, name: &str) -> Option<&Value> {
        self.registry["brands"].as_array()?.iter().find(|b| {
            b["name"]
                .as_str()
                .is_some_and(|s| s.eq_ignore_ascii_case(name))
                || b["names"].as_array().is_some_and(|names| {
                    names
                        .iter()
                        .any(|s| s.as_str().is_some_and(|s| s.eq_ignore_ascii_case(name)))
                })
        })
    }
    pub fn url(&self, name: &str) -> Option<String> {
        self.brand(name)
            .and_then(|b| b["downloadPage"].as_str())
            .map(str::to_string)
    }
    pub fn texture(&mut self, context: &egui::Context, name: &str) -> Option<egui::TextureHandle> {
        let id = self.brand(name)?["assetId"].as_str()?.to_owned();
        if let Some(texture) = self.textures.get(&id) {
            return Some(texture.clone());
        }
        let (_, bytes, _) = ASSETS.iter().find(|(asset_id, _, _)| *asset_id == id)?;
        let image = image::load_from_memory(bytes).ok()?.to_rgba8();
        let size = [image.width() as usize, image.height() as usize];
        let texture = context.load_texture(
            id.clone(),
            egui::ColorImage::from_rgba_unmultiplied(size, &image),
            egui::TextureOptions::LINEAR,
        );
        self.textures.insert(id, texture.clone());
        Some(texture)
    }
}
pub fn self_test() -> Result<usize, String> {
    for (id, bytes, checksum) in ASSETS {
        if !bytes.starts_with(b"\x89PNG\r\n\x1a\n")
            || format!("{:x}", Sha256::digest(bytes)) != *checksum
        {
            return Err(format!("Invalid bundled brand image {id}"));
        }
    }
    let brands = Brands::default();
    for name in [
        "INTA",
        "INTA Rescue",
        "GSmartControl",
        "GParted",
        "TestDisk",
        "PhotoRec",
        "Clonezilla",
        "ClamAV",
        "ddrescue",
        "YARA",
        "Sleuth Kit",
        "dislocker",
        "tcpdump",
        "ChatGPT",
        "Firefox",
        "Codex CLI",
        "Claude Code",
        "Ventoy",
        "Rufus",
    ] {
        if brands.url(name).is_none() {
            return Err(format!("Missing native brand identity: {name}"));
        }
    }
    Ok(ASSETS.len())
}
