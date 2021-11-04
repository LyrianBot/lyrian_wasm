use lyrian::model::LyrianModel;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn make_model(contents: &str) -> JsValue {
  let result: String = match LyrianModel::from_str(contents) {
    Ok(model) => model.to_json_str().unwrap(),
    Err(msg) => msg,
  };
  JsValue::from_str(&result)
}
