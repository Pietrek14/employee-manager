pub mod add;
pub mod remove;
pub mod list;

pub enum Action {
	Nop,
	Quit,
	Help,
	Add(add::AddAction),
	Remove(remove::RemoveAction),
	List(list::ListAction),
}

pub fn execute(action: Action) {
	match action {
		Action::Add(params) => {
			println!("Added {} to {}", params.person, params.department);
		},
		Action::Remove(params) => {
			println!("Removed {} from {}", params.person, params.department);
		},
		Action::List(params) => {
			match params.department {
				Some(department) => {
					println!("Listing employees from {}", department);
				},
				None => {
					println!("Listing all employees");
				}
			}
		},
		Action::Quit => {
			println!("I actually don't know how to implement this one...");
		},
		Action::Help => {
			println!("Available actions:");
			println!("\tadd - adds employee to a department");
			println!("\tremove - removes employee from a department");
			println!("\tlist - list all employees or employees from just one department");
			println!("\tquit - quits the program");
			println!("\thelp - lists all available actions");
		},
		Action::Nop => ()
	}
}