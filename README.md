# clockt

A digital clock that runs in your terminal.  
![screenshot](assets/screenshot.png)

## Features
User preference
- You can select time format, whether it 12-hours or 24-hours.
- Or select display format, whether hour:minutes:second or just hour:minute
- The default settings is hour:minute display format and 12-hours time format

## Config

Currently, you can set your preferenced option in `$(cwd)/config.conf` file.
- The config file uses key=value pair format.
- Example:
```
timeFormat=12h # 12 hours format
# timeFormat=12h # 24 hours format
displayFormat=hm # %H:%M will be displayed
# displayFormat=hms # %H:%M:%S will be displayed
```
