use crate::Route;
use dioxus::prelude::*;
//knihovna na IBAN, protože je to poměrně složité
use iban::{Bban, Iban};


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
    //pro uspech
    let mut has_sucess = false;
    let mut sucess_text = String::new();
    //pro informace
    let mut has_info = false;
    let mut info_text = String::new();

    if !input_value.is_empty() {
        input_string = input_value();
    }

    //Zcela nová a radikálně jednodušší logika oproti starší verzi
    //Pokaždé, když se zadá další číslice iban, pokusim se to parsenout jako iban
    //pokud to selže, pokusím se tam dopočítat kontrolni číslici a opět to parsenout
    //jako iban, pokud selže i to, nejedná se o validní iban

    if let Ok(val) = input_string.parse::<Iban>() {
        //pokud toto vyjde jedná se o validní iban, tutíž stačí assignout hodnoty
        //protože uživatel do pole z nějakého důvodu zadal kompletní iban
        iban_country_code = val.country_code().to_string();
        iban_check_digit = val.check_digits().to_string();

        has_sucess = true;
        sucess_text = "Vložili jste validní IBAN se správnou kontrolní číslicí.".to_string();


        let bban: Bban = val.bban();

        //try to parse bank account number
        if let Some(bban_bank) = bban.bank_identifier(){
        	iban_bank_code = bban_bank.to_string();
        }
        //pokud se nepodařilo parsenout bank account number, hodím do čísla účtu celý bban
		//teď mám, nebo nemám kód banky (ale spíš jo), takže slicenu celkový bban
		let iban_bank_len: usize = iban_bank_code.len().try_into().unwrap();
		iban_account_number = bban.as_str()[iban_bank_len..].to_string();

    }
    //pokud se nepodaří parsenout iban
    else {
        //remove all empty spaces
        let mut original_iban = input_string.clone();
        original_iban.retain(|c| !c.is_whitespace());

        //kontrola délky, protože provádím slicing a nejkratší možný valid iban je 15
        if original_iban.len() >= 15 {
            //zkontrolovat, zda pouze uživatel nedodal iban kód bez kontrolní číslice
            let nulovany_iban = format!("{}00{}", &original_iban[..2], &original_iban[2..]);

            let check_digits = 98 - iban::calculate_checksum(nulovany_iban.as_bytes());

            let calculated_iban = format!(
                "{}{:02}{}",
                &original_iban[..2],
                check_digits,
                &original_iban[2..]
            );
            //zde provést kontrolu jestli je to validní iban
            //jedná se zde o stejný if let jako je nahoře
            if let Ok(val) = calculated_iban.parse::<Iban>() {
                //pokud toto vyjde jedná se o validní iban, tutíž stačí assignout hodnoty
                //protože uživatel do pole z nějakého důvodu zadal kompletní iban
                iban_country_code = val.country_code().to_string();
                iban_check_digit = val.check_digits().to_string();

				has_info = true;
        		info_text = "Vložili jste validní IBAN bez kontrolní číslice. Program kontrolní číslici dopočetl a je zobrazena v poli Kontrola.".to_string();


                let bban: Bban = val.bban();

		        let bban: Bban = val.bban();

		        //try to parse bank account number
		        if let Some(bban_bank) = bban.bank_identifier(){
		        	iban_bank_code = bban_bank.to_string();
		        }
		        //pokud se nepodařilo parsenout bank account number, hodím do čísla účtu celý bban
				//teď mám, nebo nemám kód banky (ale spíš jo), takže slicenu celkový bban
				let iban_bank_len: usize = iban_bank_code.len().try_into().unwrap();
				iban_account_number = bban.as_str()[iban_bank_len..].to_string();
            }
            else{
            	has_error = true;
            	error_text = "Řetězec znaků co jste zadali není validní IBAN.".to_string();
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
                                "Vložte kód IBAN k analýze (můžete i s mezerami)."
                            }
                        }
                        input {
                            r#type: "text",
                            placeholder: "Např: CZ69 0710 1781 2400 0000 4159",
                            // Přepínání tříd při chybě
                            class: {
                                if has_error {
                                    "input input-bordered input-error text-error input-lg w-full font-mono"
                                } else if has_sucess {
                                    "input input-bordered input-success text-success input-lg w-full font-mono"
                                } else if has_info {
                                    "input input-bordered input-info text-info input-lg w-full font-mono"
                                } else {
                                    "input input-bordered input-primary input-lg w-full font-mono"
                                }
                            },
                            maxlength: "34", // Maximální teoretická délka IBAN
                            oninput: move |evt| {
                                input_value.set(evt.value());
                            },
                        }
                        // Zobrazení chyby
                        if has_error {
                            label { class: "label py-0",
                                span { class: "label-text-alt text-error", "{error_text}" }
                            }
                        }
                        if has_sucess {
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

                    // VIZUALIZACE JEDNOTLIVÝCH ČÁSTÍ IBAN
                    div { class: "mt-6 p-6 bg-base-200 rounded-box border border-base-300",
                        h3 { class: "text-sm font-bold uppercase tracking-widest text-center mb-6 text-base-content/70",
                            "Struktura načteného IBAN"
                        }

                        div { class: "flex flex-wrap justify-center items-start gap-2 md:gap-4",

                            // Kód země
                            div { class: "flex flex-col items-center min-w-[4rem] md:min-w-[5rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-primary bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm",
                                    "{iban_country_code}"
                                }
                                div { class: "text-xs mt-2 font-semibold", "Země" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1",
                                    "Kód státu"
                                }
                            }

                            // Kontrolní číslice (Zvýrazněna jako error barvou pro kontrolní mechanismy)
                            div { class: "flex flex-col items-center min-w-[4rem] md:min-w-[5rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-error bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm",
                                    "{iban_check_digit}"
                                }
                                div { class: "text-xs mt-2 font-semibold", "Kontrola" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1",
                                    "Ověřovací číslice"
                                }
                            }

                            // Kód banky (BBAN část 1)
                            div { class: "flex flex-col items-center min-w-[6rem] md:min-w-[7rem]",
                                div { class: "text-2xl md:text-4xl font-mono font-bold text-secondary bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm",
                                    "{iban_bank_code}"
                                }
                                div { class: "text-xs mt-2 font-semibold", "Banka" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1",
                                    "Kód instituce"
                                }
                            }

                            // Číslo účtu (BBAN část 2) - má širší kontejner, protože bývá dlouhé
                            div { class: "flex flex-col items-center min-w-[12rem] md:min-w-[16rem] max-w-full",
                                div { class: "text-lg md:text-2xl font-mono font-bold text-info bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm overflow-hidden",
                                    "{iban_account_number}"
                                }
                                div { class: "text-xs mt-2 font-semibold", "Číslo účtu" }
                                div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1",
                                    "Předčíslí a základní číslo"
                                }
                            }
                        }
                    }
                }
            }

            // SPODNÍ ČÁST: POSTUP
            div { class: "grid grid-cols-1 gap-8", // Pro IBAN dává smysl nechat na celou šířku, text je delší

                // Matematický postup
                div { class: "space-y-4",
                    h3 { class: "text-xl font-bold", "Výpočet kontrolní číslice IBAN" }
                    div { class: "mockup-code bg-base-300 text-base-content overflow-x-auto",
                        pre { "data-prefix": ">",
                            code {
                                "1. Sestaví se základní řetězec: Kód banky + Číslo účtu (tzv. BBAN)."
                            }
                        }
                        pre { "data-prefix": ">",
                            code {
                                "2. Na konec tohoto řetězce se připojí kód země a dvě nuly (např. 'CZ00')."
                            }
                        }
                        pre { "data-prefix": ">",
                            code {
                                "3. Všechna písmena v řetězci se převedou na čísla (A=10, B=11 ... Z=35)."
                            }
                        }
                        pre { "data-prefix": ">",
                            code {
                                "4. Vzniklé číslo se vydělí 97 a zjistí se zbytek (operace modulo)."
                            }
                        }
                        pre { "data-prefix": ">",
                            code {
                                "5. Tento zbytek se následně odečte od čísla 98 (Výpočet: 98 - zbytek)."
                            }
                        }
                        pre { "data-prefix": ">",
                            code {
                                "6. Pokud je výsledek jednociferný, přidá se před něj nula (např. 7 -> 07)."
                            }
                        }
                        pre { "data-prefix": ">",
                            code {
                                "7. Toto dvojčíslí se pak vloží hned za kód země a tím vznikne finální IBAN."
                            }
                        }
                    }
                }
            }
        }
    }
}
