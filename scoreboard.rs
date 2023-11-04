use std::string::String;

// *********************
// Public API functions
// *********************

pub fn start_match(text: String) -> () {
	println!("Function start_match called with parameter '{}'", text)
}

pub fn update_score(text: String) -> () {
	println!("Function update_score called with parameter '{}'", text)
}

pub fn finish_match(text: String) -> () {
	println!("Function finish_match called with parameter '{}'", text)
}

pub fn get_summary(text: String) -> () {
	println!("Function get_summary called with parameter '{}'", text)
}

// *****************************************
// Private library functions and structures
// *****************************************
