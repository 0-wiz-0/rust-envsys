This is example code how to get environmental sensor data on NetBSD
using the [envsys(4)](https://man.netbsd.org/envsys.4) framework.

Examples of the output of this program are in the [examples](examples)
directory.

If you want to deserialize the data yourself, the NetBSD kernel
returns a plist in XML format (including a terminating zero byte,
since it's a C string). The format looks like this:

```
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple Computer//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
        <key>$SENSORNAME</key>
        <array>
                <dict>
                        <key>INTEGER_KEY</key>
                        <integer>12345</integer>
                        <key>STRING_KEY</key>
                        <string>some text</string>
                        <key>BOOLEAN_KEY</key>
                        <true/>
                        ...
               </dict>
               ...         (an arbitrary number of entries like the one above, i.e. "<dict>...</dict>")
                <dict>
                        <key>device-properties</key>
                        <dict>
                                <key>device-class</key>
                                <string>something</string>
                                <key>refresh-timeout</key>
                                <integer>0x1e</integer>
                        </dict>
                </dict>
        </array>
        ... (an arbitrary number of entries like the one above, i.e. "<key>...</key><array>...</array>")
</dict>
</plist>
```
