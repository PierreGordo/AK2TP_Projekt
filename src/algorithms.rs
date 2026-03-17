// Zde v tomto souboru jsou všechny algoritmy výpočtu kontrolních číslic

//rodne cislo kotrnolní číslice
//Vstupem je zadané 9 (bez kontrolního čísla na konci) místné rodné číslo: ve stvaru String
fn rc_control_digit(rodne_cislo: &str) -> Option<i32> {

	
    if let Ok(val) = rodne_cislo.parse() {
        let rodc_int: i64 = val;
        let modulo_rodc: i32 = (rodc_int % 11) as i32;
        return Some(modulo_rodc)
    }

   	None
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
