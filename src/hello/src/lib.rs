use extendr_api::*;

#[extendr]
fn hello() -> &'static str {
    "hello"
}

#[extendr]
fn hello2() -> &'static str {
    "hello"
}


// Macro to generate exports
extendr_module! {
    mod hello;
    fn hello;
}

// Macro to generate exports
extendr_module! {
    mod hello2;
    fn hello2;
}
