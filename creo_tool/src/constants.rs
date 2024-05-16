
pub static APP_NAME: &str = "Creo Tool";
pub static USER_PROFILE : &str = env!("USERPROFILE");
pub static APPS_FOLDER : &str = "c:\\eng_apps\\";
pub static REG_ROOT : &str = "Software\\Classes\\Directory\\shell";
pub static SEARCH_DIRS : &[&str] = &["source\\repos", "primetools"];
pub static TOOLKITS_NAMES: &[&str] = &["genericbatch.dll", "publish.dll", "autoassembly.dll", 
"autodrawing.dll", "busbardesign.dll", "cabling.dll", 
"cleatdrawing.dll", "creotoolkit.dll", "edit.dll", 
"file.dll", "help.dll", "maintenance.dll", "navigate.dll" ,
"piping.dll", "standardmaintenance.dll", "windingexits.dll"];