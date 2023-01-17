# RHC

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
Specify a wordlist file to use while cracking.
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