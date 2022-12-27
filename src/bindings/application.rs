use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    pub type Application;

    #[wasm_bindgen(constructor)]
    pub fn new(options: JsValue) -> Application;

    // Accessors
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Application) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn element(this: &Application) -> JsValue; // returns jQuery

    #[wasm_bindgen(method, getter)]
    pub fn template(this: &Application) -> String;

    #[wasm_bindgen(method, getter, js_name = popOut)]
    pub fn pop_out(this: &Application) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn rendered(this: &Application) -> bool;

    #[wasm_bindgen(method, getter)]
    pub fn title(this: &Application) -> String;

    // Methods
    #[wasm_bindgen(method, js_name = getData)]
    pub fn get_data(this: &Application, options: JsValue) -> JsValue; // returns an Object or a Promise to an Object

    #[wasm_bindgen(method)]
    pub fn render(this: &Application, force: bool) -> Application;

    #[wasm_bindgen(method, js_name = render)]
    pub fn render_with_options(this: &Application, force: bool, options: JsValue) -> Application;

    #[wasm_bindgen(method, js_name = activateListeners)]
    pub fn activate_listeners(this: &Application, html: JsValue); // html is jQuery

    #[wasm_bindgen(method, js_name = activateTab)]
    pub fn activate_tab(this: &Application, tab_name: String, options: JsValue);

    #[wasm_bindgen(method, js_name = bringToTop)]
    pub fn bring_to_top(this: &Application);

    #[wasm_bindgen(method)]
    pub fn close(this: &Application, options: JsValue);

    #[wasm_bindgen(method)]
    pub fn minimize(this: &Application);

    #[wasm_bindgen(method)]
    pub fn maximize(this: &Application);  
    
    #[wasm_bindgen(method, js_name = setPosition)]
    pub fn set_position(this: &Application, position: JsValue) -> JsValue;
}