use wasm_bindgen::prelude::*;
use web_sys::{console, window, CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
struct Position {
    position_x: f64,
    position_y: f64,
}

#[wasm_bindgen]
struct Player {
    name: String,
    position: Position,
}

#[wasm_bindgen]
impl Player {
    pub fn new(name: String, position_x: f64, position_y: f64) -> Player {
        console::log_1(&std::format!("Creating player '{name}'").into());
        Player {
            name,
            position: Position {
                position_x,
                position_y,
            },
        }
    }

    // Getters
    pub fn position(&self) -> *const Position {
        &self.position
    }

    pub fn name_size(&self) -> usize {
        self.name.len()
    }

    pub fn name_ptr(&self) -> *const u8 {
        self.name.as_ptr()
    }

    // Draw to canvas element
    pub fn draw(&self, canvas_id: &str) {
        console::log_1(&std::format!("Drawing player '{}'", self.name).into());

        let document = window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();

        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        context.set_fill_style(&"green".into());
        context.set_font("25px sans-serif");

        context
            .fill_text(
                &self.name,
                self.position.position_x + 25.0,
                self.position.position_y + 25.0,
            )
            .unwrap();
        context.fill_rect(
            self.position.position_x.into(),
            self.position.position_y.into(),
            25.0,
            25.0,
        );
    }
}

// DOM-manipulation
#[wasm_bindgen]
pub fn add_greeting(root: &str) {
    let window = web_sys::window().expect("");
    let document = window.document().expect("");
    let root_node = document.get_element_by_id(root).expect("");
    root_node.set_text_content(Some("Hello world, from wasm!"));
}
