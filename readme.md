# scrmbl

a simple file encryption program.

## command structure

```console
cargo run -- {{command}} {{file}} {{password}}
```

## commands

### encrypt

```console
cargo run -- encrypt test\test.txt reallyGoodPassword123
```

Encrypts a file line by line using the password argument.

### decrypt

```console
cargo run -- decrypt test\test.txt reallyGoodPassword123
```

Decrypts a file line by line using the password given. If you use the wrong password it just doesn't decrypt the file.