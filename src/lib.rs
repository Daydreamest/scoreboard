use std::fmt;
use std::string::String;
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

	pub fn start_game(&mut self, home_name: String, away_name: String) -> Result<(), String> {
		// TODO make sure the name isn't playing a game yet
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

	pub fn finish_game(&mut self, home_name: String, away_name: String) -> Result<(), String> {
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
	pub fn start_game_with_literal_names(&mut self, home_name: &str, away_name: &str) -> Result<(), String> {
		self.start_game(String::from(home_name), String::from(away_name))
	}

	pub fn finish_game_with_literal_names(&mut self, home_name: &str, away_name: &str) -> Result<(), String> {
		self.finish_game(String::from(home_name), String::from(away_name))
	}

	fn find_game_index(&self, home_name: String, away_name: String) -> Result<usize, String> {

		for (id, game) in self.data.iter().enumerate() {
			if game.home_team.name == home_name && game.away_team.name == away_name {
				return Ok(id)
			}
		}

		Err(String::from("No such game found"))
	}
}

// ***********
// Unit tests
// ***********

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn scoreboard_is_empty_at_start() {
		let sb = ScoreBoard::new();

		assert!(sb.data.is_empty());
	}

	#[test]
	fn game_started_correctly() {
		let home_team_name = "Monaco";
		let away_team_name = "Switzerland";

		let mut sb = ScoreBoard::new();
		let result = sb.start_game_with_literal_names(home_team_name, away_team_name);

		assert!(result.is_ok());
		assert_eq!(sb.data.len(), 1);
		let Game { home_team: h, away_team: a} = sb.data.first().expect("First element is not available.");
		assert_eq!(h.name, home_team_name);
		assert_eq!(h.score, 0);
		assert_eq!(a.name, away_team_name);
		assert_eq!(a.score, 0);
	}

	#[test]
	fn game_not_started_when_both_teams_have_the_same_name() {
		let home_team_name = "Georgia";
		let away_team_name = "Georgia";
		let expected_error_message = "Georgia cannot play with itself";

		let mut sb = ScoreBoard::new();
		let result = sb.start_game_with_literal_names(home_team_name, away_team_name);

		assert!(result.is_err());
		assert!(result.err().is_some_and(|result| result == expected_error_message));
		assert!(sb.data.is_empty());
	}

	#[test]
	fn two_games_started_correctly() {
		let home_team_name_1 = "Nigeria";
		let away_team_name_1 = "Chad";
		let home_team_name_2 = "Senegal";
		let away_team_name_2 = "Algeria";

		let mut sb = ScoreBoard::new();
		let result_1 = sb.start_game_with_literal_names(home_team_name_1, away_team_name_1);
		let result_2 = sb.start_game_with_literal_names(home_team_name_2, away_team_name_2);

		assert!(result_1.is_ok());
		assert!(result_2.is_ok());
		assert_eq!(sb.data.len(), 2);
		let Game { home_team: h_1, away_team: a_1} = sb.data.get(0).expect("First element is not available.");
		assert_eq!(h_1.name, home_team_name_1);
		assert_eq!(h_1.score, 0);
		assert_eq!(a_1.name, away_team_name_1);
		assert_eq!(a_1.score, 0);
		let Game { home_team: h_2, away_team: a_2} = sb.data.get(1).expect("Second element is not available.");
		assert_eq!(h_2.name, home_team_name_2);
		assert_eq!(h_2.score, 0);
		assert_eq!(a_2.name, away_team_name_2);
		assert_eq!(a_2.score, 0);
	}

	#[test]
	fn empty_scoreboard_shows_no_results() {
		let nothing_to_show: Vec<String> = Vec::new();

		let sb = ScoreBoard::new();
		let result = sb.get_summary();

		assert_eq!(result, nothing_to_show);
	}

	#[test]
	fn new_game_shows_up_correctly() {
		let home_team_name = "India";
		let away_team_name = "Japan";
		let expected_summary = "India 0 - Japan 0";

		let mut sb = ScoreBoard::new();
		sb.start_game_with_literal_names(home_team_name, away_team_name).expect("Couldn't create the game");
		let result = sb.get_summary();

		assert_eq!(result.len(), 1);
		let r = result.get(0).expect("First element is not available.");
		assert_eq!(r, &expected_summary);
	}

	#[test]
	fn two_games_show_correctly() {
		let home_team_name_1 = "Uruguay";
		let away_team_name_1 = "Columbia";
		let home_team_name_2 = "Peru";
		let away_team_name_2 = "Chile";
		let expected_summary_1 = "Uruguay 0 - Columbia 0";
		let expected_summary_2 = "Peru 0 - Chile 0";

		let mut sb = ScoreBoard::new();
		sb.start_game_with_literal_names(home_team_name_1, away_team_name_1).expect("Couldn't create the first game");
		sb.start_game_with_literal_names(home_team_name_2, away_team_name_2).expect("Couldn't create the second game");
		let result = sb.get_summary();

		assert_eq!(result.len(), 2);
		let r_1 = result.get(0).expect("First element is not available.");
		let r_2 = result.get(1).expect("Second element is not available.");
		assert_eq!(r_1, &expected_summary_1);
		assert_eq!(r_2, &expected_summary_2);
	}

	#[test]
	fn removing_a_single_game_leaves_the_score_board_empty() {
		let home_team_name = "New Zeland";
		let away_team_name = "Philippines";
		let nothing_to_show: Vec<String> = Vec::new();

		let mut sb = ScoreBoard::new();
		sb.start_game_with_literal_names(home_team_name, away_team_name).expect("Couldn't create the game");
		let result_1 = sb.finish_game_with_literal_names(home_team_name, away_team_name);
		let result_2 = sb.get_summary();

		assert!(sb.data.is_empty());
		assert!(result_1.is_ok());
		assert_eq!(result_2, nothing_to_show);
	}

	#[test]
	fn adding_after_removal_works() {
		let home_team_name_1 = "Austria";
		let away_team_name_1 = "Belarus";
		let home_team_name_2 = "Cyprus";
		let away_team_name_2 = "Latvia";
		let expected_summary = vec![String::from("Cyprus 0 - Latvia 0")];

		let mut sb = ScoreBoard::new();
		sb.start_game_with_literal_names(home_team_name_1, away_team_name_1).expect("Couldn't create the first game");
		sb.finish_game_with_literal_names(home_team_name_1, away_team_name_1).expect("Couldn't finish the first game");
		let result_1 = sb.start_game_with_literal_names(home_team_name_2, away_team_name_2);
		let result_2 = sb.get_summary();

		assert_eq!(sb.data.len(), 1);
		assert!(result_1.is_ok());
		assert_eq!(result_2, expected_summary);
	}

	#[test]
	fn removal_on_empty_board_returns_an_error() {
		let home_team_name = "Jamaica";
		let away_team_name = "Nicaragua";
		let expected_error_message = "No such game found";
		let nothing_to_show: Vec<String> = Vec::new();

		let mut sb = ScoreBoard::new();
		let result_1 = sb.finish_game_with_literal_names(home_team_name, away_team_name);
		let result_2 = sb.get_summary();

		assert!(sb.data.is_empty());
		assert!(result_1.err().is_some_and(|result| result == expected_error_message));
		assert_eq!(result_2, nothing_to_show);
	}

	#[test]
	fn mismatched_home_and_away_names_in_removal_return_error() {
		let home_team_name = "Quatar";
		let away_team_name = "Tajikistan";
		let expected_error_message = "No such game found";
		let expected_summary = vec![String::from("Quatar 0 - Tajikistan 0")];

		let mut sb = ScoreBoard::new();
		sb.start_game_with_literal_names(home_team_name, away_team_name).expect("Couldn't create the game");
		let result_1 = sb.finish_game_with_literal_names(away_team_name, home_team_name);
		let result_2 = sb.get_summary();

		assert_eq!(sb.data.len(), 1);
		assert!(result_1.err().is_some_and(|result| result == expected_error_message));
		assert_eq!(result_2, expected_summary);
	}

	#[test]
	fn removal_of_wrong_team_returns_an_error() {
		let home_team_name_1 = "Venezuela";
		let away_team_name_1 = "Suriname";
		let home_team_name_2 = "Cambodja";
		let away_team_name_2 = "Vietnam";
		let expected_error_message = "No such game found";
		let expected_summary = vec![String::from("Venezuela 0 - Suriname 0")];

		let mut sb = ScoreBoard::new();
		sb.start_game_with_literal_names(home_team_name_1, away_team_name_1).expect("Couldn't create the game");
		let result_1 = sb.finish_game_with_literal_names(away_team_name_2, home_team_name_2);
		let result_2 = sb.get_summary();

		assert_eq!(sb.data.len(), 1);
		assert!(result_1.err().is_some_and(|result| result == expected_error_message));
		assert_eq!(result_2, expected_summary);
	}
}
