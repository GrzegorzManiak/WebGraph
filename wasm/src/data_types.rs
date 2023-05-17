use wasm_bindgen::{prelude::*, JsObject};
use web_sys::{CanvasRenderingContext2d, SvgPathElement};
use std::{collections::HashMap, time};
use uuid::{uuid, Uuid};

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


