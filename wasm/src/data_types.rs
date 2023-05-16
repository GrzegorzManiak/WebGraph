use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use std::collections::HashMap;

/*
    Point 
    
    A simple point struct, used for multiple purposes
    such as defining a point on a graph
*/
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}



/*
    DataPoint

    A simple point struct, used for multiple purposes
    such as defining a point on a graph and a few other
    things.

    Point: The point on the graph
    Value: The value of the point
    Description: A description of the point
*/
#[wasm_bindgen]
#[derive(Clone)]
pub struct DataPoint {
    pub point: Point,
    pub value: f64,
    id: String,
}

#[wasm_bindgen]
impl DataPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(
        point: Point,
        value: f64,
    ) -> DataPoint {
        // -- Generate a new id
        let id = uuid::Uuid::new_v4().to_string();

        DataPoint {
            point,
            value,
            id,
        }
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}
    


/*
    GraphInitiator

    This just contains some basic parameters for the graph
    such as the origin, size, and scale. It is used to
    initialize a Graph.

    Origin: The origin point of the graph
    Size: The size of the graph
    Scale: The scale of the graph
*/
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct GraphInitiator {
    pub origin: Point,
    pub size: Point,
    pub scale: Point,
}

#[wasm_bindgen]
impl GraphInitiator {
    #[wasm_bindgen(constructor)]
    pub fn new(
        origin: Point,
        size: Point,
        scale: Point,
    ) -> GraphInitiator {
        GraphInitiator {
            origin,
            size,
            scale,
        }
    }
}



/*
    Graph

    This contains all the information needed to draw a graph
    on a canvas. It contains the canvas context, the origin
    point, the size of the graph, and the scale of the graph.
*/
#[wasm_bindgen]
pub struct Graph {
    parent: web_sys::HtmlElement,
    canvas: web_sys::HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    originator: GraphInitiator,
}

#[wasm_bindgen]
impl Graph {
    #[wasm_bindgen(constructor)]
    pub fn new(
        parent: web_sys::HtmlElement,
        originator: GraphInitiator,
    ) -> Graph {

        // -- Create the Canvas Element
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        // -- Append the canvas to the parent
        parent.append_child(&canvas).unwrap();

        // -- Get the canvas context
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        // -- Create the Graph
        Graph {
            parent,
            canvas,
            ctx,
            originator,
        }
    }

    #[wasm_bindgen]
    pub fn get_parent(&self) -> web_sys::HtmlElement {
        self.parent.clone()
    }

    #[wasm_bindgen]
    pub fn get_canvas(&self) -> web_sys::HtmlCanvasElement {
        self.canvas.clone()
    }

    #[wasm_bindgen]
    pub fn get_ctx(&self) -> CanvasRenderingContext2d {
        self.ctx.clone()
    }

    #[wasm_bindgen]
    pub fn get_originator(&self) -> GraphInitiator {
        self.originator.clone()
    }


    #[wasm_bindgen]
    pub fn draw_line(
        &self,
        svg_string: String,
        color: String,
        width: f64,
    ) {
        // -- Get the SVG Path
        let path = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "path")
            .unwrap()
            .dyn_into::<web_sys::SvgPathElement>()
            .unwrap();

        // -- Set the SVG Path
        path.set_attribute("d", &svg_string).unwrap();

        // -- Set the SVG Path Style
        path.set_attribute("stroke", &color).unwrap();
        path.set_attribute("stroke-width", &width.to_string()).unwrap();
        path.set_attribute("fill", "none").unwrap();

        // -- Append the SVG Path to the parent
        self.parent.append_child(&path).unwrap();
    }
}



/*
    DashStyle
*/
#[wasm_bindgen]
pub struct DashStyle {
    pub dash: bool,
    pub dash_length: f64,
    pub dash_spacing: f64,
}

#[wasm_bindgen]
impl DashStyle {
    #[wasm_bindgen(constructor)]
    pub fn new(dash: bool, dash_length: f64, dash_spacing: f64) -> DashStyle {
        DashStyle { dash, dash_length, dash_spacing, }
    }

    #[wasm_bindgen]
    pub fn default() -> DashStyle {
        DashStyle { dash: false, dash_length: 0.0, dash_spacing: 0.0, }
    }

    #[wasm_bindgen]
    pub fn apply(&self, line: web_sys::SvgPathElement) -> web_sys::SvgPathElement {
        if self.dash {
            line.set_attribute("stroke-dasharray", &format!("{}, {}", self.dash_length, self.dash_spacing)).unwrap();
        }
        line
    }
}



/*
    ArrowStyle
*/
#[wasm_bindgen]
pub struct ArrowStyle {
    pub arrow: bool,
    pub arrow_length: f64,
    pub arrow_width: f64,
    pub arrow_offset: f64,
}

#[wasm_bindgen]
impl ArrowStyle {
    #[wasm_bindgen(constructor)]
    pub fn new(arrow: bool, arrow_length: f64, arrow_width: f64, arrow_offset: f64) -> ArrowStyle {
        ArrowStyle { arrow, arrow_length, arrow_width, arrow_offset, }
    }

    #[wasm_bindgen]
    pub fn default() -> ArrowStyle {
        ArrowStyle { arrow: false, arrow_length: 0.0, arrow_width: 0.0, arrow_offset: 0.0, }
    }

    #[wasm_bindgen]
    pub fn apply(&self, line: web_sys::SvgPathElement) -> web_sys::SvgPathElement {
        if self.arrow {
            line.set_attribute("marker-end", "url(#arrow)").unwrap();
        }
        line
    }
}



/*
    Line

    This contains all the information needed to draw a line
    on a graph. It contains the color, width, as well as some 
    other information.
*/
#[wasm_bindgen]
pub struct Line {
    line: web_sys::SvgPathElement,

    color: String,
    width: f64,

    dash_style: DashStyle,
    arrow_style: ArrowStyle,
}

#[wasm_bindgen]
impl Line {
    #[wasm_bindgen(constructor)]
    pub fn new(
        color: String,
        width: f64,
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
            dash_style: DashStyle::default(),
            arrow_style: ArrowStyle::default(),
        }
    }



    #[wasm_bindgen]
    pub fn calculater_style(
        &self,
    ) {
        self.dash_style.apply(self.line.clone());
        self.arrow_style.apply(self.line.clone());
    }



    #[wasm_bindgen]
    pub fn set_point(
        &self,
        data: &DataPoint,
    ) {
        let mut svg_string = String::new();
        let point = data.point;
        svg_string.push_str(&format!("M {} {}", point.x, point.y));
        self.line.set_attribute("d", &svg_string).unwrap();
    }



    #[wasm_bindgen]
    pub fn get_line(&self) -> web_sys::SvgPathElement {
        self.calculater_style();
        self.line.clone()
    }
}