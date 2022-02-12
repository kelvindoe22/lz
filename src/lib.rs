use std::fs;

#[derive(Debug)]
pub enum Cases {
    StartsWith,
    EndsWith,
    Contains
}

pub fn find_matching_files_sw(folder: &str, f_or_f: &str, case: Cases) {
    let mut got_one = false;
    let folder = folder.to_lowercase();
    let folder = &*folder;
    let iter = fs::read_dir(folder);
    if iter.is_err(){
        std::process::exit(0);
    }

    for entry in iter.unwrap(){
        let dir = entry
            .unwrap()
            .path()
            .into_os_string()
            .into_string()
            .unwrap();

        match &case {
            &Cases::StartsWith => {
                if let Some(item) = dir.to_lowercase().as_str().strip_prefix(folder){
                    if item.starts_with(f_or_f) {
                        got_one = true;
                        if folder == ".\\" {
                            println!("{item}")
                        }else {
                            println!("{dir}")
                        }
                    }
                }
            },
            &Cases::EndsWith => {
                if let Some(item) = dir.to_lowercase().as_str().strip_prefix(folder){
                    if item.ends_with(f_or_f) {
                        got_one = true;
                        if folder == ".\\" {
                            println!("{item}")
                        }else {
                            println!("{dir}")
                        }
                    }
                }
            },
            &Cases::Contains => {
                if let Some(item) = dir.to_lowercase().as_str().strip_prefix(folder){
                    if item.contains(f_or_f) {
                        got_one = true;
                        if folder == ".\\" {
                            println!("{item}")
                        }else {
                            println!("{dir}")
                        }
                    }
                }
            }
        }

        
        
    }

    

    if !got_one {
        println!(".");
    }



}


pub fn contains<T: std::cmp::PartialEq>(op: &Option<T>, item: &T)-> bool {
    match op{
        Some(e) => e == item,
        None => false,
    }
}
