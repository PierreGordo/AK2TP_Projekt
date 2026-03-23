use dioxus::prelude::*;
use crate::Route;

use crate::algorithms::ico_control_digit;


#[component]
pub fn Ico() -> Element {
    let nav = use_navigator();

    let mut input_value = use_signal(|| String::new());
    let mut input_string = String::new();

    let mut has_error = false;
    let mut error_text = String::new();

    let mut has_success = false;
    let mut sucess_text = String::new();

    let mut has_info = false;
    let mut info_text = String::new();
    
    let mut ico_base = String::new();
	let mut ico_cont_digit = String::new();

	
	if input_value.len() > 0 {
		input_string = input_value();
		//kontrola numericity
		if input_string.parse::<u32>().is_err(){
			has_error = true;
			error_text = "V poli se vyskytují nenumerické znaky.".to_string();
		}
		//check jestli uživatel zadal celé ičo
		if input_string.len() == 8 && !has_error{
			//vzatí prvních 7 znaků a passnutí na kalkulaci
			if let Some(val) = input_string.get(0..7){
				let control_digit = ico_control_digit(val);
				ico_base = val.to_string();
				//kontrola zda se rovná kontrolní číslice s zadanou kontrolní číslicí
				if let Some(val) = input_string.get(7..8){
					if val == control_digit{
						has_success = true;
						sucess_text = "Zadali jste validní IČO.".to_string();
						ico_cont_digit = control_digit;
					}
					else{
						has_error = true;
						error_text = "Zadali jste IČO se špatnou kontrolní číslicí.".to_string();
						ico_cont_digit = control_digit;
					}
				}
				
			}
			else{
				has_error = true;
				error_text = "Selhalo parsování".to_string();
			}
		}	
		//check jestli uživatel zadal ičo s kontrolní číslicí
		if input_string.len() == 7 && !has_error{
			//vzatí prvních 7 znaků a passnutí na kalkulaci
			if let Some(val) = input_string.get(0..7){
				ico_base = val.to_string();
				let control_digit = ico_control_digit(val);
				ico_cont_digit = control_digit;
				has_info = true;
				info_text = "Zadali jste validní IČO bez kontrolní číslice, která byla vypočtena.".to_string();
				
			}
			else{
				has_error = true;
				error_text = "Selhalo parsování".to_string();
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
                    h2 { class: "card-title text-2xl mb-4", "Analýza IČO" }

                    div { class: "form-control w-full",
                        label { class: "label",
                            span { class: "label-text font-semibold", "Vložte IČO." }
                        }
                        input {
                            r#type: "text",
                            placeholder: "Např: 25596641",
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
                            maxlength: "8",
                        }
                        if has_error {
                            label { class: "label py-0",
                                span { class: "label-text-alt text-error", "{error_text}" }
                            }
                        }
                        if has_success {
                            label { class: "label py-0",
                                span { class: "label-text-alt text-success", "{sucess_text}" }
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
                            "Struktura načteného IČO"
                        }
                        div { class: "flex flex-wrap justify-center items-start gap-2 md:gap-4",

                            div { class: "flex flex-col items-center min-w-[8rem] md:min-w-[10rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-primary bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm",
                                    "{ico_base}"
                                }
                                div { class: "text-xs mt-2 font-semibold", "Základní číslo" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1",
                                    "Pořadové identifikační číslo"
                                }
                            }

                            div { class: "text-2xl md:text-3xl font-bold text-base-300 mt-2 md:mt-3",
                                "-"
                            }

                            div { class: "flex flex-col items-center min-w-[4rem] md:min-w-[5rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-error bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm",
                                    "{ico_cont_digit}"
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

            div { class: "space-y-4",
                h3 { class: "text-xl font-bold", "Matematický postup výpočtu" }
                div { class: "mockup-code bg-base-300 text-base-content overflow-x-auto",
                    pre { "data-prefix": ">",
                        code { "1. Prvních 7 číslic se násobí váhami 8, 7, 6, 5, 4, 3, 2." }
                    }
                    pre { "data-prefix": ">",
                        code { "2. Vypočítá se součet těchto násobků." }
                    }
                    pre { "data-prefix": ">",
                        code { "3. Zbytek = Součet % 11." }
                    }
                    pre { "data-prefix": ">",
                        code { "4. K = 11 - Zbytek (se speciálními pravidly pro 0, 10, 1)." }
                    }
                }
            }
        }
    }
}
