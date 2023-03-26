# Improvements
In this release, the major goal was to improve the user feedback when running the application. This was achieved by implementing a central manager that controls all outgoing messages that the user sees. Each thread now has a direct line to the manager, where it can send different updates and messages. The manager then decides when and how to present this information to the user, resulting in a smoother and more informative experience.

## Bug Fixes
- Fixed a bug that caused certain flags to not trigger properly.
- Fixed a bug that presented the wrong version when running the help command.
- Fixed a bug that would cause a crash if an argument flag was misconfigured.
## New Features
- The **--charset** flag has been added to allow users to specify one of three charsets.
- The **--thread-count** flag has been added to allow users to have full control over thread consumption.
- The **--quiet** flag has been added. This flag will remove all non-essential output.
## Changes
- The verbose flag has been removed in this release. Instead, verbose output is now toggled by default, allowing users to opt-out with the --quiet flag.

## Miscellaneous
- Major improvements have been made to the user feedback. This includes better output and more available information abut an ongoing crack.