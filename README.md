# CKB ALT BN128

This repo implements the following EIPs on CKB-VM:

- https://github.com/ethereum/EIPs/blob/master/EIPS/eip-196.md
- https://github.com/ethereum/EIPs/blob/master/EIPS/eip-197.md

## Usage

```sh
# Clang is required to build and test the code.
$ clang-21 --version

# Ckb debugger is required to test the code in ckb-vm.
# Download it from https://github.com/nervosnetwork/ckb-standalone-debugger/releases.
$ ckb-debugger --version

# Build the code and run tests.
$ make all
```

## Copyright Statement

The project was originally forked from <https://github.com/paritytech/bn>. We made some assembly code performance optimizations based on its original code to optimize its running efficiency on ckb-vm.

## License

MIT OR Apache-2.0.
