# 🦜 Toutui: A TUI Audiobookshelf client for Linux  
<p align="center">
    <em>In French, being "tout ouïe" (toutui) means being all ears.</em>
</p>

<p align="center">
    🚀 <strong>Be toutui and enjoy audiobookshelf from your terminal!</strong>
</p>

<p align="center">
    <img src="assets/demo_2.gif" alt="🎬 Demo">
</p>


## ✨ Features  
✅ **Lightweight & Fast** – A minimalist, terminal-based UI (TUI) written in Rust 🦀  
✅ **Supports Books & Podcasts** – Enjoy both audiobooks and podcasts  
✅ **Sync Progress & Stats** – Keep your listening progress in sync  
✅ **Streaming Support** – Play directly without downloading  
✅ **Customizable Color Theme** – A config file will allow you to customize the color theme  
✅ **Cross-platform** - Linux <img src=".github/tux.png" align="top" width="24" alt="Tux (Linux)"/>  and macOS <img src=".github/apple.png" align="top" width="24" alt="Apple (macOS)"/>


## 🛠️ Roadmap  
**Short-term Goals**  
- Since this is a beta version, the main focus is on tracking and fixing bugs.
- Implementation of an integrated media player  
- Currently working on the next release: [v0.3.0-beta](https://github.com/AlbanDAVID/Toutui/milestone/2)

**Mid-term Goals**  
- CI/CD Implementation  
- Add future features described bellow.

## 🔮 Future features
Here are some features that could be added in future releases:
- Ability to add new podcasts from the app
- Add stats
- Offline mode
  
## ⚠️ Caution: Beta Version  
This beta app is still in **heavy development and contains bugs**.  
❗Please check [here](https://github.com/AlbanDAVID/Toutui/blob/main/known_bugs.md) for known bugs especially **MAJOR BUGS** before using the app, so you can use it with full awareness of any known issues.  
If you encounter any issues that are **not yet listed** in the Issues section or into [known bugs](https://github.com/AlbanDAVID/Toutui/blob/main/known_bugs.md), please **open a new issue** to report them.  

🔐 Although it's a beta version, you can use this app with **minimal risk** to your Audiobookshelf library.  
At worst, you may experience **sync issues**, but there is **no risk** of data loss, deletion, or irreversible changes (API is just used to retrieve books and sync them).

## 📝 Notes
### 🤝 **Contributing**  
Do not hesitate to contribute to this project by submitting your code, ideas, or feedback. Please make sure to read the [contributing guidelines](https://github.com/AlbanDAVID/Toutui/blob/main/CONTRIBUTING.md) first.

### 🍏 **macOS**
>[!WARNING]
>**Not yet supported on macOS (but we are working on it).**

### 🎨 **UI**
The **font** and **emojis** may vary depending on the terminal you are using.
To ensure the best experience, it's recommended to use terminals that support **emoji rendering** properly (e.g., Kitty, Alacritty).

### 🎧 **Using `cvlc`**
- Type `help` to see commands  
- When using **`cvlc`** (command-line VLC), make sure to use the `shutdown` command when you want to quit the listening session.
This helps ensure that your books are successfully synced and prevents any potential issues.

## 🚨 Installation Instructions

**⚠️ If you follow all the instructions but installation fails, please open an installation issue.**  

### Automatic install

>[!WARNING]
>It's quite challenging to create a script that is exhaustive for all distributions and OS. If you encounter difficulties, leave an installation issue and install the app [manually](https://github.com/AlbanDAVID/Toutui?tab=readme-ov-file#git).

#### **Install**
>[!WARNING]
>**Not yet supported on macOS (but we are working on it).**
>
>This is a beta app, please read [this](https://github.com/AlbanDAVID/Toutui?tab=readme-ov-file#%EF%B8%8F-caution-beta-version). 
```bash
git clone https://github.com/AlbanDAVID/Toutui
cd Toutui/
chmod +x hello_toutui.sh
./hello_toutui.sh install
```
Once the installation is complete, type `toutui` in your terminal to launch the app.
#### **Update**
The script will detect if a new release is available and install it if any.
```bash
./hello_toutui.sh update
```

#### **Notes**  

##### Files installed:
in `/usr/bin`
- `toutui` — The binary file (you can execute it from anywhere).

in `~/.config/toutui`
- `.env` — Contains the secret key.
- `config.toml` — Configuration file.
- `toutui.log` — Log file.
- `db.sqlite3` — SQLite database file.

### For Arch Users
🚧 Soon

### Git

❗This is a beta app, please read [this](https://github.com/AlbanDAVID/Toutui?tab=readme-ov-file#%EF%B8%8F-caution-beta-version).  
❗macOS user: read [this](https://github.com/AlbanDAVID/Toutui/blob/main/README.md#-macos).  

#### **Requirements**
- `Rust`
- `VLC`
- `SQLite3`
- `libsqlite3-dev` (for some OS, Debian for example)
- `libssl-dev`
- `Your terminal must support emojis`
- Optional, only if you use cvlc:
- `Netcat`
- `Kitty terminal emulator`

*⚠️ If you had to install a package that is not listed above, please open an installation issue.*

#### **Install**
```bash
git clone https://github.com/AlbanDAVID/Toutui
cd Toutui/
mkdir -p ~/.config/toutui
cp config.example.toml ~/.config/toutui/config.toml
```

Token encryption in the database (<u>**NOTE**</u>: replace `secret`)
```bash
echo TOUTUI_SECRET_KEY=secret >> ~/.config/toutui/.env
```

```bash
cargo run --release
```

To install in a custom location, provide the path like this:
```console
./hello_toutui.sh install /usr/bin
```

#### **Update**

When a new release is available, follow these steps:

The script will detect if a new release is available and install it if any.
```bash
chmod +x hello_toutui.sh
./hello_toutui.sh update
```
OR 
```bash
git pull https://github.com/AlbanDAVID/Toutui
cargo run --release
```

#### **Notes**  
##### Exec the binary:
```bash
cd target/release
./Toutui
```

##### Files installed:
After installation, you will have the following files in `~/.config/toutui`
- `.env` — Contains the secret key.
- `config.toml` — Configuration file.
- `toutui.log` — Log file.
- `db.sqlite3` — SQLite database file.
