use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use crate::{data_types::Padding, graph::line::Line};

/*
    Graph

    This contains all the information needed to draw a graph
    on a canvas. It contains the canvas context, the origin
    point, the size of the graph, and the scale of the graph.
*/
#[wasm_bindgen]
#[derive(Clone)]
pub struct Graph {
    id: uuid::Uuid,
    parent: web_sys::HtmlElement,
    canvas: web_sys::HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    pub padding: Padding,
}

#[wasm_bindgen]
impl Graph {
    #[wasm_bindgen(constructor)]
    pub fn new(
        parent: web_sys::HtmlElement,
    ) -> Graph {

        // -- Create the Canvas Element
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        // -- Set the canvas width and height
        canvas.set_attribute("style", "width: 100%; height: 100%;").unwrap();

        // -- Append the canvas to the parent
        parent.append_child(&canvas).unwrap();

        // -- Get the canvas context
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        // -- Create the Graph
        Graph {
            id: uuid::Uuid::new_v4(),
            parent,
            canvas,
            ctx,
            padding: Padding::default(),
        }
    }

    #[wasm_bindgen]
    pub fn get_parent(&self) -> web_sys::HtmlElement {
        self.parent.clone()
    }

    #[wasm_bindgen]
    pub fn get_canvas(&self) -> web_sys::HtmlCanvasElement {
        self.canvas.clone()
    }

    #[wasm_bindgen]
    pub fn get_ctx(&self) -> CanvasRenderingContext2d {
        self.ctx.clone()
    }

    #[wasm_bindgen]
    pub fn draw_line(
        &self,
        line: &Line,
        largest_y: f64,
        smallest_y: f64,
    ) {
        line.render_line(
            &self,
            largest_y,
            smallest_y
        );
    }

    #[wasm_bindgen]
    pub fn get_width(&self) -> f64 {
        self.canvas.width() as f64
    }

    #[wasm_bindgen]
    pub fn get_height(&self) -> f64 {
        self.canvas.height() as f64
    }

    // -- This is used to re-calculate the canvas size
    //    Plus some other things that might come into
    //    play later
    #[wasm_bindgen]
    pub fn recalculate(&self) {
        // -- Set the canvas width and height
        self.canvas.set_width(self.parent.client_width() as u32);
        self.canvas.set_height(self.parent.client_height() as u32);
    }
}


impl Graph {
    pub fn get_size(&self) -> (f64, f64) {
        (
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        )
    }

    pub fn get_offset_size(&self) -> (f64, f64) {
        (
            self.canvas.width() as f64 - self.padding.left - self.padding.right,
            self.canvas.height() as f64 - self.padding.top - self.padding.bottom,
        )
    }

    pub fn get_offset(&self) -> (f64, f64) {
        (
            self.padding.left,
            self.padding.top,
        )
    }
}