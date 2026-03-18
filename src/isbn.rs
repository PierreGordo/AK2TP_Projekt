//Postup:
/*
1) Vzít prvních 12 čísel ISBN
2) Střídavě násobit tyto čísla váhami 1 a 3
3) Sečtení takto vynásobených čísel
4) Výpočet zbytku po dělení tohoto čísla 10 (modulo)
5) Tento zbytek odečíst od čísla 10
*/

//Možná zde zakomponovat pro zajímavost i vydavatele? (čísla 6 až 9)

use dioxus::prelude::*;
use crate::Route;

//for debbuging
use std::any::{type_name, type_name_of_val};
//also for debbuging remove later 
fn print_type_of<T>(_: &T){
	tracing::info!("{}", type_name::<T>());
}


#[component]
pub fn Isbn() -> Element {
    let nav = use_navigator();
    
    // Zadaný vstup od uživatele
    let mut input_value = use_signal(|| String::new());	
	//Pole do kterých budu vkládat data
    let mut isbn_prefix = String::new();
    let mut isbn_group = String::new();
    let mut isbn_publisher = String::new();
    let mut isbn_publication = String::new();
    let mut isbn_check_digit = String::new();


	//signály pro error uživatele při vkládání ISBN
    let mut has_error = false;
    //pro text erroru
    let mut error_text = String::new();

	// ================ LOGIKA ZAZNAMENÁVÁNÍ DAT DO POLÍ ======================


	//check jestli je v poli něco
	if !input_value.is_empty(){
		//Chheck whether input string is parsable to i32 - e.g whether 
		// String vstupu od uživatele pro držení hodnoty
		let input_string = input_value();
		//pokus o split stringu -> musí tam být pomlky, jinak bych nevěděl co dát do jakého pole
		let mut fields_vec: Vec<&str> = input_string.split('-').collect();
		//remove all "" characters, occur when input is 1- the vec is ["", "1"]
		fields_vec.retain(|&elem| !elem.is_empty());

		//check polí - kolik jich tam je - zkoušel jsem tu match, ale vzhledem k tomu, že potřebuju porovnávat len s několika čísly tak
		// if se jeví jako lepší možnost
		if fields_vec.len() >= 1{		
				//this triggers if the value cant be parsed to i32 - meaning its not numeric
				if fields_vec[0].parse::<i32>().is_err(){
					has_error = true;
					error_text = "V prefixu ISBN se vyskytuje nečíselný charakter.".to_string();
				}
				else{
					//když není error, zapsat hodnotu
					isbn_prefix = fields_vec[0].to_string();
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
                    h2 { class: "card-title text-2xl mb-4", "Analýza ISBN-13 kódu" }

                    div { class: "form-control w-full",
                        label { class: "label",
                            span { class: "label-text font-semibold",
                                "Vložte prvních 12 čísel kódu ISBN (ISBN-13)"
                            }
                        }
                        //error handling v tomto inputu je velmi hloupý ale snad v
                        input {
                            r#type: "text",
                            placeholder: "Např: 978802000987",
                            //Přepínání vzhledu input fieldu pokud je error
                            class: {if has_error 
                            {"input input-bordered input-error text-error input-lg w-full font-mono"} 
                            else 
                            {"input input-bordered input-primary input-lg w-full font-mono"}},
                            maxlength: "15", // 17 to account for the - symbols, but formatted cant be more than 12/13 (without the control digit 12, with its 13)
                            //value: "{input_value}",
                            oninput: move |evt| {
                                input_value.set(evt.value());  
                            },
                        }
                        //popis případného erroru
                        if has_error {
                            label { class: "label py-0",
                                span { class: "label-text-alt text-error",
                                    "{error_text}"
                                }
                            }
                        }
                        label { class: "label",
                            span { class: "label-text-alt text-base-content/60",
                                "Zadávejte s pomlčkami"
                            }
                        }
                    }

					// VIZUALIZACE JEDNOTLIVÝCH ČÁSTÍ ISBN
                    div { class: "mt-6 p-6 bg-base-200 rounded-box border border-base-300",
                        h3 { class: "text-sm font-bold uppercase tracking-widest text-center mb-6 text-base-content/70", "Struktura načteného ISBN" }
                        div { class: "flex flex-wrap justify-center items-start gap-2 md:gap-4",
                            
                            // Prefix
                            div { class: "flex flex-col items-center w-20 md:w-24",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-primary bg-base-100 w-full h-12 md:h-14 flex items-center justify-center rounded shadow-sm", "{isbn_prefix}" }
                                div { class: "text-xs mt-2 font-semibold", "Prefix" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1", "EAN produktový kód" }
                            }
                            
                            div { class: "text-2xl md:text-3xl font-bold text-base-300 mt-2 md:mt-3", "-" }
                            
                            // Registrační skupina
                            div { class: "flex flex-col items-center w-16 md:w-20",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-secondary bg-base-100 w-full h-12 md:h-14 flex items-center justify-center rounded shadow-sm", "{isbn_group}" }
                                div { class: "text-xs mt-2 font-semibold", "Skupina" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1", "Země / Jazyk" }
                            }

                            div { class: "text-2xl md:text-3xl font-bold text-base-300 mt-2 md:mt-3", "-" }
                            
                            // Vydavatel
                            div { class: "flex flex-col items-center w-20 md:w-24",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-accent bg-base-100 w-full h-12 md:h-14 flex items-center justify-center rounded shadow-sm", "{isbn_publisher}" }
                                div { class: "text-xs mt-2 font-semibold", "Vydavatel" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1", "Identifikátor vydavatele" }
                            }

                            div { class: "text-2xl md:text-3xl font-bold text-base-300 mt-2 md:mt-3", "-" }
                            
                            // Publikace
                            div { class: "flex flex-col items-center w-24 md:w-28",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-info bg-base-100 w-full h-12 md:h-14 flex items-center justify-center rounded shadow-sm", "{isbn_publication}" }
                                div { class: "text-xs mt-2 font-semibold", "Publikace" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1", "Konkrétní kniha" }
                            }

                            div { class: "text-2xl md:text-3xl font-bold text-base-300 mt-2 md:mt-3", "-" }
                            
                            // Kontrolní číslice
                            div { class: "flex flex-col items-center w-16 md:w-20",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-error bg-base-100 w-full h-12 md:h-14 flex items-center justify-center rounded shadow-sm", "{isbn_check_digit}" }
                                div { class: "text-xs mt-2 font-semibold", "Kontrola" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1", "Ověřovací číslice" }
                            }
                        }
                    }
                }
            }

            // SPODNÍ ČÁST: POSTUP A EXPERIMENTÁLNÍ FUNKCE (API)
            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8",

                // Matematický postup
                div { class: "space-y-4",
                    h3 { class: "text-xl font-bold", "Matematický postup výpočtu" }
                    div { class: "mockup-code bg-base-300 text-base-content",
                        pre { "data-prefix": ">",
                            code { "ISBN-13 používá Modulo 10." }
                        }
                        pre { "data-prefix": ">",
                            code { "Násobení vahou 1 a 3 na střídačku:" }
                        }
                        pre { "data-prefix": ">",
                            code { "x₁*1 + x₂*3 + x₃*1 + x₄*3 ..." }
                        }
                        pre { "data-prefix": ">",
                            code { "10 - (Součet % 10) = Kontrolní číslice" }
                        }
                    }
                }

            }
        }
    }
}


fn calculate_control_digit(isbn: &str) -> String{
	//parse the string to numbers 
	//zde už víme, že máme plné 12 místné isbn, bez kontrolní číslice a zároveň víme, že se skutečně jedná o číslo.
	let mut sum: i32 = 0;
	let mut multiply_by_3: bool = false;
	
	for s_num in isbn.chars(){
		if multiply_by_3{
			//zde je 10, protože se pohybujeme v desítkové sustavě
			sum += (s_num.to_digit(10).unwrap() as i32) * 3;
			multiply_by_3 = !multiply_by_3;
		}
		else{
			sum += s_num.to_digit(10).unwrap() as i32;
			multiply_by_3 = !multiply_by_3;
		}
	}

	//zbytek po vydělení součtu 10 a odečtění tohoto čísla od čisla 10
	tracing::info!("Vypočteno {}", (10-(sum%10)).to_string());
	(10-(sum%10)).to_string()
	
}
