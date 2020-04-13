mod fetch;

use wasm_bindgen::prelude::*;
use serde_json::json;
use js_sys::Promise;
use serde::{Deserialize, Serialize};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
struct Payload {
    phone: String,
    document: String,
    origin: String,
    plan_code: String,
    credit_card_number: Option<String>,
    credit_card_expiration_month: Option<String>,
    credit_card_expiration_year: Option<String>,
    credit_card_cvv: Option<String>,
    credit_card_zip: Option<String>,
    birthday: Option<String>,
}
impl Payload {

    pub fn card(pg: PaymentGateway, card: Card) -> Payload {
        Payload {
            phone: pg.buyer.phone,
            document: pg.buyer.document,
            origin: pg.landing.referer,
            plan_code: pg.plan_code,
            credit_card_number: Option::from(card.number),
            credit_card_expiration_month: Option::from(card.month),
            credit_card_expiration_year: Option::from(card.year),
            credit_card_cvv: Option::from(card.cvv),
            credit_card_zip: pg.buyer.zip_code,
            birthday: pg.buyer.birth_day,
        }
    }

    pub fn boleto (pg: PaymentGateway) -> Payload {
        Payload {
            phone: pg.buyer.phone,
            document: pg.buyer.document,
            origin: pg.landing.referer,
            plan_code: pg.plan_code,
            birthday: pg.buyer.birth_day,
            credit_card_number: Option::from(String::from("")),
            credit_card_expiration_month: Option::from(String::from("")),
            credit_card_expiration_year: Option::from(String::from("")),
            credit_card_cvv: Option::from(String::from("")),
            credit_card_zip: Option::from(String::from("")),
        }
    }

    pub fn json (&self) -> String {
        json!(self).to_string()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Landing {
    referer: String,
    host: String,
}

#[wasm_bindgen]
impl Landing {
    #[wasm_bindgen(constructor)]
    pub fn new(referer: String, host: String) -> Landing {
        Landing {
            referer,
            host,
        }
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Buyer {
    phone: String,
    document: String,
    name: Option<String>,
    birth_day: Option<String>,
    zip_code: Option<String>,
}

#[wasm_bindgen]
impl Buyer {
    #[wasm_bindgen(constructor)]
    pub fn new(phone: String, document: String, name: Option<String>, birth_day: Option<String>, zip_code: Option<String>) -> Buyer {
        Buyer { phone, document, name, birth_day, zip_code }
    }

    pub fn details(&self) -> JsValue {
        JsValue::from_serde(&self).unwrap()
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Default)]
pub struct Card {
    number: String,
    month: String,
    year: String,
    cvv: String,
    token: Option<String>,
}

#[wasm_bindgen]
impl Card {
    #[wasm_bindgen(constructor)]
    pub fn new(number: String,
                       month: String,
                       year: String,
                       cvv: String,
                       token: Option<String>) -> Card {
        Card {
            number,
            month,
            year,
            cvv,
            token,
        }
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct PaymentGateway {
    buyer: Buyer,
    landing: Landing,
    plan_code: String
}

#[wasm_bindgen]
impl PaymentGateway {
    #[wasm_bindgen(constructor)]
    pub fn new(plan_code: String, buyer: Buyer, landing: Landing) -> PaymentGateway {
        PaymentGateway {
            buyer,
            landing,
            plan_code
        }
    }

    pub fn card(self, card: Card) -> Promise {

        let payload: Payload = Payload::card(self, card);

        fetch::post_request("/", payload.json())
    }

    pub fn boleto(self) -> Promise {

        let payload: Payload = Payload::boleto(self);

        fetch::post_request("/", payload.json())
    }
}
