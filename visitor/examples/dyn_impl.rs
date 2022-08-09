use visitor::dynamic::{Class, Database, Developer, Junior, ProjectElement, Senior, Test};

fn main() {
    let mut class = Class::default();
    let mut db = Database::default();
    let mut test = Test::default();

    let junior: Box<dyn Developer> = Box::new(Junior);
    let senior: Box<dyn Developer> = Box::new(Senior);

    println!("Task has been assigned to junior");
    println!("================================");
    class.be_written(&junior);
    db.be_written(&junior);
    test.be_written(&junior);

    println!("\nTask has been assigned to senior");
    println!("================================");
    class.be_written(&senior);
    db.be_written(&senior);
    test.be_written(&senior);
}
