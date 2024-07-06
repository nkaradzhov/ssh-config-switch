# SSH Config Switch

A tool to manage and switch between different SSH configuration files easily.

## Features

-   **List**: Display available SSH configuration profiles.
-   **Set**: Activate a selected SSH configuration profile and backup the current one.

## Installation

1. **Clone the repository**:

    ```sh
    git clone https://github.com/nkaradzhov/ssh-config-switch.git
    ```

2. **Navigate to the project directory**:

    ```sh
    cd ssh-config-switch
    ```

3. **Install**

    ```sh
    cargo install --path .
    ```

## Usage

### List Available Profiles

List all available SSH configuration profiles in the `~/.ssh/` directory.

```sh
ssh-config-switch list
```

### Set a Profile

Set a specified SSH configuration profile as the active one. The tool backs up the current `config` file to `config.backup`.

```sh
ssh-config-switch use <profile-name>
```

Replace `<profile-name>` with the name of the desired profile. For example, if you have a configuration file named `config.work` in your `.ssh` directory, use `work` as the `<profile-name>`.

### Example

To switch to the SSH configuration named `work`:

```sh
ssh-config-switch use work
```

## Contributing

Contributions are welcome! Please submit issues or pull requests for improvements or new features.

## License

This project is licensed under the MIT License.

---

Feel free to adjust the repository URL, instructions, or any other details to better suit your specific project setup.
