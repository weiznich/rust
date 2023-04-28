// check-pass
#[diagnostic::non_existing_attribute]
//~^WARN Unknown diagnostic attribute
pub trait Bar {
}

#[diagnostic::non_existing_attribute(with_option = "foo")]
//~^WARN Unknown diagnostic attribute
struct Foo;

fn main() {
}
