//to stop non snakecase warnings
#![allow(non_snake_case)]
use dioxus::prelude::*;
//for algos
mod algorithms;
//for rodne cislo ui and functionality
mod rodne_cislo;
use rodne_cislo::Rodne_cislo;
//for isbn UI and functionality
mod isbn;
use isbn::Isbn;
//for ean-13 UI and functionality
mod ean13;
use ean13::Ean13;
//for IBAN UI and functionality
mod iban_page;
use iban_page::Iban_page;
//for ICO ui and functionality
mod ico;
use ico::Ico;
//for lohn ui and functionality
mod luhn;
use luhn::KreditniKarta;




#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Home,

    #[route("/ISBN")]
    Isbn,

    #[route("/rodne_cislo")]
    Rodne_cislo,

    #[route("/ean-13")]
    Ean13,

    #[route("/iban")]
    Iban_page,

    #[route("/ico")]
    Ico,

    #[route("/luhn")]
    KreditniKarta,
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    //for logging - remove later when app complete
    dioxus::launch(|| {
        rsx! {
            //tailwind css
            document::Stylesheet { href: TAILWIND_CSS }
            Router::<Route> {}
        }
    });
}

//Function for landing page ui, mainly ui in this one, so nothing to coment about
#[component]
fn Home() -> Element {
    //for navigation - encasing the whole button messes up the alighment for some reason
    let nav = use_navigator();

    rsx! {
        div { class: "hero min-h-screen bg-base-200",

            div { class: "hero-content text-center",

                div { class: "max-w-3xl",

                    h1 { class: "text-5xl font-bold text-primary mb-6",
                        "Kalkulátor a validátor kontrolních čísel"
                    }

                    p { class: "text-lg text-base-content/80 mb-8",
                        "Interaktivní nástroj pro analýzu nejběžnějších identifikačních kódů z každodenního života. "
                        "Aplikace demonstruje praktické využití modulo aritmetiky k zabezpečení dat proti chybám při jejich přenosu či ručním přepisu."
                    }

                    div { class: "divider text-base-content/60 font-semibold mb-8",
                        "Vyberte kód pro analýzu"
                    }

                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",

                        button {
                            class: "btn btn-outline btn-primary btn-lg",
                            onclick: move |_| {
                                nav.push(Route::Rodne_cislo {});
                            },
                            "Rodné číslo (RČ)"
                        }
                        button {
                            class: "btn btn-outline btn-secondary btn-lg",
                            onclick: move |_| {
                                nav.push(Route::Isbn {});
                            },
                            "ISBN-13"
                        }
                        button {
                            class: "btn btn-outline btn-accent btn-lg",
                            onclick: move |_| {
                                nav.push(Route::Ean13 {});
                            },
                            "EAN-13"
                        }

                        button {
                            class: "btn btn-outline btn-info btn-lg",
                            //route here
                            onclick: move |_| {
                                nav.push(Route::Iban_page {});
                            },
                            "IBAN"
                        
                        }
                        button {
                            class: "btn btn-outline btn-success btn-lg",
                            onclick: move |_| {
                                nav.push(Route::Ico {});
                            },
                            "IČO (Identifikační číslo osoby)"
                        }
                        button {
                            class: "btn btn-outline btn-warning btn-lg",
                            onclick: move |_| {
                                nav.push(Route::KreditniKarta {});
                            },
                            "Luhnův algoritmus (Platební karty)"
                        }
                    }
                }
            }
        }
    }
}

