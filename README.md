# Rust Local NPM Registry Emulator

This Rust CLI tool serves as a local npm registry emulator, facilitating package pushing and pulling operations. It enables developers to maintain a local registry of npm packages for testing and development purposes. The tool allows pushing packages to the registry and pulling them for installation.

## Functionality Overview

The tool provides the following functionalities:

- **Push**: Builds the project, packs it into a tarball, and stores it in a designated folder.
- **Pull**: Retrieves the latest version of a package from the local registry and installs it.
	- Run a "postinstall" script
- **Pull Version**: Allows selecting and installing a specific version of a package from the local registry.

## Setup

To set up the Rust Local NPM Registry Emulator, follow these steps:

1. **Clone the Repository**: Clone this repository to your local machine.

2. **Build the Project**: Run `cargo build` to build the Rust project.

## Usage

### The config file
`gocar` requires a `.gocar.json` file to be present. And in this file it
expects at least the `registry` field to be pointing towards a path you will
be using as a store.
Here's a sample `.gocar.json`:
```json
{
	"registry": "/Users/<your-user-name>/.metz-registry",
	"postinstall": {
		"simulacrum": "cp -r node_modules/@metz/simulacrum/dist/assets public"
	}
}
```

### Push Command

The `push` command builds the project, creates a tarball, and pushes it to the local registry.

```bash
$ cargo run push
```

Options:
- `-s, --skip-build`: Skips the project build step before pushing.

### Pull Command

The `pull` command retrieves the latest version of a package from the local registry and installs it using yarn.

```bash
$ cargo run pull --package-name <package_name>
```

Options:
- `--package-name <package_name>`: The package you want to pull
- `-n, --no-copy`: By default, the tarball is copied and then installed. With this option, it will be installed directly from registry.
Meaning, your lockfiles and `package.json` will be referencing the local registry.

You can also configure through `.gocar.json` to run a command after you pull
a package. In the file, add an entry under the key `"postinstall"` such that
the key matches the package name and the value is the command. For example:
```json
"postinstall": {
	"your-package": "ls node-modules/your-package"
}
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
