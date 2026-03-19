//EAN -13

use dioxus::prelude::*;
use crate::Route;


#[component]
pub fn Ean13() -> Element {
	rsx! {
	    div { class: "p-6 max-w-5xl mx-auto space-y-8",
	
	        // Horní navigace / Zpět
	        button {
	            class: "btn btn-ghost gap-2",
	            "Zpět na výběr"
	        }
	
	        // HLAVNÍ KARTA S FORMULÁŘEM A VIZUALIZACÍ
	        div { class: "card bg-base-100 shadow-xl border border-base-300",
	            div { class: "card-body",
	                h2 { class: "card-title text-2xl mb-4", "Analýza EAN-13 kódu" }
	
	                div { class: "form-control w-full",
	                    label { class: "label",
	                        span { class: "label-text font-semibold",
	                            "Vložte prvních 12 čísel kódu EAN-13. Vkládejte prosím s pomlčkami nebo mezerami."
	                        }
	                    }
	                    input {
	                        r#type: "text",
	                        placeholder: "Např: 859-1234-56789",
	                        // Zde by se logikou měnila třída na 'input-error', aktuálně je nastavena defaultní primární
	                        class: "input input-bordered input-primary input-lg w-full font-mono",
	                        maxlength: "17",
	                    }
	                    // Zástupný prvek pro chybovou hlášku
	                    label { class: "label py-0 invisible", // Odstranit 'invisible' při zobrazení chyby
	                        span { class: "label-text-alt text-error",
	                            "Zástupný text pro error hlášku."
	                        }
	                    }
	                }
	
	                // VIZUALIZACE JEDNOTLIVÝCH ČÁSTÍ EAN
	                div { class: "mt-6 p-6 bg-base-200 rounded-box border border-base-300",
	                    h3 { class: "text-sm font-bold uppercase tracking-widest text-center mb-6 text-base-content/70", "Struktura načteného EAN" }
	                    div { class: "flex flex-wrap justify-center items-start gap-2 md:gap-4",
	                        
	                        // GS1 Prefix (Země/Typ)
	                        div { class: "flex flex-col items-center min-w-[5rem] md:min-w-[6rem]",
	                            div { class: "text-2xl md:text-4xl font-mono font-bold text-primary bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm", "859" }
	                            div { class: "text-xs mt-2 font-semibold", "GS1 Prefix" }
	                            div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1", "Země (např. ČR)" }
	                        }
	                        
	                        div { class: "text-2xl md:text-3xl font-bold text-base-300 mt-2 md:mt-3", "-" }
	                        
	                        // Číslo výrobce
	                        div { class: "flex flex-col items-center min-w-[5rem] md:min-w-[7rem]",
	                            div { class: "text-2xl md:text-4xl font-mono font-bold text-secondary bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm", "1234" }
	                            div { class: "text-xs mt-2 font-semibold", "Výrobce" }
	                            div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1", "Registrační číslo firmy" }
	                        }
	
	                        div { class: "text-2xl md:text-3xl font-bold text-base-300 mt-2 md:mt-3", "-" }
	                        
	                        // Číslo položky
	                        div { class: "flex flex-col items-center min-w-[5rem] md:min-w-[7rem]",
	                            div { class: "text-2xl md:text-4xl font-mono font-bold text-accent bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm", "56789" }
	                            div { class: "text-xs mt-2 font-semibold", "Produkt" }
	                            div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1", "Identifikace zboží" }
	                        }
	
	                        div { class: "text-2xl md:text-3xl font-bold text-base-300 mt-2 md:mt-3", "-" }
	                        
	                        // Kontrolní číslice
	                        div { class: "flex flex-col items-center min-w-[4rem] md:min-w-[5rem]",
	                            div { class: "text-2xl md:text-4xl font-mono font-bold text-error bg-base-100 w-full px-2 h-12 md:h-14 flex items-center justify-center rounded shadow-sm", "3" }
	                            div { class: "text-xs mt-2 font-semibold", "Kontrola" }
	                            div { class: "text-[10px] text-base-content/60 text-center leading-tight mt-1", "Ověřovací číslice" }
	                        }
	                    }
	                }
	            }
	        }
	
	        // SPODNÍ ČÁST: POSTUP
	        div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8",
	
	            // Matematický postup
	            div { class: "space-y-4",
	                h3 { class: "text-xl font-bold", "Matematický postup výpočtu" }
	                div { class: "mockup-code bg-base-300 text-base-content",
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
	        }
	    }
	}
}
