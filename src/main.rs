#![windows_subsystem = "windows"]

extern crate enigo;
extern crate hotkey;

use hotkey::Listener;
use std::env;
use std::process::Command;
use enigo::{Enigo, Key, KeyboardControllable};


fn add_to_schtasks() {
    let path = format!(
        "{}\\U2800.exe", 
        env::current_dir().unwrap().into_os_string().into_string().unwrap()
    );
    let order = format!(
        "schtasks /create /F /SC ONLOGON /RL HIGHEST /TN U2800 /TR {}", 
        path
    );
    if let Err(_) = Command::new("cmd").args(&["/C", &order]).output(){} //pls help, it's terrible
}


fn press_key() {
    let mut enigo = Enigo::new();
    enigo.key_sequence("­");
}


fn main() {
    add_to_schtasks();
    
    let mut hk = Listener::new();
    hk.register_hotkey(
        hotkey::modifiers::CONTROL,
        '0' as u32,
        || press_key(),
    )
    .unwrap();

    hk.listen();
}