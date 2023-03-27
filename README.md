# rs-game

A demonstration of some of the capabilities of my [rs-game-engine](https://github.com/Raymi306/rs-game-engine),
using my [rs-level-editor](https://github.com/Raymi306/rs-level-editor) for level creation.
This demo also features an integration with [Bevy's Entity Component System](https://docs.rs/bevy_ecs/latest/bevy_ecs/) for the implementation of logic.

This is a simple top down game with smooth movement across a tile grid. It was intended to be a simple RPG, and could still be so with future updates.

This demo features movement and collision detection of a player character and of NPCs across a tile based level.
NPCs have very simple player tracking AI, using a breadth first search pathfinding algorithm.
There is a simple menu system allowing for exiting of the game, which correctly pauses and unpauses all game logic.

Controls:
- movement: WASD, Arrow Keys
- toggle menu: Spacebar
