use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use crate::{data_types::{Point, Padding}, log};

/*
    Label

    This contains all the information needed to draw a label
    on a graph, this includes the font, colors, and other 
    text decorations, it comes preloaded with a lot of defaults
    depedning on what type of label it is.

    TODO: Finish this off, right now this is just a placeholder
*/
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Label {
    text: String,
    point: Point,
    pub padding: Padding,
    font: String,
    color: String,
    font_size: f64,
}

#[wasm_bindgen]
impl Label {
    #[wasm_bindgen(constructor)]
    pub fn new(
        text: String,
        point: Point,
        padding: Padding,
        font: String,
        color: String,
        font_size: f64,
    ) -> Label {
        Label {
            text,
            point,
            padding,
            font,
            color,
            font_size,
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
            font: "Arial".to_string(),
            color: "#2b2b2b".to_string(),
            font_size: 15.0,
        }
    }   



    #[wasm_bindgen]
    pub fn default_scale_label() -> Label {
        Label {
            text : "".to_string(),
            point: Point::new(0.0, 0.0),
            padding: Padding::default(),
            font: "Arial".to_string(),
            color: "#2b2b2b".to_string(),
            font_size: 15.0,
        }
    }


    #[wasm_bindgen]
    pub fn scale_label(
        font: Option<String>,
        color: Option<String>,
        font_size: Option<f64>,
        padding: Option<Padding>,
    ) -> Label {
        Label {
            text: "".to_string(),
            point: Point::new(0.0, 0.0),
            padding: padding.unwrap_or(Padding::default()),
            font: font.unwrap_or("Arial".to_string()),
            color: color.unwrap_or("#2b2b2b".to_string()),
            font_size: font_size.unwrap_or(15.0),
        }
    }

    #[wasm_bindgen]
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    #[wasm_bindgen]
    pub fn set_padding(&mut self, padding: Padding) {
        self.padding = padding;
    }
}



impl Label {
    
    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        ctx.save();
        self.set_styles(ctx);
        ctx.fill_text(&self.text, self.point.x, self.point.y).unwrap();
        ctx.restore();
    }

    pub fn set_styles(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_font(&format!("{}px {}", self.font_size, self.font));
        ctx.set_fill_style(&JsValue::from_str(&self.color));
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

    pub fn set(&mut self, text: String) {
        self.text = text;
    }
}