use wasm_bindgen::{prelude::*, JsObject};
use web_sys::{CanvasRenderingContext2d, SvgPathElement};
use std::{collections::HashMap, time};

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


    #[wasm_bindgen]
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    #[wasm_bindgen]
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }


    #[wasm_bindgen]
    pub fn normalize(
        &self,
        width: f64,
        height: f64,
    ) -> Point {
        let x = self.x / width;
        let y = self.y / height;

        Point::new(x, y)
    }
}



/*
    DataPoint

    A simple point struct, used for multiple purposes
    such as defining a point on a graph and a few other
    things.

    Point: The point on the graph
    Description: A description of the point
*/
#[wasm_bindgen]
#[derive(Clone)]
pub struct DataPoint {
    pub point: Point,
    pub scale_x: u32,
    pub scale_y: f64,
    id: String,
}

#[wasm_bindgen]
impl DataPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(
        scale_x: u32,
        scale_y: f64,
    ) -> DataPoint {
        // -- Generate a new id
        let id = uuid::Uuid::new_v4().to_string();

        DataPoint {
            point: Point::new(0.0, 0.0), 
            scale_x,
            scale_y,
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
#[derive(Clone)]
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

        // -- Set the canvas width and height
        canvas.set_attribute("style", "width: 100%; height: 100%;").unwrap();

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
        line: &Line
    ) {
        line.render_line(self);
    }

    #[wasm_bindgen]
    pub fn get_width(&self) -> f64 {
        self.canvas.width() as f64
    }

    #[wasm_bindgen]
    pub fn get_height(&self) -> f64 {
        self.canvas.height() as f64
    }

    // -- This is used to re-calculate the canvas size
    //    Plus some other things that might come into
    //    play later
    #[wasm_bindgen]
    pub fn recalculate(&self) {
        // -- Set the canvas width and height
        self.canvas.set_width(self.parent.client_width() as u32);
        self.canvas.set_height(self.parent.client_height() as u32);
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
    id: String,
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
            id: uuid::Uuid::new_v4().to_string(),
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
            id: uuid::Uuid::new_v4().to_string(),
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




/*
    Label

    This contains all the information needed to draw a label
    on a graph, this includes the font, colors, and other 
    text decorations, it comes preloaded with a lot of defaults
    depedning on what type of label it is.

    TODO: Finish this off, right now this is just a placeholder
*/
#[wasm_bindgen]
pub struct Label {
    text: web_sys::SvgTextElement,
    point: Point,
}

#[wasm_bindgen]
impl Label {
    #[wasm_bindgen(constructor)]
    pub fn new(
        text: String,
        point: Point,
    ) -> Label {
        Label {
            text: web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element_ns(Some("http://www.w3.org/2000/svg"), "text")
                .unwrap()
                .dyn_into::<web_sys::SvgTextElement>()
                .unwrap(),
            point,
        }
    }



    #[wasm_bindgen]
    pub fn defualt_graph_label(
        text: String,
        point: Option<Point>,
    ) -> Label {
        Label {
            text: web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element_ns(Some("http://www.w3.org/2000/svg"), "text")
                .unwrap()
                .dyn_into::<web_sys::SvgTextElement>()
                .unwrap(),
            point: point.unwrap_or(Point::new(0.0, 0.0)),
        }
    }
}