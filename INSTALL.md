# Fomi AI Assistant

Fomi is a cross-platform desktop AI assistant built with Rust, Tauri, and local AI models. It runs entirely on your hardware, ensuring complete privacy, and features local Large Language Models (LLMs), Text-to-Speech (TTS), and Speech-to-Text (STT) capabilities.

## ✨ Features
* **100% Local Inference:** No data leaves your machine.
* **Smart Installer:** Dynamically downloads necessary models and engines upon first launch.
* **Interactive Avatar:** A movable overlay avatar for seamless desktop interaction.
* **Cross-Platform:** Supports Windows, Linux, and macOS.

## 🛠 Prerequisites

### Windows
* [Rust](https://www.rust-lang.org/tools/install)
* Node.js & npm (or pnpm/yarn)
* Microsoft Visual Studio C++ Build Tools

### macOS
No additional system dependencies are required! macOS comes with native WebKit support out of the box. Ensure you have the following installed:
* [Rust](https://www.rust-lang.org/tools/install)
* Node.js & npm
* Xcode Command Line Tools (`xcode-select --install`)

### Linux (Arch-based)
Ensure you have the required WebKit and development dependencies installed:
```bash
sudo pacman -S --needed base-devel curl wget file openssl appmenu-gtk3 gtk3 libappindicator-gtk3 webkit2gtk-4.1
```

#### 🚀 Installation & Build

    Clone the repository:

```bash
    git clone [https://github.com/Rourugin/Fomi-AI-assistant.git](https://github.com/Rourugin/Fomi-AI-assistant.git)
    cd Fomi-AI-assistant
```

    Install frontend dependencies:

```bash
    npm install
```

    Run in development mode:

```bash
    npm run tauri dev
```

    Build for release:

```bash
    npm run tauri build
```

    The compiled binary will be located in src-tauri/target/release/.

## ⚙️ First Launch (Setup)

Upon running Fomi for the first time, the Initial Setup Wizard will automatically appear.
The wizard will check your system for the required components and guide you through downloading:

    LLM: (e.g., Phi-3, Qwen 2.5, or Llama 3)

    Embedder Model: (all-MiniLM-L6-v2)

    Voice Models: Piper TTS voices and Whisper STT models.

Once all components display a green Ready badge, the core AI engine will start automatically.
## 📄 License

This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0). See the LICENSE file for details.