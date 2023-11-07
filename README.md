# Scoreboard

## Coding exercise

Develop a new live World Cup scorebard library that shows all the ongoing matches and their stores

## Requirements

1. Must be a library implementation
2. Use in-memory store solution
3. Use TDD, pay attention to OO design, Clean Code and SOLID
4. Implement basic API:
	- Start a match. Assuming initial score "0-0", should capture two parameters: home team and away team
	- Update score. Should receive a pair of absolute scores: home team score and away team score
	- Finish match. Remove a match currently in progress from the scoreboard
	- Get a summary. Returns all the current matches ordered by total score, even total scores are ordered by most recent start

## Assumptions

1. No specific programming language is expected. For learning purposes, this project will use [Rust](https://www.rust-lang.org/)
2. There is no mention of thread safety. It's assumed to be a "nice to have" feature
3. A team can be playing a single match only at a given time. For example, if there is a currently ongoing match between Honduras and Costarica, neither of those two teams can be present in a newly created match
4. In all the API calls, "home" team always comes before "away" team. If the order of the teams is wrong for an operation, it returns an error
5. Getting a summary of the current results will be much more frequently used than all the other API functions combined. This will impact optimization choices

## Progress

### Base requirements

| Requirement | Status | Comments |
| ------ | ------ | ------ |
| 1. Library | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/5/50/Yes_Check_Circle.svg/240px-Yes_Check_Circle.svg.png" width="24" height="24"> | Devloped as an independant Rust library |
| 2. Storage | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/5/50/Yes_Check_Circle.svg/240px-Yes_Check_Circle.svg.png" width="24" height="24"> | Using standard vector cotainer. See "Possible additional features -> Optimization" below for more comments on this |
| 3. TDD | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/5/50/Yes_Check_Circle.svg/240px-Yes_Check_Circle.svg.png" width="24" height="24"> | TDD followed to a degree. Most functions are so small it was possible to write the final correct version for the first test and then add the other corner cases |
| 4. API | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/5/50/Yes_Check_Circle.svg/240px-Yes_Check_Circle.svg.png" width="24" height="24"> | Public `ScoreBoard` struct and its methods |
| 4.1. Start | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/5/50/Yes_Check_Circle.svg/240px-Yes_Check_Circle.svg.png" width="24" height="24"> | `ScoreBoard.start_game(home_team_name, away_team_name)` |
| 4.2. Update | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/5/50/Yes_Check_Circle.svg/240px-Yes_Check_Circle.svg.png" width="24" height="24"> | `ScoreBoard.update_score(home_team_name, home_score, away_team_name, away_score)` |
| 4.3. Finish | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/5/50/Yes_Check_Circle.svg/240px-Yes_Check_Circle.svg.png" width="24" height="24"> | `ScoreBoard.finish_game(home_team_name, away_team_name)`|
| 4.4. Summary | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/5/50/Yes_Check_Circle.svg/240px-Yes_Check_Circle.svg.png" width="24" height="24"> | `ScoreBoard.get_summary()` |

### Extra features

| Features | Status | Comments |
| ------ | ------ | ------ |
| Team uniqueness | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/5/50/Yes_Check_Circle.svg/240px-Yes_Check_Circle.svg.png" width="24" height="24"> | `start_game(team1, team2)` rejects the request if any of the teams is already playing a match |
| Thread safety | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/5/50/Yes_Check_Circle.svg/240px-Yes_Check_Circle.svg.png" width="24" height="24"> | Rust compiler provides thread safety, unless serious hacks get involved. There is no `unsafe` code in this repository |

## Installation and usage

### Installation

1. Install Rust work environment. Use the standard [Rust installation guide](https://www.rust-lang.org/learn/get-started) with all the default options
2. This should also install [Cargo](https://doc.rust-lang.org/cargo/), Rust default package manager
3. Clone this repository

### Compilation

Move to the "scoreboard" directory and run:

`> cargo build --release`

On the first execution Cargo may download dependencies and libraries. After that, the library will be compiled to a binary:

`scoreboard/target/release/libscoreboard.rlib`

### Usage

#### As Rust source code

Copy the `scoreboard.rs` file to your project and include the module anywhere you need it with:

`use scoreboard::*;`

There is an exmaple proved in `scoreboard/src/bin/example.rs` that shows typical usage of the library.

#### As compiled Rust library with rustc compiler

Copy compiled `libscoreboard.rlib` file to your project and add a flag to your compilation options:

`rustc main.rs --extern scoreboard=libscoreboard.rlib`

### Testing

To run tests move to the "scoreboard" directory and run:

`> cargo test`

## Possible additional features

### API

1. Team names are kept internally as UTF-8 strings and not verified extensively. This allows for maches like "AAAA - jskdfhgidsf", "USA - U.S.A." or any other two distinct string names. A good improvement would be to create an enum or a dictionary that keeps a list of all available names and verify against it
2. `update_score()` is very unwieldy and allows to change the score arbitrarily. In footbal the score changes come in quanta, so there should be a method `add_goal(team_name)` that adds 1 to the score of the mentioned team

### Optimization

1. Time stamps are used to verify which match started first. This may be an overkill, but it's cleaner and easier than implementing internal counters, at the cost of being less efficient on the CPU
2. `Vec` is used as a data container. There are others collections available, but even the [Rust guide](https://doc.rust-lang.org/std/collections/index.html) suggests sticking to the good, reliable vector. Alternatives could be considered to improve efficiency, but they would need profiling and real world usage of the library
3. Data is sorted after each addition, score change and removal. The sorting could be moved to the summary display method, so it would "happen" only once in the code, but this has disadvantages:
	- `get_summary()` method would have to be mutable and change the state of the score board, which is a bad design
	- `get_summary()` is expected to be called much more often than all the other API functions combined, so it has to be quick and simple. Adding sorting to it can have serious time impact for a large number of concurrent matches
	- Current implementation of `sort()` does extremely well with collections that are partly sorted or have stretches of sorted elements ([source](https://doc.rust-lang.org/std/primitive.slice.html#current-implementation-8)). As such, it should have little impact on the functions that use it now, as their changes apply to single matches and leave the rest of the collection sorted
4. Alternatives to `Vec` and `push()` could be considered that might allow to skip sorting in some cases. A newly created match has the lowes possible total score and the freshest timestamp, so sorting on `start_game()` could be probably omitted, but this requires more analysis

## Issues

1. "match" is a keyword in Rust, so the word "game" is used throughout the code in the meaning of "a match between two teams"
