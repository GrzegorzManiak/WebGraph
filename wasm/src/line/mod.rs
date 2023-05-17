use crate::data_types::{Point, Graph, GraphInitiator, DataPoint, Line, Label, DashStyle};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use uuid::uuid;


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
    lines: Vec<Line>,
    graph: Graph,
    labels: Vec<Label>,

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
            lines: Vec::new(),
            labels: Vec::new(),
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
    ) { 
        // -- Create the label
        let label = Label::new(label, Point { x: 0.0, y: 0.0 });

        // -- Bit un-orthodox, but the column that the lable
        //    will represent is dependent on the x_pos, eg
        //    if x_pos is 0, then the label will be the first
        //    column, if x_pos is 5, then the label will be
        //    the 6th column if 0,1,2,3,4,5 are the columns
        //    in the graph, it will also replace any existing
        //    label at that position
        if x_pos < self.labels.len() as u32 {
            self.labels[x_pos as usize] = label;
        } else {
            self.labels.push(label);
        }

        // -- Recalculate the graph
        self.draw();
    }


    #[wasm_bindgen]
    pub fn get_columns(&self) -> u32 {
        // -- Return the number of columns (if any)
        let mut columns = self.labels.len() as u32;
        if columns > 0 { return columns } 
       
        // -- If there are no labels, then return the number of
        //    points in the largest line
        for line in &self.lines {
            let points = line.total_points();
            if points > columns { columns = points }
        }

        // -- Return the number of columns
        columns
    }


    #[wasm_bindgen]
    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line);
    }


    #[wasm_bindgen]
    pub fn draw(&mut self) {
        // -- Recalculate the graph
        self.graph.recalculate();
        let columns = self.get_columns();

        // -- Draw the lines
        for line in &mut self.lines {
            line.set_columns(columns);
            self.graph.draw_line(line);
        }

        // -- Draw the axis lines

    }


    #[wasm_bindgen]
    pub fn get_graph(&self) -> Graph {
        self.graph.clone()
    }
}
