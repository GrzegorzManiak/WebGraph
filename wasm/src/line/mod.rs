use crate::data_types::{Point, Graph, GraphInitiator, DataPoint, Line};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use uuid::uuid;



#[wasm_bindgen]
pub struct LineGraph {
    lines: Vec<Line>,
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
            lines: Vec::new(),
            graph: Graph::new(
                parent,
                originator,
            ),
        }
    }



    #[wasm_bindgen]
    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line);
    }


    #[wasm_bindgen]
    pub fn draw(&self) {
        // -- Recalculate the graph
        self.graph.recalculate();
        
        // -- Draw the lines
        for line in &self.lines {
            self.graph.draw_line(line.clone());
        }
    }


    #[wasm_bindgen]
    pub fn get_graph(&self) -> Graph {
        self.graph.clone()
    }
}
