use convert_case::{Casing, Case};
use leptos::{*, web_sys::Event};

use crate::application::rule_elements::*;

#[component]
pub fn RustModuleApplication(cx: Scope) -> Element {
    let rule_elements = generate_rule_elements();
    let (rule_element, set_rule_element) = create_signal(cx, rule_elements[0].clone());

    view! {
        cx,
        <div>
            <div id="reLeftColumn" class="reColumn">
                <fieldset>
                    <legend>"Key"</legend>
                    <SelectKey rule_elements=rule_elements _set_rule_element=set_rule_element />
                    <i class="fas fa-info-circle large tooltipstered" id="reInfo"></i>
                </fieldset>
                <SelectOptions rule_element=rule_element />
            </div>
            // <RightColumn />
        </div>
    }
}

#[component]
fn SelectKey(cx: Scope, rule_elements: Vec<RuleElement>, _set_rule_element: WriteSignal<RuleElement>) -> Element {
    view! {
        cx,
        <select 
            id="reKey" 
            on:change=move |val: Event| {
                console_log(&format!("{val:?}\n{:?}\n{:?}", val.target(), val))
                // set_rule_element()
            }
            on:submit=move |val|
                console_log(&format!("{val:?}"))
        >
            <For each=move || rule_elements.clone() key=|rule_element| rule_element.clone()>
                {|cx, rule_element: &RuleElement| view! { cx,
                    <option value=format!("{rule_element:?}")>{ format!("{rule_element:?}").to_case(Case::Title)}</option>
                }}
            </For>
        </select>
    }
}

#[component]
fn SelectOptions(cx: Scope, rule_element: ReadSignal<RuleElement>) -> Element {
    view! {
        cx,
        <div>
            <For each=move || rule_element().fields() key=|field| field.clone()>
                {|cx, field: &Field| view! { cx,
                    <p>{format!("{field:?}")}</p>
                }}
            </For>
        </div>
    }
}

// #[component]
// fn RightColumn(cx: Scope) -> Element {
//     view! {
//         cx,
//         <div class="reColumn">
        
//         </div>
//     }
// }