# Feishu-Rust-Client 

[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue)](https://www.rust-lang.org/)
[![Bevy Version](https://img.shields.io/badge/bevy-0.11-brightgreen)](https://bevyengine.org/)
[![License](https://img.shields.io/badge/license-MIT-orange)](LICENSE)

Rust Lark GUI 


### 🖥️ GUI

### Theme

| DarkTheme | LightTheme |
|----------|----------|
| ![dark](./ui/dark.png) | ![light](./ui/light.png) |

```mermaid
graph TD
    A[Root Window] --> B[Left Navbar]
    A --> C[Top Panel]
    A --> D[Main Chat Area]
    A --> E[Bottom Input]
    B --> F[Workspace Selector]
    B --> G[Chat Categories]
    C --> H[Window Controls]
    C --> I[Search Bar]
    D --> J[Message List]
    D --> K[Message Bubbles]
    E --> L[Rich Text Editor]
    E --> M[Attachment Panel]
   

