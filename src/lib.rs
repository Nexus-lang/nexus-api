use clutils::errors::FileHandlerError;
use nexus_lib::{lexer::Lexer, parser::Parser, evaluator::Evaluator};

pub fn execute_program(path: &str) -> Result<(), FileHandlerError> {
    let mut lexer = Lexer::new(&path.into())?;
    let mut parser = Parser::new(&mut lexer);
    let mut evaluator = Evaluator::new();
    loop {
        let stmt = match parser.parse_stmt() {
            Ok(stmt) => stmt,
            Err(_) => break,
        };
        evaluator.eval_stmt(stmt);
        parser.next_token();
    }
    Ok(())
}