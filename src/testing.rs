use crate::{
    serial_print,
    serial_println,
    colors::Green,
};

/// This trait implements a `run` function that makes the test runner automatically print out tests
/// using appropriate coloring/descriptions
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    /// Run a testcase and provide proper output to the user
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("{}", Green("[ok]"));
    }
}

#[test_case]
/// Trivial test meant to pass
fn passing_test() {
    assert_eq!(1, 1);
}

#[test_case]
/// Invoke a breakpoint exception
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}

/*
#[test_case]
/// Trivial test meant to fail
fn failing_test() {
    assert_eq!(1, 0);
}

#[test_case]
fn stack_overflow_double_fault_should_panic() {
    fn stack_overflow() {
        stack_overflow();
    }
    stack_overflow();
}
*/
