use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use uuid::{uuid, Uuid};

use crate::{
    graph::{DashStyle, Label, Graph, Line}, 
    data_types::{Point, Padding, DataPoint}, log
};
pub enum AxisType { X, Y, }

/*
    XYAxis

    This struct defines the style of the X/Y axis lines
    on the graph
*/
#[wasm_bindgen]
pub struct XYAxis {
    label: Label,
    color: String,
    width: f64,
    extend_top: f64,
    extend_bottom: f64,
    dash_style: DashStyle,
}

#[wasm_bindgen]
impl XYAxis {
    #[wasm_bindgen(constructor)]
    pub fn new(
        color: String,
        width: f64,
        extend_top: f64,
        extend_bottom: f64,
        label: Option<String>,
        dash_style: Option<DashStyle>,
    ) -> XYAxis {
        XYAxis {
            color,
            width,
            extend_top,
            extend_bottom,
            dash_style: match dash_style {
                Some(style) => style,
                None => DashStyle::default(),
            },
            label: match label {
                Some(label) => Label::new(
                    label, 
                    Point { x: 0.0, y: 0.0 },
                    Padding::default()
                ),
                None => Label::new(
                    String::from(""), 
                    Point { x: 0.0, y: 0.0 },
                    Padding::default()
                )
            },
        }
    }

    #[wasm_bindgen]
    pub fn default() -> XYAxis {
        XYAxis {
            color: String::from("black"),
            extend_top: 10.0,
            extend_bottom: 10.0,
            width: 1.0,
            dash_style: DashStyle::default(),
            label: Label::defualt_graph_label("".to_string(), None)
        }
    }
}

impl XYAxis {
    pub fn draw_line(
        &self, 
        graph: &Graph,
        axis_type: AxisType,
    ) {
        // -- Get the ctx
        let ctx = graph.get_ctx();
        ctx.save();

        // -- Set the style
        ctx.set_stroke_style(&JsValue::from_str(&self.color));
        ctx.set_line_width(self.width);

        // -- Calculate the start and end points
        let (offset_x, offset_y) = graph.get_offset();
        let (offset_width, offset_height) = graph.get_offset_size();
        
        let ex_top = self.extend_top;
        let ex_bottom = self.extend_bottom;


        let (start_x, start_y, end_x, end_y) = match axis_type {
            AxisType::X => (
                offset_x - ex_bottom, 
                offset_height + offset_y, 

                offset_x + offset_width + ex_top, 
                offset_height + offset_y
            ),
            AxisType::Y => (
                offset_x, 
                offset_y + offset_height + ex_top, 

                offset_x, 
                offset_y + ex_bottom
            ),
        };

        // -- Draw the line
        ctx.begin_path();
        ctx.move_to(start_x, start_y);
        ctx.line_to(end_x, end_y);
        ctx.stroke();
    }
}



#[wasm_bindgen]
pub struct LineGraph {
    id: Uuid,
    lines: HashMap<Uuid, Line>,
    graph: Graph,
    labels: HashMap<u32, Label>,

    x_axis: XYAxis,
    y_axis: XYAxis,
}

#[wasm_bindgen]
impl LineGraph { 
    #[wasm_bindgen(constructor)]
    pub fn new(
        parent: web_sys::HtmlElement,

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
        let label = Label::new(label, Point { x: 0.0, y: 0.0 }, padding);
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
        columns
    }



    #[wasm_bindgen]
    pub fn draw(&mut self) {
        // -- Get the number of columns
        let columns = self.get_columns();
        let graph = &self.graph.clone();
        self.graph.padding = Padding::default();

        // -- Recalculate the graph
        self.graph.recalculate();

        // -- Calculate the Offsets for the
        //    Labels, Header, etc
        let bottom_offset = self.calculate_label_offset();
        self.graph.padding.top += self.y_axis.extend_top;
        self.graph.padding.bottom += self.y_axis.extend_bottom;

        self.graph.padding.left += self.x_axis.extend_top;
        self.graph.padding.right += self.x_axis.extend_bottom;
        
        // -- Render the labels
        self.render_labels(columns, bottom_offset);

        // -- Draw the lines
        self.draw_lines(columns);

        // -- Draw the axis lines
        self.x_axis.draw_line(&graph, AxisType::X);
        self.y_axis.draw_line(&graph, AxisType::Y);
    }


    #[wasm_bindgen]
    pub fn get_graph(&self) -> Graph {
        self.graph.clone()
    }
}



impl LineGraph {
    pub fn draw_lines(&mut self, columns: u32) {

        // -- Find the largest Y and the Smallest Y
        //    between all the lines
        let mut largest_y = 0.0;
        let mut smallest_y = 0.0;

        for line in &self.lines {
            let (largest, smallest) = line.1.get_largest_and_smallest_y();
            if largest > largest_y { largest_y = largest }
            if smallest < smallest_y { smallest_y = smallest }
        }


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
            let bottom_padding = label.1.get_padding().bottom;
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
                .map(|_| Label::new(
                    String::from(""), 
                    Point { x: 0.0, y: 0.0 },
                    Padding::default())
                ));
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