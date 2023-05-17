use wasm_bindgen::prelude::wasm_bindgen;


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
