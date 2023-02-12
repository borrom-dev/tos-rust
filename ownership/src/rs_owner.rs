
pub fn simple_owner() {

    let mut x = 32;
    let mut y = x;

    // The value is copy here for primitive types
    println!("x => {} y => {}", x, y);
    
    // The value is move here for non-primitive types
    let student = String::from("Lynda");
    let student_2 = student;

    // student is invalid out of life time
    // println!("{} => {}", student, student_2);
    println!("student => {}", student_2);

    println!("student => {}", student_2);

    let student = update(student_2);
    // The value here is now move
    // println!("{}", student_2);



    println!("Student {}", student);

    let mut greeting = String::from("Good morning");

    println!("{}", greeting);

    update_ref(&mut greeting);

    println!("{}", greeting);


}

 fn update(name: String) -> String {
    let other = name;
    other
}

 fn update_ref(name: &mut String) {
    name.push_str(" from Cambodia");
}
