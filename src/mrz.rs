use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn MrzPas() -> Element {
    let nav = use_navigator();
    let mut input_value = use_signal(|| String::new());

    let has_error = false;
    let error_text = "Chybný formát MRZ";
    
    let mrz_doc_num = "L898902C3";
    let mrz_dob = "740812";
    let mrz_expiry = "120415";
    let mrz_personal = "ZE184226B<<<<<";
    let mrz_overall_control = "2";

    rsx! {
        div { class: "p-6 max-w-5xl mx-auto space-y-8",
            button {
                class: "btn btn-ghost gap-2",
                onclick: move |_| { nav.push(Route::Home {}); },
                "Zpět na výběr"
            }

            div { class: "card bg-base-100 shadow-xl border border-base-300",
                div { class: "card-body",
                    h2 { class: "card-title text-2xl mb-4", "Analýza MRZ kódu (Pas - spodní řádek)" }

                    div { class: "form-control w-full",
                        label { class: "label",
                            span { class: "label-text font-semibold", "Vložte 44 znaků 2. řádku MRZ zóny." }
                        }
                        input {
                            r#type: "text",
                            placeholder: "Např: L898902C36UTO7408122F1204159ZE184226B<<<<<2",
                            class: {if has_error 
                            {"input input-bordered input-error text-error input-lg w-full font-mono uppercase"} 
                            else 
                            {"input input-bordered input-primary input-lg w-full font-mono uppercase"}},
                            oninput: move |evt| { input_value.set(evt.value()); },
                            maxlength: "44",
                        }
                        if has_error {
                            label { class: "label py-0",
                                span { class: "label-text-alt text-error", "{error_text}" }
                            }
                        }
                    }

                    div { class: "mt-6 p-6 bg-base-200 rounded-box border border-base-300",
                        h3 { class: "text-sm font-bold uppercase tracking-widest text-center mb-6 text-base-content/70", "Struktura načteného MRZ" }
                        div { class: "flex flex-wrap justify-center items-start gap-2 md:gap-4",
                            
                            div { class: "flex flex-col items-center min-w-[6rem]",
                                div { class: "text-xl md:text-2xl font-mono font-bold text-primary bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm", "{mrz_doc_num}" }
                                div { class: "text-xs mt-2 font-semibold", "Doklad" }
                            }
                            
                            div { class: "flex flex-col items-center min-w-[5rem]",
                                div { class: "text-xl md:text-2xl font-mono font-bold text-secondary bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm", "{mrz_dob}" }
                                div { class: "text-xs mt-2 font-semibold", "Narození" }
                            }

                            div { class: "flex flex-col items-center min-w-[5rem]",
                                div { class: "text-xl md:text-2xl font-mono font-bold text-accent bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm", "{mrz_expiry}" }
                                div { class: "text-xs mt-2 font-semibold", "Expirace" }
                            }

                            div { class: "flex flex-col items-center min-w-[8rem] md:min-w-[10rem]",
                                div { class: "text-xl md:text-2xl font-mono font-bold text-info bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm overflow-hidden", "{mrz_personal}" }
                                div { class: "text-xs mt-2 font-semibold", "Opt. Data" }
                            }
                            
                            div { class: "flex flex-col items-center min-w-[4rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-error bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm", "{mrz_overall_control}" }
                                div { class: "text-xs mt-2 font-semibold", "Globální KČ" }
                            }
                        }
                    }
                }
            }

            div { class: "grid grid-cols-1 gap-8",
                div { class: "space-y-4",
                    h3 { class: "text-xl font-bold", "Výpočet kontrolních znaků (MRZ)" }
                    div { class: "mockup-code bg-base-300 text-base-content overflow-x-auto",
                        pre { "data-prefix": ">", code { "1. Používá se opakující se váhový systém: 7, 3, 1." } }
                        pre { "data-prefix": ">", code { "2. Převod písmen: A=10, B=11 ... Z=35. Znak '<' = 0." } }
                        pre { "data-prefix": ">", code { "3. Každý znak se vynásobí příslušnou váhou a hodnoty se sečtou." } }
                        pre { "data-prefix": ">", code { "4. Kontrolní číslice = Součet % 10." } }
                    }
                }
            }
        }
    }
}
