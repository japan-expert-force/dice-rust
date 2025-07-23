use dice_rust::analyzer::SemanticAnalyzer;

fn main() {
    let input = "2d100";
    let mut analyzer = match SemanticAnalyzer::new(input) {
        Ok(analyzer) => analyzer,
        Err(e) => {
            eprintln!("Error during semantic analysis: {:?}", e);
            return;
        }
    };
    match analyzer.analyze() {
        Ok(ast) => println!("Semantic analysis successful! ast: {:?}", ast),
        Err(e) => eprintln!("Semantic error: {:?}", e),
    }
}
