
# swtools

Simple Work Tools.

Tools that can work happily.

# Local File Sharing

Can be used to share transfer files on a local LAN!

## Usages:

Get the swtools.exe execution file, prepare a configuration file config.json in the directory where swtools.exe is located, there are three item:


**path:** | the director that you want to share.
--|--|--
**localhost:** | the ip address that you pc.
**port:** | the port number.

for example:

```json
{
  "path": "D://",
  "localhost": "192.168.199.217",
  "port": 8090
}
```
