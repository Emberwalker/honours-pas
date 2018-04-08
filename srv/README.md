# Honours Project Allocation System Backend

## Deploying with Docker

Docker deployment is relatively simple:

1. Build image: `docker build . --tag hpas_backend:latest`
2. Create overlay network: `docker network create hpas`
3. Deploy Postgres database: `docker run -d --name hpas_postgres -e POSTGRES_PASSWORD=postgres --network hpas postgres:10`
4. Create config file and store it somewhere safe (for this example, we'll use `/etc/hpas`).
    - Populate the database URL field with `postgres:POSTGRES_PASSWORD@POSTGRES_CONTAINER/postgres`, replacing `POSTGRES_PASSWORD`
      with the value specified in the `-e` flag of the Postgres container and `POSTGRES_CONTAINER` with the name specified
      in the Postgres container `--name` value.
5. Deploy PAS container: `docker run -d --name hpas_backend --network hpas -v /etc/hpas:/opt/hpas/conf:ro hpas_backend:latest`,
   replacing `/etc/hpas` with your chosen configuration directory, which should contain a `config.toml` file.
    - If deploying the frontend on an existing reverse proxy server rather than using its Dockerfile, you'll need to
      expose the port to the outside world by adding `-p YOUR_PORT_HERE:8080/tcp`, replacing `YOUR_PORT_HERE` with the
      desired external port number. If possible, expose the port to localhost only (by prepending `127.0.0.1:`) or
      firewall it off using your OS firewall (such as `iptables` or `ufw`).
    - For advice on deploying the frontend, see the README in `web`.
6. Add an initial admin user: `docker run -it --rm --network hpas -v /etc/hpas:/opt/hpas/conf:ro hpas_backend:latest add_user --username EMAIL_ADDR --password PASSWORD --name "FULL NAME"`
    - Replace `EMAIL_ADDR` with the administrators email and `FULL NAME` with the administrators full name. `PASSWORD`
      isn't used when using OpenID/Azure AD - it only exists for logging in with `simple` authentication. Use a dummy
      value, but one must be specified.
    - No output will appear if this is successful. This is normal.

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
- Building on Windows also requires OpenSSL, which is just as annoying. For vcpkg:
    1. Install vcpkg as documented, and `vcpkg install openssl:x64-windows`
    2. Edit your user environment variables to have:
        - `VCPKG_ROOT`, set to your vcpkg directory (where vcpkg.exe is located)
        - `VCPKGRS_DYNAMIC` = `1` (enables vcpkg-rs to locate packages such as OpenSSL)

## Updating Notes

- Currently there's a bundled and patched copy of Rocket 0.3.6 in the repo. When Rocket 0.4 arrives, remove it along
  with the `[patch.crates-io]` block in `Cargo.toml`.
  - This was necessary as it depending on an older `cookie` library, which linked to `ring-asm`, which conflicted.