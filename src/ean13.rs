//EAN -13
use dioxus::prelude::*;
use crate::Route;
//pro algoritmus výpočtu kontrolní číslice EAN-13
use crate::algorithms::modulo_10_algorithm;
//pro čárové kódy
use barcoders::sym::ean13::EAN13;
use barcoders::generators::svg::SVG;

#[component]
pub fn Ean13() -> Element {

	let nav = use_navigator();

	let mut input_value = use_signal(|| String::new());

	//pro concat stringů na generování finálního barcode
	let mut full_ean_string = String::new();

	let mut ean13_gs_prefix = String::new();
	let mut ean13_vyrobce = String::new();
	let mut ean13_produkt = String::new();
	let mut ean13_control_digit = String::new();

	let mut has_error = false;
	let mut error_text = String::new();

	//proměná pro čárový kód
	let mut barcode_svg = "<p style='color: gray;'>Zde se zobrazí čárový kód po zadání EAN-13...</p>".to_string();

	//check if its empty
	if !input_value.is_empty(){

		let input_string = input_value();

		//check whether it is a number -> don't have to do it individually because they are not separated by a comma
		//have to make it u64 though because its 13 digits
		if input_string.parse::<u64>().is_err(){
			has_error = true;
			error_text = "V EAN-13 se vyskytují nečíselné znaky.".to_string();
		}

		if input_string.len() >= 3 && !has_error{
			//attempt to parse
			if let Some(val) = input_string.get(0..3){
				ean13_gs_prefix = val.to_string();
			}
		}

		if input_string.len() >= 7 && !has_error{
			//attempt to parse
			if let Some(val) = input_string.get(3..7){
				ean13_vyrobce = val.to_string();
			}
		}
		if input_string.len() >= 12 && !has_error{
			//attempt to parse
			if let Some(val) = input_string.get(7..12){
				ean13_produkt = val.to_string();
			}
			//calculate control digit
			//dont even have to check anything yahoo
			ean13_control_digit = modulo_10_algorithm(&input_string);
			//zformátování stringů dohromady
			full_ean_string = format!("{}{}", input_string, ean13_control_digit);
			//Vygenerování čárového kódu a dosazení na místo
			barcode_svg = match EAN13::new(&full_ean_string){
				Ok(ean) =>
				{
					let encoded = ean.encode();
					match SVG::new(40).generate(&encoded){
						Ok(res) => res,
						Err(_) => "<p style='color: red;'>Generování selhalo.</p>".to_string()
					}
				}
				Err(_) => 
				{
					"<p style='color: red;'>Generování selhalo.</p>".to_string()
				}
			};
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
                    h2 { class: "card-title text-2xl mb-4", "Analýza EAN-13 kódu" }

                    div { class: "form-control w-full",
                        label { class: "label",
                            span { class: "label-text font-semibold",
                                "Vložte prvních 12 čísel kódu EAN-13."
                            }
                        }
                        input {
                            r#type: "text",
                            placeholder: "Např: 859123456789",
                            class: {
                                if has_error {
                                    "input input-bordered input-error text-error input-lg w-full font-mono"
                                } else {
                                    "input input-bordered input-primary input-lg w-full font-mono"
                                }
                            },
                            oninput: move |evt| {
                                input_value.set(evt.value());
                            },
                            maxlength: "12",
                        }
                        // Zástupný prvek pro chybovou hlášku
                        if has_error {
                            label { class: "label py-0",
                                span { class: "label-text-alt text-error", "{error_text}" }
                            }
                        }
                    }

                    // VIZUALIZACE JEDNOTLIVÝCH ČÁSTÍ EAN
                    div { class: "mt-6 p-6 bg-base-200 rounded-box border border-base-300",
                        h3 { class: "text-sm font-bold uppercase tracking-widest text-center mb-6 text-base-content/70",
                            "Struktura načteného EAN"
                        }
                        div { class: "flex flex-wrap justify-center items-start gap-2 md:gap-4",

                            // GS1 prefi
                            div { class: "flex flex-col items-center min-w-[5rem] md:min-w-[6rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-primary bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm",
                                    "{ean13_gs_prefix}"
                                }
                                div { class: "text-xs mt-2 font-semibold", "GS1 Prefix" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1",
                                    "Země (např. ČR)"
                                }
                            }

                            div { class: "text-2xl md:text-3xl font-bold text-base-300 mt-2 md:mt-3",
                                "-"
                            }

                            // císlo výrobce
                            div { class: "flex flex-col items-center min-w-[5rem] md:min-w-[7rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-secondary bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm",
                                    "{ean13_vyrobce}"
                                }
                                div { class: "text-xs mt-2 font-semibold", "Výrobce" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1",
                                    "Registrační číslo firmy"
                                }
                            }

                            div { class: "text-2xl md:text-3xl font-bold text-base-300 mt-2 md:mt-3",
                                "-"
                            }

                            // Císlo položky
                            div { class: "flex flex-col items-center min-w-[5rem] md:min-w-[7rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-accent bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm",
                                    "{ean13_produkt}"
                                }
                                div { class: "text-xs mt-2 font-semibold", "Produkt" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1",
                                    "Identifikace zboží"
                                }
                            }

                            div { class: "text-2xl md:text-3xl font-bold text-base-300 mt-2 md:mt-3",
                                "-"
                            }

                            //kontrol cislice
                            div { class: "flex flex-col items-center min-w-[4rem] md:min-w-[5rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-error bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm",
                                    "{ean13_control_digit}"
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

            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8",
            	//matematický psotup
                div { class: "space-y-4",
                    h3 { class: "text-xl font-bold", "Matematický postup výpočtu" }
                    div { class: "mockup-code bg-base-300 text-base-content min-h-[220px]",
                        pre { "data-prefix": ">",
                            code { "EAN-13 používá Modulo 10 (stejně jako ISBN-13)." }
                        }
                        pre { "data-prefix": ">",
                            code { "Násobení vahou 1 a 3 zleva doprava:" }
                        }
                        pre { "data-prefix": ">",
                            code { "x₁*1 + x₂*3 + x₃*1 + x₄*3 ..." }
                        }
                        pre { "data-prefix": ">",
                            code { "10 - (Součet % 10) = Kontrolní číslice" }
                        }
                    }
                }
                //místo pro čárový kód
                div { class: "space-y-4",
                    h3 { class: "text-xl font-bold", "Čárový kód" }
                    div {
                        class: "w-full p-6 bg-white rounded-box border border-base-300 flex justify-center items-center overflow-hidden min-h-[220px]",
                        dangerous_inner_html: "{barcode_svg}",
                    }
                }
            }
        }
    }
}
