use std::fs;
use std::env;
use std::process::Command;





fn main() {
    let mut args = env::args()
                                .skip(1);
    let f_or_f = args.next().unwrap_or_else(||
         {print!(".");
         std::process::exit(0);}
        );
    
    let exe = args.next();
    
    let f_or_f = f_or_f.as_str();
    let findchar = f_or_f.rfind('/');
    let a = if findchar.is_none() {
        find(".\\", f_or_f)
    }else {
        let x = findchar.unwrap();
        let dir = &f_or_f[0..x+1];
        let file = &f_or_f[x+1..]; 
        find(dir, file)
    };
    if !a {
        print!(".");
    }else {
        if exe.is_none() {
            println!("{}",f_or_f)
        }else {
            if exe.unwrap() != String::from("-e") {
                println!("Expected -e")
            }else {
                let app = args.next().unwrap_or_else(||
                     {print!("expected executable || command found none"); std::process::exit(0)}
                    );
                
                let command = &*format!("{} {}", app, f_or_f);
                
                Command::new("powershell")
                    .args(["/C",command])
                    .spawn()
                    .expect("Unexpected error");

            }
        }
    }
}

fn find(folder: &str, f_or_f: &str) -> bool{
    let mut a = false;
    let b = fs::read_dir(folder);
    if b.is_err() {
        return a;
    }
    for entry in b.unwrap() {
        let dir = entry.unwrap().path().into_os_string().into_string().unwrap();
        if dir.as_str().strip_prefix(folder).unwrap_or(dir.as_str()).to_lowercase().to_string() == format!("{}",f_or_f.to_lowercase()) {
            a = true;
            break;
        }
    }
    a
}


#[test]
fn test_cmd(){
    Command::new("powershell")
                .args(["/C","code ."])
                .spawn()
                .expect("siade");
}