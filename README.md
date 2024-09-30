# Grabby
A simple data exfiltration tool for some common windows applications written in rust.

Currently, this copies credentials files from MySQL Workbench, Filezilla, Popular Web Browsers, Windows user credentials, and saved wifi passwords into a folder called 'grabby_files' wherever the executable is run.


### To Do
- base 64 decode filezilla passwords
- decrypt firefox and chrome-based browser data

For an actually useful tool like this, check out [The LaZagne Project](https://github.com/AlessandroZ/LaZagne).