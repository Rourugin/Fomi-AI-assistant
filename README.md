<div align="center">

# 🤖 Fomi AI Assistant
### Fully Offline Modular Intelligence

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue?style=for-the-badge&logo=tauri)](https://tauri.app/)
[![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux-lightgrey?style=for-the-badge&logo=linux)](https://github.com/Rourugin/AI-assistant)
[![Model](https://img.shields.io/badge/AI%20Model-Llama%203.2%203B-yellow?style=for-the-badge&logo=meta)](https://huggingface.co/)

*Your private, local, and extensible AI companion.*

[Report Bug](https://github.com/your-username/fomi/issues) · [Request Feature](https://github.com/your-username/fomi/issues)

</div>

---

## 🚀 About The Project

**Fomi** is a local assistant built to respect your privacy. Unlike cloud AI (ChatGPT, Claude), Fomi runs **entirely on your hardware**. It uses a plugin system to interact with your OS, allowing it to perform real tasks, not just chat.

### 🌟 Key Features
* 🧠 **Local Brain**: Powered by `Llama-3.2-3B` (GGUF), running on CPU.
* 🔌 **Plugin System**: Modular architecture using `Mutex` for thread-safe management.
* 💾 **Persistence**: Remembers your settings and active plugins across restarts.
* 🛡️ **Privacy First**: Zero data leaves your machine.

---

## 🏗️ Architecture Status

| Module | Status | Tech Stack |
| :--- | :--- | :--- |
| **Core Framework** | ✅ Stable | Rust, Tauri 2.0 |
| **Plugin Manager** | ✅ Stable | File System, Serde JSON |
| **AI Engine** | 🚧 In Progress | `llama-cpp-2`, GGUF |
| **UI / Frontend** | 🔄 Planned | HTML/JS (Later React/Svelte) |

---

## 🛠️ Getting Started

### Prerequisites
* **Rust**: Stable toolchain.
* **C++ Build Tools**: `cmake`, `clang` (required for AI engine compilation).
* **Node.js**: For frontend bundling.

### Installation

1.  **Clone the repo**
    ```bash
    git clone [https://github.com/your-username/fomi.git](https://github.com/your-username/fomi.git)
    ```
2.  **Download the Brain**
    * Get `Llama-3.2-3B-Instruct-Q4_K_M.gguf`.
    * Place it in `src-tauri/models/model.gguf`.
3.  **Run Development Build**
    ```bash
    cargo tauri dev
    ```

---

## 🧩 Roadmap

- [x] **Phase 1: Foundation** (Architecture, File System, Configs)
- [ ] **Phase 2: The Brain** (Llama integration, Chat Loop) 
- [ ] **Phase 3: The Body** (Connecting AI to Plugins)
- [ ] **Phase 4: The Face** (Modern UI implementation)

---

<div align="center">
    <i>Built with ❤️ in Rust</i>
</div>