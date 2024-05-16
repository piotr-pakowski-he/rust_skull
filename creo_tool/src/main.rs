mod constants;
mod creotkdat;
mod registry;

use std::path::Path;
use std::{env, fs, io};

fn main() -> io::Result<()> {

    println!();
    println!("Welcome to Creo Desktiop Tool");
    println!();
    println!("env::args: {:?}", env::args());
    println!();

    match registry::register_in_registry() {
        Ok(_) => println!("✅ Initialization done"),
        Err(_) => panic!("❗ Can not get access to registry"),
    }

    let _ = visit_dirs(Path::new(constants::APPS_FOLDER), "parametric.exe");

    let mut all_dlls: Vec<String> = vec![];

    for search_path in constants::SEARCH_DIRS {
        let current_path = Path::new(constants::USER_PROFILE).join(search_path);        
        let result = visit_dirs(&current_path.as_path(), "dll");
        all_dlls.append(&mut result.unwrap());
    }

    let mut filtered: Vec<String> = all_dlls
        .into_iter()
        .filter(|path| {
            constants::TOOLKITS_NAMES
                .iter()
                .any(|e| Path::new(&str::to_lowercase(&path)).ends_with(&str::to_lowercase(&e)))
        })        
        .collect();

    filtered.sort();

    for dll in &filtered {
        println!("✅ {:?}", dll);
    }

    let result = filtered
        .into_iter()
        .map(|e: String| {
            Path::new(&e)
                .components()
                .rev()
                .map(|f| f.as_os_str().to_string_lossy() + "|")
                .take(4)
                .fold("".to_string(), |acc, v| acc + &v)
        })        
        .into_iter();

    for dll in itertools::sorted(result) {
        println!("✅ DLL: {:?}", dll);
    }

    Ok(())
}

fn visit_dirs(dir: &Path, pattern: &str) -> io::Result<Vec<String>> {
    let mut result = vec![];
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {            
            let path = &entry?.path();
            match path.extension() {
                None => if path.is_dir() { result.append(visit_dirs(&path, pattern)?.as_mut()) } ,
                Some(_) => result.push(format!("{}", path.to_str().unwrap()))
            }
        }
    }
    Ok(result)
}
