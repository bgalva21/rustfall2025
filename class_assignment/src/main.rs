struct Student{
    name: String,
    major: String
}

impl Student{
    fn new (n:String, m:String ) -> Self{
        Self{
            name: n,
            major: m
        }
    }
    

    fn get_major(&self) ->&String {
        return &self.major;
    }

    fn set_major(&mut self, new_major : String){
        self.major = new_major;
    }
}

fn main(){
    let mut my_student = Student::new("Bob".to_string(),"CS".to_string());
    println!("{} is a {} major", my_student.name, my_student.get_major());

    my_student.set_major("business".to_string());

    println!("{} is a {} major", my_student.name, my_student.get_major());
}