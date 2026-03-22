# Contributing to Fomi AI

Thank you for your interest in Fomi AI! We are building a private, modular, and "living" AI assistant, and we love community help.

## Development Stack
- **Backend:** Rust, Tauri v2, llama-cpp-2.
- **Frontend:** Vanilla JS, HTML, CSS.
- **AI:** Local LLMs (GGUF format).

## How to Get Started
1. **Fork the Repository:** Create your own copy of the project.
2. **Setup Environment:** - Install Rust and Cargo.
   - Install Node.js (if needed for frontend dependencies).
   - Prepare the `models/` directory with the required files:
     - `models/model.gguf` (Core LLM, e.g., Mistral 7B).
     - `models/embedder_model.gguf` (FastEmbed `all-MiniLM-L6-v2`).
     - `models/voice/stt/whisper.bin` (Whisper STT).
     - `models/voice/tts/*.onnx` and `*.json` (Piper TTS).
3. **Run in Dev Mode:**
   ```bash
   cargo tauri dev

## Contribution Rules

1. **Language:** All code, comments, and commit messages must be in English.

2. **Architecture:** - Keep the core (Rust) lightweight.

        - Business logic for features should be implemented as Plugins.

        - Frontend should remain simple and performant.

3. **Privacy First:** Never add features that require an internet connection without explicit user permission and a "Local First" alternative.

## Style Guide

-   **Rust:** Use cargo fmt before submitting.

-   **CSS:** Use variables for colors (refer to main.css).

-   **Commits:** Use Conventional Commits (e.g., feat: add plugin manager, fix: window dragging).

## Questions?

    Join our [Discord Server](https://discord.gg/7cDum5pk) or open a GitHub Issue.