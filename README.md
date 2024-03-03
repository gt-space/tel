# TEL-software
Software for the TEL board

## Instructions
### Dependencies
On an Ubuntu-based development system, do the following setup steps:

1. **Rust Install**

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

    _Note: if you have installed Rust through `apt`, the below commands may not work. Use rustup instead._

2. **Rust cross compilation tools**

    1. Install [Cross](https://github.com/cross-rs/cross)
        ```sh
        cargo install -f cross
        ```

    2. then support for compiling to BeagleBone Black

        ```sh
        rustup target add armv7-unknown-linux-musleabihf    # MUSL libc
        rustup target add armv7-unknown-linux-gnueabihf     # GNU libc
        ```

3. **GCC ARM Cross-Compiler**

    ```sh
    sudo apt install gcc-arm-linux-gnueabihf
    ```

### Building
#### For BeagleBone Black

1. **Run Build**

    These two commands will build with two different implementations of the standard library. The
    `-gnueabihf` build is what the flight-computer code uses, but because the library is
    dynamically linked your host system may have a version incompatible with the BeagleBone's.

    The `-musleabihf` build will statically link the MUSL libc implementation, so it avoids the
    version mismatch issue.

    ```sh
    cargo build --target armv7-unknown-linux-musleabihf     # Will statically link MUSL libc
    cargo build --target armv7-unknown-linux-gnueabihf      # Will dynamically link glibc
    ```

2. **Deploy to BeagleBone**

    ```sh
    ./deploy.sh [lib] beaglebone.local            # For a tethered beaglebone
    ```

    where `[lib]` is either `musl` or `gnu`, depending on which build you wish to deploy.

### Testing
#### For BeagleBone Black

1. **Build Tests**

    Differences between the 2 implementations are outlined above in [Building](#building). Running the command will output
    the path to the test executable.

    ```sh
    cargo test --no-run --target armv7-unknown-linux-musleabihf     # Will statically link MUSL libc
    cargo test --no-run --target armv7-unknown-linux-gnueabihf      # Will dynamically link glibc
    ```

2. **Deploy to BeagleBone**

    ```sh
    scp [test executable path] debian@[beaglebone hostname]:~/tel-sw-tests            # For a tethered beaglebone
    ```

3. **Running**

    SSH into the beaglebone and run the following command. Sudo is required for tests involving GPIO pins.

    ```sh
    sudo ./tel-sw-tests
    ```