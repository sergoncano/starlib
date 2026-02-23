# Starlib

Starlib is a library for grid-based TUI game development. It takes care of collisions, rendering layers, creating menus and titles, controlling the flow of events among entities in the game and also provides nice abstractions for certain other aspects of a game.

### Refactor

This branch is a brand new version of the library. I didn't feel like the code in the main branch was maintainable or clean enough for me to publish it as a crate. This branch aims to address the architectural issues in the original one while also adding more functionality. Eventually, it will replace main.

### Roadmap:

- Non-blocking global input .
- Create `InternalEvent` enum and separate it from user-defined enums.
- Create time-based event handling.
- Upload to crates.io
