slint::include_modules!();
use std::rc::Rc;
use slint::{ SharedString, ModelRc,  VecModel};



use serde::{Deserialize, Serialize};
// use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct CatAPIResponse {
    data: Vec<String>
}

fn get_cat_fact() -> String {
    let url = format!("https://meowfacts.herokuapp.com/");

    let data = reqwest::blocking::get(url).expect("request failed").text().expect("request failed");

    let parsed_first_cat_fact: CatAPIResponse = serde_json::from_str(&data).expect("request failed");

    return parsed_first_cat_fact.data[0].clone();
}


fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // ITEMS

    let  initial_items: Rc<VecModel<SharedString>> = Rc::new(VecModel::from(vec![ "Item 1".into(),
        "Item 2".into(),
        "Item 3".into(),
        "Item 4".into(),
        "Item 5".into(),
        "Item 6".into(),
        "Item 7".into(),
        "Item 8".into(),
        "Item 9".into(),
        "Item 10".into()]));

    let initial_items_rc =  ModelRc::from(initial_items.clone());

    ui.set_items(initial_items_rc);
    
    // ----------------------------
    
    // RANDOM CAT FACT


    let initial_cat_fact = SharedString::from( get_cat_fact() );

    ui.set_cat_fact(initial_cat_fact);

    ui.on_request_new_cat_fact({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_cat_fact(SharedString::from( get_cat_fact() ));
        }
    });


    
    // ----------------------------

    // COUNTER

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.on_request_decrease_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() - 1);
        }
    });

    // ----------------------------


    ui.run()
}
