# 🚑 HealthCLI

Manage and perform health checks for your system with style and simplicity!

## 🌟 Features

*   🔍 Run health checks for your system components.
*   📝 Register new health checks tailored to your needs.
*   📋 List all registered health checks.
*   🔧 Flexible filters by name or type.

## 🚀 Installation

1.  Clone the repository:

    `git clone https://github.com/Trusted97/healthcli.git`

2.  Build the project:

    `cargo build --release`

3.  Run the binary:

    `./target/release/healthcli`


### 📖 Usage

HealthCLI provides three main commands: `check`, `register`, and `list`.

### 1\. Run Health Checks (`check`)

Run all health checks, or filter by name or type:

healthcli check \[OPTIONS\]

#### Options:

*   `-n, --name <NAME>`: Run a specific health check by its name.
*   `-t, --check-type <TYPE>`: Run checks of a specific type (e.g., `url`, `database`, `disk`).

#### Examples:

*   Run all checks: `healthcli check`

*   Run a specific check: `healthcli check --name "Ping Google"`

*   Run checks of type `url`: `healthcli check --check-type url`


### 2\. Register New Health Checks (`register`)

Add a new health check:

``` bash
  healthcli register --name <NAME> --check-type <TYPE>
```

#### Examples:

*   Register a new URL check:
``` bash
  healthcli register --name "Ping Google" --check-type url
```

*   Register a disk space check: 

``` bash
  healthcli register --name "Check Disk Space" --check-type disk
```


### 3\. List All Registered Checks (`list`)

Display all registered health checks: `healthcli list`


## 🤝 Contributing

We welcome contributions to improve HealthCLI! Feel free to submit issues or pull requests to make this tool even better. 💪


## 📜 License

This project is licensed under the [MIT License](LICENSE).