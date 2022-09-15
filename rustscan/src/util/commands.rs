use crate::util;

// this stores the api key in a text file which most definitely is not best practice
// but i wanted to practice working with file i/o in rust...
// plan to change in the future

pub fn set_api_key(args: Vec<String>) {
    util::api::set_api_key(args);    
}

pub fn get_api_key() {
    util::api::get_api_key();
}

pub fn query() {
    util::api::query();
}

pub fn get_os_type() -> String {
    let info = os_info::get();
    println!("OS type: {}", info.os_type());
    
    return info.os_type().to_string();
}

pub fn set_rule() {
    let os = get_os_type();

    if os == "Mac OS" {
        util::firewall::set_pfctl_rule();
    } else if os == "Ubuntu" {
        util::firewall::set_iptables_rule();
    } else {
        println!("windows");
    }
}

pub fn clear_chain() {
    let os = get_os_type();

    if os == "Mac OS" {
        util::firewall::clear_pfctl_chain();
    } else if os == "Ubuntu" {
        util::firewall::clear_iptables_chain();
    } else {
        println!("windows");
    }
}
