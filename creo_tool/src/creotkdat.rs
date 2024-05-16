use std::path::PathBuf;

static _CREO_TK_DAT_RECORD :  &str = 
"
NAME         {} \
EXEC_PATH    {} \
TEXT_PATH    {} \
DELAY_START  FALSE \
ALLOW_STOP   TRUE \
STARTUP      dll \
END \
";

fn _create_file(path: &str, toolkits: &Vec<PathBuf>) {
    for toolkit in toolkits {
        println!("{path}{}", toolkit.display());
    }
}