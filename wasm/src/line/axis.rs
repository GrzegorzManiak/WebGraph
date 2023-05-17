use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use crate::{graph::{Label, DashStyle, Graph}, log};

use super::Grid;
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
    grid: bool,
    pub extend_top: f64,
    pub extend_bottom: f64,
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
        grid: Option<bool>,
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
                Some(label) => Label::defualt_graph_label(label, None),
                None => Label::defualt_graph_label("".to_string(), None)
            },
            grid: match grid {
                Some(grid) => grid,
                None => true,
            },
        }
    }

    #[wasm_bindgen]
    pub fn default() -> XYAxis {
        XYAxis {
            color: String::from("#949494"),
            extend_top: 10.0,
            extend_bottom: 10.0,
            width: 1.5,
            grid: true,
            dash_style: DashStyle::default(),
            label: Label::defualt_graph_label("".to_string(), None),
        }
    }
}

impl XYAxis {
    pub fn draw_line(
        &self, 
        graph: &Graph,
        axis_type: AxisType,
        grid: &Grid,
        seperations: u32,
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
                offset_y - ex_bottom
            ),
        };

        // -- Draw the grid lines
        log(&format!("Drawing grid lines: {}", self.grid));
        log(&format!("Seperations: {}", seperations));
        for i in 0..seperations + 1 {
            ctx.begin_path();
            let (x1, y1, x2, y2) = match axis_type {
                AxisType::X => (
                    offset_x + (i as f64 * (offset_width / seperations as f64)),
                    offset_y - ex_bottom,
                    offset_x + (i as f64 * (offset_width / seperations as f64)),
                    offset_y + offset_height + ex_top,
                ),
                AxisType::Y => (
                    offset_x - ex_bottom,
                    offset_y + (i as f64 * (offset_height / seperations as f64)),
                    offset_x + offset_width + ex_top,
                    offset_y + (i as f64 * (offset_height / seperations as f64)),
                ),
            };

            ctx.move_to(x1, y1);
            ctx.line_to(x2, y2);

            ctx.set_line_width(grid.width);
            ctx.set_stroke_style(&JsValue::from_str(&grid.color));
            
            ctx.stroke();
        }

        // -- Draw the main axis lines
        ctx.begin_path();
        ctx.move_to(start_x, start_y);
        ctx.line_to(end_x, end_y);

        ctx.set_line_width(self.width);
        ctx.set_stroke_style(&JsValue::from_str(&self.color));
        ctx.stroke();
    }
}
