use godot::prelude::*;

// Определяем класс, который будет доступен в Godot
#[derive(GodotClass)]
#[class(base=Node)]
pub struct ChessAnalyzer {
    // Поля на Rust-стороне, могут быть любыми
    counter: i32,
}

#[godot_api]
impl INode for ChessAnalyzer {
    fn init(base: Base<Node>) -> Self {
        Self { counter: 0 }
    }
}

#[godot_api]
impl ChessAnalyzer {
    // Метод, который можно вызвать из GDScript
    #[func]
    fn hello_world(&self) -> String {
        "Привет из Rust!".to_string()
    }

    #[func]
    fn increment_counter(&mut self) {
        self.counter += 1;
    }

    #[func]
    fn get_counter(&self) -> i32 {
        self.counter
    }
}