use std::process::Command;
use std::path::PathBuf;

use gtk::gio;

use webkit2gtk::{WebView, ScriptDialog};
use webkit2gtk::traits::WebViewExt;

use dbus::arg;
use dbus::arg::{RefArg, Variant};

use crate::util::refarg_to_string;

pub fn handler(webview: &WebView, dialog: &ScriptDialog) -> bool {
    let message = dialog.message().unwrap();
    let message = message.as_str();

    if !message.starts_with("!") {
        return false;
    }

    let mut command  = message.split(" ");

    match command.next().unwrap_or("") {
        "!shutdown" => {
            Command::new("shutdown") 
                .args(command.collect::<Vec<&str>>())
                .spawn()
                .ok();
            },
        _ => eprintln!("No such message: {}", message),
    }

    true
}

pub struct MessageSender {
    pub webview: WebView,
}

impl MessageSender {
    pub fn update_tray_icon(&self, item: arg::PropMap) {
        let icon_theme_path = refarg_to_string(item.get("IconThemePath").unwrap());

        let mut js = String::with_capacity(item.len() * 5);
        for (key, value) in &item {
            let value = match &key[..] {
                "IconName" | "AttentionIconName" | "OverlayIconName" => Self::get_icon_path(&refarg_to_string(value), &icon_theme_path),
                _ => refarg_to_string(value),
            };
           js.push_str(&format!("\"{}\":\"{}\",", key, value)); 
        }
        let js = &format!("registerTrayItem({{{}}})",js);
        println!("{}",js);
        self.webview.run_javascript(js, None::<&gio::Cancellable>, |result| match result {
            Ok(_) => (),
            Err(error) => eprintln!("{}", error),
        });
    }
    fn get_icon_path(icon: &str, path: &str) -> String {
        if let Ok(mut icons) = linicon::lookup_icon(icon).with_search_paths(&[path]){
            while let Some(icon) = icons.next() {
                if let Ok(i) = icon {
                    return i.path.to_str().unwrap_or("").to_owned();
                }
            }
        }
        String::new()
    }
}
