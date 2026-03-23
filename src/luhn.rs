use dioxus::prelude::*;
use crate::Route;

use crate::algorithms::luhn_algo;

#[component]
pub fn KreditniKarta() -> Element {
    let nav = use_navigator();
    let mut input_value = use_signal(|| String::new());

    let mut input_string = String::new();
    
    let mut has_error = false;
    let mut error_text = String::new();

    let mut has_success = false;
    let mut success_text = String::new();

    let mut has_info = false;
    let mut info_text = String::new();
    
    let mut card_main_part = String::new();
    let mut card_control_digit = String::new();

    if input_value.len() > 0 {
    	input_string = input_value();

    	if input_string.parse::<u64>().is_err(){
    		has_error = true;
    		error_text = "V čísle karty se vyskytují nečíselné znaky.".to_string();
    	}

    	//lepší než len() -> len počitá bajty ne počet znaků
    	if input_string.chars().count() == 15 && !has_error{
    		//je třeba dopočíst kontrolní číslici
    		card_main_part = input_string.clone();
    		card_control_digit = luhn_algo(&input_string);
    		has_info = true;
    		info_text = "Vložili jste číslo karty bez kontrolní číslice a ta vám byla dopočtena.".to_string();
    	}
    	if input_string.chars().count() == 16 && !has_error{
    		//je třeba validovat kontrolní číslici
    		//bohužel .get() operuje na bázi bajtů, ne počtu znaků, naštestí se tam nemůže vyskytovat nic jiného jak čísla
    		if let Some(val) = input_string.get(0..15){
    			card_main_part = val.to_string();
    			let luhn_res = luhn_algo(val);
    			//kontrola zda se rovná vypočítaná číslice té zadané
    			if let Some(cd) = input_string.get(15..16){
    				if luhn_res == cd{
    					has_success = true;
    					success_text = "Zadali jste číslo karty se správnou kontrolní číslicí.".to_string();
    					card_control_digit = luhn_res;
    				}
    				else{
    					has_error = true;
    					error_text = "Zadali jste číslo karty s nesprávnou kontrolní číslicí. Správná je zobrazena níže.".to_string();
    					card_control_digit = luhn_res;
    				}
    			}
    		}
    	}
    }

    rsx! {
        div { class: "p-6 max-w-5xl mx-auto space-y-8",
            button {
                class: "btn btn-ghost gap-2",
                onclick: move |_| {
                    nav.push(Route::Home {});
                },
                "Zpět na výběr"
            }

            div { class: "card bg-base-100 shadow-xl border border-base-300",
                div { class: "card-body",
                    h2 { class: "card-title text-2xl mb-4",
                        "Analýza platební karty (Luhnův algoritmus)"
                    }

                    div { class: "form-control w-full",
                        label { class: "label",
                            span { class: "label-text font-semibold",
                                "Vložte číslo platební karty bez mezer. (podporovány pouze 15/16 místná čísla)"
                            }
                        }
                        input {
                            r#type: "text",
                            placeholder: "Např: 453201234567890",
                            class: {
                                if has_error {
                                    "input input-bordered input-error text-error input-lg w-full font-mono"
                                } else if has_success {
                                    "input input-bordered input-success text-success input-lg w-full font-mono"
                                } else if has_info {
                                    "input input-bordered input-info text-info input-lg w-full font-mono"
                                } else {
                                    "input input-bordered input-primary input-lg w-full font-mono"
                                }
                            },
                            oninput: move |evt| {
                                input_value.set(evt.value());
                            },
                            maxlength: "16",
                        }
                        if has_error {
                            label { class: "label py-0",
                                span { class: "label-text-alt text-error", "{error_text}" }
                            }
                        }
                        if has_success {
                            label { class: "label py-0",
                                span { class: "label-text-alt text-success", "{success_text}" }
                            }
                        }
                        if has_info {
                            label { class: "label py-0",
                                span { class: "label-text-alt text-info", "{info_text}" }
                            }
                        }
                    
                    }

                    div { class: "mt-6 p-6 bg-base-200 rounded-box border border-base-300",
                        h3 { class: "text-sm font-bold uppercase tracking-widest text-center mb-6 text-base-content/70",
                            "Struktura načtené karty"
                        }
                        div { class: "flex flex-wrap justify-center items-start gap-2 md:gap-4",

                            div { class: "flex flex-col items-center min-w-[16rem] md:min-w-[20rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-primary bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm",
                                    "{card_main_part}"
                                }
                                div { class: "text-xs mt-2 font-semibold", "Číslo karty" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1",
                                    "Základní číslo (bez kontrolní číslice)"
                                }
                            }

                            div { class: "text-2xl md:text-3xl font-bold text-base-300 mt-2 md:mt-3",
                                "-"
                            }

                            div { class: "flex flex-col items-center min-w-[4rem] md:min-w-[5rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-error bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm",
                                    "{card_control_digit}"
                                }
                                div { class: "text-xs mt-2 font-semibold", "Kontrola" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1",
                                    "Luhnův výpočet"
                                }
                            }
                        }
                    }
                }
            }

            div { class: "grid grid-cols-1 gap-8",
                div { class: "space-y-4",
                    h3 { class: "text-xl font-bold", "Postup výpočtu kontrolní číslice (Luhn)" }
                    div { class: "mockup-code bg-base-300 text-base-content overflow-x-auto",
                        pre { "data-prefix": ">",
                            code { "1. Postupuje se zprava doleva (od předposlední číslice)." }
                        }
                        pre { "data-prefix": ">",
                            code { "2. Každá druhá číslice se vynásobí dvěma." }
                        }
                        pre { "data-prefix": ">",
                            code { "3. Pokud je výsledek > 9, sečtou se jeho cifry (nebo se odečte 9)." }
                        }
                        pre { "data-prefix": ">",
                            code { "4. Všechny číslice se sečtou." }
                        }
                        pre { "data-prefix": ">",
                            code {
                                "5. Kontrolní číslici získáme dosazením součtu do vzorce: (10 - (součet % 10)) % 10"
                            }
                        }
                    }
                }
            }
        }
    }
}
