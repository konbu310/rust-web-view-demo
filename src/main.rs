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
        styles = inline_style(include_str!("js-src/static/style.css")),
        scripts = inline_script(include_str!("js-src/dist/index.js"))
    );

    builder()
        .title("Rust WebView Demo")
        .content(Content::Html(html))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
