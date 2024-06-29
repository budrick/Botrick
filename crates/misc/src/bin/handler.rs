use std::collections::HashMap;

use color_eyre::eyre::Result;
// use std::future::Future;

/*
This is what impls will define
*/
trait Handler<T> {
    fn call(self);
}

/*
impl for Handler on FnOnce's
*/
impl<F> Handler<()> for F
where
    F: FnOnce() -> &'static str + Clone + Send + 'static,
{
    fn call(self) {
        println!("Func returned: {}", self())
    }
}

/*
trait for Handler on FnOnce taking an argument
*/
trait HandlerWithArgs<T> {
    fn call(self, s: &str) -> String;
}

/*
impl for Handler on FnOnce taking an argument
*/
impl<F> HandlerWithArgs<&str> for F
where
    F: FnOnce(&str) -> String + Clone + Send + 'static,
{
    fn call(self, s:&str) -> String {
        format!("Strung: {}", self(s))
    }
}

/*
A function we're going to impl Handler on
*/
fn testfn() -> &'static str {
    "Hello there"
}

fn testfnwithargs(s: &str) -> String {
    return s.to_string();
}

/*
A function that calls our function with impl Handler on it
*/
fn callfn<H>(f: H)
where 
    H: FnOnce() -> &'static str + Clone + Send + 'static,
{
    f.clone().call();
}

// STRATEGY:
// Impl Handler for desired function signature
// Create a Struct that represents a handler
// implement into_handler for FnOnce to convert it into a struct witha copy of the handler function


// Run our program in general
fn main() -> Result<()> {
    color_eyre::install()?;

    callfn(testfn);
    callfn(testfn);

    Ok(())
}
