# Rust Development Environment Installer

A comprehensive installer that sets up a complete Rust development environment on Ubuntu systems, including VS Code with essential extensions and all necessary tools.

## 🚀 Features

- **System Package Installation**: Installs essential build tools and development packages
- **NVM & Node.js**: Installs Node Version Manager and Node.js for JavaScript development
- **Rust Toolchain**: Installs the latest Rust compiler and Cargo package manager
- **VS Code**: Installs Visual Studio Code with curated extensions for Rust development
- **Progress Tracking**: Beautiful progress indicators throughout the installation
- **Error Handling**: Comprehensive error handling and validation

## 📋 Prerequisites

- **Ubuntu 22.04+** (other Debian-based systems may work but are not officially supported)
- **Internet connection** for downloading packages and tools
- **sudo access** for system package installation

## 🛠️ Installation

### Step 1: Clone or Download the Repository

```bash
git clone https://github.com/your-username/rust-sys-installer.git
cd rust-sys-installer
```

Or download and extract the ZIP file.

### Step 2: Build the Project

```bash
# Install Rust if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Build the release version
cargo build --release
```

### Step 3: Run the Installer

```bash
./install.sh
```

**Note**: The installer script automatically handles its own executable permissions, so you don't need to manually run `chmod +x install.sh` first.

**Note**: The installer requires sudo access for system package installation. You'll be prompted for your password when needed.

## 📦 What Gets Installed

### System Packages
- `curl`, `git`, `wget`
- `gpg`, `apt-transport-https`
- Build tools and development libraries

### Development Tools
- **NVM (Node Version Manager)** - For managing Node.js versions
- **Node.js** - Latest LTS version via NVM
- **Rust** - Latest stable toolchain with Cargo

### VS Code Extensions
- **Rust Analyzer** - Language server for Rust
- **Code Runner** - Run code snippets
- **Prettier** - Code formatting
- **GitLens** - Enhanced Git capabilities
- **Error Lens** - Inline error display
- **ES7 React/Redux Snippets** - JavaScript/React snippets
- **GitHub Copilot** - AI-powered code assistance
- And more...

### VS Code Settings
The installer configures VS Code with optimized settings for Rust development, including:
- Format on save
- Inline suggestions
- Rust-specific analyzer settings
- Custom code runner configurations

## 🎯 Usage After Installation

After successful installation, you'll have:

1. **Rust toolchain** ready to use:
   ```bash
   cargo --version
   rustc --version
   ```

2. **VS Code** with all extensions installed and configured

3. **NVM and Node.js** for JavaScript development:
   ```bash
   node --version
   npm --version
   ```

## 🔧 Troubleshooting

### Permission Issues
If you get permission errors:
```bash
# Make sure the script is executable
chmod +x install.sh

# Run with explicit bash if needed
bash install.sh
```

### Build Failures
If the build fails:
```bash
# Clean and rebuild
cargo clean
cargo build --release
```

### Installation Fails
If the installer fails:
- Check your internet connection
- Ensure you have sudo privileges
- Try running individual commands manually to isolate issues

### VS Code Not Found
If VS Code extensions fail to install:
- Make sure VS Code is properly installed: `code --version`
- Try installing extensions manually: `code --install-extension rust-lang.rust-analyzer`

## 🏗️ Development

To modify the installer:

```bash
# Development build
cargo build

# Run tests
cargo test

# Check code quality
cargo clippy
```

## 📄 License

This project is open source. See LICENSE file for details.

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📞 Support

If you encounter issues:

1. Check the troubleshooting section above
2. Review the installation logs for error messages
3. Create an issue on GitHub with:
   - Your Ubuntu version: `lsb_release -a`
   - Error messages
   - Steps to reproduce

---

**Happy coding with Rust! 🦀**
