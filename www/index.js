// import * as wasm from "landing";
//
// wasm.run();

import {Buyer, PaymentGateway, Landing, Card} from "landing";

// const buyer = new Buyer("51994825959","027.555.030-323", "Andrew Rodrigues Brunoro", "16/05/1991", "91040220");
const landing = new Landing("OI_VANTAGENS", window.location.host);
const buyer   = new Buyer("51994825959","0272");
const payment = new PaymentGateway("1514", buyer, landing);
const card    = new Card("4444333322221111", "02", "2019", "065");

payment.card(card)
    .then((res) => {
        console.log(res)
    })
    .catch((err) => {
        console.log(err);
    });
