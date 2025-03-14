// This should result in an error saying that the method signature of the
// implementation does not match the declaration.

library test;

trait Foo {
    fn foo(x: u64) -> str[7];
}

struct S {
    x: u64,
}

impl Foo for S {
    fn foo(s: str[7]) -> str[7] {
        s
    }
}
