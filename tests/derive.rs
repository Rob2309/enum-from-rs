use from_rs::From;


#[derive(From)]
enum SomeEnum {
    VariantA(#[from] i32),
    VariantB{#[from] _a: i64},
}
