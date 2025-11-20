mod rua;

fn main() {
    println!("Running main");
    rua::lexer::run();
    rua::ast::run();
    rua::interpreter::run();
}
