# NJSC
## New Java Serial Connector

### Summary

This project will provide a Java library which provides serial port connectivity. The project will be a spiritual
successor to the jSSC project. It will be compatible with Java 1.5 and will use standardized interfaces similar to other
Java connectors. It will ship multiple variants: a normal standalone JAR and an OSGi bundle JAR. If appropriate, it will
also ship a Java 1.8 version of each variant which utilizes newer interfaces.

Native code will be written in Rust and packaged in the JAR (so that it does not need to be pre-installed on the target
system). For now, this might limit the potential compile targets, but Rust will quickly expand into the embedded devices
world.

Note to self: check out [RustJNI](https://github.com/Monnoroch/RustJni).

### Install Tools

Install the rust compiler. For a full installation guide, click [here](https://doc.rust-lang.org/stable/book/installing-rust.html),
or just run the following command: `curl -sf -L https://static.rust-lang.org/rustup.sh | sh`.

Install a JDK, if you don't already have one installed.

### Compile and Run

In the `njsc` directory, which has the `Cargo.toml` file, run `cargo build --release`. Compile and run the Java file.
