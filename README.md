
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

Move : the (sokoban) character walking in one direction or pushing a crate in one direction

Step : elementary parts of a move, involving an element at a location and the element at an adjactent location

Puzzle : a representation of the current level to solve, a collection of piece

Element : an object in the puzzle : Wall, Floor, Goal, Crate, Stored (crate), Sokoban, (sokoban) On Goal

rules of elementary steps:


step | E[c] | E[c+d] | comment |
| ---  | --- | --- | --- | --- |
| before |  F  |  x  |        |
| after |  F  |  x  | no move |
| | | | |
| before |  G  |  x  |        |
| after |  G  |  x  | no move |
| | | | |
| before |  W  |  x  |        |
| after |  W  |  x  | no move |
| | | | |
| before |  x  |  W  |        |
| after |  x  |  W  | blocked by wall |
| | | | |
| before |  C  |  C  |        |
| after |  C  |  C  | blocking crate |
| | | | |
| before |  C  |  T  |        |
| after |  C  |  T  | blocking stored crate |
| | | | |
| before |  T  |  C  |        |
| after |  T  |  C  | blocked stored crate |
| | | | |
| before |  T  |  T  |        |
| after |  T  |  T  | blocked stored crates |
| | | | |
| before |  S  |  F  |        |
| after |  F  |  S  | walking |
| | | | |
| before |  S  |  G  |        |
| after |  F  |  O  | walking on goal |
| | | | |
| before |  O  |  F  |        |
| after |  G  |  S  | walking from goal |
| | | | |
| before |  O  |  G  |        |
| after |  G  |  O  | walking frow goal to goal |
| | | | |
| before |  C  |  F  |        |
| after |  F  |  C  | pushed crate |
| | | | |
| before |  C  |  G  |        |
| after |  F  |  T  | stored crate |
| | | | |
| before |  T  |  F  |        |
| after |  G  |  C  | de-stored crate |
| | | | |
| before |  T  |  G  |        |
| after |  G  |  T  | crate pushed frow goal to goal |


rules of moves :


step | E[c] | E[c+d] | E[c+d+d] | comment |
| ---  | --- | --- | --- | --- | --- |
| before |  S  |  F  | x |         |
| after |  F  |  S  | x  | walking |
| | | | | |
| before |  S  |  W  | x |         |
| after |  S  |  W  | x  | blocked by wall |
| | | | | |
| before |  S  |  G  | x |         |
| after |  F  |  O  | x  | walking on goal |
| | | | | |
| before |  O  |  F  | x |         |
| after |  G  |  S  | x  | walking off goal |
| | | | | |
| before |  O  |  G  | x |         |
| after |  G  |  O  | x  | walking on goals |
| | | | | |
| before |  S  |  C  | W |         |
| after |  S  |  C  | W  | crate blocked by wall |
| | | | | |
| before |  S  |  T  | W |         |
| after |  S  |  T  | W  | stored crate blocked by wall |
| | | | | |
| before |  O  |  C  | W |         |
| after |  O  |  C  | W  | crate blocked by wall |
| | | | | |
| before |  O  |  T  | W |         |
| after |  O  |  T  | W  | stored crate blocked by wall |
| | | | | |
| before |  S  |  C  | C |         |
| after |  S  |  C  | C  | crate blocked by crate |
| | | | | |
| before |  S  |  T  |  C |         |
| after |  S  |  T  | C  | stored crate blocked by crate |
| | | | | |
| before |  S  |  T  | T |         |
| after |  S  |  T  | T  | stored crate blocked by stored crate |
| | | | | |
| before |  S  |  C  | T |         |
| after |  S  |  C  | T  | crate blocked by stored crate |
| | | | | |
| before |  O  |  C  | C |         |
| after |  O  |  C  | C  | crate blocked by crate |
| | | | | |
| before |  O  |  T  |  C |         |
| after |  O  |  T  | C  | stored crate blocked by crate |
| | | | | |
| before |  O  |  T  | T |         |
| after |  O  |  T  | T  | stored crate blocked by stored crate |
| | | | | |
| before |  O  |  C  | T |         |
| after |  O  |  C  | T  | crate blocked by stored crate |
| | | | | |
| before |  S  |  C  | F |         |
| after |  F  |  S  | C  | pushing a crate |
| | | | | |
| before |  S  |  C  | G |         |
| after |  F  |  S  | T  | storing a crate |
| | | | | |
| before |  S  |  T  | F |         |
| after |  F  |  G  | C  | destoring a crate |
| | | | | |
| before |  S  |  T  | G |         |
| after |  F  |  G  | T  | moving a crate on goals |

To make a move, get elements in coords, coords + direction, coords + direction + direction, (C,D,E)

- if step(D,E) = NoMove, move(C,D,E) = Some([Step(D,E),Step(C,D)]
- if step(D,E) = Blocked, Move(C,D,E) = None
- if step(D,E) = Move, move(C,D,E) = Some([Step(D,E), Step(C,D)])
- if step(D,E) = Move and Step(C,D) = Blocked, move(C,D,E) = None

to reverse (undo) a move, apply reversed steps in reverse order
all valid moves are reversible

reverse([Step(D,E),Step(C,D)] = [Step(D,C),Step(E,D)]


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


