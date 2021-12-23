use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

use gtk_layer_shell as layer_shell;
use layer_shell::Edge;

use webkit2gtk::traits::WebViewExt;
use webkit2gtk::{WebContext, WebView, WebViewExtManual, UserContentManager};

mod config;

pub fn run(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("WebBar")
        .default_height(config::HEIGHT)
        .build();

    layer_shell::init_for_window(&window);
    layer_shell::set_layer(&window, layer_shell::Layer::Overlay);
    layer_shell::auto_exclusive_zone_enable(&window);

    layer_shell::set_anchor(&window, Edge::Left, true);
    layer_shell::set_anchor(&window, Edge::Top, true);
    layer_shell::set_anchor(&window, Edge::Right, true);
    layer_shell::set_anchor(&window, Edge::Bottom, false);
    
   
    let context = WebContext::default().unwrap();
    let webview = WebView::new_with_context_and_user_content_manager(&context, &UserContentManager::new());
    
    webview.load_uri("https://crates.io");

    window.add(&webview);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
