# Web-docs for the `wivir` project:

## The concept

For now, just an idea. Hopefully I get some time and build the knowledge to make
it into a full-fledged network manager for Linux through the terminal :)

---

## Dependencies / Requirements:
- `NetworkManager` as a network daemon.
- A Linux system.

---

## Getting started ↓

1. Clone the repository onto your system and move to the cloned repo:

```bash
git clone https://github.com/mrs4ndman/wivir.git
cd wivir
```

2. Build the executable with `cargo`:

```bash
cargo build
```

- For slightly better perfomance, use the `--release / -r` flag:
  ```bash
  cargo build --release
  ```

3. Move the executable found in `target/debug/wivir` (or
   `target/release/wivir`) to a directory that is included in your `$PATH`
   environment variable (check your path inclusion with the command `echo
   $PATH`):

- For regular compilation:
```bash
cp target/debug/wivir <directory in path>
```

- For optimized compilation:
```bash
cp target/release/wivir <directory in path>
```

4. Open a new terminal and execute the `wivir` command:

```bash
wivir
```




