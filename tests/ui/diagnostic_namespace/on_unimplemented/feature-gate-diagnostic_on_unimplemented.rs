// check-fail


#[diagnostic::on_unimplemented(message = "Foo")]
//~^ERROR use of undeclared crate or module `diagnostic
pub trait Bar {
}

fn main() {
}
