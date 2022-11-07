# Alien shooter, Rust version

## Setup

1. [Install Rust](https://www.rust-lang.org/tools/install).
2. Install Rust's cross-compiler for ARM.
    - `rustup target add armv7a-none-eabi`.
3. Make sure you have Xilinx SDK installed.
    - Xilinx SDK is already installed on TC219 machines.
    - More information about installing Xilinx SDK at [exercise guide](https://github.com/RHamalainen/comp.ce.100-rust-exercise-guide).

## Build and run

- Clone the repository, and change working directory.
    - `git clone https://github.com/RHamalainen/alien-shooter-template-rs/`
    - `cd alien-shooter-template-rs`
- Set environment variables that point to Xilinx SDK toolchain.
    - If you use `bash` as a terminal.
        - `source ./scripts/tc219.env`
    - If you use `powershell` as a terminal.
        - `. ./scripts/tc219.ps1`
- Build the binary.
    - `cargo build`
- Run the binary on a connected PYNQ-Z1:
    - Turn the PYNQ's power on.
    - Open a "Xilinx Software Command Line Tool 2017.2"-prompt.
    - Navigate to the project directory.
    - Run `source run_on_pynq.tcl`.
        - This initializes and runs the built program on a connected PYNQ.
- Check board's output.
    - See `Open an input-output interface to the board` at [the online exercise guide](https://github.com/RHamalainen/comp.ce.100-rust-exercise-guide/blob/master/src/2_build-and-run.md) for how to see the output using PuTTY. The program needs to be re-run to get the output to show on the terminal.

## Directory structure and files

|Path|Description|
|---|---|
|`.cargo/config`|A file describing the location and options of the C-linker.|
|`Cargo.toml`|A Rust manifest describing dependencies.|
|`Cargo.lock`|A generated file describing the last successful build configuration. Commit along with changes to dependencies.|
|`pynq/`|Extra files required to cross-compile on PYNQ-Z1 and program the FPGA.|
|`run_on_pynq.tcl`|A `tickle`-script to initialize the FPGA and run the project on PYNQ.|
|`rust-toolchain`|A file to ask `rustup` to use a particular toolchain.|
|`src/`|The Rust source code.|

## Troubleshooting

- `cargo build` fails with can't find crate for `core`.
    * The Rust `core` library cannot be compiled for the target.
    * The likely cause is that the correct component for the target is not added via `rustup`.
    * You can add the component for the Cortex-A cross-compiler with `rustup target add armv7a-none-eabi`.
- `cargo build` fails with "warning: couldn't execute `llvm-config --prefix`".
    * Likely cause: cargo cannot detect the LLVM toolchain, LLVM needs to be installed.
    * LLVM can be installed from https://releases.llvm.org/download.html.
- `cargo build` fails with "cannot detect Xilinx SDK at `C:/Xilinx`".
    * The `libxil` FFI dependency cannot locate the Xilinx toolchain automatically.
    * !?!?!? TODO: OLD INFO. Make sure that Xilinx SDK is installed and set its path using `export XILINX_SDK=/path/to/Xilinx/SDK/version`.
- `cargo build` fails with "error: linker `arm-none-eabi-gcc` not found".
    * The appropriate linker is not found.
    * arm-none-eabi-gcc can be installed on debian (like Ubuntu) using apt/dpkg using `apt-get install gcc-arm-none-eabi`.
    * If the build system cannot find the linker, use `.cargo/config` to point `cargo` to a functioning linker using the key `linker = /location/of/linker-executable`.
        * Xilinx ships with a functioning cross-linker, so if you can find the directory where the SDK is installed, the Windows path to the linker will be `"</location/of/Xilinx>/SDK/<version>/2017.2/gnu/aarch32/nt/gcc-arm-none-eabi/bin/arm-none-eabi-gcc"`.
        * On linux, `nt` will be `lin` instead.
- `cargo build` fails with "error: linker `C:/.../arm-none-eabi-gcc` not found".
    * The GCC linker used for this work is not available at its pre-configured location at `.cargo/config`.
    * See above for a potential solution.
- `source run_on_pynq.tcl` returns "no targets found with ...".
    * Make sure the PYNQ-Z1 is turned on and connected.
