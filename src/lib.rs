/* 
Merges all the namespaces into one.
Probably horrible practice, but 
 - collisions haven't been a problem
 - its convenient not to have a bunch of imports
 - qualifying feels like clutter
 - rust-analyser shows all the relevant info on hover anyway
*/

mod bindings {
    mod application;
    pub use application::*;

    mod hooks;
    pub use hooks::*;

    mod notifications;
    pub use notifications::*;
}
pub use bindings::*;

mod data_structures;
pub use data_structures::*;

mod application {
    mod rust_module_application;
    pub use rust_module_application::*;

    mod rule_elements;
}
use application::{RustModuleApplication, RustModuleApplicationProps};

use leptos::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main() {
    Hooks::on(Ready, Closure::new(|_| 
        console_log("Rust Module | Hello From Rust!")
    ));
    Hooks::on(Ready, Closure::new(|_| {
        let options = ApplicationOptions {
            width: 750,
            height: 600,
            id: String::from("rust-module-application"),
            template: String::from("modules/rust-module/application.html"),
            title: String::from("Rust Module Application"),
            ..ApplicationOptions::default()
        };
            
        Application::new(to_value(&options).unwrap())
            .render(true);
        // log(&format!("Application: {app:#?}"))
    }));
    
    Hooks::on(RenderApplication, Closure::new(|sheet: Element| {
        if sheet.id() != "rust-module-application" {return}
        if let Some(application) = document().get_element_by_id(&sheet.id()) {
            mount(
                application
                    .children()
                    .get_with_index(1)
                    .unwrap()
                    .unchecked_into(),
                |cx| view! { cx, <RustModuleApplication /> }
            );
            info("Rust Module | Application Rendered");
            console_log(&format!("{application:?}"));
            console_log(&format!("{sheet:?}\n\n{}", sheet.id()));
        } else {
            info("The id matches, but the element couldn't be found");
        }
    }));
}
