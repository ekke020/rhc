# RHC

## About
RHC is an implementation of "[John the ripper](https://github.com/openwall/john)" written in Rust. Currently it supports a lot less features than its corresponding C implementation. It's a hoby project that I have created to get a better understanding of the Rust programming language. The current build is able to crack roughly *100.000* passwords a second on a i5-10600K.

## Usage
To use RHC, simply run the rhc command in your terminal after installing the binary. Here are a few examples to help you get started:

- Bruteforce operation with a specified value defined by the `-p` flag:
  ```bash
  $ rhc -p 78d8045d684abd2eece923758f3cd781489df3a48e1278982466017f
  ``` 
- The same operation as above, but using a list of possible values from a file:
  ```bash
  $ rhc ... -w ./path/to/file.txt
  ``` 
- Note that there are multiple flags available for customization, such as the target range of the password and the algorithm to use.

## Installation
You can either build directly from source with make or download the latest version from [releases](https://github.com/ekke020/RHC).

To build the rhc project from source, you will need to have the [Cargo](https://doc.rust-lang.org/cargo/) package manager installed on your system. If you do not have Cargo installed, you can find installation instructions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

### Building from source
- Clone the repository
  ```bash
  $ git clone https://github.com/<username>/rhc.git
  ```
- Navigate to the repository
  ```bash
  $ cd rhc
  ```
- Run make to build and install the project
  ```bash
  $ make # builds in release mode
  $ make install # installs the binary in ~/.cargo/bin/
  ```
- Make clean to remove the build artifacts
  ```bash
  $ make clean # Removes artifacts
  ```

### Uninstalling with make
The binary can be uninstalled by calling `make uninstall`, this will delete all artifacts related to the binary. **NOTE** that this does not delete the source files, this has to be done with `make clean`.

## Planned algorithm support
rhc currently only supports the SHA-2 algorithm family. The plan is to expand the algorithm support in the future to support the listed algorithm families.

- **SHA-1**: A widely used hashing algorithm that is considered insecure due to weaknesses found in the algorithm.
- **bcrypt**: A password hashing function that uses the Blowfish cryptographic algorithm and is designed to be slow and computationally expensive making it more resistant to brute-force attacks.
- **scrypt**: A password-based key derivation function that is designed to be more secure against hardware brute-force attacks.
- **whirlpool**: A cryptographic hash function that takes an input (or 'message') and returns a 512-bit (64-byte) hash value, it is considered to be very secure.

## Planned flags
There are only a handful flags available at the moment but I plan to add more in the near future. Below is a list of some of the flags that I intend to add.


Specify the maximum length of the words to generate when cracking a hash, should work in conjunction with the `--length` flag to target a length-range of words.
```bash
rhc [OPTIONS]... --max-length [unsigned integer]
``` 
Specifies the number of threads to utilize while cracking.
```bash
rhc [OPTIONS]... --threads [unsigned integer]
``` 
<s>Specify a wordlist file to use while cracking.</s>
```bash
rhc [OPTIONS]... --wordlist [./path/to/wordlist]
``` 
Specify an output file to save the result of the crack.
```bash
rhc [OPTIONS]... --output [./path/to/outputfile]
``` 
Suppresses all output except the final result. Could be useful if the tool is run as part of a script.
```bash
rhc [OPTIONS]... --quiet
``` 