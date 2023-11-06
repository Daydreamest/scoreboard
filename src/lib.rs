use std::fmt;
use std::string::String;
use std::string::ToString;
use std::vec::Vec;

// *********************
// Public API functions
// *********************

pub struct ScoreBoard {
	data: Vec<Game>
}

impl ScoreBoard {
	pub fn new() -> ScoreBoard {
		ScoreBoard { data: Vec::new() }
	}

	pub fn start_game<T: ToString, U: ToString>(&mut self, home: T, away: U) -> Result<(), String> {
		// TODO make sure the name isn't playing a game yet

		let home_name = home.to_string();
		let away_name = away.to_string();

		println!("Function start_game called with parameters: '{0}' and '{1}'", home_name, away_name);

		if home_name == away_name {
			return Err(format!("{} cannot play with itself", home_name));
		}

		self.data.push(
			Game {
				home_team : Team { name: home_name, score: 0 },
				away_team : Team { name: away_name, score: 0 }
			}
		);

		Ok(())
	}

	pub fn update_score(text: String) -> Result<(), String> {
		println!("Function update_score called with parameter '{}'", text);
		Ok(())
	}

	pub fn finish_game<T: ToString, U: ToString>(&mut self, home: T, away: U) -> Result<(), String> {

		let home_name = home.to_string();
		let away_name = away.to_string();

		println!("Function finish_game called with parameters: '{0}' and '{1}'", home_name, away_name);

		match self.find_game_index(home_name, away_name) {
			Ok(game_index) => { let _ = self.data.remove(game_index); },
			Err(e) => return Err(e),
		}

		Ok(())
	}

	pub fn get_summary(&self) -> Vec<String> {
		println!("Function get_summary called");
		
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

struct Team {
	name: String,
	score: u8,
}

impl fmt::Display for Team {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.score)
    }
}

struct Game {
	home_team: Team,
	away_team: Team,
}

impl fmt::Display for Game {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.home_team.to_string(), self.away_team.to_string())
    }
}

impl ScoreBoard {
	fn find_game_index(&self, home_name: String, away_name: String) -> Result<usize, String> {

		for (id, game) in self.data.iter().enumerate() {
			if game.home_team.name == home_name && game.away_team.name == away_name {
				return Ok(id)
			}
		}

		Err(String::from("Couldn't find a game for removal"))
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
		let Game { home_team: h, away_team: a} = sb.data.first().expect("First element is not available.");
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
		let Game { home_team: h_1, away_team: a_1} = sb.data.get(0).expect("First element is not available.");
		assert_eq!(h_1.name, HOME_TEAM_NAME_1);
		assert_eq!(h_1.score, 0);
		assert_eq!(a_1.name, AWAY_TEAM_NAME_1);
		assert_eq!(a_1.score, 0);
		let Game { home_team: h_2, away_team: a_2} = sb.data.get(1).expect("Second element is not available.");
		assert_eq!(h_2.name, HOME_TEAM_NAME_2);
		assert_eq!(h_2.score, 0);
		assert_eq!(a_2.name, AWAY_TEAM_NAME_2);
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
		assert_eq!(r_1, SCORELESS_GAME_1);
		assert_eq!(r_2, SCORELESS_GAME_2);
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
		let expected_summary = vec![SCORELESS_GAME_2];

		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the first game");
		sb.finish_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't finish the first game");
		let result_1 = sb.start_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2);
		let result_2 = sb.get_summary();

		assert_eq!(sb.data.len(), 1);
		assert!(result_1.is_ok());
		assert_eq!(result_2, expected_summary);
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
		let expected_summary = vec![SCORELESS_GAME];

		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME, AWAY_TEAM_NAME).expect("Couldn't create the game");
		let result_1 = sb.finish_game(AWAY_TEAM_NAME, HOME_TEAM_NAME);
		let result_2 = sb.get_summary();

		assert_eq!(sb.data.len(), 1);
		assert!(result_1.err().is_some_and(|result| result == REMOVAL_ERROR_MESSAGE));
		assert_eq!(result_2, expected_summary);
	}

	#[test]
	fn removal_of_a_match_with_wrong_home_team_returns_an_error() {
		let expected_summary = vec![SCORELESS_GAME_1];
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the game");
		let result_1 = sb.finish_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_1);
		let result_2 = sb.get_summary();

		assert_eq!(sb.data.len(), 1);
		assert!(result_1.err().is_some_and(|result| result == REMOVAL_ERROR_MESSAGE));
		assert_eq!(result_2, expected_summary);
	}

	#[test]
	fn removal_of_a_match_with_wrong_away_team_returns_an_error() {
		let expected_summary = vec![SCORELESS_GAME_1];
		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the game");
		let result_1 = sb.finish_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_2);
		let result_2 = sb.get_summary();

		assert_eq!(sb.data.len(), 1);
		assert!(result_1.err().is_some_and(|result| result == REMOVAL_ERROR_MESSAGE));
		assert_eq!(result_2, expected_summary);
	}

	#[test]
	fn removal_of_wrong_teams_returns_an_error() {
		let expected_summary = vec![SCORELESS_GAME_1];

		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the game");
		let result_1 = sb.finish_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2);
		let result_2 = sb.get_summary();

		assert_eq!(sb.data.len(), 1);
		assert!(result_1.err().is_some_and(|result| result == REMOVAL_ERROR_MESSAGE));
		assert_eq!(result_2, expected_summary);
	}

	#[test]
	fn removing_the_last_game_works() {
		let expected_summary = vec![SCORELESS_GAME_1];

		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the first game");
		sb.start_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2).expect("Couldn't create the second game");
		let result_1 = sb.finish_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2);
		let result_2 = sb.get_summary();

		assert_eq!(sb.data.len(), 1);
		assert!(result_1.is_ok());
		assert_eq!(result_2, expected_summary);
	}

	#[test]
	fn removing_the_first_game_works() {
		let expected_summary = vec![SCORELESS_GAME_2];

		let mut sb = ScoreBoard::new();
		sb.start_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1).expect("Couldn't create the first game");
		sb.start_game(HOME_TEAM_NAME_2, AWAY_TEAM_NAME_2).expect("Couldn't create the second game");
		let result_1 = sb.finish_game(HOME_TEAM_NAME_1, AWAY_TEAM_NAME_1);
		let result_2 = sb.get_summary();

		assert_eq!(sb.data.len(), 1);
		assert!(result_1.is_ok());
		assert_eq!(result_2, expected_summary);
	}

	#[test]
	fn removing_the_mid_game_works() {
		let expected_summary = vec![SCORELESS_GAME_1, SCORELESS_GAME_2];

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

}
