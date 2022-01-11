use dbus::arg::{RefArg, ArgType};

pub fn refarg_to_string(value: &dyn RefArg) -> String {
    if let Some(s) = value.as_str() {return s.to_owned()};
    if let Some(i) = value.as_i64() {return format!("{}", i)};
    if let Some(f) = value.as_f64() {return format!("{}", f)};
    format!("{:?}", value)
}
