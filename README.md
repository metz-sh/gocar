# Rust Local NPM Registry Emulator

This Rust CLI tool serves as a local npm registry emulator, facilitating package pushing and pulling operations. It enables developers to maintain a local registry of npm packages for testing and development purposes. The tool allows pushing packages to the registry and pulling them for installation.

## Functionality Overview

The tool provides the following functionalities:

- **Push**: Builds the project, packs it into a tarball, and stores it in a designated folder.
- **Pull**: Retrieves the latest version of a package from the local registry and installs it.
- **Pull Version**: Allows selecting and installing a specific version of a package from the local registry.

## Setup

To set up the Rust Local NPM Registry Emulator, follow these steps:

1. **Clone the Repository**: Clone this repository to your local machine.

2. **Build the Project**: Run `cargo build` to build the Rust project.

3. **Set Registry Path**: Ensure that the `REGISTRY_PATH` constant in the code points to the directory where you want to store the local registry.

## Usage

### Push Command

The `push` command builds the project, creates a tarball, and pushes it to the local registry.

```bash
$ cargo run push
```

Options:
- `-s, --skip_build`: Skips the project build step before pushing.

### Pull Command

The `pull` command retrieves the latest version of a package from the local registry and installs it using yarn.

```bash
$ cargo run pull --package_name <package_name>
```

### Pull Version Command

The `pullversion` command allows selecting and installing a specific version of a package from the local registry.

```bash
$ cargo run pullversion --package_name <package_name>
```

## Example

To push the current project to the local registry without rebuilding:

```bash
$ cargo run push -s
```

To pull the latest version of a package named "example-package" from the local registry:

```bash
$ cargo run pull --package_name example-package
```

To pull a specific version of "example-package" from the local registry:

```bash
$ cargo run pullversion --package_name example-package
```

## Dependencies

- `clap`: A command-line argument parser for Rust.
- `dialoguer`: A Rust library for user-friendly command-line interfaces.
