# Honours Project Allocation System Backend

## Building Notes
- The Diesel CLI does _not work properly_ on Windows. Running Diesel CLI from a Windows Subsystem for Linux installation
works fine however. For Windows 10:
    - Install Ubuntu from the Microsoft Store.
    - Launch Ubuntu and configure it appropriately.
    - `sudo apt install libmysqlclient-dev libpq-dev libsqlite3-dev sqlite3 mysql-client postgresql-client`
    - `curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly`
    - Exit and relaunch Ubuntu.
    - `cargo install diesel_cli`
    - `cd /mnt/c/Users/your_user_here/path/to/honours_pas/srv`
    - Use Diesel CLI as normal.