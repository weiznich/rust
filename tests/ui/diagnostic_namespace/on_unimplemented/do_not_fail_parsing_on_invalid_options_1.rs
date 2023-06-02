#![feature(diagnostic_namespace, diagnostic_on_unimplemented)]
// check-fail

#[diagnostic::on_unimplemented(unsupported = "foo")]
//~^WARN Malformed on_unimplemented attribut
//~|WARN Malformed on_unimplemented attribut
trait Foo {}

#[diagnostic::on_unimplemented(message = "Baz")]
// todo: generate a warning here as well
struct Bar {}

#[diagnostic::on_unimplemented(message = "Boom", unsupported = "Bar")]
//~^WARN Malformed on_unimplemented attribut
//~|WARN Malformed on_unimplemented attribut
trait Baz {}

#[diagnostic::on_unimplemented(message = "Boom", on(_Self = "i32", message = "whatever"))]
//~^WARN Malformed on_unimplemented attribut
//~|WARN Malformed on_unimplemented attribut
trait Boom {}

#[diagnostic::on_unimplemented = "boom"]
//~^WARN Malformed on_unimplemented attribut
trait Doom {}

fn take_foo(_: impl Foo) {}
fn take_baz(_: impl Baz) {}
fn take_boom(_: impl Boom) {}

fn main() {
    take_foo(1_i32);
    //~^ERROR the trait bound `i32: Foo` is not satisfied
    take_baz(1_i32);
    //~^ERROR Boom
    take_boom(1_i32);
    //~^ERROR Boom
}
