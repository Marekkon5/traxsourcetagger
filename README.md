# ![Logo](https://raw.githubusercontent.com/Marekkon5/traxsourcetagger/main/assets/32x32.png) Traxsource Tagger

Simple Rust + Webview app to automatically tag your music collection using data from Traxsource.

![Screenshot](https://i.imgur.com/YzXZOx2.png)

# Compatibility
<table>
    <thead>
        <tr>
            <th>Tested on platform</th>
            <th>Works correctly</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>Windows 7</td>
            <td>❌</td>
        </tr>
        <tr>
            <td>Windows 10</td>
            <td>✅</td>
        </tr>
        <tr>
            <td>macOS El Capitan</td>
            <td>❌</td>
        </tr>
        <tr>
            <td>macOS Catalina</td>
            <td>✅</td>
        </tr>
        <tr>
            <td>macOS Big Sur</td>
            <td>✅</td>
        </tr>
        <tr>
            <td>Linux</td>
            <td>✅</td>
        </tr>
    </tbody>
</table>


# Troubleshooting

### MacOS:
If you get a warning on MacOS, this app can't be opened for whatever reason:  
- Click Apple icon on top left
- Click System Preferences
- Click Security & Privacy
- Click Open Anyway

# Compiling

You have to install NodeJS (using your package manager/installer) and Rust (https://rustup.rs/)

Download repository:
```
git clone https://github.com/Marekkon5/traxsourcetagger
```

Compile UI:
```
cd client
npm i
npm run build
cd ..
```

Compile:
```
cargo build --release
```

Then you can also strip (Linux/Mac only) and compress (NOTE: upx doesn't work on Mac Big Sur) the binary:
```
strip discogstaggerrs
upx -9 discogstaggerrs
```

# Credits

BasCurtiz - Request, idea, tester, trailer, design.

# Support

If you wish to support me you can donate at paypal.me/marekkon5