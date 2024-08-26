use mb108_sys::mbRect;

#[derive(Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub(crate) fn from_mb(rc: &mbRect) -> Self {
        return Rect {
            x: rc.x,
            y: rc.y,
            width: rc.w,
            height: rc.h,
        };
    }
}
