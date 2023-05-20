// check-fail
#[diagnostic::non_existing_attribute]
//~^ERROR use of undeclared crate or module `diagnostic
pub trait Bar {
}

#[diagnostic::non_existing_attribute(with_option = "foo")]
//~^ERROR use of undeclared crate or module `diagnostic
struct Foo;

fn main() {
}
