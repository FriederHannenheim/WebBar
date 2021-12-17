use gtk::prelude::*;
use gtk::{Application};

fn main() {
    let app = Application::builder()
        .application_id("com.fhannenheim.WebBar")
        .build();

    app.connect_activate(webbar::run);

    app.run();
}
