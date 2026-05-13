# Rank

Rank is a graphical visualization and simulation tool for the **Banker's Algorithm** (a resource allocation and deadlock avoidance algorithm used in operating systems). It is built in Rust using the GPUI framework.

## Features

- **Interactive UI**: A clean, table-based interface to manage processes and resource matrices (Allocation, Max, Need, and Available).
- **Deadlock Avoidance Simulation**: Implements the Banker's Algorithm to check system safety and identify safe sequences.
- **Dynamic Updates**: Add processes, modify resources, and dynamically step through the algorithm state.
- **Built with GPUI**: Leverages the high-performance UI framework from the creators of the Zed editor.

## Getting Started

### Prerequisites

You will need the Rust toolchain installed.

### Running the App

Clone the repository and run the application using Cargo:

```bash
git clone https://github.com/HuaGu-Dragon/rank
cd rank
bash ./scripts/bootstrap
cargo run --release
```

## Development

- `src/algo.rs`: Core logic for the Banker's Algorithm.
- `src/table.rs`: UI rendering of the resource table.
- `src/form.rs`: User inputs for new processes and resources.


## Dependencies

- **gpui**: High-performance UI framework for Rust.
- **gpui-component**: UI components for the GPUI framework.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
