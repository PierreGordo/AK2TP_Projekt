use dioxus::prelude::*;
//for algos
mod algorithms;
//for rodne cislo ui and functionality
mod rodne_cislo;
use rodne_cislo::Rodne_cislo;
//for logging - remove later when app complete
use tracing;
mod isbn;
use isbn::Isbn;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Home,

    #[route("/ISBN")]
    Isbn,

    #[route("/rodne_cislo")]
    Rodne_cislo,
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
                        "Generátor detekčních kódů"
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
                        button { class: "btn btn-outline btn-accent btn-lg", "EAN-13" }

                        button {
                            class: "btn btn-outline btn-info btn-lg",
                            //route here
                            onclick: move |_| {
                                nav.push(Route::Rodne_cislo {});
                            },
                            "IBAN"
                        
                        }
                    }
                }
            }
        }
    }
}

