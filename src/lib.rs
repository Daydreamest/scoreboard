use std::string::String;

// *********************
// Public API functions
// *********************

pub fn start_match(text: String) -> String {
	println!("Function start_match called with parameter '{}'", text);
	return text;
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

// *****************************************
// Private library functions and structures
// *****************************************



// *****************************************
// Tests
// *****************************************

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_started() {
		let txt = "Test".to_string();
        let result = start_match(txt.clone());
        assert_eq!(result, txt);
    }
}