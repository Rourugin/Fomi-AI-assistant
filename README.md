# AI Assistant 🤖

> ⚠️ **The project is in the early stages of development** - this is still an architectural framework, not a finished product.

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-purple.svg)](https://tauri.app/)
[![Status: Active Development](https://img.shields.io/badge/Status-Active%20Development-blue.svg)](https://github.com/yourname/ai-pc-assistant)

## 🎯 Project philosophy

AI Assistant is a **fully local** alternative to cloud-based assistants. Your data stays on your device, and running on low-end hardware is our priority.

**Key principles:**
- 🔒 **Privacy** — no cloud APIs, everything is local.
- 🖥️ **Efficiency** — optimization for 8GB+ RAM
- 🦀 **Reliability** — Rust core + secure plugin isolation
- 🆓 **Openness** — MIT License, community first

## 🚀 Current status

**The project is under active development.** We are working on creating the core of the system.

### ✅ What's already working:
- Rust + Tauri project framework
- Plugin manifest system with encapsulation
- Basic permissions and security system

### 🔄 What's in development now:
- Plugin manager with loading/unloading
- WASM runtime for plugin isolation
- Voice interface integration

### 📅 Roadmap (High-Level)
1. **Phase 1**: System Framework (plugins, security, UI) — *current*
2. **Phase 2**: AI Foundation (local LLM, STT/TTS models)
3. **Phase 3**: Basic Plugins (PC Management, Games)
4. **Phase 4**: Polishing and Ecosystem


## 🏗️ Архитектура

AI_PC_Assistant/
├── src-tauri/ # Rust-core (Tauri)
│ ├── plugin_system/ # Plugin system
│ ├── ai/ # AI-modules
│ └── voice/ # Voice interface
├── plugins/ # Plugins (WASM/native)
└── models/ # Local AI-models


**Texh stack:** Rust, Tauri, WASM, llama.cpp, whisper.cpp


## 👨‍💻 For developers and observers

The project is at the **architectural design stage**. This means:

### You can:
- Study architectural solutions
- Follow developments through Issues and Commits
- Discuss technical solutions (through Issues)

### It's too early to:
- Install and use
- Submit code without a deep understanding of the architecture
- Wait for a ready-made plugin API

### If you're a Rust developer:
The source code is of interest as an example of building a safe module system in Rust with encapsulation, traits, and a permissions system.

## 📄 License

The project's license has not yet been selected and will be determined at a later stage of development.

## 🔗 Follow the development

- **Repository:** [GitHub](https://github.com/Rourugin/AI-assistant)
- **Discussion:** For now, via repository issues
- **Contacts:** Will be added when the first working kernel is released

---

*This project is under active development. The API and architecture are subject to significant change.*