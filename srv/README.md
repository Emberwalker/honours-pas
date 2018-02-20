# Honours Project Allocation System Backend

## Building Notes
- Building on Windows is all sorts of wonky due to Diesel/libpq. There's two fixes; copying DLLs into the working dir,
or just giving up and using Windows Subsystem for Linux to build, run and use Diesel CLI instead.
    - Copy in DLLs route:
        1. Get the required libraries.
            - This could mean installing the official EnterpriseDB distro of Postgres ("Command line tools" option) or
            using Microsoft's `vcpkg` to download and build libpq. You could also compile it manually, probably. I
            recommend using `vcpkg`, however.
            - If using `vcpkg`, make sure to add `$VCPKG_ROOT/installed/x64-windows/lib` to your `LIB` env var.
        2. Copy the matching DLLs for `libeay32`, `ssleay32` and `libpq` into this directory.
            - For `vcpkg`, these are in `$VCPKG_ROOT/installed/x64-windows/bin`.
        3. Use Diesel CLI and Cargo as normal.
    - Windows Subsystem for Linux route:
        1. Install Ubuntu from the Microsoft Store.
        2. Launch Ubuntu and configure it appropriately.
        3. `sudo apt install libmysqlclient-dev libpq-dev libsqlite3-dev sqlite3 mysql-client postgresql-client`
        4. `curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly`
        5. Exit and relaunch Ubuntu.
        6. `cargo install diesel_cli`
        7. `cd /mnt/c/Users/your_user_here/path/to/honours_pas/srv`
        8. Use Diesel CLI and Cargo as normal in a WSL session.
            - If generating the schema with `diesel print-schema`, remember to run `unix2dos` on the resulting file.
            `unix2dos` can be installed with `sudo apt install dos2unix` (yes, the name is flipped).