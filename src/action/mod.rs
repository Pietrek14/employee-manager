pub mod add;
pub mod remove;
pub mod list;
pub mod effect;

use crate::company::Company;
use effect::ActionEffect;

pub enum Action {
	Nop,
	Quit,
	Help,
	Add(add::AddAction),
	Remove(remove::RemoveAction),
	List(list::ListAction),
}

pub fn execute(action: Action, on: &mut Company) -> ActionEffect {
	match action {
		Action::Add(params) => {
			let department = on.entry(params.department.as_str().to_string()).or_insert(vec![]);

			(*department).push(params.person.as_str().to_string());

			println!("Added {} to {}", params.person, params.department);

			ActionEffect::Nop
		},
		Action::Remove(params) => {
			let department = on.get_mut(&params.department.as_str().to_string());

			match department {
				Some(department) => {
					let index = department.iter().position(|employee| *employee == params.person);

					match index {
						Some(index) => {
							(*department).remove(index);

							println!("Removed {} from {}", params.person, params.department);
						},
						None => {
							eprintln!("{} is not an employee in {}", params.person, params.department);
						}
					}
				},
				None => {
					eprintln!("Such a department as {} doesn't exist!", params.department);
				}
			}

			ActionEffect::Nop
		},
		Action::List(params) => {
			fn print_employees(employees: &Vec<String>) {
				let mut sorted = employees[..].to_vec();

				sorted.sort();

				for employee in sorted {
					println!("\t{}", employee);
				}
			}

			match params.department {
				Some(department) => {
					println!("Listing employees from {}", department);

					let employees = on.get(&department);

					if let Some(employees) = employees {
						print_employees(employees);
					} else {
						eprintln!("Such a department as {} doesn't exist!", department);
					}
				},
				None => {
					println!("Listing all employees");

					for (department, employees) in on {
						println!("{}", department);

						print_employees(employees);
					}
				}
			}

			ActionEffect::Nop
		},
		Action::Quit => {
			println!("Goodbye!");
			ActionEffect::Quit
		},
		Action::Help => {
			println!("Available actions:");
			println!("\tadd - adds employee to a department");
			println!("\tremove - removes employee from a department");
			println!("\tlist - list all employees or employees from just one department");
			println!("\tquit - quits the program");
			println!("\thelp - lists all available actions");

			ActionEffect::Nop
		},
		Action::Nop => ActionEffect::Nop
	}
}