<p align="center">
  <img src="https://github.com/identicons/app.png" width="100" height="100" style="border-radius: 20%">
</p>

<p align="center">
  <h2 align="center">Favordex: 智能书签管理工具</h2>
  <p align="center">
    基于 <b>Tauri + Rust + Vue 3</b> 构建的 AI 智能书签管理工具。重新定义收藏与整理。
  </p>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Version-1.0.0--BETA-blue?style=for-the-badge" alt="Version">
  <img src="https://img.shields.io/badge/License-MIT-emerald?style=for-the-badge" alt="License">
  <img src="https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux-orange?style=for-the-badge" alt="Platform">
</p>

---

中文 | [English](./README.md)
### 🌟 项目概览

**Favordex** 是一款开源桌面应用，旨在解决浏览器书签乱如麻的痛点。通过集成 AI（OpenAI/Ollama）能力，它能自动对书签进行分类和清理失效链接。

### ✨ 核心特性

- **🤖 AI 智能分类**: 自动识别网页内容，将书签整理至逻辑清晰的文件夹中。
- **🔍 链接健康检查**: 一键分析并移除失效或 404 链接。
- **⚡ 原生性能**: 基于 Tauri 和 Rust 开发，极致低内存占用，响应丝滑。
- **🔒 隐私至上**: API 密钥和书签数据均存储在本地，敏感信息绝不上传至非授权服务器。

### 🛠️ 技术栈

- **Frontend**: Vue 3, Vite, Tailwind CSS, Lucide Icons
- **Backend**: Tauri v2, Rust
- **AI Integration**: OpenAI API / Ollama (Local LLM)

### 🚀 快速上手

#### 前提条件
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Node.js](https://nodejs.org/) (v18+)

#### 安装步骤
如果你只需要使用 Favordex，无需安装开发环境，直接在 [Releases](https://github.com/bao-cn/favordex/releases) 页面下载对应系统的版本包即可。

1. **克隆仓库**
   ```bash
   git clone https://github.com/bao-cn/favordex.git
   cd favordex
   ```
2. **安装依赖**
   ```bash
   pnpm install
   ```
3. **运行应用**
   ```bash
   pnpm tauri dev
   ```
4. **打包构建**
   ```bash
   pnpm tauri build
   ```

### 🤝 参与贡献
开源社区的魅力在于协作与分享。我们非常欢迎并感谢任何形式的贡献：

1. Fork the Project (派生项目)
2. Create your Feature Branch (创建特性分支)
3. Commit your Changes (提交更改)
4. Push to the Branch (推送到分支)
5. Open a Pull Request (开启拉取请求)

### 📄 开源协议
本项目采用 MIT 协议 开源。详情请参阅 [LICENSE](./LICENSE) 文件。

<p align="center">
Developed by <b>Gabriel</b> • Built with 🦀 Rust & 💚 Vue
</p>
