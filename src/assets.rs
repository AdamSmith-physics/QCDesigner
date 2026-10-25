use anyhow::anyhow;
use gpui_kit::{AssetSource, Result, SharedString};
use gpui_kit::assets::Assets as ComponentAssets;
use rust_embed::RustEmbed;
use std::borrow::Cow;

// Embeds everything under src/bin/gpui-layout-design/ matching icons/**/*.svg.
// Each file is accessible at its path relative to that folder, e.g. "icons/share.svg".
#[derive(RustEmbed)]
#[folder = "src/"]
#[include = "assets/**/*.svg"]
struct LocalAssets;

// --- CompositeAssets ---
//
// Checks gpui-kit-assets (built-in icons) first, then falls back to
// LocalAssets (your custom SVGs). Pass this to Application::with_assets().

pub struct CompositeAssets;

impl AssetSource for CompositeAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
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
