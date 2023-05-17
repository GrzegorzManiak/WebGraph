use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use uuid::Uuid;

use crate::{
    graph::{DashStyle, Label, Graph, Line}, 
    data_types::{Point, Padding}, log
};

mod scaling;
use scaling::autoscale_graph;

mod axis;
use axis::{
    XYAxis,
    AxisType
};

#[wasm_bindgen]
pub struct Grid {
    color: String,
    width: f64,
}

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    pub fn new(
        color: String,
        width: f64,
    ) -> Grid {
        Grid {
            color,
            width,
        }
    }

    #[wasm_bindgen]
    pub fn default() -> Grid {
        Grid {
            color: "#949494".to_string(),
            width: 1.0,
        }
    }
}


#[wasm_bindgen]
pub struct LineGraph {
    id: Uuid,
    lines: HashMap<Uuid, Line>,
    graph: Graph,
    grid: Grid,
    labels: HashMap<u32, Label>,

    x_axis: XYAxis,
    y_axis: XYAxis,
}

#[wasm_bindgen]
impl LineGraph { 
    #[wasm_bindgen(constructor)]
    pub fn new(
        parent: web_sys::HtmlElement,
        grid: Option<Grid>,
        x_axis: Option<XYAxis>,
        y_axis: Option<XYAxis>,
    ) -> LineGraph {
        LineGraph {
            id: uuid::Uuid::new_v4(),
            lines: HashMap::new(),
            labels: HashMap::new(),
            graph: Graph::new(parent),

            x_axis: match x_axis {
                Some(axis) => axis,
                None => XYAxis::default(),
            },

            y_axis: match y_axis {
                Some(axis) => axis,
                None => XYAxis::default(),
            },

            grid: match grid {
                Some(grid) => grid,
                None => Grid::default(),
            },
        }
    }


    #[wasm_bindgen]
    pub fn set_label(
        &mut self, 
        label: String, 
        x_pos: u32,
        padding: Option<Padding>,
    ) { 
        // -- Set the padding to defualt if none is provided
        let padding = match padding {
            Some(padding) => padding,
            None => Padding::default(),
        };

        // -- Create the label
        let mut label = Label::defualt_graph_label(label, None);
        label.padding = padding;
        self.labels.insert(x_pos, label);

        // -- Recalculate the graph
        self.draw();
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
        if columns > 0 { return columns }
        2
    }



    #[wasm_bindgen]
    pub fn draw(&mut self) {
        // -- Get the number of columns
        let columns = self.get_columns();
        self.graph.padding = Padding::default();

        // -- Recalculate the graph
        self.graph.recalculate();

        // -- Calculate the Offsets for the
        //    Labels, Header, etc
        let bottom_offset = self.calculate_label_offset();
        self.graph.padding.top += self.y_axis.extend_top + 25.0;
        self.graph.padding.bottom += self.y_axis.extend_bottom;

        self.graph.padding.left += self.x_axis.extend_top;
        self.graph.padding.right += self.x_axis.extend_bottom;
        
        // -- Render the labels
        self.render_labels(columns, bottom_offset);
        let graph = &self.graph.clone();

        // -- Draw the lines
        let (
            max_data,
            min_data
        ) = self.get_min_max_y();

        // -- Draw the axis lines
        self.x_axis.draw_line(&graph, AxisType::X, &self.grid, columns - 1);
        self.y_axis.draw_line(&graph, AxisType::Y, &self.grid, 10);

        
        // -- Draw the lines
        self.draw_lines(columns, max_data, min_data);
    }


    #[wasm_bindgen]
    pub fn get_graph(&self) -> Graph {
        self.graph.clone()
    }
}



impl LineGraph {
    pub fn draw_lines(
        &mut self, 
        columns: u32,
        largest_y: f64,
        smallest_y: f64,
    ) -> (f64, f64) {

        // -- Reverse the lines, as the first line should be drawn 
        //    on top, etc
        let mut lines = self.lines.clone();
        let mut line_values = lines.values_mut().collect::<Vec<&mut Line>>();
        line_values.reverse();


        // -- Draw the lines
        for line in &mut line_values {
            line.set_columns(columns);
            self.graph.draw_line(&line, largest_y, smallest_y);
        }

        // -- Return the largest and smallest Y values
        (largest_y, smallest_y)
    }


    pub fn get_min_max_y(&self) -> (f64, f64) {
        let mut largest_y = 0.0;
        let mut smallest_y = 0.0;

        for line in &self.lines {
            let (largest, smallest) = line.1.get_largest_and_smallest_y();
            if largest > largest_y { largest_y = largest }
            if smallest < smallest_y { smallest_y = smallest }
        }

        (largest_y, smallest_y)
    }


    fn sort_labels(&self, reverse: bool) -> Vec<Label> {
        let mut keys = self.labels.keys().cloned().collect::<Vec<u32>>();
        keys.sort_by(|a, b| a.cmp(&b));

        // -- Return the labels
        if reverse { keys.reverse() }
        keys.iter().map(|key| self.labels.get(key).unwrap().clone()).collect()
    }



    pub fn calculate_label_offset(&mut self) -> f64 {
        let (mut label_max_height, mut label_max_width) = (0.0, 0.0);
        let context = self.graph.get_ctx(); 
        let mut label_pad_bottom = 0.0;

        for label in &self.labels {
            let (width, height) = label.1.get_padded_size(&context);
            if width > label_max_width { label_max_width = width }
            if height > label_max_height { label_max_height = height }

            // -- Check for the higest label bottom padding
            let bottom_padding = label.1.padding.bottom;
            if bottom_padding > label_pad_bottom { label_pad_bottom = bottom_padding }
        }

        label_max_width = label_max_width / 2.0;

        // -- Set the label offset
        self.graph.padding.set_padding_if_larger(
            Some(label_max_height), 
            Some(label_max_width),
            Some(label_max_width), 
            None,
        );

        // -- Return the label offset
        label_pad_bottom
    }



    pub fn render_labels(
        &mut self, 
        columns: u32,
        bottom_offset: f64,
    ) {
        // -- Grab the Graph context
        let context = self.graph.get_ctx();
        let mut labels = self.sort_labels(false);

        // -- Get the size of the canvas
        let (_width, height) = self.graph.get_size();
        let (offset_x, _offset_y) = self.graph.get_offset();
        let (offset_width, _offset_height) = self.graph.get_offset_size();

        // -- Check if theres enoguh labels to render
        if labels.len() < columns as usize {
            // -- How many labels are we off by?
            let off_by = columns - labels.len() as u32;

            // -- Pad the labels
            labels.extend((0..off_by)
                .map(|_| Label::defualt_graph_label("".to_string(), None)));
        }

        // -- Render the labels
        let column_width = offset_width / (columns - 1) as f64;
        let mut i = 0;

        for label in &mut self.labels {
            // -- Calculate the X position
            let x_pos = column_width * i as f64;
            let (text_width, _text_height) = label.1.get_size(&context);

            // -- Set the text into the center
            let x_pos = x_pos - (text_width / 2.0) + offset_x;
            
            // -- Set the X position
            label.1.set_position(Point { 
                x: x_pos, 
                y: height - bottom_offset
            });
            label.1.render(&context);
            i += 1;
        }
    }
}