# CS128H-project

## Group Name
* CS128H Squad

## Group Members and NetIDs
* Jiayi Qian, sqian9
* Zimeng Li, zimengl4

## Project Introduction
- Description:
    - A multiplayer version of the classic 2D game Pac-Man
- Goals, Objectives, and Why we chose this project:
    - Realize the basic playing rules and playing methods of Pac-Man
    - Devise the network functionality for Pac-Man
    - Branch and build off of the skills we learned
    - Explore and Familiarize ourselves with networking and graphics in Rust


## Technical Overview
We want to utilize some libraries in Rust to help develop this game, for example, bracket_lib. There are many functions that helps build the fundamental blocks of our game, such as the tick function, main_loop function, and some pathfinding algorithms. We will use the tick function and main_loop function to handle the game state of Pac-Man. We will implement pathfinding algorithms for the ghost characters.

- Checkpoint 1:
      We plan to realize the Graphics and half the game logic: Generation of the maze and the dots in the maze, the basic functionality of the player, and different game states.
- Checkpoint 2
      We plan to realize the last half of the game logic section and some of the networking sessions.

- Graphics
    - Create a basic UI of the maze with grids, dots in the maze as points, and pac-mans.  The UI will be using Macroquad
    - **Task List**
        - [x] Window using [Macroquad](https://github.com/not-fl3/macroquad)
        - [x] Maze with dots inside
        - [x] Four colored ghosts body & movements
        - [x] Pac-Man movements
- Game logic
    - Handle interactions between Pac-Man, ghosts, and their environment.
    - **Task List**
        - [x] Ghosts, Pac-Man to wall collision
        - [x] Pac-Man eats the dots, ghosts
        - [x] Ghosts eat and chase the Pac-Man
        - [x] Ghosts run out of the block 
- Networking (Unsure whether we can make it)
    - Synchronize client data and broadcast a global game state.
    - **Task List**
        - Client
            - [x] Update their maximum score
            - [x] Receiving and updating game state
            - [x] Serializing and sending game state
        - Server
            - [x] Storing users' maximum score and displaying the score of the maximum player
            - [x] Handling client connections and disconnections
            - [x] Broadcast game state
## Possible Challenges

- Learning and fully mastering the methods provided by bracket_lib will definitely take some time.
- Creating a UI in rust
- Exploring the networking and graphics in rust that's completely new to both of us
- The game Pac-Man consists of a lot of graphic components which can be hard to handle properly.

## References
 "Pac-Man Official Website – History". Pac-Man Official Website.
