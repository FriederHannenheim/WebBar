use std::process::Command;
use std::path::PathBuf;

use gtk::gio;

use webkit2gtk::{WebView, ScriptDialog};
use webkit2gtk::traits::WebViewExt;

use dbus::arg;

use crate::util::refarg_to_string;

pub fn handler(_webview: &WebView, dialog: &ScriptDialog) -> bool {
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
        let icon_theme_path = item.get("IconThemePath");
        let icon_theme_path = match icon_theme_path {
            Some(p) => PathBuf::from(&refarg_to_string(&p)),
            None => PathBuf::new(),
        };
        let icon_theme_name = match icon_theme_path.file_name() {
            Some(s) => s.to_str(),
            None => None,
        };
        let icon_theme_path = match icon_theme_path.parent() {
            Some(p) => p.to_str().unwrap_or(""),
            None => "",
        };

        let mut js = String::with_capacity(item.len() * 5);
        for (key, value) in &item {
            let value = match &key[..] {
                "IconName" | "AttentionIconName" | "OverlayIconName" => Self::get_icon_path(&refarg_to_string(value), &icon_theme_name, &icon_theme_path),
                _ => refarg_to_string(value),
            };
            js.push_str(&format!("\"{}\":\"{}\",", key, value)); 
        }
        let js = &format!("registerTrayIcon({{{}}})",js);
        println!("{}",js);
        self.webview.run_javascript(js, None::<&gio::Cancellable>, |result| match result {
            Ok(_) => (),
            Err(error) => eprintln!("{}", error),
        });
    }
    fn get_icon_path(icon_name: &str, theme_name: &Option<&str>, theme_path: &str) -> String {
        println!("Searching for {} in {} in path {}", icon_name, theme_name.unwrap_or(""), theme_path);
        match theme_name {
            None => {
                if let Some(icon) = linicon::lookup_icon(icon_name).next() {
                    if let Ok(i) = icon {
                        return i.path.to_str().unwrap_or("").to_owned();
                    }
                    println!("{:?}", icon);
                }
            },
            Some(name) => {
                if let Ok(icon) = linicon::lookup_icon(icon_name).with_search_paths(&[theme_path]) 
                {
                    if let Some(icon) = icon.from_theme(name).next() {
                        if let Ok(i) = icon {
                            return i.path.to_str().unwrap_or("").to_owned();
                        }
                    }
                }
            },
        }
        String::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_icon_path() {
        let path = MessageSender::get_icon_path("wireshark", &None::<&str>, "");
        assert_eq!("/usr/share/icons", &path[..]);
    }
}
