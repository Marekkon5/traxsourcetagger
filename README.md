# ![Logo](https://raw.githubusercontent.com/Marekkon5/traxsourcetagger/main/assets/32x32.png) Traxsource Tagger

Simple Rust + Webview app to automatically tag your music collection using data from Traxsource.

![Screenshot](https://i.imgur.com/Jqm2Lk1.png)

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

### Windows:
If you get an error MSVCP140.dll, VCRUNTIME140.dll, or VCRUNTIME140_1.dll not found, 
Install the latest Microsoft Visual C++ Redistributable from here: https://aka.ms/vs/16/release/vc_redist.x64.exe and open the .exe again.  

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

Request: atarbig, Bas Curtiz, rosgr100  
Tester, trailer, design, comparison: Bas Curtiz  
Trailer: https://youtu.be/7ZHE8WJR918  
Strictness comparison: https://docs.google.com/spreadsheets/d/1E-ObqZTVL0gJTK4W267uWGdgsumvtZxJNW4CYfveuRk/edit?usp=sharing  

# Support

If you wish to support me you can donate at paypal.me/marekkon5