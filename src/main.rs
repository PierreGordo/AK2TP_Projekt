use dioxus::prelude::*;
//for algos
mod algorithms;
//for logging - remove later when app complete
use tracing;



#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[route("/")]
    Home,

    #[route("/ISBN")]
    Isbn,

    #[route("/rodne_cislo")]
    Rodne_cislo,

}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
	//for logging - remove later when app complete
    dioxus::launch(|| {
        rsx! {
        //tailwind css
        document::Stylesheet {href: TAILWIND_CSS}
        Router::<Route> {}}
    });
}

//Function for landing page ui, mainly ui in this one, so nothing to coment about
#[component]
fn Home() -> Element {

//for navigation - encasing the whole button messes up the alighment for some reason
let nav = use_navigator();

rsx! {
        div {

            class: "hero min-h-screen bg-base-200",
            
            div {
                class: "hero-content text-center",
                
                div {
                    class: "max-w-3xl",
                    

                    h1 { 
                        class: "text-5xl font-bold text-primary mb-6", 
                        "Generátor detekčních kódů" 
                    }
                    

                    p {
                        class: "text-lg text-base-content/80 mb-8",
                        "Interaktivní nástroj pro analýzu nejběžnějších identifikačních kódů z každodenního života. "
                        "Aplikace demonstruje praktické využití modulo aritmetiky k zabezpečení dat proti chybám při jejich přenosu či ručním přepisu."
                    }
                    

                    div {
                        class: "divider text-base-content/60 font-semibold mb-8",
                        "Vyberte kód pro analýzu"
                    }
                    
                    div {
                        class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                        
                        button { 
                            class: "btn btn-outline btn-primary btn-lg",
                            onclick: move |_| {nav.push(Route::Rodne_cislo {});}, 
                            "Rodné číslo (RČ)" 
                        }
                        button { 
                            class: "btn btn-outline btn-secondary btn-lg",
                            onclick: move |_| {nav.push(Route::Isbn {});}, 
                            "ISBN-13" 
                        }
                        button { 
                            class: "btn btn-outline btn-accent btn-lg", 
                            "EAN-13" 
                        }
                        
                        button { 
                            class: "btn btn-outline btn-info btn-lg", 
                            //route here
                            onclick: move |_| {nav.push(Route::Rodne_cislo {});},
                            "IBAN" 
							
                        }
                    }
                }
            }
        }
    }
}

fn Isbn() -> Element {
    rsx! {
    "This is the ISBN page."}
}



fn Rodne_cislo() -> Element {


	//for navigation - encasing the whole button messes up the alighment for some reason
	let nav = use_navigator();
    // Stav pro výběr kódu a zadaný vstup
    let mut input_value = use_signal(|| String::new());
	//placeholder na vypočtený control digit - opět je to empty character aby to html drželo formu
	let mut calculated_control_digit = use_signal(||" ‌‌‌".to_string());
    rsx! {
        div { class: "p-6 max-w-5xl mx-auto space-y-8",
            
            // Horní navigace / Zpět
            button { 
                class: "btn btn-ghost gap-2",
                onclick: move |_| {nav.push(Route::Home {});},
                "Zpět na výběr" 
            }

            // HLAVNÍ KARTA S FORMULÁŘEM
            div { class: "card bg-base-100 shadow-xl border border-base-300",
                div { class: "card-body",
                    h2 { class: "card-title text-2xl mb-4", 
								"Analýza rodného čísla"
                    }

                    div { class: "form-control w-full",
                        label { class: "label",
                            span { class: "label-text font-semibold", "Vložte datový základ (bez kontrolní číslice)" }
                        }
                        input { 
                            r#type: "text",
                            placeholder: "Např: 980215423",
                            class: "input input-bordered input-primary input-lg w-full font-mono",
                            value: "{input_value}",
                            oninput: move |evt| 
                            		{
                            		input_value.set(evt.value());
									//trigger when rodné číslo is the corrent len
                            		if input_value.len() == 9 {
                            			// 												input value is type Signal<String> -> .read() unwraps it somehow and .as_str() converts to &str
                            			let result: Option<i32> = algorithms::rc_control_digit(input_value.read().as_str());
                            			match result{
                            				//Actually now that I think about it, even though my function returns i32
                            				//representing the calculated value as String seems like a better idea
                            				Some(val) => {let calculated_control_digit_int: i32 = val;
                            							  calculated_control_digit.set(calculated_control_digit_int.to_string());
                            							  },
                            						//Have to use set here, because it is a signal
                            				None => {calculated_control_digit.set("Neplatné RČ.".to_string());},
                            				}
                            			}
                            		//Zde kontrola, zda je kontrolní číslice platná, pokud zadá uživatel všech 10 čísel
                            		
                            		else{
                            			//placeholder na vypočtený control digit - opět je to empty character aby to html drželo formu
                            			calculated_control_digit.set(" ‌‌‌".to_string());
                            		}
                            }
                        }
                        label { class: "label",
                            span { class: "label-text-alt text-base-content/60", 
                                "Systém automaticky dopočítá zbytek pomocí modulo aritmetiky." 
                            }
                        }
                    }
                }
            }

            // STŘEDNÍ ČÁST: VÝSLEDKY A STATISTIKY
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                
                // Panel pro výsledek
                div { class: "stats shadow bg-primary text-primary-content col-span-1",
                    div { class: "stat",
                        div { class: "stat-title text-primary-content/80", "Kontrolní číslice" }
                        div { class: "stat-value", "{calculated_control_digit}" } // ZDE JE OPET INVISIBLE CHARACTER, NEMAZAT -> pořád tam je ale přes proměnou
                        div { class: "stat-desc text-primary-content/80", "Vypočteno metodou Modulo 10" }
                    }
                }

                // Panel pro celkový kód
                div { class: "stats shadow col-span-2",
                    div { class: "stat",
                        div { class: "stat-title", "Kompletní validní kód" }
                        div { class: "stat-value tracking-widest", "{input_value}{calculated_control_digit}" } // Je tady zero width chararcte rv tom aby to držel formu !!!! NEMAZAT CO JE ZA IMPUT VALUE !!!!
                        div { class: "stat-actions",
                            button { class: "btn btn-sm btn-success", "Kopírovat" }
                        }
                    }
                }
            }

            // SPODNÍ ČÁST: POSTUP A VIZUALIZACE
            div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8",
                
                // Matematický postup
                div { class: "space-y-4",
                    h3 { class: "text-xl font-bold", "Matematický postup výpočtu" }
                    div { class: "mockup-code bg-base-300 text-base-content",
                        pre { "data-prefix": ">", code { "Krok 1: Součet lichých pozic = 31" } }
                        pre { "data-prefix": ">", code { "Krok 2: Součet sudých pozic * 3 = 51" } }
                        pre { "data-prefix": ">", code { "Krok 3: (31 + 51) mod 10 = 2" } }
                        pre { "data-prefix": ">", class: "text-success", code { "Krok 4: Výsledek = 10 - 2 = 8" } }
                    }
                }

                // Vizualizace (Čárový kód / QR)
                div { class: "flex flex-col items-center justify-center p-8 bg-white rounded-xl border-2 border-dashed border-base-300",
                    div { class: "text-center space-y-4",
                        // Sem přijde vygenerované SVG/PNG čárového kódu
                        div { class: "w-64 h-32 bg-slate-200 flex items-center justify-center text-slate-400 font-mono",
                            "Barcode Preview"
                        }
                        button { class: "btn btn-outline", "Stáhnout jako PNG" }
                    }
                }
            }
        }
    }
}
