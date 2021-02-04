use serde::{Deserialize, Serialize};
use serde_json;
use web_view::*;

fn main() {
    let html = format!(
        r#"
		<!doctype html>
		<html>
			<head>
				{styles}
			</head>
			<body>
		        <div id="root"></div>
				{scripts}
			</body>
		</html>
		"#,
        styles = inline_style(include_str!("../view/public/global.css"))
            + &inline_style(include_str!("../view/public/build/bundle.css")),
        scripts = inline_script(include_str!("../view/public/build/bundle.js"))
    );

    builder()
        .title("Rust WebView Demo")
        .content(Content::Html(html))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            use Cmd::*;
            match serde_json::from_str(arg).unwrap() {
                Init => println!("Initialize app."),
                Log { text } => println!("{}", text),
            }
            Ok(())
        })
        .run()
        .unwrap();
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Init,
    Log { text: String },
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
