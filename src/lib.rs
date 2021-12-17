use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk_layer_shell as layer_shell;
use layer_shell::Edge;
pub fn run(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("WebBar")
        .build();
    
    layer_shell::init_for_window(&window);
    layer_shell::set_layer(&window, layer_shell::Layer::Top);
    layer_shell::auto_exclusive_zone_enable(&window);
    
    layer_shell::set_anchor(&window, Edge::Left, true);
    layer_shell::set_anchor(&window, Edge::Top, true);
    layer_shell::set_anchor(&window, Edge::Right, true);
    layer_shell::set_anchor(&window, Edge::Bottom, false);


    window.show();
}
