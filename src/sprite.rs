use std::{cell::RefCell, rc::Rc};
use mlua::UserData;

pub struct Sprite{
    file: String,
    x: i32,
    y: i32,
    size: i32,
}

impl Sprite{
    pub fn new(file: String, x: i32, y: i32, size: i32) -> Self {
        Sprite { file, x, y, size }
    }

    pub fn draw(&self){
        println!("Trying to draw {}", self.file);
    }
}

pub struct SpriteHandle(pub Rc<RefCell<Sprite>>);

impl UserData for SpriteHandle {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| Ok(this.0.borrow().x));
        fields.add_field_method_get("y", |_, this| Ok(this.0.borrow().y));
        fields.add_field_method_get("size", |_, this| Ok(this.0.borrow().size));
        fields.add_field_method_get("file", |_, this| Ok(this.0.borrow().file.clone()));


        fields.add_field_method_set("x", |_, this, x:i32| Ok(this.0.borrow_mut().x = x));
        fields.add_field_method_set("y", |_, this, y:i32| Ok(this.0.borrow_mut().y = y));
        fields.add_field_method_set("size", |_, this, size:i32| Ok(this.0.borrow_mut().size = size));
        fields.add_field_method_set("file", |_, this, file:String| Ok(this.0.borrow_mut().file = file.clone()));
    }
}
