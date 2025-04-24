# Embedded Study

This repository serves as a hands-on study of embedded ARM development in Rust. It demonstrates how to generate Peripheral Access Crates (PACs) for STM32 microcontrollers by converting SVD descriptions into type-safe, idiomatic Rust APIs.

---

## 📋 Table of Contents

- [Prerequisites](#-prerequisites)
- [Installation](#-installation)
- [Generating PACs](#-generating-pacs)
- [Using Generated PACs](#-using-generated-pacs)
- [Project Structure](#-project-structure)

---

## 🔧 Prerequisites

Before you begin, ensure you have the following:

- **Rust and Cargo**: Install via [rustup](https://rustup.rs/).
- **Git**: For cloning repositories and version control.
- **CMSIS-SVD Files**: Download the `.svd` files for your target STM32 series from STMicroelectronics.

---

## 🚀 Installation

1. **Clone this repository**

   ```bash
   git clone https://github.com/dgaraujo89/embedded.git
   cd embedded
   ```

2. **Fetch the `cmsis-svd-stm32` tool**

   ```bash
   git clone https://github.com/modm-io/cmsis-svd-stm32.git
   ```

## 🛠️ Generating PACs

1. **Run the generator**

   Use the `svd2rust` binary to produce PAC crates:
   
   ```bash
   mkdir pacs/stm103_pac
   svd2rust -i /cmsis-svd-stm32/STM32F103.svd -o pacs/stm103_pac
   ```

    For more details about `svd2rust` access the documentation [here](https://docs.rs/svd2rust/latest/svd2rust/).

---

## 📦 Using Generated PACs

To include a generated PAC in your embedded project:

1. Add a path dependency in your `Cargo.toml`:

   ```toml
   [dependencies]
   stm103_pac = { path = "../pacs/stm103_pac" }
   ```

2. In your code, import and initialize the peripheral:

   ```rust
   use stm103_pac::Peripherals;

   fn main() {
       let dp = Peripherals::take().unwrap();
       // Access registers, e.g., dp.RCC.apb1enr.modify(|_, w| w.tim2en().enabled());
   }
   ```

---

## 🗂️ Project Structure

```
embedded-study/
│
├── pacs/                   # Output directory for generated PAC crates
│   ├── stm103_pac/
│   └── ...
└── stm103-study            # Study project for stm103
├── README.md               # This file
└── LICENSE
```
