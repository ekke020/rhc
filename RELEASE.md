# Improvements
This release adds support for SHA-1 and makes some efforts to improve the performance of the already existing
SHA-2 implementation. A lot of effort also went into restructuring the algorithm section of the code. This work should help increase the rate at which new algorithms are introduced in the future.

## Bug Fixes
- Fixed a bug that caused the wrong length to be displayed after a successful crack.
## New Features
- The introduction of SHA-1.
## Changes
- The **--algorithm** flag has been updated with the addition of SHA-1. The new algorithm can be used by calling `--algorithm sha1` when running rhc.
## Miscellaneous
- Some performance improvements have been made to the existing implementation of SHA-2.