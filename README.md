# 🚀 Rust + Slint UI Demo (Animated Native UI)

A modern desktop UI built in Rust using **Slint**, showcasing smooth animations, glow effects, and SVG-powered visuals — without relying on a browser, Electron, or JavaScript.

## ✨ Features

- Animated UI components
- Time-driven animation system
- Glow effects and modern styling
- Clean separation between Rust logic and `.slint` UI
- SVG-based visuals for crisp scaling

## 🧠 Why This App Is Interesting

This project shows that Rust can power real, polished desktop UIs — not just CLI tools.

## 🏗️ Project Structure

```
src/
 ├── main.rs
ui/
 ├── *.slint
assets/
 ├── *.svg
```

## 🔧 How It Works

Rust drives animation timing:

```
let phase = (start.elapsed().as_secs_f32() / 3.6).fract();
```

Slint handles visuals and reacts to state changes.

## ▶️ How to Run

### Clone

```
git clone https://github.com/feenix100/ruststyle.git
cd ruststyle
```

### Run

```
cargo run
```

## 🧪 Try It

- Change animation timing
- Swap SVG assets
- Modify UI in `.slint` files

## 📸 Screenshot

<!-- Add screenshot here -->

## 📦 Tech Stack

- Rust
- Slint
- SVG

## 📄 License

MIT
