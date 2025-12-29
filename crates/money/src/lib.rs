mod bindings {
    wit_bindgen::generate!({
        path: "../../wit/money",
    });

    use super::MoneyCore;
    export!(MoneyCore);
}

mod money_core;

pub struct MoneyCore;
