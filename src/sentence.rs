use crate::action;
use action::Action;
use action::add::AddAction;
use action::remove::RemoveAction;
use action::list::ListAction;
use action::effect::ActionEffect;
use crate::company::Company;

pub fn execute(sentence: Vec<&str>, on: &mut Company) -> ActionEffect {
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
		"remove" => {
			if sentence_length != 4 {
				eprintln!("Incorrect syntax! Correct remove syntax: Remove [person] from [department]. Remember not to include spaces in arguments.");
				Action::Nop
			} else {
				if sentence[2].to_lowercase() != "from" {
					eprintln!("Incorrect syntax! Correct remove syntax: Remove [person] from [department]. Remember not to include spaces in arguments.");
					Action::Nop
				} else {
					Action::Remove(RemoveAction { person: sentence[1].to_string(), department: sentence[3].to_string() })
				}
			}
		},
		"list" => {
			match sentence_length {
				1 => {
					Action::List(ListAction { department: None })
				},
				3 => {
					if sentence[1].to_lowercase() != "from" {
						eprintln!("Incorrect syntax! Correct list syntax: List from [department]. Alternative: List. Remember not to include spaces in arguments.");
						Action::Nop
					} else {
						Action::List(ListAction {department: Some(sentence[2].to_string()) })
					}
				},
				_ => {
					eprintln!("Incorrect syntax! Correct list syntax: List from [department]. Alternative: List. Remember not to include spaces in arguments.");
					Action::Nop
				}
			}
		},
		"help" => {
			if sentence_length != 1 {
				eprintln!("Incorrect syntax! Correct add syntax: Help");
				Action::Nop
			} else {
				Action::Help
			}
		},
		"quit" => {
			if sentence_length != 1 {
				eprintln!("Incorrect syntax! Correct add syntax: Quit");
				Action::Nop
			} else {
				Action::Quit
			}
		}
		_ => {
			eprintln!("Unknown action: {}! To get the list of available actions, type in 'help'.", sentence[0]);
			Action::Nop
		}
	};

	action::execute(action, on)
}