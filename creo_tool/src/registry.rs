use std::io;
use winreg::{enums::HKEY_CURRENT_USER, RegKey};
use crate::constants;

pub fn register_in_registry() -> io::Result<()> {    
    let exe_path = format!("{} %1", std::env::current_exe()?.display());    
    let hkr = RegKey::predef(HKEY_CURRENT_USER).open_subkey(constants::REG_ROOT)?;
    let (sub_key, _) = hkr.create_subkey(constants::APP_NAME)?;
    let (sub_key, _) = sub_key.create_subkey("command")?;
    sub_key.set_value("", &exe_path) 
}