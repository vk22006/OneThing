![Logo for OneThing](assets/header-dark.png)

![Release](https://img.shields.io/github/v/release/vk22006/OneThing)
![Platforms](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-blue)
![GitHub top language](https://img.shields.io/github/languages/top/vk22006/OneThing)
[![License](https://img.shields.io/github/license/vk22006/OneThing)](LICENSE)

---

> A focused productivity app built to help me finish what I start.

## Why OneThing?

I struggled with:

- Starting multiple projects but not finishing them
- Context switching between tools (Productivity and TODO apps)
- Losing track of progress over time

**OneThing** exists to solve a single problem:  
👉 **focus on one project at a time and make visible progress every day.**

This is a personal tool first, built to match my workflow. It also serves as a learning project to enhance my software engineering skills.

![Main screen (initial version)](assets/main_page_1.2.1.gif)

## Core Features

- **Native Desktop Experience**: Advanced window management with a custom splash screen.
- Project-based TODO list  
- Deadlines per project  
- Project ideas + tool notes  
- Daily progress logging  
- Optional daily reminder notification  
- Simple progress visibility over time  
- **Customizable Themes**: Choose from Light, Dark, Baroque Blue, Forest, Celestial Night, Earthy, and Charcoal.
- **Dynamic Design**: High-contrast, focused visual system.

No cloud. No accounts. No distractions.

## Tech Stack

- **Frontend:** SvelteKit + Tailwind CSS
- **Desktop wrapper:** Tauri v2 (Rust)
- **Language:** TypeScript / Svelte / Rust
- **Storage:** Local file storage (Tauri Store API) & localStorage
- **System Integration:** Native desktop notifications

> The app is intentionally built as a web-first application so it can evolve into a desktop app without a rewrite.

## Project Status

✅ **Completed (Maintenance Mode)**

This project successfully met its original design goals of providing a focused, local-first workspace for project-based productivity. It is now considered feature-complete.

---

## Roadmap (High-Level)

- [x] Core project + Daily TODO workflow
- [x] Local persistence
- [x] Progress logging
- [x] Deadline tracking
- [x] Desktop notifications
- [x] **v1.1.2: Native Splash Screen**
- [x] **v1.1.2: Visual Identity Redesign (Red Theme)**
- [x] **v1.1.3: New Themes (Forest, Celestial Night, Earthy, Charcoal)**
- [x] **v1.1.4: Theme Persistence & Modern UI Refresh**
- [x] **v1.2.0: Keyboard-First Workflow**
- [x] **v1.2.1: Project Prioritization**
- [x] Polishing + stability
- [x] **Cross-platform builds (Windows, macOS, Linux)**

## Design Philosophy

- One primary project at a time
- Minimal UI, maximum clarity
- Offline-first
- Local data ownership
- Built for long-term personal use, not virality

## What This Project Demonstrates

- Ability to design software around real personal problems
- Incremental development and scope control
- Frontend state management
- Desktop app architecture with web + native integration

## Download

Pre-built binaries are available on the [Releases page](https://github.com/vk22006/OneThing/releases/latest).

> **Note:** For macOS users, you may need to right-click → Open the first time due to Gatekeeper.

## License

This project is licensed under the GPL-3.0 License — see the [LICENSE](LICENSE) file for details.
