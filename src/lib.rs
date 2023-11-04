#![allow(dead_code)]

use std::string::String;
use std::vec::Vec;

// *********************
// Public API functions
// *********************

pub struct ScoreBoard {
	data: Vec<Match>
}

impl ScoreBoard {
	pub fn new() -> ScoreBoard {
		ScoreBoard { data: Vec::new() }
	}

	pub fn start_match(&mut self, home_name: String, away_name: String) -> Result<(), String> {
		// TODO make sure the name isn't playing a match yet
		println!("Function start_match called with parameters: '{0}' and '{1}'", home_name, away_name);

		if home_name == away_name {
			return Err(format!("{} cannot play with itself", home_name));
		}

		let home_team = Team { name: home_name, score: 0 };
		let away_team = Team { name: away_name, score: 0 };
		let new_match = Match { home_team, away_team };
		self.data.push(new_match);
		Ok(())
	}

	pub fn update_score(text: String) -> i32 {
		println!("Function update_score called with parameter '{}'", text);
		return 0;
	}

	pub fn finish_match(text: String) -> i32 {
		println!("Function finish_match called with parameter '{}'", text);
		return 0;
	}

	pub fn get_summary(text: String) -> i32 {
		println!("Function get_summary called with parameter '{}'", text);
		return 0;
	}
}


// *****************************************
// Private library functions and structures
// *****************************************

struct Team {
	name: String,
	score: u8,
}

struct Match {
	home_team: Team,
	away_team: Team,
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

		assert_eq!(sb.data.len(), 0);
	}

	#[test]
	fn match_started_correctly() {
		let home_team_name = String::from("Monaco");
		let away_team_name = String::from("Switzerland");

		let mut sb = ScoreBoard::new();
		let result = sb.start_match(home_team_name.clone(), away_team_name.clone());

		assert!(result.is_ok());
		assert_eq!(sb.data.len(), 1);
		let m = sb.data.first().expect("First element is not available.");
		let Match { home_team: h, away_team: a} = m;
		assert_eq!(h.name, home_team_name);
		assert_eq!(h.score, 0);
		assert_eq!(a.name, away_team_name);
		assert_eq!(a.score, 0);
	}

	#[test]
	fn match_not_started_when_both_teams_have_the_same_name() {
		let home_team_name = String::from("Georgia");
		let away_team_name = String::from("Georgia");
		let expected_error_message = String::from("Georgia cannot play with itself");

		let mut sb = ScoreBoard::new();
		let result = sb.start_match(home_team_name.clone(), away_team_name.clone());

		assert!(result.is_err());
		assert!(result.err().is_some_and(|result| result == expected_error_message));
		assert_eq!(sb.data.len(), 0);
	}
}
