use std::env;
use std::{thread, time};


use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk::glib;
use gtk::gdk;

use glib::MainContext;

use gdk::RGBA;

use gtk_layer_shell as layer_shell;
use layer_shell::Edge;

use webkit2gtk::traits::{WebViewExt, WebInspectorExt, SettingsExt};
use webkit2gtk::{WebContext, WebView, WebViewExtManual, UserContentManager};

use crate::messages::MessageSender;

mod config;
mod messages;
mod tray;
mod util;

pub fn run(app: &Application) {
    let window = init_window(app);  

    let webview = init_webview();

    window.add(&webview);
    window.show_all();

    webview.connect_script_dialog(messages::handler);
    webview.set_background_color(&RGBA{red: 0.0,green: 0.0,blue: 0.0,alpha: 0.0});

    let message_sender = MessageSender { webview: webview};
    let (sender, receiver) = MainContext::channel(glib::PRIORITY_DEFAULT);
    
    thread::spawn(move || {
        // FIXME: Should be using a callback to wait until the page is loaded
        thread::sleep(time::Duration::from_millis(5000));
        tray::connect_to_dbus(sender).unwrap();
    });

    receiver.attach(None,move |item| {
        message_sender.update_tray_icon(item);
        glib::Continue(true)
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    gtk::main();
}

fn init_webview() -> WebView {    
    let user_content_manager = UserContentManager::new();
    let context = WebContext::default().unwrap();
    let webview = WebView::new_with_context_and_user_content_manager(&context, &user_content_manager);


    if !env::var("WEBBAR_INSPECTOR").is_err() {
        let settings = WebViewExt::settings(&webview).unwrap();
        settings.set_enable_developer_extras(true);

        let inspector = webview.inspector().unwrap();
        inspector.show();
    }
    webview.load_uri("file:///home/fried/.config/webbar/index.html");

    webview
}


fn init_window(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("WebBar")
        .default_height(config::HEIGHT)
        .width_request(0)
        .height_request(config::HEIGHT)
        .build();

    layer_shell::init_for_window(&window);
    layer_shell::set_layer(&window, layer_shell::Layer::Overlay);
    layer_shell::auto_exclusive_zone_enable(&window);

    layer_shell::set_anchor(&window, Edge::Left, true);
    layer_shell::set_anchor(&window, Edge::Top, true);
    layer_shell::set_anchor(&window, Edge::Right, true);
    layer_shell::set_anchor(&window, Edge::Bottom, false);

    window
}

