# Program na kalkulaci a validaci kontrolních číslic

Interaktivní webová aplikace postavená na jazyce Rust (za použití frameworku [Dioxus](https://dioxuslabs.com/)) umožňující analýzu a generování kontrolních číslic pro běžně používané identifikátory a standardní kódy. Cílem projektu je demonstrovat využití modulo aritmetiky jako obrany proti chybám a překlepům.

## Podporované formáty

- **Rodné číslo (RČ)** - Validace a výpočet kontrolní číslice pomocí modulo 11.
- **ISBN-13** - Mezinárodní číslo knihy, výpočet modulo 10.
- **EAN-13** - Standardní čárové kódy zboží, výpočet modulo 10 (váhování 1, 3).
- **IBAN** - Mezinárodní formát čísla bankovního účtu, ověření modulo 97.
- **IČO (Identifikační číslo osoby)** - Validace a výpočet kontrolní číslice pomocí modulo 11 se sestupnými váhami.
- **Platební karty (Luhnův algoritmus)** - Validace a výpočet kontrolní číslice kreditních a debetních karet pomocí Luhnova algoritmu.

## Struktura projektu

- `src/main.rs` - Hlavní `Router` a rozcestník (UI domovské stránky).
- `src/algorithms.rs` - Jádro matematických algoritmů obsahující výpočty pro jednotlivé formáty.
- `src/...` - Specifické UI moduly komponent (`rodne_cislo.rs`, `ean13.rs`, `isbn.rs`, `iban_page.rs`, `ico.rs`, `luhn.rs`).

## Dokumentace

Podrobnější uživatelská a technická HTML dokumentace se nachází v souboru `dokumentace.html` umístěném v kořenovém adresáři.

## Instalace a spuštění (Dioxus 0.7+)

Projekt využívá Dioxus 0.7 s automatickou integrací Tailwind CSS. 

1. Ujistěte se, že máte nainstalovaný jazyk [Rust](https://www.rust-lang.org/tools/install).
2. Nainstalujte nástroj `dx` pro Dioxus (případně použijte `cargo install dioxus-cli`):
   ```bash
   curl -sSL [http://dioxus.dev/install.sh](http://dioxus.dev/install.sh) | sh
   ```
3. Spusťte aplikaci příkazem v kořenu projektu: ```bash dx serve ```. Aplikace poté bude automaticky dostupná v prohlížeči na `http://localhost:8080`
