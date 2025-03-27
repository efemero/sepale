use qrcode_generator::QrCodeEcc;
use base64::prelude::*;

// 
#[tauri::command]
fn get_qrcode(name: &str, bic: &str, iban: &str, amount: &str, reference: &str, text: &str) -> String {
    dbg!(&amount);
    let epc_data = format!(r#"BCD
002
1
SCT
{bic}
{name}
{iban}
EUR{amount}
{reference}
{text}
"#);
    
    let qr = qrcode_generator::to_svg_to_string(&epc_data, QrCodeEcc::High, 1024, None::<&str>).unwrap();
    let qr_b64 = BASE64_STANDARD.encode(qr);
    format!(r#"<img class="center" width="80%" src="data:image/svg+xml;base64,{qr_b64}"/>"#)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_qrcode])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

