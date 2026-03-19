// Zde v tomto souboru jsou všechny algoritmy výpočtu kontrolních číslic

//rodne cislo kotrnolní číslice
//Vstupem je zadané 9 (bez kontrolního čísla na konci) místné rodné číslo: ve stvaru String
pub fn calculate_rc_control_digit(rodne_cislo: &str) -> Option<i32> {
    //Vzatí prvních 9 digitů (pokud uživatel zadá 10, tak chci rovnou provést kontrolu)
    // why does this work wtf? reference to reference to string? huh?
    //let rc_copy = &rodne_cislo[..9];
    //BUT HERE IT TAKES WITHOUT REFERENCE HUHUHUHUU? bro i suck at rust
    if let Ok(val) = rodne_cislo[..9].parse() {
        let rodc_int: i64 = val;
        let mut modulo_rodc: i32 = (rodc_int % 11) as i32;
        //pokud je modulo 10 tak vrátit 0
        if modulo_rodc == 10{
        	modulo_rodc == 0;
        }
        return Some(modulo_rodc);
    }

    None
}

//Zde algoritmus jak pro EAN-13 tak pro ISBN - aka modulo 10 a nastřídačku *1 a *3
pub fn modulo_10_algorithm(isbn: &str) -> String{
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
	let mut res = (10-(sum%10)).to_string();
	if res == "10"{
		res = "0".to_string();
	}
	res
	//pokud je výsledek 10, tak se výsledkem stane nula
	
}



//Tests of functionality
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc_control_digit() {
        //for this the control digit is 5
        let result = rc_control_digit("770406334");
        assert_eq!(result, Some(5));
    }
    #[test]
    fn test_control_digit2() {
        // for this the control digit will be 13 - aka the parse failed
        let result = rc_control_digit("770b406334");
        assert_eq!(result, None);
    }
}
