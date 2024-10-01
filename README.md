# Grabby
A simple data exfiltration tool for some common windows applications.

This copies credential files from MySQL Workbench, FileZilla, Popular Web Browsers, Windows user credentials, and saved wifi passwords into a folder called 'grabby_files' wherever the executable is run. That folder is then added to a zip archive, base64 encoded, and sent over a tcp socket. All copied files are deleted after transmission to clean up.


### To Do
- base 64 decode filezilla passwords
- decrypt firefox and chrome-based browser data

For an actually useful tool, check out [The LaZagne Project](https://github.com/AlessandroZ/LaZagne).