use wasm_bindgen::prelude::*;
use web_sys::{console, window, CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
struct Player {
    name: String,
    position_x: f64,
    position_y: f64,
}

//Public
#[wasm_bindgen]
impl Player {
    pub fn new(name: String) -> Player {
        console::log_1(&std::format!("Creating player '{name}'").into());
        Player {
            name: name,
            position_x: 0.0,
            position_y: 0.0,
        }
    }

    // Draw to canvas
    pub fn draw(&self) {
        console::log_1(&std::format!("Drawing player '{}'", self.name).into());

        let document = window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();

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
            .fill_text(&self.name, self.position_x + 25.0, self.position_y + 25.0)
            .unwrap();
        context.fill_rect(self.position_x.into(), self.position_y.into(), 25.0, 25.0);
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
