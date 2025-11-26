// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this
enum Position{
    Maintenance,
    Marketing,
    Manager,
    Line,
    Kitchen,
    Assembly
}

struct Employee {
    emp_type: Position,
    employed: bool
}
fn has_access(employee: &Employee)-> Result<(),String>{
    match employee.employed{
        false => return Err("terminated".to_owned()),
        _ => (),
    }

    match employee.emp_type {
        Position::Maintenance => Ok(()),
        Position::Marketing => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("invalid position".to_owned())

    }
}

fn print_aceess(employee: &Employee) -> Result<(),String>{
    let attempt_access = has_access(employee)?;
    println!("access ok");
    Ok(())
}
fn main() {
    let new_employee = Employee {
        emp_type: Position::Manager,
        employed: true,
    };

    match print_aceess(&new_employee){
        Err(e) => println!("access denied: {:?}",e),
        _ => (),
    }
}
