use crate::action;
use action::Action;
use action::add::AddAction;

pub fn execute(sentence: Vec<&str>) {
	let sentence_length = sentence.len();

	let action = match sentence[0].to_lowercase().as_str() {
		"add" => {
			if sentence_length != 4 {
				eprintln!("Incorrect syntax! Correct add syntax: Add [person] to [department]. Remember not to include spaces in arguments.");
				Action::Nop
			} else {
				if sentence[2].to_lowercase() != "to" {
					eprintln!("Incorrect syntax! Correct add syntax: Add [person] to [department]. Remember not to include spaces in arguments.");
					Action::Nop
				} else {
					Action::Add(AddAction { person: sentence[1].to_string(), department: sentence[3].to_string() })
				}
			}
		},
		_ => {
			eprintln!("Unknown action: {}!", sentence[0]);
			Action::Nop
		}
	};

	action::execute(action);
}