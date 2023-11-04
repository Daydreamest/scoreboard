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
4. Getting a summary of the current results will be much more frequently used than all the other API functions combined. This will impact optimization choices

## Progress

### Base requirements

| Requirement | Status | Comments |
| ------ | ------ | ------ |
| 1. Library | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/c/c4/No_icon_red.svg/240px-No_icon_red.svg.png" width="24" height="24"> | |
| 2. Storage | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/c/c4/No_icon_red.svg/240px-No_icon_red.svg.png" width="24" height="24"> | |
| 3. TDD | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/c/c4/No_icon_red.svg/240px-No_icon_red.svg.png" width="24" height="24"> | |
| 4. API | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/c/c4/No_icon_red.svg/240px-No_icon_red.svg.png" width="24" height="24"> | |
| 4.1. Start | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/c/c4/No_icon_red.svg/240px-No_icon_red.svg.png" width="24" height="24"> | |
| 4.2. Update | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/c/c4/No_icon_red.svg/240px-No_icon_red.svg.png" width="24" height="24"> | |
| 4.3. Finish | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/c/c4/No_icon_red.svg/240px-No_icon_red.svg.png" width="24" height="24"> | |
| 4.4. Summary | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/c/c4/No_icon_red.svg/240px-No_icon_red.svg.png" width="24" height="24"> | |

### Extra features

| Features | Status | Comments |
| ------ | ------ | ------ |
| Team uniqueness | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/c/c4/No_icon_red.svg/240px-No_icon_red.svg.png" width="24" height="24"> | |
| Thread safety | <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/c/c4/No_icon_red.svg/240px-No_icon_red.svg.png" width="24" height="24"> | |

## Instannation and usage

### Installation

1. Install Rust work environment. Use the standard [Rust installation guide](https://www.rust-lang.org/learn/get-started) with all the default options
2. This should also install [Cargo](https://doc.rust-lang.org/cargo/), Rust default package manager
3. Clone this repository

### Compilation

Move to the "scoreboard" directory and run:

`> cargo build --release`

On the first execution Cargo may download dependencies and libraries. After that, the library will be compiled to a biniary:

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

### Optimization

