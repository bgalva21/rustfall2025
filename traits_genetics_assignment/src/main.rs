trait ShowInfo {
    fn show_info(&self);
}


trait Student: ShowInfo {
    fn gpa(&self) -> f32;
    fn major(&self) -> &str;
}

struct Undergrad {
    name: String,
    major: String,
    gpa: f32,
    year: u32,
}

impl ShowInfo for Undergrad {
    fn show_info(&self) {
        println!("Undergrad Student:");
        println!("  Name:  {}", self.name);
        println!("  Major: {}", self.major);
        println!("  GPA:   {:.2}", self.gpa);
        println!("  Year:  {}", self.year);
        println!();
    }
}

impl Student for Undergrad {
    fn gpa(&self) -> f32 {
        self.gpa
    }

    fn major(&self) -> &str {
        &self.major
    }
}

struct GradStudent {
    name: String,
    major: String,
    gpa: f32,
    thesis_title: String,
}

impl ShowInfo for GradStudent {
    fn show_info(&self) {
        println!("Graduate Student:");
        println!("  Name:   {}", self.name);
        println!("  Major:  {}", self.major);
        println!("  GPA:    {:.2}", self.gpa);
        println!("  Thesis: {}", self.thesis_title);
        println!();
    }
}

impl Student for GradStudent {
    fn gpa(&self) -> f32 {
        self.gpa
    }

    fn major(&self) -> &str {
        &self.major
    }
}


struct Enrollment {
    students: Vec<Box<dyn Student>>,
}

impl Enrollment {
    fn new() -> Self {
        Self {
            students: Vec::new(),
        }
    }

    fn enroll<S>(&mut self, student: S)
    where
        S: Student + 'static,
    {
        self.students.push(Box::new(student));
    }

    fn show_all(&self) {
        for s in &self.students {
            s.show_info();
        }
    }
}


fn main() {
    let u1 = Undergrad {
        name: "Alice".to_string(),
        major: "Computer Science".to_string(),
        gpa: 3.7,
        year: 2,
    };

    let g1 = GradStudent {
        name: "Bob".to_string(),
        major: "Data Science".to_string(),
        gpa: 3.9,
        thesis_title: "The best thesis every".to_string(),
    };

    let mut enrollment = Enrollment::new();
    enrollment.enroll(u1);
    enrollment.enroll(g1);

    enrollment.show_all();
}
