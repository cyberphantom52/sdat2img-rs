# sdat2img-rs

`sdat2img-rs` is a Rust implementation of the `sdat2img` tool, which is used to convert Android sparse data files (`*.new.dat` or `*.new.dat.br`) and their associated transfer list files (`*.transfer.list`) into usable system images (`*.img`). This tool is essential for developers and enthusiasts working with Android firmware and custom ROMs.

## Features
- **High Performance**: Leveraging Rust's performance and safety features.
- **Easy to Use**: Simple command-line interface for straightforward conversions.

## Installation

### Prerequisites

- **Rust Toolchain**: Ensure you have Rust installed. You can download it from [rustup.rs](https://rustup.rs/).

### Build from Source

1. **Clone the Repository**

   ```bash
   git clone https://github.com/cyberphantom52/sdat2img-rs.git
   cd sdat2img-rs
   ```

2. **Build the Project**

   Use Cargo to build the executable in release mode:

   ```bash
   cargo build --release
   ```

   The compiled binary will be located at `target/release/sdat2img-rs`.

3. **Install the Binary** (Optional)

   You can install the binary to your system using:

   ```bash
   cargo install --path .
   ```

## Usage

```bash
sdat2img-rs -t <transfer_list> -s <sparse_image> -o <output_image>
```

### Arguments

- `-t`, `--transfer-list`: **(Required)** Path to the transfer list file (`*.transfer.list`).
- `-s`, `--sparse-image`: **(Required)** Path to the sparse data file (`*.new.dat` or `*.new.dat.br`).
- `-o`, `--output`: **(Required)** Path for the output image file (`*.img`).

### Example

```bash
./sdat2img-rs -t system.transfer.list -s system.new.dat -o system.img
```

This command will convert `system.new.dat` using `system.transfer.list` into a usable `system.img` file.

### Handling Compressed `.new.dat.br` Files

If your `.new.dat` file is compressed (ends with `.br`), you need to decompress it first:

```bash
brotli -d system.new.dat.br -o system.new.dat
```

Then run `sdat2img-rs` as usual.

## Understanding Android Sparse Data

### What is Android Sparse Data?

Android Sparse Data is a storage format used by the Android operating system to efficiently represent disk images that contain large amounts of empty (zeroed) space. In many Android system images, particularly those for the `system` and `vendor` partitions, there are extensive regions of unused space or blocks filled with zeros.

Instead of storing these empty blocks explicitly—which would result in large image files with a lot of redundant data—Android uses a sparse representation where only the non-empty (non-zero) data blocks are stored alongside metadata indicating where they should be placed in the full image.

In Android firmware, especially during OTA updates and when flashing new system images, sparse data files are used to optimize the deployment process.

- **Sparse Data File (`*.new.dat`)**: Contains the actual non-zero data blocks that need to be written to specific locations on the partition.
- **Transfer List (`*.transfer.list`)**: Provides instructions on how to reconstruct the full image, detailing where each block from the sparse data file should be placed.

The combination of these two files allows the Android update system to reconstruct the entire filesystem efficiently without having to handle large amounts of empty space. This approach is beneficial for both storage space and reduces the time needed to apply updates.

## License

This project is licensed under the [GPL-3.0 License](LICENSE).

## Contributing

Contributions are welcome! If you have suggestions, bug reports, or patches, feel free to contribute to the project.

## Acknowledgements

- Inspired by the original [`sdat2img.py`](https://github.com/xpirt/sdat2img) script by [xpirt](https://github.com/xpirt).
- Thanks to the Rust community for their support and contributions.
