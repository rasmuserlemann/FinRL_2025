# dev/my_crate

Test project to extract number of BTC transaction over the last 24hr.

---

## Table of Contents

1. [Installation](#installation)

---

## Installation

Follow these steps to install and run the project locally.

### Prerequisites

- **Rust**: Ensure you have Rust installed. If not, install it using [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Clone the repository**: 
  ```bash
    git clone https://github.com/rasmuserlemann/FinRL_2025.git
    cd FinRL_2025
    ```

- **Set up environment variables**: 
  ```bash
    touch .env
    ```
    Add the required environment variables to the .env file
    ```bash
    BTC_RPC_USER=your_username
    BTC_RPC_PASSWORD=your_password
    BTC_RPC_URL=http://127.0.0.1:8332
    ```

- **Build the project**: Use Cargo to build the project:
  ```bash
    cargo build
    ```

- **Run the project**: 
  ```bash
    cargo run
    ```