use serde::Deserialize;
use serde_json::Value as JsonValue;
use serde_wasm_bindgen::from_value;
use typst::foundations::{Dict, FromValue, IntoValue, Value};
use typst_as_lib::TypstEngine;
use wasm_bindgen::prelude::*;

use typst_pdf::PdfOptions;

const IS_DEBUG: bool = cfg!(debug_assertions);

#[derive(Default, Deserialize)]
struct RenderOptions {
    template: String,
    font: Vec<u8>,
}

#[wasm_bindgen]
pub fn init_logging() {
    if IS_DEBUG {
        console_error_panic_hook::set_once();
    }

    let _ = console_log::init_with_level(log::Level::Info).map_err(log_error);
}

#[wasm_bindgen]
pub fn render_pdf(render_options: JsValue, data: JsValue) -> Result<Vec<u8>, JsValue> {
    let options = RenderOptions::try_from(render_options)?;

    let template = TypstEngine::builder()
        .main_file(options.template.as_str())
        .fonts([options.font])
        .build();

    let json_value = from_value(data).map_err(log_error)?;
    let input_data = json_to_typst(json_value);
    let dict = Dict::from_value(input_data).unwrap();

    log::debug!("Template built");

    let doc = template.compile_with_input(dict).output.map_err(log_error)?;

    log::debug!("Template compiled");

    let pdf = typst_pdf::pdf(&doc, &PdfOptions::default()).expect("Error exporting PDF");

    log::debug!("PDF compiled");

    Ok(pdf)
}

fn log_error<E>(error: E) -> String
where
    E: std::error::Error,
{
    let error_string = error.to_string();
    log::error!("{error_string}");
    error_string
}

fn json_to_typst(val: JsonValue) -> Value {
    match val {
        JsonValue::Null => Value::None,
        JsonValue::Bool(b) => b.into_value(),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                (i as i32).into_value()
            } else if let Some(f) = n.as_f64() {
                f.into_value()
            } else {
                Value::None
            }
        }
        JsonValue::String(s) => s.into_value(),
        JsonValue::Array(arr) => {
            arr.into_iter().map(json_to_typst).collect::<Vec<_>>().into_value()
        }
        JsonValue::Object(map) => {
            let dict = map.into_iter().map(|(k, v)| (k.into(), json_to_typst(v)));
            Dict::from_iter(dict).into_value()
        }
    }
}

impl TryFrom<JsValue> for RenderOptions {
    type Error = String;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        from_value(value).map_err(log_error)
    }
}
