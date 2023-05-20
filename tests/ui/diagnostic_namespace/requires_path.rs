#![feature(diagnostic_namespace)]
// check-fail

#[diagnostic]
//~^ERROR Diagnostic attributes requires a path with 2 segments
//~|ERROR Diagnostic attributes requires a path with 2 segments
//~|ERROR cannot find attribute `diagnostic` in this scope
pub struct Bar;


fn main() {
}
