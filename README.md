# Starlib
Starlib is a library for grid-based TUI game development. It takes care of collisions, rendering layers, creating menus and titles, controlling the flow of events among entities in the game and also provides nice abstractions for certain other aspects of a game.  

### Quickstart
Import the neccessary modules, create a `Planet` (which is essentially the grid the game is played in) and add in the decorations and the colliders in the constructor. The function `collider_vector_from_map` is quite useful for this.
Create any entities needed implementing the `EventDriven` trait. Create a `Level` with the planet and the entities in a vector.
Then it's done, whenever you run `level.game_loop()` the created stage will run. Menus and titles may be used for extra flavor. 
  
### To do:
- Create `InternalEvent` enum and separate it from user-defined enums.
- Concurrently process events in the queue through a std::mpsc
- Add QoL collider constructors
- Create a few more examples
- Upload to crates.io
