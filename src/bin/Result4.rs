#[derive(Debug)]
enum ProtectedLocation {
    All,
    Office,
    Warehouse,
}

impl ProtectedLocation {
    fn require_access_level(&self) -> u16 {
        match self {
            Self::All => 1000,
            Self::Office => 800,
            Self::Warehouse => 500,
        }
    }
}

#[derive(Debug)]
struct Database;

impl Database {
    fn connect() -> Result<Self, String> {
        Ok(Database)
    }

    fn find_employee(&self, name: &str) -> Result<Employee, String> {
        // let error = format!("{} not found", name); 
        match name {
            "Anita" => Ok(Employee {
                name: "Anita".to_string(),
            }),
            "Brody" => Ok(Employee {
                name: "Brody".to_string(),
            }),
            "Catherine" => Ok(Employee {
                name: "Catherine".to_string(),
            }),
            _ => Err(format!("{} not found", name)),
        }
    }

    fn get_keycard(&self, employee: &Employee) -> Result<Keycard, String> {
        match employee.name.as_str() { // as.str() hace referencia a la cadena de caracteres de la estructura Employee 
            "Anita" => Ok(Keycard { access_level: 1000 }), 
            "Brody" => Ok(Keycard { access_level: 800 }),
            other => Err(format!("{} has no keycard", other)),
        }
    }
}

#[derive(Debug)]
struct Employee {
    name: String,
}

#[derive(Debug)]
struct Keycard {
    access_level: u16,
}

#[derive(Clone, Copy, Debug)]
enum AuthorizationStatus {
    Allow,
    Deny,
}

fn authorize(
    employee_name: &str,
    location: ProtectedLocation,
) -> Result<AuthorizationStatus, String> {
    let db = Database::connect()?;

    let employee = db.find_employee(employee_name)?;

    let keycard = db.get_keycard(&employee)?;

    if keycard.access_level >= location.require_access_level() {
        Ok(AuthorizationStatus::Allow)
    } else {
        Ok(AuthorizationStatus::Deny)
    }
}

fn main() {
    let anita_authorized = authorize("Anita", ProtectedLocation::Warehouse);
    let brody_authorized = authorize("Brody", ProtectedLocation::Office);
    let catherine_authorized = authorize("Catherine", ProtectedLocation::All);

    println!("{:?}", anita_authorized);
    println!("{:?}", brody_authorized);
    println!("{:?}", catherine_authorized);
}
