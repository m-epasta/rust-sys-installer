#!/bin/bash
set -e

echo "Building Rust Sys Installer .deb package..."

# Build the Rust binary first
echo "Building release binary..."
cargo build --release

# Create package structure
echo "Creating package structure..."
rm -rf deb-package
mkdir -p deb-package/DEBIAN deb-package/usr/bin

# Copy control files
cat > deb-package/DEBIAN/control << 'EOF'
Package: rust-sys-installer
Version: 0.1.0
Section: utils
Priority: optional
Architecture: amd64
Maintainer: m-epasta <m-epasta@github.com>
Depends:
Description: Rust Development Environment Installer
 A tool to automatically install and configure a complete Rust development
 environment on Ubuntu systems, including Rust, Node.js with NVM, and
 VS Code with recommended extensions and settings.
'EOF'

# Copy postinst
cat > deb-package/DEBIAN/postinst << 'EOF'
#!/bin/bash
set -e

echo "Running Rust System Installer..."
/usr/bin/rust-sys-installer

echo "Installation completed!"
'EOF'

chmod +x deb-package/DEBIAN/postinst

# Copy binary
cp target/release/rust-sys-installer deb-package/usr/bin/
chmod +x deb-package/usr/bin/rust-sys-installer

# Build the package
echo "Building .deb package..."
dpkg-deb --build deb-package rust-sys-installer_0.1.0_amd64.deb

echo "Package built: rust-sys-installer_0.1.0_amd64.deb"
