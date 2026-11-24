
# Sokoban-rs

Implement a Sokoban game in rust with gtk4, one step at a time.

##  Todo List

- [ ] sounds
- [ ] store levels done and the score (minimum moves to win)
- [x] goto to a marked floor square  (g followed by a b c d or e) if it's on the puzzle and reachable without pushing any crate
- [x] mark the floor square (m followed by a b c d or e) where sokoban is located
- [x] count moves 
- [x] chain the levels
- [x] undo moves and restart level
- [x] have a star covering the crates when puzzle is solved
- [x] detect when puzzle is solved
- [x] have the sokoban character blocked by blocked crates
- [x] have the sokoban character push crates, making them move
- [x] have the sokoban character blocked by walls
- [x] have the sokoban character move in the level
- [x] darken tiles that are outside the walls
- [x] given a level map with floors, walls, store locations, crates, and sokoban, draw the map in the window
- [x] several graphical elements are drawn, not just the logo
- [x] keys can move the logo up,down,left,right
- [x] display the logo at a fixed point inside a window larger than the logo
- [x] on key 'q', exit the app
- [x] show a logo
- [x] show a hello world gtk app

## Design items

### Model

Coords : a pair of coordinates, row and column

Level : a map of Coords to Piece

Direction : Left Up Down Right

Step : a move from the character from one location in one direction, including pushing a crate or not

Puzzle : a representation of the current level to solve, a collection of piece

Piece : an area or object in the puzzle : Wall, Floor, Goal, Crate, Stored Crate, Sokoban, Sokoban On Goal

A piece in coords c can be changed when "covered" by another piece:

- Floor in c + Sokoban → Sokoban in c
- Floor in c + Crate → Crate in c
- Goal in c + Sokoban → Sokoban on Goal in c
- Goal in c + Crate → Stored Crate in c

A piece can be changed when "uncovered" by another piece:

- Sokoban in c - Sokoban → Floor in c
- Sokoban On Goal in c - Sokoban → Goal in c
- Crate in c - Crate → Floor in c
- Stored Crate in c - Crate → Goal in c

Sokoban in c can be "moved" in direction d

- Sokoban in c * c+d = Floor in c, p[c+d] + Sokoban in c+d
- Sokoban on Goal in c * c+d Goal in c, p[c+d] + Sokoban in c+d

A piece p can be "pushed" by Sokoban in c towards direction d :

- Sokoban in c ^ Crate in c+d = p[c+d+d] + Crate in c+d+d, p[c+d] - Crate in c+d, p[c+d] + Sokoban in c+d, p[c] - Sokoban in c

A step can be reversed by doing the opposite operations in reverse order:

- reversing Sokoban in c * c+d = reversing Sokoban in c+d * c = p[c+d] - Sokoban in c+d, p[c] + Sokoban in c
- reversing Sokoban in c ^ Crate in c+d = p[c] + Sokoban in c, p[c+d] - Sokoban in c+d, p[c+d] + Crate in c+d, p[c+d+d] - Crate in c+d+d

Sokoban and Crates cannot cover any other piece than Floor or Goal

Thus a Crate cannot cover a Crate

Use railway programming to check possible moves and execute them

### Graphics

Tile : a 32x32 pixels picture representing a piece

Sprite : a graphic entity formed with a tile that can placed and moved on a drawing area

Background : A drawing area that was built with tiles of all the fixed pieces and snaphot

Game : the main process of the application:
- build the level
- listen to keyboard strokes
- process keyboard strokes:
    - up,right,down,left, (or d,r,s,t on my keyboard)
    - u for undo, * for restart
    - n for next level, p for previous level
    - q for quitting

### Features

- animated set up of level
- undo, restart
- set a mark, go to a mark
- save level score


