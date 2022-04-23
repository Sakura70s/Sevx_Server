use chrono::Local;

pub fn print_log(msg: String) {
    let fmt = "%Y-%m-%d %H:%M:%S";
    let now = Local::now().format(fmt);
    println!("{}  SUCCESS: \"{}\"", now, msg);
}