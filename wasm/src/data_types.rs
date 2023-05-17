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

pub type DataPointMap = HashMap<Uuid, DataPoint>;

/*
    Padding

    A simple struct that defines the padding of a graph
    and other things such as labels
*/
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Padding {
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
    pub top: f64,
}

#[wasm_bindgen]
impl Padding {
    #[wasm_bindgen(constructor)]
    pub fn new(
        bottom: f64,
        left: f64,
        right: f64,
        top: f64,
    ) -> Padding {
        Padding {
            bottom,
            left,
            right,
            top,
        }
    }

    #[wasm_bindgen]
    pub fn default() -> Padding {
        Padding {
            bottom: 0.0,
            left: 0.0,
            right: 0.0,
            top: 0.0,
        }
    }
}


impl Padding {
    pub fn set_padding(
        &mut self,
        bottom: f64,
        left: f64,
        right: f64,
        top: f64,
    ) {
        // -- Set the padding
        self.bottom = bottom;
        self.left = left;
        self.right = right;
        self.top = top;
    }



    pub fn set_padding_if_larger(
        &mut self, 
        p_bottom: Option<f64>, 
        p_left: Option<f64>, 
        p_right: Option<f64>, 
        p_top: Option<f64>
    ) {
        if let Some(pb) = p_bottom {
            if pb > self.bottom { self.bottom = pb; }
        }
    
        if let Some(pl) = p_left {
            if pl > self.left { self.left = pl; }
        }
    
        if let Some(pr) = p_right {
            if pr > self.right { self.right = pr; }
        }
    
        if let Some(pt) = p_top {
            if pt > self.top { self.top = pt; }
        }
    }
}


