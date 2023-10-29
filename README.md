# Rustflix

Rustflix is a command-line tool written in Rust for managing your movie collection. It leverages the power of Rust, Diesel for PostgreSQL database management, and the Clap library for command-line argument parsing.

## Installation

Before using Rustflix, you need to ensure that you have Rust and Diesel set up on your system. Follow the steps below to install the necessary dependencies:

### 1. Install Rust

If you don't have Rust installed, you can do so by following the official installation guide for your platform: [Rust Installation Guide](https://www.rust-lang.org/tools/install).

### 2. Install Diesel CLI and Setup Database

Rustflix uses PostgreSQL as its database. To set up Diesel, follow these steps:

```bash
# Install Diesel CLI
cargo install diesel_cli --no-default-features --features postgres

# Configure the database connection URL (modify it to match your database setup)
export DATABASE_URL=postgres://username:password@localhost/rustflix

# Create the database
diesel database setup
```


### 3. Run the Project on Your Local Machine

### Clone the Rustflix repository
```
git clone https://github.com/your-username/rustflix.git
cd rustflix
```

### Build and install Rustflix as a command-line tool
```
cargo build
cargo install --path .
```

### Usage
```
rustflix --help
```
