use anyhow::anyhow;
use gpui::{AssetSource, Result, SharedString};
use gpui_component_assets::Assets as ComponentAssets;
use rust_embed::RustEmbed;
use std::borrow::Cow;

use crate::utils::SvgStore;

// Embeds everything under src/bin/gpui-layout-design/ matching icons/**/*.svg.
// Each file is accessible at its path relative to that folder, e.g. "icons/share.svg".
#[derive(RustEmbed)]
#[folder = "src/"]
#[include = "assets/**/*.svg"]
struct LocalAssets;

// --- CompositeAssets ---
//
// Checks the runtime `SvgStore` first (LaTeX-rendered SVGs generated at
// runtime by Typst), then gpui-component-assets (built-in icons), then falls
// back to LocalAssets (your custom SVGs). Pass this to Application::with_assets().

pub struct CompositeAssets {
    svg_store: SvgStore,
}

impl CompositeAssets {
    pub fn new(svg_store: SvgStore) -> Self {
        Self { svg_store }
    }
}

impl AssetSource for CompositeAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if let Some(data) = self.svg_store.load(path)? {
            return Ok(Some(data));
        }
        if let Ok(Some(data)) = ComponentAssets.load(path) {
            return Ok(Some(data));
        }
        LocalAssets::get(path)
            .map(|f| Some(f.data))
            .ok_or_else(|| anyhow!("asset not found: \"{path}\""))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut results = ComponentAssets.list(path)?;
        results.extend(
            LocalAssets::iter()
                .filter_map(|p| p.starts_with(path).then(|| p.into())),
        );
        Ok(results)
    }
}
