//! # Score board
//!
//! Provides a simple score board for following the results of the currently played games in a World Cup

use std::cmp::Ordering;
use std::fmt;
use std::string::{String, ToString};
use std::time::Instant;
use std::vec::Vec;

use log::{debug, trace, warn};

// *********************
// Public API functions
// *********************

/// Score board representation
pub struct ScoreBoard {
	/// In-memory data storage, using `Game` struct as a representation of a single ongoing game
	data: Vec<Game>
}

impl ScoreBoard {
	/// Returns a newly created, empty score board
	pub fn new() -> ScoreBoard {
		ScoreBoard { data: Vec::new() }
	}

	/// Starts a game between two teams, with initial score 0 - 0
	///
	/// # Arguments
	///
	/// * `home` - Name of the home team. Must be either a `String` or a type that is convertable to `String`
	/// * `away` - Name of the away team. Must be either a `String` or a type that is convertable to `String`
	///
	/// # Errors
	///
	/// * When the two provided names are the same
	/// * When any of the provided team is currently playing a match
	///
	/// # Examples
	///
	/// ```
	/// let mut expected_result: Vec<String> = Vec::new();
	/// expected_result.push(String::from("Japan 0 - Indonesia 0"));
	///
	/// let mut sb = scoreboard::ScoreBoard::new();
	/// sb.start_game("Japan", "Indonesia");
	/// let summary = sb.get_summary();
	/// assert_eq!(summary, expected_result);
	/// ```
	pub fn start_game<T: ToString, U: ToString>(&mut self, home: T, away: U) -> Result<(), String> {

		let home_name = home.to_string();
		let away_name = away.to_string();

		trace!("Trying to start a game for teams: '{}' and '{}'", home_name, away_name);

		if home_name == away_name {
			warn!("{} cannot play with itself", home_name);
			return Err(format!("{} cannot play with itself", home_name));
		}

		self.check_if_currently_playing(&home_name, &away_name)?;

		self.data.push(
			Game {
				home_team : Team { name: home_name, score: 0 },
				away_team : Team { name: away_name, score: 0 },
				start_time: Instant::now(),
			}
		);

		trace!("Game started");

		self.sort();

		Ok(())
	}

	/// Updates a score of a running match with absolute values
	///
	/// # Arguments
	///
	/// * `home` - Name of the home team. Must be either a `String` or a type that is convertable to `String`
	/// * `new_home_score` - A new score to be set for the home team
	/// * `away` - Name of the away team. Must be either a `String` or a type that is convertable to `String`
	/// * `new_away_score` - A new score to be set for the away team
	///
	/// # Errors
	///
	/// * When there is no active match between the given teams
	///
	/// # Examples
	///
	/// ```
	/// let mut expected_result: Vec<String> = Vec::new();
	/// expected_result.push(String::from("Japan 2 - Indonesia 0"));
	///
	/// let mut sb = scoreboard::ScoreBoard::new();
	/// sb.start_game("Japan", "Indonesia");
	/// sb.update_score("Japan", 2, "Indonesia", 0);
	/// let summary = sb.get_summary();
	/// assert_eq!(summary, expected_result);
	/// ```
	pub fn update_score<T: ToString, U: ToString>(&mut self, home: T, new_home_score: u8, away: U, new_away_score: u8) -> Result<(), String> {
		let home_name = home.to_string();
		let away_name = away.to_string();

		trace!("Updating score to: {} {} - {} {}", home_name, new_home_score, away_name, new_away_score);

		match self.find_game_index(&home_name, &away_name) {
			Ok(game_index) => {
				let new_game_result = Game {
					home_team : Team { name: home_name, score: new_home_score },
					away_team : Team { name: away_name, score: new_away_score },
					start_time : self.data[game_index].start_time,
				};

				let _ = std::mem::replace(&mut self.data[game_index], new_game_result);
			},
			Err(_) => {
				warn!("Couldn't find a game for update");
				return Err(String::from("Couldn't find a game for update"))
			},
		}

		trace!("Update successful");

		self.sort();

		Ok(())
	}

	/// Finishes a match and removes it from the score board
	///
	/// # Arguments
	///
	/// * `home` - Name of the home team. Must be either a `String` or a type that is convertable to `String`
	/// * `away` - Name of the away team. Must be either a `String` or a type that is convertable to `String`
	///
	/// # Errors
	///
	/// * When there is no active match between the given teams
	///
	/// # Examples
	///
	/// ```
	/// let mut expected_result: Vec<String> = Vec::new();
	///
	/// let mut sb = scoreboard::ScoreBoard::new();
	/// sb.start_game("Japan", "Indonesia");
	/// sb.update_score("Japan", 2, "Indonesia", 0);
	/// sb.finish_game("Japan", "Indonesia");
	/// let summary = sb.get_summary();
	/// assert_eq!(summary, expected_result);
	/// ```
	pub fn finish_game<T: ToString, U: ToString>(&mut self, home: T, away: U) -> Result<(), String> {
		let home_name = home.to_string();
		let away_name = away.to_string();

		trace!("Ending a game bewteen '{}' and '{}'", home_name, away_name);

		match self.find_game_index(&home_name, &away_name) {
			Ok(game_index) => { let _ = self.data.remove(game_index); },
			Err(_) => {
				warn!("Couldn't find a game for removal");
				return Err(String::from("Couldn't find a game for removal"))
			},
		}

		trace!("Game removed successfully");

		self.sort();

		Ok(())
	}

	/// Provides the current status of the scoreboard, with all current matches listed. The matches are ordered by total score (the highest coming first) and, in the case of the same score, by start time (the earliest match coming first)
	///
	/// # Returns
	///
	/// * A vector of strings, each string containing the home team, its score, the away team and its score
	///
	/// # Examples
	///
	/// ```
	/// let mut expected_result: Vec<String> = Vec::new();
	/// expected_result.push(String::from("Japan 0 - Indonesia 0"));
	///
	/// let mut sb = scoreboard::ScoreBoard::new();
	/// sb.start_game("Japan", "Indonesia");
	/// let summary = sb.get_summary();
	/// assert_eq!(summary, expected_result);
	/// ```
	pub fn get_summary(&self) -> Vec<String> {
		trace!("Getting the score board summary");
		
		let mut result = Vec::new();

		for game in &self.data {
			result.push(game.to_string());
		}

		return result;
	}
}

// *****************************************
// Private library functions and structures
// *****************************************

/// A representation of a team
struct Team {
	/// Team's name
	name: String,
	/// Team's score
	score: u8,
}

impl fmt::Display for Team {
	/// Implementation of `Display` trait, allowing it to be converted to a String
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.score)
    }
}

/// A representation of a match
struct Game {
	/// Home team structure
	home_team: Team,
	/// Away team structure
	away_team: Team,
	/// Timestamp of the start of the match
	start_time: Instant,
}

impl Game {
	/// Calculates a total score of the match, which is a sum of the scores of both teams
	fn get_total_score(&self) -> u8 {
		return self.home_team.score + self.away_team.score;
	}
}

impl fmt::Display for Game {
	/// Implementation of `Display` trait, allowing it to be converted to a String
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.home_team.to_string(), self.away_team.to_string())
    }
}

impl ScoreBoard {
	/// Finds a match that the given team is currently playing
	///
	/// # Arguments
	///
	/// * `team_name` - name of the team to search for
	///
	/// # Returns
	///
	/// * Index to the match in `data` structure that holds the match of a given team
	///
	/// # Errors
	///
	/// * When the given team is not currently playing any matches
	///
	fn find_game_index_of_team(&self, team_name: &String) -> Result<usize, String> {
		trace!("Looking for {} in the score board", team_name);

		for (id, game) in self.data.iter().enumerate() {
			if &game.home_team.name == team_name || &game.away_team.name == team_name {
				debug!("Team {} is currently playing a game", team_name);
				return Ok(id)
			}
		}

		debug!("Couldn't find a game of team {}", team_name);

		Err(format!("Couldn't find a game of team {}", team_name))
	}

	/// Finds a match between the two given
	///
	/// # Arguments
	///
	/// * `home_name` - name of the home team to search for
	/// * `away_name` - name of the away team to search for
	///
	/// # Returns
	///
	/// * Index to the match in `data` structure that holds the match of these two teams
	///
	/// # Errors
	///
	/// * When the given teams are not currently playing any matches
	///
	fn find_game_index(&self, home_name: &String, away_name:&String) -> Result<usize, String> {
		trace!("Looking for a game between {} and {}", home_name, away_name);

		match self.find_game_index_of_team(&home_name) {
			Ok(game_index) => {
				let game = self.data.get(game_index).unwrap();
				if &game.home_team.name == home_name && &game.away_team.name == away_name {
					debug!("Teams {} and {} are playing a game now", home_name, away_name);
					return Ok(game_index)
				} else {
					debug!("Team {} isn't playing with {} currently", home_name, away_name);
					return Err(format!("Team {} isn't playing with {} currently", home_name, away_name))
				}
			},
			Err(_) => {
				debug!("Couldn't find a game of teams: {} and {}", home_name, away_name);
				return Err(format!("Couldn't find a game of teams: {} and {}", home_name, away_name))
			},
		}
	}

	/// Sorts the `data` structure. Matches with high total scores should come before the ones with low scoring, otherwise matches that started the earliest should come before the matches that started after them
	fn sort(&mut self) {
		trace!("Sorting the games");

		self.data.sort_by(|a, b| {
			if a.get_total_score() < b.get_total_score() {
				Ordering::Greater	// Because reverse order is needed, from greatest to smallest
			} else if a.get_total_score() > b.get_total_score() {
				Ordering::Less		// Because reverse order is needed, from greatest to smallest
			} else {
				if a.start_time < b.start_time {
					Ordering::Greater	// TODO Because second ordering is also reversed, from greatest timestamp (i.e. freshest game) to lowest
				} else if a.start_time > b.start_time {
					Ordering::Less		// TODO Because second ordering is also reversed, from greatest timestamp (i.e. freshest game) to lowest
				} else {
					Ordering::Equal
				}
			}
		});

		trace!("Games sorted");
	}

	/// Checks if any of the two given teams are currently in any matches
	///
	/// # Arguments
	///
	/// * `name_1` - name of a team
	/// * `name_2` - name of a team
	///
	/// # Errors
	///
	/// * When any of the given teams is currently in any active matches
	///
	fn check_if_currently_playing(&self, name_1: &String, name_2:&String) -> Result<(), String> {
		trace!("Checking if teams {} and {} are currently playing a game", name_1, name_2);

		match self.find_game_index_of_team(&name_1) {
			Ok(_) => {
				debug!("Team {} is currently playing a game", name_1);
				return Err(format!("{} is currently playing a game", name_1))
			},
			Err(_) => ()
		}

		match self.find_game_index_of_team(&name_2) {
			Ok(_) => {
				debug!("Team {} is currently playing a game", name_2);
				return Err(format!("{} is currently playing a game", name_2));
			}
			Err(_) => ()
		}

		trace!("Teams {} and {} are not playing any games", name_1, name_2);

		Ok(())
	}

}

// ***********
// Unit tests
// ***********

#[cfg(test)]
mod tests {
    use super::*;

	const HOME_TEAM_NAME: &str = "Monaco";
	const AWAY_TEAM_NAME: &str = "Switzerland";
	const SCORELESS_GAME: &str = "Monaco 0 - Switzerland 0";

	const HOME_TEAM_NAME_1: &str = "Nigeria";
	const AWAY_TEAM_NAME_1: &str = "Chad";
	const SCORELESS_GAME_1: &str = "Nigeria 0 - Chad 0";
	const HOME_TEAM_NAME_2: &str = "Senegal";
	const AWAY_TEAM_NAME_2: &str = "Algeria";
	const SCORELESS_GAME_2: &str = "Senegal 0 - Algeria 0";

	const NOTHING_TO_SHOW: Vec<String> = Vec::new();
	const REMOVAL_ERROR_MESSAGE: &str = "Couldn't find a game for removal";
	const UPDATE_ERROR_MESSAGE: &str = "Couldn't find a game for update";
	
	fn get_summary_of_scoreless_game(id: u8) -> Vec<String> {
		match id {
			1 => return vec![String::from(SCORELESS_GAME_1)],
			2 => return vec![String::from(SCORELESS_GAME_2)],
			_ => return vec![String::from(SCORELESS_GAME)],
		}
	}

	fn get_team_already_paying_message(team_name: &str) -> String {
		return format!("{} is currently playing a game", team_name);
	}

	#[test]
	fn scoreboard_is_empty_at_start() {
		let sb = ScoreBoard::new();

		assert!(sb.data.is_empty());
	}

	#[test]
	fn game_started_correctly() {
		let mut sb = ScoreBoard::new();
		let result = sb.start_game(HOME_TEAM_NAME, AWAY_TEAM_NAME);

		assert!(result.is_ok());
		assert_eq!(sb.data.len(), 1);
		let Game { home_team: h, away_team: a, start_time: _} = sb.data.first().expect("First element is not available.");
		assert_eq!(h.name, HOME_TEAM_NAME);
		assert_eq!(h.score, 0);
		assert_eq!(a.name, AWAY_TEAM_NAME);
		assert_eq!(a.score, 0);
	}

	#[test]
	fn game_not_started_when_both_teams_have_the_same_name() {
		let expected_error_message = format!("{} cannot play with itself", HOME_TEAM_NAME);

		let mut sb = ScoreBoard::new();
		let result = sb.start_game(HOME_TEAM_NAME, HOME_TEAM_NAME);

		assert!(result.is_err());
		assert!(result.err().is_some_and(|result| result == expected_error_message));
		assert!(sb.data.is_empty());
	}

	#[test]
	fn two_games_started_correctly() {
		let mut sb = ScoreBoard::new();
		let result_1 = sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1);
		let result_2 = sb.start_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2);

		assert!(result_1.is_ok());
		assert!(result_2.is_ok());
		assert_eq!(sb.data.len(), 2);
		let Game { home_team: h_1, away_team: a_1, start_time: _} = sb.data.get(0).expect("First element is not available.");
		assert_eq!(h_1.name, HOME_TEAM_NAME_2);
		assert_eq!(h_1.score, 0);
		assert_eq!(a_1.name, AWAY_TEAM_NAME_2);
		assert_eq!(a_1.score, 0);
		let Game { home_team: h_2, away_team: a_2, start_time: _} = sb.data.get(1).expect("Second element is not available.");
		assert_eq!(h_2.name, HOME_TEAM_NAME_1);
		assert_eq!(h_2.score, 0);
		assert_eq!(a_2.name, AWAY_TEAM_NAME_1);
		assert_eq!(a_2.score, 0);
	}

	#[test]
	fn empty_scoreboard_shows_no_results() {
		let sb = ScoreBoard::new();
		let result = sb.get_summary();

		assert_eq!(result, NOTHING_TO_SHOW);
	}

	#[test]
	fn new_game_shows_up_correctly() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME, AWAY_TEAM_NAME).expect("Couldn't create the game");
		let result = sb.get_summary();

		assert_eq!(result.len(), 1);
		let r = result.get(0).expect("First element is not available.");
		assert_eq!(r, SCORELESS_GAME);
	}

	#[test]
	fn two_games_show_correctly() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the first game");
		sb.start_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2).expect("Couldn't create the second game");
		let result = sb.get_summary();

		assert_eq!(result.len(), 2);
		let r_1 = result.get(0).expect("First element is not available.");
		let r_2 = result.get(1).expect("Second element is not available.");
		assert_eq!(r_1, SCORELESS_GAME_2);
		assert_eq!(r_2, SCORELESS_GAME_1);
	}

	#[test]
	fn removing_a_single_game_leaves_the_score_board_empty() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME, AWAY_TEAM_NAME).expect("Couldn't create the game");
		let result_1 = sb.finish_game(HOME_TEAM_NAME, AWAY_TEAM_NAME);
		let result_2 = sb.get_summary();

		assert!(sb.data.is_empty());
		assert!(result_1.is_ok());
		assert_eq!(result_2, NOTHING_TO_SHOW);
	}

	#[test]
	fn adding_after_removal_works() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the first game");
		sb.finish_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't finish the first game");
		let result_1 = sb.start_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2);
		let result_2 = sb.get_summary();

		assert_eq!(sb.data.len(), 1);
		assert!(result_1.is_ok());
		assert_eq!(result_2, get_summary_of_scoreless_game(2));
	}

	#[test]
	fn removal_on_empty_board_returns_an_error() {
		let mut sb = ScoreBoard::new();
		let result_1 = sb.finish_game(HOME_TEAM_NAME, AWAY_TEAM_NAME);
		let result_2 = sb.get_summary();

		assert!(sb.data.is_empty());
		assert!(result_1.err().is_some_and(|result| result == REMOVAL_ERROR_MESSAGE));
		assert_eq!(result_2, NOTHING_TO_SHOW);
	}

	#[test]
	fn mismatched_home_and_away_names_in_removal_return_an_error() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME, AWAY_TEAM_NAME).expect("Couldn't create the game");
		let result_1 = sb.finish_game(AWAY_TEAM_NAME, HOME_TEAM_NAME);
		let result_2 = sb.get_summary();

		assert_eq!(sb.data.len(), 1);
		assert!(result_1.err().is_some_and(|result| result == REMOVAL_ERROR_MESSAGE));
		assert_eq!(result_2, get_summary_of_scoreless_game(0));
	}

	#[test]
	fn removal_of_a_match_with_wrong_home_team_returns_an_error() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the game");
		let result_1 = sb.finish_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_1);
		let result_2 = sb.get_summary();

		assert_eq!(sb.data.len(), 1);
		assert!(result_1.err().is_some_and(|result| result == REMOVAL_ERROR_MESSAGE));
		assert_eq!(result_2, get_summary_of_scoreless_game(1));
	}

	#[test]
	fn removal_of_a_match_with_wrong_away_team_returns_an_error() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the game");
		let result_1 = sb.finish_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_2);
		let result_2 = sb.get_summary();

		assert_eq!(sb.data.len(), 1);
		assert!(result_1.err().is_some_and(|result| result == REMOVAL_ERROR_MESSAGE));
		assert_eq!(result_2, get_summary_of_scoreless_game(1));
	}

	#[test]
	fn removal_of_wrong_teams_returns_an_error() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the game");
		let result_1 = sb.finish_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2);
		let result_2 = sb.get_summary();

		assert_eq!(sb.data.len(), 1);
		assert!(result_1.err().is_some_and(|result| result == REMOVAL_ERROR_MESSAGE));
		assert_eq!(result_2, get_summary_of_scoreless_game(1));
	}

	#[test]
	fn removing_the_last_game_works() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the first game");
		sb.start_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2).expect("Couldn't create the second game");
		let result_1 = sb.finish_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2);
		let result_2 = sb.get_summary();

		assert_eq!(sb.data.len(), 1);
		assert!(result_1.is_ok());
		assert_eq!(result_2, get_summary_of_scoreless_game(1));
	}

	#[test]
	fn removing_the_first_game_works() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the first game");
		sb.start_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2).expect("Couldn't create the second game");
		let result_1 = sb.finish_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1);
		let result_2 = sb.get_summary();

		assert_eq!(sb.data.len(), 1);
		assert!(result_1.is_ok());
		assert_eq!(result_2, get_summary_of_scoreless_game(2));
	}

	#[test]
	fn removing_the_mid_game_works() {
		let expected_summary = vec![SCORELESS_GAME_2, SCORELESS_GAME_1];

		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the first game");
		sb.start_game(HOME_TEAM_NAME, AWAY_TEAM_NAME).expect("Couldn't create the second game");
		sb.start_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2).expect("Couldn't create the third game");
		let result_1 = sb.finish_game(HOME_TEAM_NAME, AWAY_TEAM_NAME);
		let result_2 = sb.get_summary();

		assert_eq!(sb.data.len(), 2);
		assert!(result_1.is_ok());
		assert_eq!(result_2, expected_summary);
	}

	#[test]
	fn creating_and_removing_many_games_leaves_an_empty_board() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the first game");
		sb.start_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2).expect("Couldn't create the second game");
		sb.start_game(HOME_TEAM_NAME, AWAY_TEAM_NAME).expect("Couldn't create the third game");
		let result_1 = sb.finish_game(HOME_TEAM_NAME, AWAY_TEAM_NAME);
		let result_2 = sb.finish_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1);
		let result_3 = sb.finish_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2);
		let result_4 = sb.get_summary();

		assert_eq!(sb.data.len(), 0);
		assert!(result_1.is_ok());
		assert!(result_2.is_ok());
		assert!(result_3.is_ok());
		assert_eq!(result_4, NOTHING_TO_SHOW);
	}

	#[test]
	fn changing_a_score_for_a_home_team_in_exisitng_game_works() {
		let expected_summary = vec![format!("{} 1 - {} 0", HOME_TEAM_NAME, AWAY_TEAM_NAME)];

		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME, AWAY_TEAM_NAME).expect("Couldn't create the game");
		let result_1 = sb.update_score(HOME_TEAM_NAME, 1, AWAY_TEAM_NAME, 0);
		let result_2 = sb.get_summary();

		assert!(result_1.is_ok());
		assert_eq!(result_2, expected_summary);
	}

	#[test]
	fn changing_a_score_for_an_away_team_in_exisitng_game_works() {
		let expected_summary = vec![format!("{} 0 - {} 1", HOME_TEAM_NAME, AWAY_TEAM_NAME)];

		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME, AWAY_TEAM_NAME).expect("Couldn't create the game");
		let result_1 = sb.update_score(HOME_TEAM_NAME, 0, AWAY_TEAM_NAME, 1);
		let result_2 = sb.get_summary();

		assert!(result_1.is_ok());
		assert_eq!(result_2, expected_summary);
	}

	#[test]
	fn changing_a_score_for_both_teams_in_exisitng_game_works() {
		let expected_summary = vec![format!("{} 2 - {} 3", HOME_TEAM_NAME, AWAY_TEAM_NAME)];

		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME, AWAY_TEAM_NAME).expect("Couldn't create the game");
		let result_1 = sb.update_score(HOME_TEAM_NAME, 2, AWAY_TEAM_NAME, 3);
		let result_2 = sb.get_summary();

		assert!(result_1.is_ok());
		assert_eq!(result_2, expected_summary);
	}

	#[test]
	fn changing_the_score_for_empty_score_board_is_an_error() {
		let mut sb = ScoreBoard::new();
		let result_1 = sb.update_score(HOME_TEAM_NAME, 0, AWAY_TEAM_NAME, 1);
		let result_2 = sb.get_summary();

		assert!(result_1.err().is_some_and(|result| result == UPDATE_ERROR_MESSAGE));
		assert_eq!(result_2, NOTHING_TO_SHOW);
	}

	#[test]
	fn changing_the_score_for_nonexistant_game_is_an_error() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the game");
		let result_1 = sb.update_score(HOME_TEAM_NAME_2, 0, AWAY_TEAM_NAME_2, 1);
		let result_2 = sb.get_summary();

		assert!(result_1.err().is_some_and(|result| result == UPDATE_ERROR_MESSAGE));
		assert_eq!(result_2, get_summary_of_scoreless_game(1));
	}

	#[test]
	fn changing_the_score_for_wrong_home_team_is_an_error() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the game");
		let result_1 = sb.update_score(HOME_TEAM_NAME_2, 0, AWAY_TEAM_NAME_1, 1);
		let result_2 = sb.get_summary();

		assert!(result_1.err().is_some_and(|result| result == UPDATE_ERROR_MESSAGE));
		assert_eq!(result_2, get_summary_of_scoreless_game(1));
	}

	#[test]
	fn changing_the_score_for_wrong_away_team_is_an_error() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the game");
		let result_1 = sb.update_score(HOME_TEAM_NAME_1, 0, AWAY_TEAM_NAME_2, 1);
		let result_2 = sb.get_summary();

		assert!(result_1.err().is_some_and(|result| result == UPDATE_ERROR_MESSAGE));
		assert_eq!(result_2, get_summary_of_scoreless_game(1));
	}

	#[test]
	fn changing_the_score_for_mismatched_home_and_away_teams_is_an_error() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME, AWAY_TEAM_NAME).expect("Couldn't create the game");
		let result_1 = sb.update_score(AWAY_TEAM_NAME, 0, HOME_TEAM_NAME, 1);
		let result_2 = sb.get_summary();

		assert!(result_1.err().is_some_and(|result| result == UPDATE_ERROR_MESSAGE));
		assert_eq!(result_2, get_summary_of_scoreless_game(0));
	}

	#[test]
	fn changing_the_score_for_first_team_of_many_works() {
		let expected_summary = vec![format!("{} 1 - {} 0", HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1), String::from(SCORELESS_GAME_2)];

		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the first game");
		sb.start_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2).expect("Couldn't create the second game");
		let result_1 = sb.update_score(HOME_TEAM_NAME_1, 1, AWAY_TEAM_NAME_1, 0);
		let result_2 = sb.get_summary();

		assert!(result_1.is_ok());
		assert_eq!(result_2, expected_summary);
	}

	#[test]
	fn changing_the_score_for_last_team_of_many_works() {
		let expected_summary = vec![format!("{} 0 - {} 1", HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2), String::from(SCORELESS_GAME_1)];

		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the first game");
		sb.start_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2).expect("Couldn't create the second game");
		let result_1 = sb.update_score(HOME_TEAM_NAME_2, 0, AWAY_TEAM_NAME_2, 1);
		let result_2 = sb.get_summary();

		assert!(result_1.is_ok());
		assert_eq!(result_2, expected_summary);
	}

	#[test]
	fn removing_game_with_changed_score_works() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME, AWAY_TEAM_NAME).expect("Couldn't create the game");
		let result_1 = sb.update_score(HOME_TEAM_NAME, 0, AWAY_TEAM_NAME, 1);
		let result_2 = sb.finish_game(HOME_TEAM_NAME, AWAY_TEAM_NAME);
		let result_3 = sb.get_summary();

		assert!(result_1.is_ok());
		assert!(result_2.is_ok());
		assert_eq!(result_3, NOTHING_TO_SHOW);
	}

	#[test]
	fn changing_score_of_removed_game_is_an_error() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME, AWAY_TEAM_NAME).expect("Couldn't create the game");
		sb.finish_game(HOME_TEAM_NAME, AWAY_TEAM_NAME).expect("Couldn't finish a game");
		let result_1 = sb.update_score(HOME_TEAM_NAME, 0, AWAY_TEAM_NAME, 1);
		let result_2 = sb.get_summary();

		assert!(result_1.err().is_some_and(|result| result == UPDATE_ERROR_MESSAGE));
		assert_eq!(result_2, NOTHING_TO_SHOW);
	}

	#[test]
	fn sorting_of_updated_games_works() {
		let expected_summary_1 = vec![format!("{} 0 - {} 1", HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2), String::from(SCORELESS_GAME_1)];
		let expected_summary_2 = vec![format!("{} 2 - {} 2", HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1), format!("{} 0 - {} 1", HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2)];
		let expected_summary_3 = vec![format!("{} 3 - {} 2", HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2), format!("{} 2 - {} 2", HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1)];
		let expected_summary_4 = vec![format!("{} 3 - {} 3", HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1), format!("{} 3 - {} 2", HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2)];

		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the first game");
		sb.start_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2).expect("Couldn't create the second game");
		sb.update_score(HOME_TEAM_NAME_2, 0, AWAY_TEAM_NAME_2, 1).expect("Couldn't update the second game");
		let result_1 = sb.get_summary();
		sb.update_score(HOME_TEAM_NAME_1, 2, AWAY_TEAM_NAME_1, 2).expect("Couldn't update the first game");
		let result_2 = sb.get_summary();
		sb.update_score(HOME_TEAM_NAME_2, 3, AWAY_TEAM_NAME_2, 2).expect("Couldn't update the second game");
		let result_3 = sb.get_summary();
		sb.update_score(HOME_TEAM_NAME_1, 3, AWAY_TEAM_NAME_1, 3).expect("Couldn't update the first game");
		let result_4 = sb.get_summary();

		assert_eq!(result_1, expected_summary_1);
		assert_eq!(result_2, expected_summary_2);
		assert_eq!(result_3, expected_summary_3);
		assert_eq!(result_4, expected_summary_4);
	}

	#[test]
	fn secondary_sorting_by_start_time_works() {
		let expected_summary_1 = vec![format!("{} 1 - {} 1", HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1), String::from(SCORELESS_GAME_2)];
		let expected_summary_2 = vec![format!("{} 1 - {} 1", HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2), format!("{} 1 - {} 1", HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1)];

		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the first game");
		sb.start_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2).expect("Couldn't create the second game");
		sb.update_score(HOME_TEAM_NAME_1, 1, AWAY_TEAM_NAME_1, 1).expect("Couldn't update the eariler game");
		let result_1 = sb.get_summary();
		sb.update_score(HOME_TEAM_NAME_2, 1, AWAY_TEAM_NAME_2, 1).expect("Couldn't update the later game");
		let result_2 = sb.get_summary();

		assert_eq!(result_1, expected_summary_1);
		assert_eq!(result_2, expected_summary_2);
	}

	#[test]
	fn home_team_cannot_be_added_to_a_second_concurrent_match() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the first game");
		let result_1 = sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_2);
		let result_2 = sb.get_summary();

		assert!(result_1.err().is_some_and(|result| result == get_team_already_paying_message(HOME_TEAM_NAME_1)));
		assert_eq!(result_2,get_summary_of_scoreless_game(1));
	}

	#[test]
	fn away_team_cannot_be_added_to_a_second_concurrent_match() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the first game");
		let result_1 = sb.start_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_1);
		let result_2 = sb.get_summary();

		assert!(result_1.err().is_some_and(|result| result == get_team_already_paying_message(AWAY_TEAM_NAME_1)));
		assert_eq!(result_2,get_summary_of_scoreless_game(1));
	}

	#[test]
	fn home_team_cannot_be_added_to_a_second_concurrent_match_as_away_team() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the first game");
		let result_1 = sb.start_game(HOME_TEAM_NAME_2, HOME_TEAM_NAME_1);
		let result_2 = sb.get_summary();

		assert!(result_1.err().is_some_and(|result| result == get_team_already_paying_message(HOME_TEAM_NAME_1)));
		assert_eq!(result_2,get_summary_of_scoreless_game(1));
	}

	#[test]
	fn away_team_cannot_be_added_to_a_second_concurrent_match_as_home_team() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the first game");
		let result_1 = sb.start_game(AWAY_TEAM_NAME_1, AWAY_TEAM_NAME_2);
		let result_2 = sb.get_summary();

		assert!(result_1.err().is_some_and(|result| result == get_team_already_paying_message(AWAY_TEAM_NAME_1)));
		assert_eq!(result_2,get_summary_of_scoreless_game(1));
	}

	#[test]
	fn both_teams_cannot_start_a_new_match_mismatched() {
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the first game");
		let result_1 = sb.start_game(AWAY_TEAM_NAME_1, HOME_TEAM_NAME_1);
		let result_2 = sb.get_summary();

		assert!(result_1.err().is_some_and(|result| result == get_team_already_paying_message(AWAY_TEAM_NAME_1)));
		assert_eq!(result_2,get_summary_of_scoreless_game(1));
	}

	#[test]
	fn match_will_not_start_if_both_teams_are_already_playing() {
		let expected_summary = vec![String::from(SCORELESS_GAME_2), String::from(SCORELESS_GAME_1)];

		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the first game");
		sb.start_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2).expect("Couldn't create the second game");
		let result_1 = sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_2);
		let result_2 = sb.get_summary();

		assert!(result_1.err().is_some_and(|result| result == get_team_already_paying_message(HOME_TEAM_NAME_1)));
		assert_eq!(result_2, expected_summary);
	}

	#[test]
	fn grand_example() {
		let expected_summary = vec![
			String::from("Uruguay 6 - Italy 6"),
			String::from("Spain 10 - Brazil 2"),
			String::from("Mexico 0 - Canada 5"),
			String::from("Argentina 3 - Australia 1"),
			String::from("Germany 2 - France 2"),
		];

		let mut sb = ScoreBoard::new();
		sb.start_game("Mexico", "Canada").unwrap();
		sb.update_score("Mexico", 0, "Canada", 1).unwrap();
		sb.start_game("Spain", "Brazil").unwrap();
		sb.update_score("Mexico", 0, "Canada", 2).unwrap();
		sb.update_score("Spain", 1, "Brazil", 1).unwrap();
		sb.update_score("Spain", 1, "Brazil", 2).unwrap();
		sb.start_game("Germany", "France").unwrap();
		sb.update_score("Mexico", 0, "Canada", 3).unwrap();
		sb.update_score("Germany", 1, "France", 0).unwrap();
		sb.update_score("Mexico", 0, "Canada", 4).unwrap();
		sb.update_score("Germany", 1, "France", 1).unwrap();
		sb.update_score("Germany", 1, "France", 2).unwrap();
		sb.start_game("Uruguay", "Italy").unwrap();
		sb.start_game("Argentina", "Australia").unwrap();
		sb.update_score("Uruguay", 1, "Italy", 1).unwrap();
		sb.update_score("Germany", 2, "France", 2).unwrap();
		sb.update_score("Uruguay", 2, "Italy", 2).unwrap();
		sb.update_score("Argentina", 1, "Australia", 1).unwrap();
		sb.update_score("Mexico", 0, "Canada", 5).unwrap();
		sb.update_score("Uruguay", 3, "Italy", 3).unwrap();
		sb.update_score("Argentina", 3, "Australia", 1).unwrap();
		sb.update_score("Spain", 10, "Brazil", 2).unwrap();
		sb.update_score("Uruguay", 6, "Italy", 6).unwrap();

		let result = sb.get_summary();

		assert_eq!(result, expected_summary);
	}
}
