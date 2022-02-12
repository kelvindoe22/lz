use std::env;
use std::fs;
use std::process::Command;
mod lib;

use lib::Cases;





fn main() {
    let mut args = env::args().skip(1);
    let f_or_f = args.next().unwrap_or_else(|| {
        print!(".");
        std::process::exit(0);
    });

    let exe = args.next();

    let f_or_f = f_or_f.as_str();
    let findchar = f_or_f.rfind('/');
    

    let (dir, item) = if findchar.is_none(){
        (".\\",f_or_f)
    }else {
        let x = findchar.unwrap();
        (&f_or_f[0..x + 1], &f_or_f[x + 1..] )
    };

    let sw = String::from("-sw");
    let ew = String::from("-ew");
    let c = String::from("-c");
    let e = String::from("-e");
    //-sw (starts with)

    if lib::contains(&exe, &sw){
        lib::find_matching_files_sw(dir, item, Cases::StartsWith)
    }else if lib::contains(&exe, &ew){
        lib::find_matching_files_sw(dir, item, Cases::EndsWith)
    }else if lib::contains(&exe, &c){
        lib::find_matching_files_sw(dir, item, Cases::Contains)
    }
    else if lib::contains(&exe, &e){
        if !find(dir, item) {
            println!(".");
            std::process::exit(0);
        }

        let app = args.next().unwrap_or_else(|| {
            print!("expected executable || command found none");
            std::process::exit(0)
        });

        let command = &*format!("{} {}", app, f_or_f);

        Command::new("powershell")
            .args(["/C", command])
            .spawn()
            .expect("Unexpected error");
    }
    else {
        if find(dir, item) {
            println!("{}",f_or_f)
        }else {
            println!(".")
        }
    }

}   
    // -e case here
    
fn find(folder: &str, f_or_f: &str) -> bool {
    let mut a = false;
    let b = fs::read_dir(folder);
    if b.is_err() {
        return a;
    }
    for entry in b.unwrap() {
        let dir = entry
            .unwrap()
            .path()
            .into_os_string()
            .into_string()
            .unwrap();
        if dir
            .as_str()
            .strip_prefix(folder)
            .unwrap_or(dir.as_str())
            .to_lowercase()
            .to_string()
            == format!("{}", f_or_f.to_lowercase())
        {
            a = true;
            break;
        }
    }
    a
}
