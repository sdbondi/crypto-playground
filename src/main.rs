extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate web_view;

use std::str;
use web_view::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserData {
    name: String,
    counter: u32,
}

const HTML_TEMPLATE: &'static str = include_str!("../ui-dist/index.html");

fn invoke_handler(webview: &mut WebView<UserData>, arg: &str) -> WVResult {
    println!("INVOKED");
    match serde_json::from_str(arg).unwrap() {
        Cmd::Init => rpc_call(
            webview,
            "run",
            serde_json::to_string(webview.user_data()).unwrap(),
        ),
        Cmd::Increment { count } => {
            println!("{}", count);
            rpc_call(webview, "inc", format!(r#"{{ "by": {} }}"#, count))
        }
        Cmd::Decrement { count } => {
            println!("{}", count);
            rpc_call(webview, "dec", format!(r#"{{ "by": {} }}"#, count))
        }
        Cmd::Exit => {
            webview.terminate();
            Ok(())
        }
    }
}

fn main() {
    let html = HTML_TEMPLATE.replace("{css}", r#"body { background: #ababab; }"#);
    let html = html.replace("{js}", include_str!("../ui-dist/main.js"));

    web_view::builder()
        .title("Crypto playground")
        .content(Content::Html(html))
        .size(320, 480)
        .resizable(false)
        .debug(true)
        .user_data(UserData {
            name: "Chimp".to_string(),
            counter: 1,
        })
        .invoke_handler(invoke_handler)
        .run()
        .unwrap();
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Init,
    Increment { count: u32 },
    Decrement { count: u32 },
    Exit,
}

fn rpc_call(webview: &mut WebView<UserData>, func: &str, payload: String) -> WVResult {
    webview.eval(&format!("rpc.{}({})", func, payload))
}

// macro_rules! webview_rpc {
//     ($wv: expr, fn: expr (, $args: expr)*) => (
//         webview.eval(format!(
//             "rpc.{}({})",
//         ))
//     )
// }
