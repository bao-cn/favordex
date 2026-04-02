<p align="center">
  <img src="https://github.com/identicons/app.png" width="100" height="100" style="border-radius: 20%">
</p>

<p align="center">
  <h2 align="center">Favordex: AI-Powered Smart Bookmark Manager</h2>
  <p align="center">
    Built with <b>Tauri + Rust + Vue 3</b>. Redefining how you collect and organize the web.
  </p>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Version-0.1.0--Alpha-blue?style=for-the-badge" alt="Version">
  <img src="https://img.shields.io/badge/License-MIT-emerald?style=for-the-badge" alt="License">
  <img src="https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-orange?style=for-the-badge" alt="Platform">
</p>

---
[中文](./README_CN.md) | English
### 🌟 Overview 

**Favordex** is an open-source desktop application designed to solve the chaos of unorganized browser bookmarks. By leveraging AI (OpenAI/Ollama), it automatically categorizes and cleans your bookmark library.


### ✨ Key Features 

- **🤖 AI Intelligent Categorization**: Automatically identifies webpage content and organizes bookmarks into logical folders.
- **🔍 Link Health Check**: One-click analysis to find and remove broken or 404 links.
- **⚡ Native Performance**: Built on Tauri and Rust, ensuring ultra-low memory usage and blazing-fast response.
- **🔒 Privacy First**: API keys and bookmark data are stored locally. Sensitive info is never uploaded to unauthorized servers.

### 🛠️ Tech Stack 

- **Frontend**: Vue 3, Vite, Tailwind CSS, Lucide Icons
- **Backend**: Tauri v2, Rust
- **AI Integration**: OpenAI API / Ollama (Local LLM)

### 🚀 Getting Started 

#### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Node.js](https://nodejs.org/) (v18+)

#### Installation
If you only want to use Favordex, you can download the pre-built binaries from the [Releases page](https://github.com/bao-cn/favordex/releases).


1. **Clone the repo**
   ```bash
   git clone https://github.com/bao-cn/favordex.git
   cd favordex
   ```
2. **Install dependencies**
   ```bash
   pnpm install
   ```
3. **Run the application**
   ```bash
   pnpm tauri dev
   ```
4. **Build for production**
   ```bash
   pnpm tauri build
   ```

### 🤝 Contributing
Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are greatly appreciated.

1. Fork the Project
2. Create your Feature Branch
3. Commit your Changes
4. Push to the Branch
5. Open a Pull Request

### 📄 License
Distributed under the MIT License. See [LICENSE](./LICENSE) for more information.

<p align="center">
Developed by <b>Gabriel</b> • Built with 🦀 Rust & 💚 Vue
</p>
