use std::fs;
use std::env;

const PREFIX: &str = ".\\";



fn main() {
    let f_or_f = env::args()
                                .skip(1)
                                .next()
                                .unwrap_or_else(
                                    || {
                                        print!(".");
                                        std::process::exit(0)
                                    }
                                );
    

    let mut a = false;
    for entry in fs::read_dir(".").unwrap(){
        let dir = entry.unwrap().path().into_os_string().into_string().unwrap();
        if dir.as_str().to_lowercase().to_string() == format!("{}{}",PREFIX,f_or_f.as_str().to_lowercase()) {
            a = true
        }
    }
    if a{print!("{}",f_or_f)}else{print!(".")}
}