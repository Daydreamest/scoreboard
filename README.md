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
	- Fisnish match. Remove a match currently in progress from the scoreboard
	- Get a summary. Returns all the current matches ordered by total score, even total scores are ordered by most recent start

## Assumptions

1. No specific programming language is expected. For learning purposes, this project will use [Rust](https://www.rust-lang.org/)
2. There is no mention of thread safety. It's assumed to be a "nice to have" feature
3. A team can be playing a single match only at a given time. For example, if there is a currently ongoing match between Honduras and Costarica, neither of those two teams can be present in a newly created match

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

## Possible additional features

### API

### Optimization

