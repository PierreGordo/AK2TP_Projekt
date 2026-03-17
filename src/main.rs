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
    let mut calculated_control_digit = use_signal(|| " ‌‌‌".to_string());
	//na manipulování toho zda je viditelný infobox, či ne - buď "opacity-100 scale-100 visible" nebo "opacity-0 scale-95 invisible"
	let mut visibility_state = use_signal(|| String::from("opacity-0 scale-95 invisible"));
	//potřebuju znova kontrol digit, je to hloupé, ano -> udělam to? také ano, protoze to bude fungovat
	let mut calculated_control_digit_second = use_signal(|| " ‌‌‌".to_string());
	//potebuji znova input value, protože jinak to spolu fajtí a je to na houby - opět hloupá věc, ale co mám dělat :((
	let mut input_value_valid_code = use_signal(|| String::new());
    
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
                            span { class: "label-text font-semibold", "Vložte datový základ (buď bez kontrolní číslice, nebo s ní pro kontrolu rodného čísla)" }
                        }
                        input {
                            r#type: "text",
                            placeholder: "Např: 980215423",
                            class: "input input-bordered input-primary input-lg w-full font-mono",
                            //this could fix my problems
                            maxlength: "10",
                            value: "{input_value}",
                            oninput: move |evt|
                                    {                                   	
                                    input_value.set(evt.value());
                                    //trigger when rodné číslo is the corrent len
                                    //make the infobox warning go invisible, when user types in again, has to be at the top, because if at the bottom, it overrides the 
                                    //command to make it visible
                                    visibility_state.set(String::from("opacity-0 scale-95 invisible"));
                                    input_value_valid_code.set(evt.value());
                                    match input_value.len()
                                    {
                                    9 =>
                                        {
                                        // 												input value is type Signal<String> -> .read() unwraps it somehow and .as_str() converts to &str
                                            let result: Option<i32> = algorithms::rc_control_digit(input_value.read().as_str());
                                            match result{
                                                //Actually now that I think about it, even though my function returns i32
                                                //representing the calculated value as String seems like a better idea
                                                Some(val) => {let calculated_control_digit_int: i32 = val;
                                                              calculated_control_digit.set(calculated_control_digit_int.to_string());
                                                              calculated_control_digit_second.set(calculated_control_digit_int.to_string());
                                                              },
                                                        //Have to use set here, because it is a signal
                                                None => {calculated_control_digit.set("Neplatné RČ.".to_string());},
                                                }
                                            }
                                    //Pokud uživatel zadá celé RČ - provést kontrolu číslice
                                    10 =>
                                    {
                        					//this is made so that if you enter 10 digit RČ, it does not show up at the end
                        					//v kompletní valdiní kód políčku
                        					calculated_control_digit_second.set(" ‌‌‌".to_string());
                                            let result: Option<i32> = algorithms::rc_control_digit(input_value.read().as_str());
											match result{
												//Actually now that I think about it, even though my function returns i32
												//representing the calculated value as String seems like a better idea
												Some(val) => 
												{	
													let calculated_control_digit_int: i32 = val;
													calculated_control_digit.set(calculated_control_digit_int.to_string());
													//Check whether the calculated control number is different to that the user inputted
													if !input_value.read().ends_with(&calculated_control_digit_int.to_string()){
														//This method was incredibly stupid so I am replacing it with a info box warning
														//calculated_control_digit.set("Neplatný kontrolní digit RČ".to_string());
														visibility_state.set(String::from("opacity-100 scale-100 visible"));
														//also would be probably good here to set the Kompletní validní kód, to an acutaally valid code
														//that is too complex, i will just erase it
														input_value_valid_code.set(" ‌‌‌".to_string());
													}
													},
												//Have to use set here, because it is a signal
												None => {calculated_control_digit.set("Neplatné RČ.".to_string());},
											}
											
                                    }

                                    _ => {
                                            //placeholder na vypočtený control digit - opět je to empty character aby to html drželo formu
                                            calculated_control_digit.set(" ‌‌‌".to_string());
                                            calculated_control_digit_second.set(" ‌‌‌".to_string());
                                        }
                                    }
                                    //Check jestli uživatel skutečně zadává čísla
                                    if evt.value().trim().parse::<i64>().is_err() && input_value.len() != 0
                                    {
                                    	calculated_control_digit.set("Neplatné RČ.".to_string());
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
                        div { class: "stat-value tracking-widest", "{input_value_valid_code}{calculated_control_digit_second}" } // Je tady zero width chararcte rv tom aby to držel formu !!!! NEMAZAT CO JE ZA IMPUT VALUE !!!!
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
                        pre { "data-prefix": ">", code { "Zbytek po vydělení rodného čísla 11." } }
                        pre { "data-prefix": ">", code { "V programování se pro tuto operaci používá" } }
                        pre { "data-prefix": ">", code { "operátor % - modulo." } }
                        pre { "data-prefix": ">", code { "{input_value} % 11 = {calculated_control_digit}" } }
                    }
                }

            //Warning pokud se zadá neplatné 10 místné rodné číslo
			div { class: "space-y-4",

			h3 { class: "text-xl font-bold", "Infobox" }
			div { 
			class: "transition-all duration-300 ease-out {visibility_state}",
			//I have to wrap this infoallert in this div, to make it appearable and reapearrable
			div {
	            role: "alert", 
	            class: "alert alert-warning",
	            svg {
	                xmlns: "http://www.w3.org/2000/svg",
	                class: "h-6 w-6 shrink-0 stroke-current",
	                fill: "none",
	                view_box: "0 0 24 24",
	                path {
	                    stroke_linecap: "round",
	                    stroke_linejoin: "round",
	                    stroke_width: "2",
	                    d: "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
	                }
	            }
	            span { "Varování: zadali jste rodné číslo s neplatnou kontrolní číslicí!" }
        	}
        	}
        	}
            

				
            }
        }
    }
}
