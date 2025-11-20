mod goon_lang;

fn main() {
    println!("Running the Goon Machine");

    goon_lang::lexer::run();
    goon_lang::ast::run();
    goon_lang::interpreter::run();
}
