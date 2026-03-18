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

	// Used for storing the data since modyfing the Signal type input_value does weird stuff
    let mut input_string = String::new();
    
    // Zástupné signály pro vizualizaci rozpárování ISBN kódu
    // Zde později napojíš svou logiku pro rozdělení vstupního řetězce
    let mut isbn_prefix = use_signal(|| "978".to_string());
    let mut isbn_group = use_signal(|| "80".to_string());
    let mut isbn_publisher = use_signal(|| "200".to_string());
    let mut isbn_publication = use_signal(|| "0987".to_string());
    let mut isbn_check_digit = use_signal(|| "6".to_string());

    // Signály pro experimentální HTTP data
    let mut api_book_title = use_signal(|| String::new());
    let mut api_book_author = use_signal(|| String::new());
    let mut api_book_year = use_signal(|| String::new());

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
                        input {
                            r#type: "text",
                            placeholder: "Např: 978802000987",
                            class: "input input-bordered input-primary input-lg w-full font-mono",
                            maxlength: "15", // 17 to account for the - symbols, but formatted cant be more than 12/13 (without the control digit 12, with its 13)
                            //value: "{input_value}",
                            oninput: move |evt| {

                                //This evt.value func returns a string
                                input_value.set(evt.value());
								//isbn_prefix.set(evt.value());

								//try to trim out the - symbols between the numbers
								//does this copy or take ownership?
								input_string = evt.value();
								//this splits the number by -
								let binding = input_string.clone();
								let numbers_vec: Vec<&str> = binding.split('-').collect();
								//Set the input string to be without the -
								input_string = numbers_vec.join("");
								tracing::info!("{:?}", numbers_vec);
								
								
								//filter -
								tracing::info!("Formatted string is {input_string}");

								//Check jestli je zadaná hodnota numerická
								if input_string.trim().parse::<i64>().is_err() && input_value.len() != 0 {
									tracing::info!("GG není numerická, je tam něco jiného jak čísla a -");
								}
								else {
									//tato část je zvláštní, ale nenapadlo mě jak to jinak vyřešit a zabránit crashi
									//než tu naspamovat ty if statementy
									//nastavení prefixu
									if numbers_vec.len() >= 1{
										isbn_prefix.set(String::from(numbers_vec[0]));
									}
									//nastavení skupiny
									if numbers_vec.len() >= 2{
										isbn_group.set(String::from(numbers_vec[1]));
									}
									//nastavení vydavatele
									if numbers_vec.len() >= 3{
										isbn_publisher.set(String::from(numbers_vec[2]));
									}
									//nastavení publikace a výpočet kontrolní číslice
									if numbers_vec.len() >= 4{
										isbn_publication.set(String::from(numbers_vec[3]));

										isbn_check_digit.set(calculate_control_digit(&input_string));
									}
									
									
								}
								
                                
                                
								
                            },
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
                            
                            // Prefix (pouze u ISBN-13)
                            div { class: "flex flex-col items-center w-20 md:w-24",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-primary bg-base-100 w-full py-2 rounded shadow-sm text-center", "{isbn_prefix}" }
                                div { class: "text-xs mt-2 font-semibold", "Prefix" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1", "EAN produktový kód" }
                            }
                            
                            div { class: "text-2xl md:text-3xl font-bold text-base-300 mt-2 md:mt-3", "-" }
                            
                            // Registrační skupina
                            div { class: "flex flex-col items-center w-16 md:w-20",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-secondary bg-base-100 w-full py-2 rounded shadow-sm text-center", "{isbn_group}" }
                                div { class: "text-xs mt-2 font-semibold", "Skupina" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1", "Země / Jazyk" }
                            }

                            div { class: "text-2xl md:text-3xl font-bold text-base-300 mt-2 md:mt-3", "-" }
                            
                            // Vydavatel
                            div { class: "flex flex-col items-center w-20 md:w-24",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-accent bg-base-100 w-full py-2 rounded shadow-sm text-center", "{isbn_publisher}" }
                                div { class: "text-xs mt-2 font-semibold", "Vydavatel" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1", "Identifikátor vydavatele" }
                            }

                            div { class: "text-2xl md:text-3xl font-bold text-base-300 mt-2 md:mt-3", "-" }
                            
                            // Publikace
                            div { class: "flex flex-col items-center w-24 md:w-28",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-info bg-base-100 w-full py-2 rounded shadow-sm text-center", "{isbn_publication}" }
                                div { class: "text-xs mt-2 font-semibold", "Publikace" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1", "Konkrétní kniha" }
                            }

                            div { class: "text-2xl md:text-3xl font-bold text-base-300 mt-2 md:mt-3", "-" }
                            
                            // Kontrolní číslice
                            div { class: "flex flex-col items-center w-16 md:w-20",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-error bg-base-100 w-full py-2 rounded shadow-sm text-center", "{isbn_check_digit}" }
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

                // Experimentální funkce - HTTP Requesty (Získání informací o knize)
                div { class: "space-y-4",
                    h3 { class: "text-xl font-bold flex items-center gap-2", 
                        "Experimentální API funkce"
                        span { class: "badge badge-secondary badge-sm", "BETA" }
                    }
                    div { class: "card bg-base-200 shadow-sm border border-base-300",
                        div { class: "card-body p-5",
                            p { class: "text-sm text-base-content/70 mb-4", 
                                "Zde bude možné po kliknutí poslat HTTP GET request (např. na Google Books API) pro zjištění metadat o knize podle zadaného ISBN." 
                            }
                            
                            // Akční tlačítko pro request (Zatím bez logiky)
                            button { 
                                class: "btn btn-primary w-full mb-4",
                                // Zde přidáš async onclick pro fetch dat
                                "Vyhledat informace o knize v API"
                            }

                            // Formulář pro zobrazení výsledků z API
                            div { class: "space-y-3",
                                div { class: "form-control w-full",
                                    label { class: "label py-1", span { class: "label-text text-xs", "Název knihy" } }
                                    input { r#type: "text", class: "input input-bordered input-sm w-full", readonly: true, value: "{api_book_title}", placeholder: "Čeká na data..." }
                                }
                                div { class: "form-control w-full",
                                    label { class: "label py-1", span { class: "label-text text-xs", "Autor" } }
                                    input { r#type: "text", class: "input input-bordered input-sm w-full", readonly: true, value: "{api_book_author}", placeholder: "Čeká na data..." }
                                }
                                div { class: "form-control w-full",
                                    label { class: "label py-1", span { class: "label-text text-xs", "Rok vydání" } }
                                    input { r#type: "text", class: "input input-bordered input-sm w-full", readonly: true, value: "{api_book_year}", placeholder: "Čeká na data..." }
                                }
                            }
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
