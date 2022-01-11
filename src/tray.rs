use dbus::blocking::stdintf::org_freedesktop_dbus::RequestNameReply as Reply;
use dbus::message::MatchRule;
use dbus::blocking::Connection;
use dbus::channel::MatchingReceiver;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::arg;

use dbus_crossroads::{Crossroads, Context, IfaceBuilder};

use std::error::Error;
use std::io::{Error as IOError, ErrorKind};
use std::time::Duration;

use gtk::glib;
use glib::Sender;


pub fn connect_to_dbus(sender: Sender<arg::PropMap>) -> Result<(), Box<dyn Error>> {
    let c = Connection::new_session()?;

    match c.request_name("org.kde.StatusNotifierWatcher", true, false, true)? {
        Reply::PrimaryOwner | Reply::AlreadyOwner => (),
        _ => return Err(Box::new(IOError::new(ErrorKind::PermissionDenied, "Another system tray is active"))),
    };

    let mut cr = Crossroads::new();

    let iface_token = cr.register("org.kde.StatusNotifierWatcher", setup_watcher);

    cr.insert("/StatusNotifierWatcher", &[iface_token], vec![]);

    c.start_receive(MatchRule::new_method_call(),Box::new(move |msg, conn| {
        cr.handle_message(msg, conn).unwrap();
        true
    }));

    c.add_match(MatchRule::new_signal("org.kde.StatusNotifierWatcher", "StatusNotifierItemRegistered"), move |_:(), conn, msg| {
        let dest_path = msg.get1::<String>().unwrap();
        let mut dest_path = dest_path.split(";");
        let p = conn.with_proxy(dest_path.next().unwrap(), dest_path.next().unwrap(), Duration::from_millis(1000));
        let metadata: arg::PropMap = p.get_all("org.kde.StatusNotifierItem").unwrap();
        sender.send(metadata);
        true
    })?;

    loop { c.process(Duration::from_millis(1000))?; };
}

fn setup_watcher(b: &mut IfaceBuilder<Vec<String>>) {

    b.property("RegisteredStatusNotifierItems")
        .get(|_, items| Ok(items.clone()));

    b.property("IsStatusNotifierHostRegistered")
        .get(|_, _| Ok(true));

    b.property("ProtocolVersion")
        .get(|_, _| Ok(0));

    // TODO: Maybe we don't need these signals
    let status_notifier_item_registered = 
        b.signal::<(String,), _>("StatusNotifierItemRegistered", ("service",)).msg_fn();

    let _status_notifier_item_unregistered = 
        b.signal::<(String,), _>("StatusNotifierItemUnregistered", ("service",)).msg_fn();

    let status_notifier_host_registered = 
        b.signal::<(), _>("StatusNotifierHostRegistered", ()).msg_fn();

    // Register a new icon to be shown in the tray
    b.method("RegisterStatusNotifierItem", ("service",), (), 
        move |ctx: &mut Context, items, (service,):(String,)| {
            let service = ctx.message().sender().unwrap().to_string() + ";" + &service;
            items.push(service.clone());

            let signal = status_notifier_item_registered(ctx.path(), &(service.clone(),));
            ctx.push_msg(signal);

            println!("New Item registered {}", service);
            Ok(())
        }
    );

    // Register a host which will display the icons
    b.method("RegisterStatusNotifierHost", ("service",), (),
        move |ctx: &mut Context, _, (_host,):(String,)| {
            let signal = status_notifier_host_registered(ctx.path(), &());
            ctx.push_msg(signal);

            Ok(())
        }
    );
}
