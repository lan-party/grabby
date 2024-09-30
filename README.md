# Grabby
A simple data exfiltration tool for some common windows applications written in rust.

Currently, this copies credential files from MySQL Workbench, FileZilla, Popular Web Browsers, Windows user credentials, and saved wifi passwords into a folder called 'grabby_files' wherever the executable is run.


### To Do
- base 64 decode filezilla passwords
- decrypt firefox and chrome-based browser data
- Decide on and implement the actual exfiltration of collected data over some network protocol or web api

For an actually useful tool like this, check out [The LaZagne Project](https://github.com/AlessandroZ/LaZagne).