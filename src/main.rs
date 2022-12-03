pub use hideable_proc_macros::Hideable;
pub use hideable_types::Hideable as HideableTrait;
use serde::Serialize;
pub use hideable_types::FieldAttribute::*;


pub use serde_json::to_value;
fn main() {
    println!("Hello, world!");
}

#[derive(Serialize, Clone)]
struct InnerStruct {
    pub a: u16
}

#[allow(dead_code)]
#[derive(Serialize, Clone)]
enum Enumeration {
    ABC,
    EDG
}

#[derive(Hideable, Serialize, Clone)]
struct TestStruct {
    #[mark(User)]
    pub a: String,
    #[mark(Employee)]
    pub b: String,
    #[mark(Employee, User)]
    pub c: InnerStruct,
    #[mark(User)]
    pub d: Enumeration
}


#[cfg(test)]
mod tests {

    use hideable_types::Hideable;

    use crate::TestStruct;

    #[test]
    fn test_test() {
        let a = TestStruct { a: "aa".into(), b: "".into(), c: crate::InnerStruct { a: 1 }, d: crate::Enumeration::ABC };
        print!("{:?}", a.hide_fields(vec!["User".into()]));
    }
}