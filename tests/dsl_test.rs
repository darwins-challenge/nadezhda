#[macro_use]
extern crate nadezhda;

use nadezhda::grammar::Program;

#[test]
fn dsl_stop_should_work() {
    let program: Program = stop!();

    assert_eq!(program, Program::Stop);
}

#[test]
fn dsl_forward_should_work() {
    let program: Program = forward!(stop!());

    assert_eq!(program, Program::Forward(Box::new(Program::Stop)));
}

#[test]
fn dsl_backward_should_work() {
    let program: Program = backward!(stop!());

    assert_eq!(program, Program::Backward(Box::new(Program::Stop)));
}


#[test]
fn dsl_complex_expression_should_work() {
    let program: Program = forward!(forward!(forward!(backward!(stop!()))));

    assert_eq!(program, Program::Forward(
        Box::new(Program::Forward(
            Box::new(Program::Forward(
                Box::new(Program::Backward(
                    Box::new(Program::Stop)
                ))
            ))
        ))
    ));
}
