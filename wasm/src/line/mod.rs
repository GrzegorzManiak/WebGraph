use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use uuid::{uuid, Uuid};

use crate::{
    graph::{DashStyle, Label, Graph, Line}, 
    data_types::{Point, GraphInitiator}
};


/*
    XYAxis

    This struct defines the style of the X/Y axis lines
    on the graph
*/
#[wasm_bindgen]
pub struct XYAxis {
    label: Label,
    color: String,
    width: u32,
    dash_style: DashStyle,
}

#[wasm_bindgen]
impl XYAxis {
    #[wasm_bindgen(constructor)]
    pub fn new(
        color: String,
        width: u32,
        label: Option<String>,
        dash_style: Option<DashStyle>,
    ) -> XYAxis {
        XYAxis {
            color,
            width,
            dash_style: match dash_style {
                Some(style) => style,
                None => DashStyle::default(),
            },
            label: match label {
                Some(label) => Label::new(label, Point { x: 0.0, y: 0.0 }),
                None => Label::new(String::from(""), Point { x: 0.0, y: 0.0 })
            },
        }
    }

    #[wasm_bindgen]
    pub fn default() -> XYAxis {
        XYAxis {
            color: String::from("black"),
            width: 1,
            dash_style: DashStyle::default(),
            label: Label::new(String::from(""), Point { x: 0.0, y: 0.0 }),
        }
    }
}



#[wasm_bindgen]
pub struct LineGraph {
    id: Uuid,
    lines: HashMap<Uuid, Line>,
    graph: Graph,
    labels: HashMap<Uuid, Label>,

    x_axis: XYAxis,
    y_axis: XYAxis,
}

#[wasm_bindgen]
impl LineGraph { 
    #[wasm_bindgen(constructor)]
    pub fn new(
        parent: web_sys::HtmlElement,
        originator: GraphInitiator,

        x_axis: Option<XYAxis>,
        y_axis: Option<XYAxis>,
    ) -> LineGraph {
        LineGraph {
            id: uuid::Uuid::new_v4(),
            lines: HashMap::new(),
            labels: HashMap::new(),
            graph: Graph::new(
                parent,
                originator,
            ),

            x_axis: match x_axis {
                Some(axis) => axis,
                None => XYAxis::default(),
            },
            y_axis: match y_axis {
                Some(axis) => axis,
                None => XYAxis::default(),
            },
        }
    }


    #[wasm_bindgen]
    pub fn set_label(
        &mut self, 
        label: String, 
        x_pos: u32,
    ) -> String { 
        // -- Create the label
        let label = Label::new(label, Point { x: 0.0, y: 0.0 });
        let id = uuid::Uuid::new_v4();

        self.labels.insert(id, label);

        // -- Recalculate the graph
        self.draw();
        id.to_string()
    }



    #[wasm_bindgen]
    pub fn add_line(
        &mut self, 
        line: Line
    ) -> String {
        // -- Add the line to the graph
        let id = uuid::Uuid::new_v4();
        self.lines.insert(id, line);
        
        // -- Return the ID of the line
        id.to_string()
    }



    #[wasm_bindgen]
    pub fn get_columns(&self) -> u32 {
        // -- Return the number of columns (if any)
        let mut columns = self.labels.len() as u32;
        if columns > 0 { return columns } 
       
        // -- If there are no labels, then return the number of
        //    points in the largest line
        for line in &self.lines {
            let points = line.1.total_points();
            if points > columns { columns = points }
        }

        // -- Return the number of columns
        columns
    }



    #[wasm_bindgen]
    pub fn draw(&mut self) {
        // -- Recalculate the graph
        self.graph.recalculate();
        let columns = self.get_columns();

        // -- Find the largest Y and the Smallest Y
        //    between all the lines
        let mut largest_y = 0.0;
        let mut smallest_y = 0.0;

        for line in &self.lines {
            let (largest, smallest) = line.1.get_largest_and_smallest_y();
            if largest > largest_y { largest_y = largest }
            if smallest < smallest_y { smallest_y = smallest }
        }

        // -- Draw the lines
        for line in &mut self.lines {
            line.1.set_columns(columns);
            self.graph.draw_line(&line.1, largest_y, smallest_y);
        }

        // -- Draw the axis lines

    }


    #[wasm_bindgen]
    pub fn get_graph(&self) -> Graph {
        self.graph.clone()
    }
}
