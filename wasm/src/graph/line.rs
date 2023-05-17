use wasm_bindgen::prelude::*;
use std::{collections::HashMap, time};

use crate::{
    data_types::{DataPoint, DataPointMap}, 
    graph::{
        style::{DashStyle, ArrowStyle}, 
        graph::Graph
    }, 
    log
};

use super::label::Label;

/*
    Line

    This contains all the information needed to draw a line
    on a graph. It contains the color, width, as well as some 
    other information.
*/
#[wasm_bindgen]
#[derive(Clone)]
pub struct Line {
    points: DataPointMap,

    color: String,
    width: f64,

    label: Label,
    dash_style: DashStyle,
    arrow_style: ArrowStyle,
    columns: u32,
}

#[wasm_bindgen]
impl Line {
    #[wasm_bindgen(constructor)]
    pub fn new(
        color: String,
        width: f64,
        lable: Option<Label>,
        dash_style: Option<DashStyle>,
        arrow_style: Option<ArrowStyle>,
    ) -> Line {
        Line {
            color,
            width,
            columns: 0,
            points: DataPointMap::new(),
            label: lable.unwrap_or(
                Label::defualt_graph_label(
                    "".to_string(), 
                    Option::None
                )),
            dash_style: dash_style.unwrap_or(DashStyle::default()),
            arrow_style: arrow_style.unwrap_or(ArrowStyle::default()),
        }
    }



    #[wasm_bindgen]
    pub fn default() -> Line {
        Line {
            color: String::from("black"),
            width: 1.0,
            columns: 0,
            points: DataPointMap::new(),
            label: Label::defualt_graph_label(
                "".to_string(), 
                Option::None
            ),
            dash_style: DashStyle::default(),
            arrow_style: ArrowStyle::default(),
        }
    }



    #[wasm_bindgen]
    pub fn set_point(
        &mut self,
        data: &mut DataPoint,
    ) -> String {
        // -- Set the point Y, since thats the only thing that
        //    we currently know, X will be calculated later once
        //    we have all the points
        data.point.set_y(data.scale_y);

        // -- Check if another DataPoint exists on the same X scale
        //    if so, then we need to add the Y value to the existing
        //    DataPoint
        for (_, point) in self.points.iter_mut() {
            if point.scale_x != data.scale_x { continue; }
            point.point.set_y(data.scale_y);
            return point.get_id();
        }

        //  If the value is unique, then we need to add
        //  it to the DataPoint to the points vector
        let id = uuid::Uuid::new_v4();
        self.points.insert(id, data.clone());

        // -- Return the id of the point as a string
        id.to_string()
    }


    // -- Getters and Setters
    #[wasm_bindgen]
    pub fn get_color(&self) -> String { self.color.clone() }

    #[wasm_bindgen]
    pub fn set_color(&mut self, color: String) { self.color = color; }


    #[wasm_bindgen]
    pub fn get_width(&self) -> f64 { self.width }

    #[wasm_bindgen]
    pub fn set_width(&mut self, width: f64) { self.width = width; }


    #[wasm_bindgen]
    pub fn get_label(&self) -> Label { self.label.clone() }

    #[wasm_bindgen]
    pub fn set_label(&mut self, label: Label) { self.label = label; }
}



impl Line {
    pub fn sort(&self, reverse: bool) -> Vec<DataPoint> {
        // -- Get all the points, disregard the keys
        let mut points = self.points.iter().map(|(_, v)| v.clone()).collect::<Vec<_>>();

        // -- Sort the points based on the x value
        points.sort_by(|a, b| a.scale_x.partial_cmp(&b.scale_x).unwrap());

        // -- Reverse the points if needed
        if reverse { points.reverse(); }
        points
    }

    pub fn get_largest_and_smallest_y(&self) -> (f64, f64) {
        let mut largest_y = 0.0;
        let mut smallest_y = 0.0;

        // -- Loop through all the points and find the largest and smallest Y
        for (_, point) in self.points.iter() {
            if point.scale_y > largest_y { largest_y = point.scale_y; }
            if point.scale_y < smallest_y { smallest_y = point.scale_y; }
        }

        (largest_y, smallest_y)
    }

    pub fn total_points(&self) -> u32 {
        self.points.len() as u32
    }

    pub fn set_columns(&mut self, columns: u32) {
        self.columns = columns;
    }

    pub fn render_line(
        &self, 
        graph: &Graph,
        largest_y: f64,
        smallest_y: f64,
    ){
        // -- Get the points
        let points = self.sort(false);

        // -- Create the Path2D object
        let path = web_sys::Path2d::new().unwrap();
        let mut i = 0;

        // -- Check if theres any columns
        if self.columns < 1 { return; }

        let width = graph.get_width(); 
        let height = graph.get_height();
        let column_width = width / (self.columns - 1) as f64;

        // -- Loop through all the points and set the X and Y
        for mut point in points {

            // -- Set the point Y 
            point.point.set_y(point.scale_y);
            point.point.set_x(column_width * i as f64);

            // -- Normalize the Y value
            let normalized_y = (point.point.y - smallest_y) / (largest_y - smallest_y);
            let normalized_y = 1.0 - normalized_y;
            point.point.set_y(height * normalized_y);


            // -- Draw the line
            path.line_to(
                point.point.x.into(),
                point.point.y.into()
            );

            // -- Move the path to the next point
            i += 1;
        }


        let ctx = graph.get_ctx();
        ctx.set_stroke_style(&JsValue::from_str(&self.color));
        ctx.set_line_width(self.width);

        // -- Line styles
        self.dash_style.apply(&ctx);
        self.arrow_style.apply(&ctx);
        
        // -- Stroke the path
        ctx.stroke_with_path(&path);
    }
}