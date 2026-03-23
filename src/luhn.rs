use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn KreditniKarta() -> Element {
    let nav = use_navigator();
    let mut input_value = use_signal(|| String::new());

    let has_error = false;
    let error_text = "Příklad chyby";
    
    let card_bin = "453201";
    let card_account = "23456789";
    let card_control_digit = "0";

    rsx! {
        div { class: "p-6 max-w-5xl mx-auto space-y-8",
            button {
                class: "btn btn-ghost gap-2",
                onclick: move |_| { nav.push(Route::Home {}); },
                "Zpět na výběr"
            }

            div { class: "card bg-base-100 shadow-xl border border-base-300",
                div { class: "card-body",
                    h2 { class: "card-title text-2xl mb-4", "Analýza platební karty (Luhnův algoritmus)" }

                    div { class: "form-control w-full",
                        label { class: "label",
                            span { class: "label-text font-semibold", "Vložte číslo platební karty." }
                        }
                        input {
                            r#type: "text",
                            placeholder: "Např: 453201234567890",
                            class: {if has_error 
                            {"input input-bordered input-error text-error input-lg w-full font-mono"} 
                            else 
                            {"input input-bordered input-primary input-lg w-full font-mono"}},
                            oninput: move |evt| { input_value.set(evt.value()); },
                            maxlength: "19",
                        }
                        if has_error {
                            label { class: "label py-0",
                                span { class: "label-text-alt text-error", "{error_text}" }
                            }
                        }
                    }

                    div { class: "mt-6 p-6 bg-base-200 rounded-box border border-base-300",
                        h3 { class: "text-sm font-bold uppercase tracking-widest text-center mb-6 text-base-content/70", "Struktura načtené karty" }
                        div { class: "flex flex-wrap justify-center items-start gap-2 md:gap-4",
                            
                            div { class: "flex flex-col items-center min-w-[6rem] md:min-w-[8rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-primary bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm", "{card_bin}" }
                                div { class: "text-xs mt-2 font-semibold", "BIN/IIN" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1", "Identifikace vydavatele" }
                            }
                            
                            div { class: "text-2xl md:text-3xl font-bold text-base-300 mt-2 md:mt-3", "-" }
                            
                            div { class: "flex flex-col items-center min-w-[10rem] md:min-w-[12rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-secondary bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm", "{card_account}" }
                                div { class: "text-xs mt-2 font-semibold", "Číslo účtu" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1", "Individuální číslo klienta" }
                            }

                            div { class: "text-2xl md:text-3xl font-bold text-base-300 mt-2 md:mt-3", "-" }
                            
                            div { class: "flex flex-col items-center min-w-[4rem] md:min-w-[5rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-error bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm", "{card_control_digit}" }
                                div { class: "text-xs mt-2 font-semibold", "Kontrola" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1", "Luhnův výpočet" }
                            }
                        }
                    }
                }
            }

            div { class: "grid grid-cols-1 gap-8",
                div { class: "space-y-4",
                    h3 { class: "text-xl font-bold", "Postup výpočtu kontrolní číslice (Luhn)" }
                    div { class: "mockup-code bg-base-300 text-base-content overflow-x-auto",
                        pre { "data-prefix": ">", code { "1. Postupuje se zprava doleva (od předposlední číslice)." } }
                        pre { "data-prefix": ">", code { "2. Každá druhá číslice se vynásobí dvěma." } }
                        pre { "data-prefix": ">", code { "3. Pokud je výsledek > 9, sečtou se jeho cifry (nebo se odečte 9)." } }
                        pre { "data-prefix": ">", code { "4. Všechny číslice se sečtou." } }
                        pre { "data-prefix": ">", code { "5. Součet modulo 10 musí být roven 0." } }
                    }
                }
            }
        }
    }
}
