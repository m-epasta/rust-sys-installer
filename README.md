# PREAMBULE

For now, this repo only supports ubuntu, others os will maybe be added later

# INSTALLATION

## Option 1: .deb Package (Recommended - Simplest)

Download the latest `.deb` file from the [releases page](https://github.com/m-epasta/rust-sys-installer/releases) and install it:

```bash
sudo dpkg -i rust-sys-installer_*.deb
```

The package will automatically install all dependencies and configure your development environment.

## Option 2: Manual Installation (for Live USB / Try Mode)

### Option 2a: Using .deb Package (Simplest for Try Mode)

1. Download the `.deb` file from the [releases page](https://github.com/m-epasta/rust-sys-installer/releases)
2. Copy the `.deb` file to your USB drive or cloud storage
3. Boot into Ubuntu try mode
4. Copy the `.deb` file to `/tmp`: `cp /media/your_os/your_usb_drive/rust-sys-installer_0.1.0_amd64.deb /tmp/`
5. Install with: `sudo dpkg -i /tmp/rust-sys-installer_0.1.0_amd64.deb`

### Option 2b: Using Script and Binary (Original Method)

This script contains (in release): a bash script named <code>install.sh</code> and a binary file named <code>rust-sys-installer</code>: actual script that installs the packages.

All you have to do is to follow these simple steps:
1. copy the release files into an USB drive or a cloud DRIVE that you can plug/access later in your computer (ubuntu in try mode)

2. place the 2 release files into a folder

3. Plug your USB booter into your computer

4. follow the instructions and stop at installation process (try mode)
5. now copy your folder into /tmp you can do it by running this bash line in the terminal:
<code>cp -r /media/your_os/your_usb_drive/your_folder</code> or, if your folder is in the cloud, connect to it with firefox(you can launch it with <code>firefox</code> into a terminal) and then copy it manualy to /tmp folder

6. cd into /tmp: run <code>cd /tmp</code>

7. make the install.sh file executable with this line: <code>chmod +x your_folder/install.sh</code>

8. finnaly, run the executable with **sudo privilege**: <code>sudo ./your_folder/install.sh</code>

# Script actions

The script first updates to latest compatible apt and apt-get packages manager to make sure the installation is going to work

Then, it installs some system packages and dependencies useful/necessary to code in rust and also some to code in modern typescript(nodejs runtime).

It finally installs vscode via snap --classic and some extensions that I use for DX.

# CONTRIBUTIONS & FUTURE

The app is for now, designed for a specific need that I found, so every thing is designed for my needs. The app may add generi help or better interaction like customizing the settings.json and extensions.json configs files etc ...
So if you find this idea useful with a potential, contact me on discord: @psykokwak6049 or just do an issue/pull request on the repository.
I do not plan for now to continue this repo because my need is fullfilled, stil open to contributions/propositions.
