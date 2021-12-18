use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use gtk::gio;

use gtk_layer_shell as layer_shell;
use layer_shell::Edge;

use webkit2gtk::traits::{WebViewExt, SettingsExt, WebContextExt, WebInspectorExt};
use webkit2gtk::{WebContext, WebView, WebViewExtManual, UserContentManager};

mod config;

pub fn run(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("WebBar")
        .default_height(config::HEIGHT)
        .
        .build();
    
    layer_shell::init_for_window(&window);
    layer_shell::set_layer(&window, layer_shell::Layer::Top);
    layer_shell::auto_exclusive_zone_enable(&window);
    layer_shell::set_keyboard_interactivity(&window, true);

    layer_shell::set_anchor(&window, Edge::Left, true);
    layer_shell::set_anchor(&window, Edge::Top, true);
    layer_shell::set_anchor(&window, Edge::Right, true);
    layer_shell::set_anchor(&window, Edge::Bottom, false);

    let context = WebContext::default().unwrap();
    //context.set_web_extensions_initialization_user_data(&"webkit".to_variant());
    //context.set_web_extensions_directory("../webkit2gtk-webextension-rs/example/target/debug/");

    let webview = WebView::new_with_context_and_user_content_manager(&context, &UserContentManager::new());
    
    /*webview.load_uri("/home/fried/.config/webbar/layout.html");*/
    webview.load_uri("https://crates.io");
    window.add(&webview);

    let settings = WebViewExt::settings(&webview).unwrap();
    settings.set_enable_developer_extras(true);


    let inspector = webview.inspector().unwrap();
    inspector.show();

    window.show_all();

    // webview.run_javascript("alert('Hello');", None::<&gio::Cancellable>, |_result| {});

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
