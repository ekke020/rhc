# Improvements

This release has two primary goals in mind. The first is to improve upon the different strategies available in rhc, including significant performance improvements for the Incremental strategy and the addition of two new charsets. The second goal is to clean up the code structure, with the aim of creating a cohesive and easy-to-follow codebase that will facilitate smoother development in the future.

## Bug Fixes
- Fixed a bug where the wrong help text would display when targeting certain flags.
## New Features
- The **--mode** flag has been added to allow users to specify the desired strategy.
- Added two new charsets in addition to the ASCII-95 set. 
  - The **COMMON** set contains the 77 most common characters.
  - The **NO_SPECIAL** set contains no special characters.
- Added the **--max-length** and **--min-length** flag.
- Modes will now be detected automatically.
## Changes
- The **Bruteforce** strategy was renamed to **Incremental**.
- The password **flag** was renamed to **target**.
## Miscellaneous
- Significant performance improvements when calculating the words in **Incremental** mode.
- Improved the message feedback during runtime.
- Large overhaul of the codebase, improved code quality and introduced a cohesive structure.