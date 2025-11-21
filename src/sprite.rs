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
}

impl UserData for Sprite {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| Ok(this.x));
        fields.add_field_method_get("y", |_, this| Ok(this.y));
        fields.add_field_method_get("file", |_, this| Ok(this.file.clone()));
        fields.add_field_method_get("size", |_, this| Ok(this.size));


        fields.add_field_method_set("x", |_, this, x:i32| Ok(this.x = x));
        fields.add_field_method_set("y", |_, this, y:i32| Ok(this.y = y));
        fields.add_field_method_set("size", |_, this, size:i32| Ok(this.size = size));
        fields.add_field_method_set("file", |_, this, file:String| Ok(this.file = file.clone()));
    }
}
