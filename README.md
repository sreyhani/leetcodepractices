## Coverage

- install llvm-cov

```bash
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

cargo binstall cargo-llvm-cov
```

- run tests with coverage

```bash
cargo llvm-cov --html
```
