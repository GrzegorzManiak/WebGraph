use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use crate::data_types::{Point, Padding};

/*
    Label

    This contains all the information needed to draw a label
    on a graph, this includes the font, colors, and other 
    text decorations, it comes preloaded with a lot of defaults
    depedning on what type of label it is.

    TODO: Finish this off, right now this is just a placeholder
*/
#[wasm_bindgen]
#[derive(Clone)]
pub struct Label {
    text: String,
    point: Point,
    padding: Padding
}

#[wasm_bindgen]
impl Label {
    #[wasm_bindgen(constructor)]
    pub fn new(
        text: String,
        point: Point,
        padding: Padding,
    ) -> Label {
        Label {
            text,
            point,
            padding,
        }
    }



    #[wasm_bindgen]
    pub fn defualt_graph_label(
        text: String,
        point: Option<Point>,
    ) -> Label {
        Label {
            text,
            point: point.unwrap_or(Point::new(0.0, 0.0)),
            padding: Padding::default(),
        }
    }



    #[wasm_bindgen]
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}



impl Label {
    pub fn get_padding(&self) -> Padding {
        self.padding
    }
    
    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        ctx.save();
        self.set_styles(ctx);
        ctx.fill_text(&self.text, self.point.x, self.point.y).unwrap();
        ctx.restore();
    }

    pub fn set_styles(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_fill_style(&JsValue::from_str("black"));
        ctx.set_font("20px Arial");
    }

    pub fn set_position(&mut self, point: Point) {
        self.point = point;
    }

    pub fn get_size(&self, ctx: &CanvasRenderingContext2d) -> (f64, f64) {
        ctx.save();
        self.set_styles(ctx);
        let size = ctx.measure_text(&self.text).unwrap();
        ctx.restore();
        
        (
            size.width(), 
            size.actual_bounding_box_ascent()
        )
    }

    pub fn get_padded_size(&self, ctx: &CanvasRenderingContext2d) -> (f64, f64) {
        let (width, height) = self.get_size(ctx);
        (
            width + self.padding.left + self.padding.right,
            height + self.padding.top + self.padding.bottom,
        )
    }
}