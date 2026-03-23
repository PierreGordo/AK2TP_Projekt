use dioxus::prelude::*;
use crate::Route;
//pro algoritmus výpočtu kontrolní číslice rodného čísla
use crate::algorithms::calculate_rc_control_digit;

#[component]
pub fn Rodne_cislo() -> Element {
    let nav = use_navigator();

    //Vstup do input field
    let mut input_value = use_signal(|| String::new());

    //rok
    let mut rc_rok = String::new();
    let mut rc_mesic = String::new();
    let mut rc_den = String::new();
    let mut rc_koncovka = String::new();
    let mut rc_control_digit = String::new();

    //error handlign
    let mut has_error = false;
    let mut error_text = String::new();

	//Check zda je vstupní pole prázdné
    if !input_value.is_empty() {
    	//Vstup ve stringu pro jednodušší manipulaci
    	//Zavolání input_value() z toho vyplývá výsledek typ String
    	let input_string = input_value();
    	//je v řetězci dost znaků pro rok
    	if input_string.len() >= 2 && !has_error{
    		//experiment String.get() - non panicking slice
    		if let Some(val) = input_string.get(0..2){
    									//String.get returns str i think
    									//so i have to do to_string
				//check jestli se jedná o číslo
				//check zda se nevyskytují nečíselné charaktery
    			if val.parse::<u32>().is_err(){
    				has_error = true;
    				error_text = "V zadaném rodném čísle (políčko rok) se vyskytují nečíselné znaky.".to_string();
    			}
    			else{
    				rc_rok = val.to_string();
    			}
    		}
    	} 

    	//je v řetězci dost znaků pro měsíc
    	if input_string.len() >= 4 && !has_error{
    		//experiment String.get() - non panicking slice
    		if let Some(val) = input_string.get(2..4){
    									//String.get returns str i think
    									//so i have to do to_string
				//check jestli se jedná o číslo
				//check zda se nevyskytují nečíselné charaktery
    			if val.parse::<u32>().is_err(){
    				has_error = true;
    				error_text = "V zadaném rodném čísle (políčko měsíc) se vyskytují nečíselné znaky.".to_string();
    			}
    			else{
    				rc_mesic = val.to_string();
    			}
    		}
    	} 
    	//kontrola zda je dost pro den
    	if input_string.len() >= 6 && !has_error{
    		//experiment String.get() - non panicking slice
    		if let Some(val) = input_string.get(4..6){
    									//String.get returns str i think
    									//so i have to do to_string
				//check jestli se jedná o číslo
				//check zda se nevyskytují nečíselné charaktery
    			if val.parse::<u32>().is_err(){
    				has_error = true;
    				error_text = "V zadaném rodném čísle (políčko den) se vyskytují nečíselné znaky.".to_string();
    			}
    			else{
    				rc_den = val.to_string();
    			}
    			
    		}
    	} 
    	//kontrola zda je dost pro koncovku
    	if input_string.len() >= 9 && !has_error{
    		//experiment String.get() - non panicking slice
    		if let Some(val) = input_string.get(6..9){
    									//String.get returns str i think
    									//so i have to do to_string
				//check jestli se jedná o číslo
				//check zda se nevyskytují nečíselné charaktery
    			if val.parse::<u32>().is_err(){
    				has_error = true;
    				error_text = "V zadaném rodném čísle (políčko den) se vyskytují nečíselné znaky.".to_string();
    			}
    			else{
    					rc_koncovka = val.to_string();
    					//nyní už máme dostatečně dlouhý řetězec a ve správném tvaru
    					if let Some(rc_cd) = calculate_rc_control_digit(&input_string){
    						rc_control_digit = rc_cd.to_string();
    				}
    			}
    		}
    	}

    	
    }

    rsx! {
        div { class: "p-6 max-w-5xl mx-auto space-y-8",

            // Horní navigace / Zpět
            button {
                class: "btn btn-ghost gap-2",
                onclick: move |_| {
                    nav.push(Route::Home {});
                },
                "Zpět na výběr"
            }

            // HLAVNÍ KARTA S FORMULÁŘEM A VIZUALIZACÍ
            div { class: "card bg-base-100 shadow-xl border border-base-300",
                div { class: "card-body",
                    h2 { class: "card-title text-2xl mb-4", "Analýza rodného čísla (RČ)" }

                    div { class: "form-control w-full",
                        label { class: "label",
                            span { class: "label-text font-semibold",
                                "Vložte prvních 9 čísel rodného čísla bez lomítka."
                            }
                        }
                        input {
                            r#type: "text",
                            placeholder: "Např: 980215423",
                            // Zde by se logikou měnila třída na 'input-error' při špatném zadání
                            class: {
                                if has_error {
                                    "input input-bordered input-error text-error input-lg w-full font-mono"
                                } else {
                                    "input input-bordered input-primary input-lg w-full font-mono"
                                }
                            },
                            maxlength: "9",
                            oninput: move |evt| {
                                input_value.set(evt.value());
                            },
                        }
                        // error handling
                        if has_error {
                            label { class: "label py-0",
                                span { class: "label-text-alt text-error", "{error_text}" }
                            }
                        }
                    }

                    // VIZUALIZACE JEDNOTLIVÝCH ČÁSTÍ RČ
                    div { class: "mt-6 p-6 bg-base-200 rounded-box border border-base-300",
                        h3 { class: "text-sm font-bold uppercase tracking-widest text-center mb-6 text-base-content/70",
                            "Struktura načteného RČ"
                        }
                        div { class: "flex flex-wrap justify-center items-start gap-2 md:gap-4",

                            // Rok
                            div { class: "flex flex-col items-center min-w-[4rem] md:min-w-[5rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-primary bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm",
                                    "{rc_rok}"
                                }
                                div { class: "text-xs mt-2 font-semibold", "Rok" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1",
                                    "Rok narození"
                                }
                            }

                            // Měsíc
                            div { class: "flex flex-col items-center min-w-[4rem] md:min-w-[5rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-secondary bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm",
                                    "{rc_mesic}"
                                }
                                div { class: "text-xs mt-2 font-semibold", "Měsíc" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1",
                                    "Ženy mají +50"
                                }
                            }

                            // Den
                            div { class: "flex flex-col items-center min-w-[4rem] md:min-w-[5rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-accent bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm",
                                    "{rc_den}"
                                }
                                div { class: "text-xs mt-2 font-semibold", "Den" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1",
                                    "Den narození"
                                }
                            }

                            div { class: "text-2xl md:text-3xl font-bold text-base-300 mt-2 md:mt-3",
                                "/"
                            }

                            // Koncovka (Pořadové číslo)
                            div { class: "flex flex-col items-center min-w-[5rem] md:min-w-[6rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-info bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm",
                                    "{rc_koncovka}"
                                }
                                div { class: "text-xs mt-2 font-semibold", "Koncovka" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1",
                                    "Pořadí v daný den"
                                }
                            }

                            // Kontrolní číslice
                            div { class: "flex flex-col items-center min-w-[4rem] md:min-w-[5rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-error bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm",
                                    "{rc_control_digit}"
                                }
                                div { class: "text-xs mt-2 font-semibold", "Kontrola" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1",
                                    "Ověřovací číslice"
                                }
                            }
                        }
                    }
                }
            }

            // SPODNÍ ČÁST: POSTUP

            // Matematický postup
            div { class: "space-y-4",
                h3 { class: "text-xl font-bold", "Matematický postup výpočtu" }
                div { class: "mockup-code bg-base-300 text-base-content",
                    pre { "data-prefix": ">",
                        code { "Celé 10místné číslo musí být dělitelné 11." }
                    }
                    pre { "data-prefix": ">",
                        code { "V programování používáme operátor % (modulo)." }
                    }
                    pre { "data-prefix": ">",
                        code { "Zbytek po vydělení prvních 9 číslic číslem 11" }
                    }
                    pre { "data-prefix": ">",
                        code { "se stává kontrolní číslicí." }
                    }
                    pre { "data-prefix": ">",
                        code { "Výjimka: Pokud je zbytek 10," }
                    }
                    pre { "data-prefix": ">",
                        code { "kontrolní číslice je 0." }
                    }
                }
            }
        }
    }
}
