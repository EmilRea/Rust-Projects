use std::collections::HashMap;

#[derive(Debug)] // Prints the error when needed
pub enum ReportError {
    StudentNotFound,
}
pub trait Report {
 fn generate_report(&self) -> String;
}

struct Gradebook {
    grades: HashMap::<String, f32>,
    
}

impl Gradebook {
    fn new() -> Gradebook {
        Gradebook { grades: HashMap::new() }
    }
    
    pub fn add_grade(&mut self, name: String, grade: f32) {
        self.grades.insert(name, grade);
    }
    
    pub fn get_letter_grade(&self, name: &String) -> Result<char, ReportError> {
        let result = self.grades.get(name);
        match result {
            Some(value) => {
                if *value < 60.0 { return Ok('F'); }
                else if *value < 70.0 { return Ok('D'); }
                else if *value < 80.0 { return Ok('C'); }
                else if *value < 90.0 { return Ok('B'); }
                Ok('A')
            },
            None => Err(ReportError::StudentNotFound),
        }
    }
}

impl Report for Gradebook {
    fn generate_report(&self) -> String {
        let mut report: String = String::new();
        for (name, _) in &self.grades { 
            match self.get_letter_grade(name) {
                Ok(grade) => report.push_str(&format!("{}: {}\n", name, grade)),
                Err(_) => {}
            }
        }
        report
    }

}

fn main() {
    let mut gb = Gradebook::new();
    gb.add_grade(String::from("Alice"), 95.0);
    gb.add_grade(String::from("Bob"), 72.5);
    gb.add_grade(String::from("Charlie"), 60.2);

    // Test individual student lookup
    match gb.get_letter_grade(&String::from("Alice")) {
        Ok(grade) => println!("Alice's grade: {}", grade),
        Err(_) => println!("Student not found."),
    }

    match gb.get_letter_grade(&String::from("David")) {
        Ok(_) => (),
        Err(e) => println!("Failed to find David: {:?}", e),
    }

    // Test the Report trait
    println!("\n--- Class Report ---");
    println!("{}", gb.generate_report());
}