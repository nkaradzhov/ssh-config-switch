# SSH Config Switch

A tool to manage and switch between different SSH configuration files easily

## Features

-   **List**: Display available SSH configuration profiles
-   **Use**: Activate a selected SSH configuration profile and backup the current one

## Assumptions

1. Location of SSH Profiles:

    - All available SSH configuration profiles should be located in the user's `~/.ssh/` directory

2. Naming Convention:

    - SSH configuration profiles must follow a specific naming pattern: config.\<name>
    - For example, if you have profiles for `work` and `personal` use, they should be named `config.work` and `config.personal`, respectively.

3. Backup:

    - Before switching profiles, the program creates a backup of the current config file, named `config.backup`. This ensures you can restore the previous configuration if needed

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

### Use a Profile

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
