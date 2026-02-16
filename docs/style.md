# Style Guide & Color Palette

The BASTION palette conveys **military-grade security, speed, and modern technology**. It combines the solidity of dark backgrounds (the data abyss) with neon accents that represent the flow and transformation of clean data.

## Color Palette Overview

### Base Colors (Backgrounds & Structure)

Used for the application's background, terminal UI, and control panels. They provide a native, enterprise-grade "Dark Mode" feel.

| Color               | Hex       | RGB          | Primary Use                                      |
| ------------------- | --------- | ------------ | ------------------------------------------------ |
| ⬛ **Abyss Black**  | `#0B101E` | `11, 16, 30` | Main app/web background, terminal background.    |
| 🟦 **Bastion Navy** | `#16203A` | `22, 32, 58` | Cards, secondary panels, and modals.             |
| 🔲 **Slate Gray**   | `#2A3655` | `42, 54, 85` | Borders, dividers, and inactive/disabled states. |

### Accent Colors (The Identity Gradient)

These are your brand colors. The logo's gradient flows from Cyan (fast ingestion) to Purple (Python transformation).

| Color                | Hex       | RGB            | Primary Use                                                          |
| -------------------- | --------- | -------------- | -------------------------------------------------------------------- |
| 💠 **Cyber Cyan**    | `#00D4FF` | `0, 212, 255`  | Primary buttons, active links, Rust components, data ingestion flow. |
| 🔮 **Worker Purple** | `#8A2BE2` | `138, 43, 226` | Hover effects, Python worker icons, transformation states.           |
| 🔷 **Electric Blue** | `#2D5BFF` | `45, 91, 255`  | Mid-point color for charts, graphs, and gradients.                   |

### Semantic Colors (The Firewall)

Crucial for a Data Gateway. Users need to know in a millisecond if a payload passed, was blocked, or was sent to quarantine.

| Color             | Hex       | RGB            | Meaning in BASTION                                                         |
| ----------------- | --------- | -------------- | -------------------------------------------------------------------------- |
| 🟢 **Gold Layer** | `#00FF9D` | `0, 255, 157`  | **Success:** Valid data, correct schema, saved to Parquet.                 |
| 🔴 **Drop Red**   | `#FF3366` | `255, 51, 102` | **Blocked:** Invalid data, attack detected, connection dropped.            |
| 🟡 **Quarantine** | `#FFBE0B` | `255, 190, 11` | **Warning:** Data sent to Dead Letter Queue (DLQ) or slow worker detected. |

### Typography (Text)

| Color              | Hex       | RGB             | Primary Use                                                 |
| ------------------ | --------- | --------------- | ----------------------------------------------------------- |
| ⬜ **Clean White** | `#F0F4F8` | `240, 244, 248` | Primary text, headings (H1, H2), maximum readability.       |
| 🌫️ **Muted Steel** | `#94A3B8` | `148, 163, 184` | Secondary text, descriptions, placeholders, subtle metrics. |
