use wasm_bindgen::prelude::*;
use js_sys::JsString;
use reqwest;

#[wasm_bindgen]
pub async fn make_get_request(url: &str) -> Result<String, JsValue> {
    match make_get_request_internal(url).await {
        Ok(body) => Ok(body),
        Err(err) => {
            let err_msg = format!("Request failed: {}", err);
            Err(JsValue::from(JsString::from(err_msg)))
        }
    }
}

async fn make_get_request_internal(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}
