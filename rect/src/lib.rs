pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    pub fn area(&self) -> u32 {
        self.height * self.width
    }
    pub fn contain(&self, child: &Rectangle) -> bool {
        self.width > child.width && self.height > child.height
    }
}
