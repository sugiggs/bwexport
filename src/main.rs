use serde_json::{Result, Value};
use std::fs::File;
use std::io::{self, BufWriter, Write, Error};
use std::process::Command;


fn main()  -> io::Result<()> {
    //untyped_example();
    let mut sess_key : String = String::new();
    println!("Enter Session Key:");
    std::io::stdin().read_line(&mut sess_key)?;
    let json_data = get_items(&sess_key);
    println!("Export started");
    parse_json(&json_data);
    println!("Export Finished");
    Ok(())
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn parse_json(json_data: &str) -> Result<()> {
    let v: Vec<Value> = serde_json::from_str(json_data)?;
    let mut data = "collections,type,name,notes,fields,reprompt,login_uri,login_username,login_password,login_totp\n".to_owned();
    let f = File::create("Export_Org_Items.csv").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(data.as_bytes()).expect("Unable to write data");

    for vault_item in &v {
        if !vault_item["organizationId"].is_null() {
            let item_notes = if vault_item["notes"].is_null() {
                ""
            } else {
                vault_item["notes"].as_str().unwrap()
            };
            let item_name = if vault_item["name"].is_null() {
                ""
            } else {
                vault_item["name"].as_str().unwrap()
            };

            if vault_item["type"] == 1 {
                let item_username = if vault_item["login"]["username"].is_null() {
                    ""
                } else {
                    vault_item["login"]["username"].as_str().unwrap()
                };
                let item_password = if vault_item["login"]["password"].is_null() {
                    ""
                } else {
                    vault_item["login"]["password"].as_str().unwrap()
                };
                let item_totp = if vault_item["login"]["totp"].is_null() {
                    ""
                } else {
                    vault_item["login"]["totp"].as_str().unwrap()
                };

                let item_uris = if vault_item["login"]["uris"][0]["uri"].is_null() {
                    "".to_owned()
                } else {
                    let aaaa : &Vec<Value> = vault_item["login"]["uris"].as_array().unwrap();
                    let mut temp_uri : String = "".to_owned();
                    for uri_item in aaaa {
                        temp_uri.push_str(uri_item["uri"].as_str().unwrap());
                        temp_uri.push_str(",");
                    }
                    temp_uri
                    
                };
         
                //data = println!("\"{}\",{},{},\"{}\",\"{}\",{},\"{}\",{},\"{}\",{}","","login",item_name,item_notes,"",vault_item["reprompt"],item_uris,item_username,item_password,item_totp);
                data = format!("\"{}\",{},{},\"{}\",\"{}\",{},\"{}\",{},\"{}\",{}\n","","login",item_name,item_notes,"",vault_item["reprompt"],item_uris,item_username,item_password,item_totp);
                //println!("{}",&data);
                f.write_all(data.as_bytes()).expect("Unable to write data");
            } else if vault_item["type"] == 2 {
                data = format!("\"{}\",{},{},\"{}\",\"{}\",{},\"{}\",{},\"{}\",{}\n","","note",item_name,item_notes,"",vault_item["reprompt"],"","","","");
                f.write_all(data.as_bytes()).expect("Unable to write data");
            }
    
        }

    }
    Ok(())
}

fn read_input(sess_key: &mut String) -> io::Result<()> {
    
    Ok(())
}
fn get_items(sess_key: &str) -> String {
    //"44XrYTsrAI/Rm5cSKrVOh/2wL9/06dXBxdnpYT32WK2B8jWMp7YQULmWktF6CYqfjkAxiQIAT+8GZ4ljcZpR6g==";
    let output = Command::new("bw.exe")
        .arg("list")
        .arg("items")
        .arg("--session")
        .arg(sess_key)
        .output()
        .expect("failed to execute process");
    //println!("stdout: {}", String::from_utf8_lossy(&output.stdout));        
    format!("{}",String::from_utf8_lossy(&output.stdout))
}

