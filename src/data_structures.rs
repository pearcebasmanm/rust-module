#[derive(Default, Serialize)]
pub struct Position {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
    scale: f32
}

use serde::Serialize;

#[derive(Default, Serialize)]
pub struct ApplicationOptions {
    pub width: i32,
    pub height: i32,
    pub top: i32,
    pub left: i32,
    pub scale: f32,
    pub pop_out: bool,
    pub minimizable: bool,
    pub resizable: bool,
    pub id: String,
    pub template: String,
    pub title: String,
    pub new: bool
}

#[derive(Default, Serialize)]
pub struct RenderOptions {
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
    pub scale: f32,
    pub focus: bool,
    pub render_context: String,
}
