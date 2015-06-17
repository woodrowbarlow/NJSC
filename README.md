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

### Building from Source

#### Tools Required

* Compatible Operating System Libraries. NJSC is currently designed to work with the Serial libraries provided with *nix (Linux, Unix, and OSX) operating systems and those with the Windows operating system (more version information to come). Other operating systems will work if they have compatible system calls (more specifics to come).
* [Java Compiler](http://openjdk.java.net/). I'm using the OpenJDK, version 1.8. Another option is the Oracle JDK binaries. Your compiler must support Java 1.5 compliance. Java is a high-level object-oriented programming language which runs on the Java Virtual Machine in order to create compile-once, run-anywhere portable binaries. NJSC uses Java to provide the front-end interfaces and API for the library.
* [Rust Compiler](http://www.rust-lang.org/). I'm using the official compiler, version 1.0.0. Rust is a systems programming language that runs blazingly fast, prevents nearly all segfaults, and guarantees thread safety. NJSC usrs Rust to provide the native interaction with the operating system's serial port APIs.
* A Dynamic Library Linker, such as is provided with the [GNU Compiler Collection](https://gcc.gnu.org/) is needed in order to create native dynamic library files (`.dll` files on Windows, `.so` files on Linux, and `.dylib` files on OSX).
* A Makefile buildsystem, such as [GNU Make](http://www.gnu.org/software/make/). Makefiles are used to define the build process, and a buildsystem is used to execute the various build tasks with a single command.
* [Git Version Control](https://git-scm.com/) (optional, for contributing). Git is a distributed version control system. Git will allow you to easily clone this project and is the accepted method of contributing to the project.
* [LLVM Backend](http://llvm.org/) (optional, for cross-compiling). If you plan to use this library on a non-standard CPU architecture that is not supported out-of-the-box by the Rust compiler, you will need to use an LLVM backend to generate cross-compiled code.

#### Suggested Linux Installation Instructions

1. Install OpenJDK 1.8. ([Installation Guide](http://openjdk.java.net/install/index.html))
   * For RPM-based Distributions:
     * `su -c "yum install java-1.8.0-openjdk-devel"` (for yum package manager)
     * `su -c "dnf install java-1.8.0-openjdk-devel"` (for dnf package manager)
   * For Debian-based Distributions:
     * `su -c "apt-get install openjdk-8-jdk"`
2. Install the Rust Compiler 1.0.0. ([Installation Guide](https://doc.rust-lang.org/stable/book/installing-rust.html))
   * `su -c "curl -sf -L https://static.rust-lang.org/rustup.sh | sh"`
3. Install the GNU Compiler Collection. ([Installation Guide](https://gcc.gnu.org/install/))
   * For RPM-based Distributions:
     * `su -c "yum install gcc"` (for yum package manager)
     * `su -c "dnf install gcc"` (for dnf package manager)
   * For Debian-based Distributions:
     * `su -c "apt-get install gcc"`
4. Install Git Version Control. ([Installation Guide](https://git-scm.com/download/linux))
   * For RPM-based Distributions:
     * `su -c "yum install git"` (for yum package manager)
     * `su -c "dnf install git"` (for dnf package manager)
   * For Debian-based Distributions:
     * `su -c "apt-get install git"`
5. Clone the Project. ([Git Basics](https://git-scm.com/book/en/v2/Git-Basics-Getting-a-Git-Repository#Cloning-an-Existing-Repository))
   * While in the directory where you want to keep git repositories:
     * `git clone https://github.com/woodrowbarlow/NJSC`

### Compile and Run

(a better build system is coming)
In the `njsc` directory, which has the `Cargo.toml` file, run `cargo build --release`. Compile and run the Java file.
