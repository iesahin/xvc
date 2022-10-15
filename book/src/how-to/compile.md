# How to Compile Xvc

## Why would you compile?

- You want to use Xvc on a platform that we don't distribute the binary.
- You want a smaller binary size by removing features that you don't use. 
- You like your software compiled. 
- It's easier to use `cargo` than other means to install for you.
- Fix a bug for yourself.
- Contribute!

## Install Rust

You must have Rust installed on your system. 

If you have a sensible terminal on your system:

```shell
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Otherwise refer to [other installation methods](https://forge.rust-lang.org/infra/other-installation-methods.html) page. 


## Clone the repository

Clone the repository from Emre's Github repository.

```shell
$ git clone https://github.com/iesahin/xvc -b latest
```

The `latest` tag refers to the latest stable release. If you're willing to fight with compilation errors, you can also use `main` branch directly. 

## Compile without default features
