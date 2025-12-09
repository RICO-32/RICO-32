use mlua::UserData;

#[derive(Debug, Clone, Copy)]
pub struct MousePress {
    pub just_pressed: bool,
    pub pressed: bool,
    pub x: i32,
    pub y: i32,
}

impl Default for MousePress {
    fn default() -> Self {
        MousePress { just_pressed: false, pressed: false, x: -1, y: -1 }
    }
}

//Important so that lua can access them
impl UserData for MousePress {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("just_pressed", |_, this| Ok(this.just_pressed));
        fields.add_field_method_get("pressed", |_, this| Ok(this.pressed));
        fields.add_field_method_get("x", |_, this| Ok(this.x));
        fields.add_field_method_get("y", |_, this| Ok(this.y));
    }
}
