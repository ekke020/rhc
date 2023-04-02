# RHC

## About
RHC is a Rust-based implementation of the popular password cracking tool "[John the ripper](https://github.com/openwall/john)". This program offers two attack modes, namely dictionary and incremental, and features several flags that can be used to speed up the process.

During the development of RHC, I set out to challenge myself and improve my skills in Rust programming and cryptography. The project allowed me to gain valuable experience in areas such as code structure, algorithm optimization, and multi-threading, while also contributing to the broader open-source community.

With that said, I have decided to archive the project for the time being, as I feel it has achieved its intended goals. However, the codebase remains available for anyone interested in exploring the implementation details or using it as a reference for their own projects.

## Usage
To use RHC, simply run the rhc command in your terminal after installing the binary. Here are a few examples to help you get started:

- Bruteforce operation with a specified value defined by the `-t` flag:
  ```bash
  $ rhc -t 78d8045d684abd2eece923758f3cd781489df3a48e1278982466017f
  ``` 
- The same operation as above, but using a list of possible values from a file:
  ```bash
  $ rhc ... -w ./path/to/file.txt
  ``` 
- Note that there are multiple flags available for customization, such as the target range of the password and the algorithm to use.

## Installation
You can download and install the latest version of rhc either by building it from source with make, or by downloading a pre-compiled binary from the [releases](https://github.com/ekke020/rhc/releases) page.

To build the rhc project from source, you will need to have the [Cargo](https://doc.rust-lang.org/cargo/) package manager installed on your system. If you do not have Cargo installed, you can find installation instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

### Building from source
- Clone the repository.
  ```bash
  $ git clone https://github.com/<username>/rhc.git
  ```
- Navigate to the repository.
  ```bash
  $ cd rhc
  ```
- Run make to build and install the project.
  ```bash
  $ make # builds in release mode
  $ make install # installs the binary in ~/.cargo/bin/
  ```
- Make clean to remove the build artifacts.
  ```bash
  $ make clean # Removes artifacts
  ```

### Uninstalling with make
The binary can be uninstalled by calling `make uninstall`, this will delete all artifacts related to the binary. **NOTE** that this does not delete the source files, this has to be done with `make clean`.
