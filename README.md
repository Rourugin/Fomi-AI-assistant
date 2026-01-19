<div align="center">

# 🤖 Fomi AI Assistant
### Fully Offline Modular Intelligence

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue?style=for-the-badge&logo=tauri)](https://tauri.app/)
[![License](https://img.shields.io/badge/License-AGPLv3-red?style=for-the-badge)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux-lightgrey?style=for-the-badge&logo=linux)](https://github.com/Rourugin/Fomi-AI-assistant)
[![Model](https://img.shields.io/badge/AI%20Model-Llama%203.2%203B-yellow?style=for-the-badge&logo=meta)](https://huggingface.co/)

*Your private, local, and extensible AI companion.*

[Report Bug](https://github.com/Rourugin/Fomi-AI-assistant/issues) · [Request Feature](https://github.com/Rourugin/Fomi-AI-assistant/issues)

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

## 🔮 Vision & Business Model

We believe that AI should be a tool you *own*, not a service you *rent*. Fomi is designed to be the "Operating System" for your personal intelligence.

### The Philosophy (AGPLv3)
Fomi Core is open-source under the **GNU AGPLv3** license. This ensures that the core technology remains free and open forever. If anyone builds upon Fomi's core to create a service, they must share their improvements back with the community. We prevent corporate "embrace, extend, extinguish" tactics.

### Sustainability & Monetization
To keep development active without selling user data, we plan two revenue streams:

1.  **Fomi Cloud (SaaS)**: While Fomi runs locally, we will offer an optional encrypted cloud sync and heavy-duty inference hosting for low-end devices.
2.  **Verified Plugins Store**: Developers can publish advanced plugins. We review them for security and quality. Revenue is shared (70/30) with plugin creators, fostering a healthy ecosystem where developers get paid for extending Fomi's capabilities.

*The Core will always be free. You pay only for convenience and specialized extensions.*

---

## 🏗️ Architecture Status

| Module | Status | Tech Stack |
| :--- | :--- | :--- |
| **Core Framework** | ✅ Stable | Rust, Tauri 2.0 |
| **Plugin Manager** | ✅ Stable | File System, Serde JSON |
| **AI Engine**      | ✅ Stable | `llama-cpp-2`, GGUF |
| **UI / Frontend**  | 🚧 In Progress | HTML/JS (Later React/Svelte) |

---

## 🛠️ Getting Started

### Prerequisites
* **Rust**: Stable toolchain.
* **C++ Build Tools**: `cmake`, `clang` (required for AI engine compilation).
* **Node.js**: For frontend bundling.

### Installation

1.  **Clone the repo**
    ```bash
    git clone [https://github.com/Rourugin/Fomi-AI-assistant.git](https://github.com/Rourugin/Fomi-AI-assistant.git)
    ```
2.  **Download the Brain**
    * Get `Llama-3.2-3B-Instruct-Q4_K_M.gguf`.
    * Place it in `./models/model.gguf`.
3.  **Run Development Build**
    ```bash
    cargo tauri dev
    ```

---

## 🧩 Roadmap

- [x] **Phase 1: Foundation** (Architecture, File System, Configs)
- [x] **Phase 2: The Brain** (Llama integration, Chat Loop) 
- [ ] **Phase 3: The Body** (Connecting AI to Plugins)
- [ ] **Phase 4: The Face** (Modern UI implementation)

---

## 🤝 Community & Contributing

Join the development discussion!
[**Click here to join our Discord Server**](https://discord.gg/7cDum5pk)

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and guidelines.

---

<div align="center">
    <i>Built with ❤️ in Rust</i>
</div>