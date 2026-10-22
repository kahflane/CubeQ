//! Locale RPC methods: serve the in-memory locale registry.

use cubeq_api::jsonrpc::error_codes::*;
use cubeq_api::jsonrpc::{JsonRpcError, LocaleOption, LocalesListResponse};

fn rpc_err(code: i32, msg: impl Into<String>) -> JsonRpcError {
    JsonRpcError {
        code,
        message: msg.into(),
        data: None,
    }
}

/// Attribution span keyed to the calling TUI session.
fn locale_span(tui_id: Option<&str>) -> ::cubeq_log::Span {
    ::cubeq_log::info_span!(
        target: "cubeq_log_internal_scope",
        "cubeq_scope",
        session_key = %tui_id.unwrap_or("rpc"),
        channel = "rpc",
    )
}

/// Handle `locales/list` — the embedded locale registry. No network.
pub fn handle_locales_list(tui_id: Option<&str>) -> Result<serde_json::Value, JsonRpcError> {
    let span = locale_span(tui_id);
    let _guard = span.enter();
    let locales: Vec<LocaleOption> = crate::i18n::available_locales()
        .iter()
        .map(|o| LocaleOption {
            code: o.code.clone(),
            label: o.label.clone(),
        })
        .collect();
    ::cubeq_log::record!(
        DEBUG,
        ::cubeq_log::Event::new(module_path!(), ::cubeq_log::Action::Note)
            .with_attrs(::serde_json::json!({ "count": locales.len() })),
        "locales/list served from embedded registry"
    );
    serde_json::to_value(LocalesListResponse { locales })
        .map_err(|e| rpc_err(INTERNAL_ERROR, e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_locales_list_returns_embedded_registry() {
        let v = handle_locales_list(Some("tui-1")).unwrap();
        let locales = v.get("locales").and_then(|x| x.as_array()).unwrap();
        assert!(!locales.is_empty(), "registry should never be empty");
        let first = &locales[0];
        assert!(first.get("code").and_then(|c| c.as_str()).is_some());
        assert!(
            first.get("label").and_then(|c| c.as_str()).is_some(),
            "label missing from entry: {first}"
        );
    }
}
