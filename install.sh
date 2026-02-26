#!/bin/bash
#
# Pi-Swarm One-Line Installer
# Supports: Binary download (fast) or Build from source (fallback)
#

set -e

echo "🥧 Installing Pi-Swarm..."

# Detect OS and architecture
OS="linux"
ARCH="amd64"
RUST_TARGET="x86_64-unknown-linux-gnu"

if [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
fi

if [[ $(uname -m) == "aarch64" ]] || [[ $(uname -m) == "arm64" ]]; then
    ARCH="arm64"
    RUST_TARGET="aarch64-unknown-linux-gnu"
fi

echo "📱 Platform: $OS-$ARCH"

# Settings
INSTALL_DIR="/usr/local/bin"
GITHUB_USER="Pi-Swarm"
REPO="pibot-swarm-v2"
VERSION="v8.0.1"
BINARY_NAME="pi-${OS}-${ARCH}"
URL="https://github.com/${GITHUB_USER}/${REPO}/releases/download/${VERSION}/${BINARY_NAME}"

# Check if can write to /usr/local/bin
if [ -w "$INSTALL_DIR" ]; then
    INSTALL_PATH="$INSTALL_DIR/pi"
else
    INSTALL_DIR="$HOME/.local/bin"
    INSTALL_PATH="$INSTALL_DIR/pi"
    mkdir -p "$INSTALL_DIR"
fi

# Function to install from binary
install_binary() {
    echo "⬇️  Downloading binary..."
    local tmp_file="/tmp/pi-swarm-$$"
    
    if command -v curl >/dev/null; then
        if curl -fsSL "$URL" -o "$tmp_file" 2>/dev/null; then
            mv "$tmp_file" "$INSTALL_PATH"
            chmod +x "$INSTALL_PATH"
            rm -f "$tmp_file"
            echo "✅ Binary installed!"
            return 0
        fi
    elif command -v wget >/dev/null; then
        if wget -q "$URL" -O "$tmp_file" 2>/dev/null; then
            mv "$tmp_file" "$INSTALL_PATH"
            chmod +x "$INSTALL_PATH"
            rm -f "$tmp_file"
            echo "✅ Binary installed!"
            return 0
        fi
    fi
    
    rm -f "$tmp_file"
    return 1
}

# Function to build from source
install_build() {
    echo "⚙️  Binary not available. Building from source..."
    
    # Check for Rust
    if ! command -v cargo >/dev/null 2>&1; then
        echo "🔧 Rust not found. Installing..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    
    # Create temp build directory
    BUILD_DIR="/tmp/pi-swarm-build-$$"
    mkdir -p "$BUILD_DIR"
    cd "$BUILD_DIR"
    
    # Clone repository
    echo "📥 Cloning repository..."
    git clone --depth 1 "https://github.com/${GITHUB_USER}/${REPO}.git" .
    
    # Build
    echo "🔨 Building (this may take a few minutes)..."
    cargo build --release
    
    # Install binary
    cp "target/release/pi" "$INSTALL_PATH"
    chmod +x "$INSTALL_PATH"
    
    # Cleanup
    cd /
    rm -rf "$BUILD_DIR"
    
    echo "✅ Built and installed from source!"
}

# Try binary first, fallback to build
if ! install_binary; then
    echo "⚠️  Pre-built binary not available for your platform."
    read -p "Build from source? (requires Rust) [Y/n] " reply
    reply=${reply:-Y}
    if [[ "$reply" =~ ^[Yy]$ ]]; then
        install_build
    else
        echo "❌ Installation cancelled."
        echo ""
        echo "To install manually:"
        echo "  1. git clone https://github.com/${GITHUB_USER}/${REPO}.git"
        echo "  2. cd ${REPO}"
        echo "  3. cargo build --release"
        echo "  4. sudo cp target/release/pi /usr/local/bin/"
        exit 1
    fi
fi

# Add to PATH if needed
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo 'export PATH="$PATH:'$INSTALL_DIR'"' >> ~/.bashrc
    export PATH="$PATH:$INSTALL_DIR"
    echo '📌 Added to PATH'
fi

echo ""
echo "✅ Pi-Swarm installed!"
echo "   Location: $INSTALL_PATH"
echo "   Version: $(pi --version 2>/dev/null || echo 'unknown')"

# Setup Ollama
echo ""
read -p "🤖 Install Ollama for local AI? (y/n) " reply
if [ "$reply" = "y" ]; then
    echo "Installing Ollama..."
    curl -fsSL https://ollama.com/install.sh | sh
    echo "Pulling AI model (qwen2.5:1.5b)..."
    ollama pull qwen2.5:1.5b
    echo "✅ Ollama ready!"
fi

# Setup Telegram token
echo ""
echo "═══════════════════════════════════════"
echo "  🤖 Telegram Bot Setup"
echo "═══════════════════════════════════════"
echo ""
echo "1. Message @BotFather on Telegram"
echo "2. Send /newbot"
echo "3. Choose a name (e.g., MyPiSwarm)"
echo "4. Choose username (e.g., mypiswarm_bot)"
echo "5. Copy the token shown"
echo ""
read -p "📋 Paste your Telegram token: " token

if [ -n "$token" ]; then
    CONFIG_DIR="$HOME/.pi-swarm"
    mkdir -p "$CONFIG_DIR"
    echo "{\"telegram\":{\"token\":\"$token\",\"enabled\":true}}" > "$CONFIG_DIR/config.json"
    chmod 600 "$CONFIG_DIR/config.json"
    echo "✅ Configuration saved to ~/.pi-swarm/config.json"
    echo ""
    echo "🎮 Start your bot:"
    echo "   pi telegram"
else
    echo "⚠️  No token provided. Set it later with:"
    echo "   pi config telegram.token YOUR_TOKEN"
fi

echo ""
echo "═══════════════════════════════════════"
echo "  🥧 Pi-Swarm Ready!"
echo "═══════════════════════════════════════"
echo ""
echo "Quick commands:"
echo "  pi --help       Show help"
echo "  pi status       System status"
echo "  pi telegram     Start the bot"
echo ""
