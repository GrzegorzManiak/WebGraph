use wasm_bindgen::prelude::*;
use crate::{data_types::DataPoint, graph::{style::{DashStyle, ArrowStyle}, graph::Graph}};

use super::label::Label;

/*
    Line

    This contains all the information needed to draw a line
    on a graph. It contains the color, width, as well as some 
    other information.
*/
#[wasm_bindgen]
pub struct Line {
    line: web_sys::SvgPathElement,
    points: Vec<DataPoint>,

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
            line: web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element_ns(Some("http://www.w3.org/2000/svg"), "path")
                .unwrap()
                .dyn_into::<web_sys::SvgPathElement>()
                .unwrap(),
            color,
            width,
            columns: 0,
            points: Vec::new(),
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
            line: web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element_ns(Some("http://www.w3.org/2000/svg"), "path")
                .unwrap()
                .dyn_into::<web_sys::SvgPathElement>()
                .unwrap(),
            color: String::from("black"),
            width: 1.0,
            columns: 0,
            points: Vec::new(),
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
    ) {
        // -- Set the point Y, since thats the only thing that
        //    we currently know, X will be calculated later once
        //    we have all the points
        data.point.set_y(data.scale_y);

        // -- Append the DataPoint to the points vector
        self.points.push(data.clone());
    }



    #[wasm_bindgen]
    pub fn total_points(&self) -> u32 {
        self.points.len() as u32
    }


    #[wasm_bindgen]
    pub fn set_columns(&mut self, columns: u32) {
        self.columns = columns;
    }



    #[wasm_bindgen]
    pub fn render_line(&self, graph: &Graph){
        // -- Sort the points based on the x value
        let points = &mut self.points.clone();
        points.sort_by(|a, b| a.point.x.partial_cmp(&b.point.x).unwrap());

        // -- Create the Path2D object
        let mut path = web_sys::Path2d::new().unwrap();
        let mut i = 0;

        // -- Check if theres any columns
        if self.columns < 1 { return; }

        let width = graph.get_width(); 
        let height = graph.get_height();
        let column_width = width / (self.columns - 1) as f64;

        // -- Get the largest Y and smallest Y
        let mut largest_y = 0.0;
        let mut smallest_y = 0.0;
        for point in points.to_owned() {
            if point.point.y > largest_y { largest_y = point.point.y; }
            if point.point.y < smallest_y { smallest_y = point.point.y; }
        }

        // -- Loop through all the points and set the X and Y
        for point in points {
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
        ctx.stroke_with_path(&path);
    }
}