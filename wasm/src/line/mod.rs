use crate::data_types::{Point, Graph, GraphInitiator, DataPoint};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use uuid::uuid;



#[wasm_bindgen]
pub struct LineGraph {
    points: Vec<DataPoint>,
    graph: Graph,

}

#[wasm_bindgen]
impl LineGraph { 
    #[wasm_bindgen(constructor)]
    pub fn new(
        parent: web_sys::HtmlElement,
        originator: GraphInitiator,
    ) -> LineGraph {
        LineGraph {
            points: Vec::new(),
            graph: Graph::new(
                parent,
                originator,
            ),
        }
    }

    #[wasm_bindgen]
    pub fn add_point(
        &mut self,
        point: DataPoint,
    ) {
        self.points.push(point);
    }
}
