//Desarrolla una función en Python que reciba un número flotante y retorne su valor 
//absoluto, su doble y su mitad.

use std::io;

fn leer_f32(mensaje: &str) -> f32 {

	loop {

		println!("{}", mensaje);
		
		let mut entrada = String::new();
		io::stdin().read_line(&mut entrada)
		.expect("Error. Entrada Inválida\n");

		match entrada.trim().parse::<f32>() {

			Ok(num) => return num,
			Err(_) => println!("Error. Vuelve a intentarlo.\n"),
			
		}
	}
}

fn RetorNum(numI: f32) -> (f32, f32, f32) {

	let numAbs = numI.abs();
	
	let numDo = numI * 2.0;

	let numMit = numI / 2.0;
	
	(numAbs, numDo, numMit)
	 
}

fn main() {

    println!("***PROGRAMA QUE RETORNA UN NÚMERO***\n");

	let numI = leer_f32("Ingresá un número: \n");
	
	let (numAbs, numDo, numMit) = RetorNum(numI);

	println!("Salida:\n El número absoluto del número es {},\n El doble del número es {},\n La mitad del número es {}", numAbs, numDo, numMit);
	
    println!("***PROGRAMA TERMINADO***\n");
    
}
