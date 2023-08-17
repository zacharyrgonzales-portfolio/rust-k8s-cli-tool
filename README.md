# Kubernetes Namespace Viewer

## Overview

The Kubernetes Namespace Viewer is a command-line tool written in Rust that allows users to view all resources in a specific Kubernetes namespace. It leverages the `kube` crate to interact with the Kubernetes API and provides a simple and intuitive interface for querying resources.

## Features

- List all Pods in a specified namespace.
- Easy to use command-line interface.
- Extensible design for adding support for other resource types.

## Prerequisites

- Rust 1.40 or higher
- Access to a Kubernetes cluster (v1.20 or higher recommended)

## Installation

Clone the repository and navigate to the project directory:
git clone https://github.com/zacharyrgonzales-portfolio/rust-k8s-cli-tool.git
cd k8s-namespace-viewer

Build the project:
cargo build --release

The binary will be available in the `target/release` directory.

## Usage

To view all resources in a specific namespace, run:
cargo run -- <namespace>

Replace `<namespace>` with the name of the namespace you want to view.

## Dependencies

- `kube`: For interacting with the Kubernetes API.
- `clap`: For building the command-line interface.

## Contributing

Contributions are welcome! Please read the [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Support and Contact

If you encounter any issues or have questions, please open an issue or contact the maintainer at zacharyrgonzales.azure@gmail.com
