use crate::Route;
use dioxus::prelude::*;
//knihovna na IBAN, protože je to poměrně složité
use iban::{Iban, Bban};

//zřejmně nejjednodušší způsob konotroly kódu země - gigantický seznam -> jsou zde pouze země, které užívájí IBAN
pub const IBAN_COUNTRY_CODES: &[&str] = &[
    "AD", "AE", "AL", "AT", "AZ", "BA", "BE", "BG", "BH", "BI", "BR", "BY", "CH", "CR", "CY", "CZ",
    "DE", "DJ", "DK", "DO", "EE", "EG", "ES", "FI", "FO", "FR", "GB", "GE", "GI", "GL", "GR", "GT",
    "HR", "HU", "IE", "IL", "IQ", "IS", "IT", "JO", "KW", "KZ", "LB", "LC", "LI", "LT", "LU", "LV",
    "LY", "MC", "MD", "ME", "MG", "MK", "ML", "MN", "MR", "MT", "MU", "MZ", "NE", "NI", "NL", "NO",
    "OM", "PK", "PL", "PS", "PT", "QA", "RO", "RS", "RW", "SA", "SC", "SD", "SE", "SI", "SK", "SM",
    "SN", "SO", "ST", "SV", "SY", "TG", "TL", "TN", "TR", "UA", "VA", "VG", "XK",
];

//funkce na vyhledávání v seznamu pomocí binary search
pub fn is_valid_iban_country(code: &str) -> bool {
    IBAN_COUNTRY_CODES.binary_search(&code).is_ok()
}

#[component]
pub fn Iban_page() -> Element {
    let nav = use_navigator();

    // Vstup do input field
    let mut input_value = use_signal(|| String::new());

    // will hold the iban not formatted value
    let mut input_string = String::new();

    // Proměnné pro zobrazení
    let mut iban_country_code = String::new();
    let mut iban_check_digit = String::new();
    let mut iban_bank_code = String::new();
    let mut iban_account_number = String::new();

    // Error handling
    let mut has_error = false;
    let mut error_text = String::new();

    if !input_value.is_empty() {
        //bind input value to input string
        input_string = input_value();
        //convert input to list by spaces - if no spaces are included errors will be thrown
        //because of the lenght of the individual elements
        let mut fields_vec: Vec<&str> = input_string.split(' ').collect();
        //remove empty spaces -> ' ' from the vector
        //remove all spaces from the iban
        fields_vec.retain(|&c| !c.is_empty());
        
        //now opět ve starých kolejích -> kontrolování délky, přičemž první dva znaky budou country code
        if fields_vec.len() >= 1 {
            //check jestli je to validní IBAN country code
            if fields_vec[0].len() != 2{
            	has_error = true;
            	error_text = "Country kód musí mít přesně 2 znaky.".to_string();
            }
            else
            {
	            if is_valid_iban_country(fields_vec[0]) {
	                iban_country_code = fields_vec[0].to_string();
	            } 
	            else {
	                has_error = true;
	                error_text = "Vložili jste country kód (první 2 alfabetické znaky) buď ve špatném formátu, nebo jste nezadali zemi, která používá IBAR".to_string();
	            }
	        }
        }
        //další 4 čísla jsou číslem banky
        //HOAX, může to být 3-8 čísel a nemusí to bý ani čísla
        if fields_vec.len() >= 2 {
        	//kontrola délky
        	if !(fields_vec[1].len() >= 3 && fields_vec[1].len() <= 8){
        		has_error = true;
        		error_text = "Číslo banky musí mít nejméně 3 a nejvíce 8 číslic.".to_string();
        	}
        	else{      		
  	            //kontrola numericity
	            if fields_vec[1].parse::<u32>().is_err() {
	                has_error = true;
	                error_text = "V kódu banky se vyskytují nečíselné znaky.".to_string();
	            } else {
	                iban_bank_code = fields_vec[1].to_string();
	        	}
        	}
        }
        //dalších 16 čísel je čístlo účtu a předčíslí
        if fields_vec.len() >= 3 {
            //kontrola numericity - opťe je třeba použít u64 - moc čísel
            if fields_vec[2].parse::<u64>().is_err() {
                has_error = true;
                error_text = "V čísle účtu se vyskytují nečíselné znaky.".to_string();
            } else {
                iban_account_number = fields_vec[2].to_string();
				//pokud je vše ok sloučit jednotlivé komponenty vektoru a lupnout do Iban
				let joined_field_vec = fields_vec.join("");
				//let iban: IbanField = joined_field_vec.parse();
				if let Ok(res) = joined_field_vec.parse::<Iban>(){
					let iban: Iban = res;
				}
				else{
					has_error = true;
					error_text = "Invalidní číslo IBAN, nelze parsenout.".to_string();
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
                    h2 { class: "card-title text-2xl mb-4", "Analýza IBAN kódu" }

                    div { class: "form-control w-full",
                        label { class: "label",
                            span { class: "label-text font-semibold",
                                "Vložte kód IBAN k analýze (můžete i s mezerami). Vložte bez dvou čísel za znakem země, jedná se o kontrolní číslo."
                            }
                        }
                        input {
                            r#type: "text",
                            placeholder: "Např: CZ(zde je kontrolní dvoučíslí) 0100 0000 0012 3456 7890",
                            // Přepínání tříd při chybě
                            class: {if has_error
                            {"input input-bordered input-error text-error input-lg w-full font-mono"}
                            else
                            {"input input-bordered input-primary input-lg w-full font-mono"}},
                            maxlength: "34", // Maximální teoretická délka IBAN
                            oninput: move |evt| {
                                input_value.set(evt.value());
                            },
                        }
                        // Zobrazení chyby
                        if has_error {
                            label { class: "label py-0",
                                span { class: "label-text-alt text-error",
                                    "{error_text}"
                                }
                            }
                        }
                    }

                    // VIZUALIZACE JEDNOTLIVÝCH ČÁSTÍ IBAN
                    div { class: "mt-6 p-6 bg-base-200 rounded-box border border-base-300",
                        h3 { class: "text-sm font-bold uppercase tracking-widest text-center mb-6 text-base-content/70", "Struktura načteného IBAN" }

                        div { class: "flex flex-wrap justify-center items-start gap-2 md:gap-4",

                            // Kód země
                            div { class: "flex flex-col items-center min-w-[4rem] md:min-w-[5rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-primary bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm", "{iban_country_code}" }
                                div { class: "text-xs mt-2 font-semibold", "Země" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1", "Kód státu" }
                            }

                            // Kontrolní číslice (Zvýrazněna jako error barvou pro kontrolní mechanismy)
                            div { class: "flex flex-col items-center min-w-[4rem] md:min-w-[5rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-error bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm", "{iban_check_digit}" }
                                div { class: "text-xs mt-2 font-semibold", "Kontrola" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1", "Ověřovací číslice" }
                            }

                            // Kód banky (BBAN část 1)
                            div { class: "flex flex-col items-center min-w-[6rem] md:min-w-[7rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-secondary bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm", "{iban_bank_code}" }
                                div { class: "text-xs mt-2 font-semibold", "Banka" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1", "Kód instituce" }
                            }

                            // Číslo účtu (BBAN část 2) - má širší kontejner, protože bývá dlouhé
                            div { class: "flex flex-col items-center min-w-[12rem] md:min-w-[16rem] max-w-full",
                                div { class: "text-lg md:text-2xl font-mono font-bold text-info bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm overflow-hidden", "{iban_account_number}" }
                                div { class: "text-xs mt-2 font-semibold", "Číslo účtu" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1", "Předčíslí a základní číslo" }
                            }
                        }
                    }
                }
            }

            // SPODNÍ ČÁST: POSTUP
            div { class: "grid grid-cols-1 gap-8", // Pro IBAN dává smysl nechat na celou šířku, text je delší

                // Matematický postup
                div { class: "space-y-4",
                    h3 { class: "text-xl font-bold", "Matematický postup výpočtu" }
                    div { class: "mockup-code bg-base-300 text-base-content overflow-x-auto",
                        pre { "data-prefix": ">",
                            code { "Validace IBAN probíhá pomocí algoritmu Modulo 97." }
                        }
                        pre { "data-prefix": ">",
                            code { "1. Přesuňte první 4 znaky (země + kontrolní číslice) na konec." }
                        }
                        pre { "data-prefix": ">",
                            code { "2. Převeďte písmena na čísla (A=10, B=11 ... Z=35)." }
                        }
                        pre { "data-prefix": ">",
                            code { "3. Vznikne obrovské číslo (často přes 20 cifer)." }
                        }
                        pre { "data-prefix": ">",
                            code { "4. Zbytek po dělení tohoto čísla číslem 97 musí být přesně 1." }
                        }
                    }
                }
            }
        }
    }
}
