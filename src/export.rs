use crate::ts::{ExportConfiguration, TsExportError};
use crate::*;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::sync::Mutex;

/// Global type store for collecting custom types to export.
///
/// Populated by `#[ctor]` functions defined in the [`Type`](derive@crate::Type) macro.
pub static TYPES: Lazy<Mutex<BTreeMap<&'static str, DataTypeExt>>> = Lazy::new(Default::default);

/// Exports all types in the [`TYPES`](static@crate::export::TYPES) map to the provided TypeScript file.
pub fn ts(path: &str) -> Result<(), TsExportError> {
    ts_with_cfg(&ExportConfiguration::default(), path)
}

/// Exports all types in the [`TYPES`](static@crate::export::TYPES) map to the provided TypeScript file but allow you to provide a configuration for the exporter.
pub fn ts_with_cfg(cfg: &ExportConfiguration, path: &str) -> Result<(), TsExportError> {
    let mut out = "// This file has been generated by Specta. DO NOT EDIT.\n\n".to_string();

    for typ in (*TYPES.lock().expect("Failed to acquire lock on 'TYPES'")).values() {
        if typ.export.unwrap_or(cfg.export_by_default.unwrap_or(true)) {
            out += &ts::export_datatype(cfg, typ)?;
            out += "\n\n";
        }
    }

    std::fs::write(path, out).map_err(Into::into)
}
