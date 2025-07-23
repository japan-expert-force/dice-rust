use dice_rust::parser::Parser;

fn main() {
    let input = "2d100";
    let mut parser = Parser::new(input);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Parse error: {e}");
            return;
        }
    };
    println!("Parsed AST: {ast:?}");
}
