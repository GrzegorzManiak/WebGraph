use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::CanvasRenderingContext2d;


/*
    DashStyle
*/
#[wasm_bindgen]
#[derive(Clone, Copy)]
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
}

impl DashStyle {
    pub fn apply(&self, ctx: &CanvasRenderingContext2d) {
        if !self.dash { return; }

        let dash = js_sys::Array::new();
        dash.push(&self.dash_length.into());
        dash.push(&self.dash_spacing.into());

        ctx.set_line_dash(&dash).unwrap();
    }
}



/*
    ArrowStyle
*/
#[wasm_bindgen]
#[derive(Clone, Copy)]
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
}

impl ArrowStyle {
    pub fn apply(&self, ctx: &CanvasRenderingContext2d) {
        if !self.arrow { return; }

        let arrow_length = self.arrow_length;
        let arrow_width = self.arrow_width;
        let arrow_offset = self.arrow_offset;

        ctx.line_to(arrow_offset, arrow_width);
        ctx.line_to(arrow_offset, -arrow_width);
        ctx.line_to(arrow_offset + arrow_length, 0.0);
        ctx.line_to(arrow_offset, arrow_width);
    }
}