# efivar-rs

[![Build Status](https://github.com/iTrooz/efivar-rs/actions/workflows/build.yml/badge.svg)](https://github.com/iTrooz/efivar-rs/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/badge/docs-main-blue.svg)](https://docs.rs/efivar/)
[![codecov](https://codecov.io/gh/iTrooz/efivar-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/iTrooz/efivar-rs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

This repository contains the source code for:

* [![crates.io](https://img.shields.io/crates/v/efivar.svg)](https://crates.io/crates/efivar) [efivar](efivar) - A Rust crate to read and write EFI variables
* [![crates.io](https://img.shields.io/crates/v/efivarcli.svg)](https://crates.io/crates/efivarcli) [efivarcli](efivarcli) - A command-line tool to manage the UEFI boot manager

The efivarcli tool supports both Windows and Linux:

* Windows: administrative rights are required to both *read* and *write* variables.
* Linux: the efivar filesystem should be mounted at /sys/firmware/efi/efivars on all major
  distros. With the default settings, standard users should be able to read
  variables, while writing to variables requires being root.

## Development status

***This project is still under heavy development. Its public interface should
not be considered stable.**

## Disclaimer

**Altering your firmware's EFI variables is a potentially dangerous action, as
it may prevent your computer from booting if some vendor-specific variables are
deleted, or if your firmware does not implement the UEFI specification
correctly. You are solely responsible for determining whether this tool is
compatible with your equipment and other software installed on your equipment.
You are also solely responsible for the protection of your equipment and backup
of your data, and the maintainers of this project will not be liable for any
damages you may suffer in connection with using, modifying, or distributing this
tool.**

## References

- [The UEFI specification, version 2.7](http://www.uefi.org/sites/default/files/resources/UEFI_Spec_2_7.pdf)
- [efibootmgr](https://github.com/rhboot/efibootmgr)
- [efivars and efivarfs](https://blog.fpmurphy.com/2012/12/efivars-and-efivarfs.html)

## Team

### Maintainer
iTrooz

### Original author

Alixinne <alixinne@pm.me>
