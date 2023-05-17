use wasm_bindgen::prelude::*;
use crate::data_types::Point;

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
    text: web_sys::SvgTextElement,
    point: Point,
}

#[wasm_bindgen]
impl Label {
    #[wasm_bindgen(constructor)]
    pub fn new(
        text: String,
        point: Point,
    ) -> Label {
        Label {
            text: web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element_ns(Some("http://www.w3.org/2000/svg"), "text")
                .unwrap()
                .dyn_into::<web_sys::SvgTextElement>()
                .unwrap(),
            point,
        }
    }



    #[wasm_bindgen]
    pub fn defualt_graph_label(
        text: String,
        point: Option<Point>,
    ) -> Label {
        Label {
            text: web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element_ns(Some("http://www.w3.org/2000/svg"), "text")
                .unwrap()
                .dyn_into::<web_sys::SvgTextElement>()
                .unwrap(),
            point: point.unwrap_or(Point::new(0.0, 0.0)),
        }
    }
}