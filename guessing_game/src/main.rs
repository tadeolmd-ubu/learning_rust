use std::cmp::Ordering;
use std::io;

use rand::Rng;


/* 
-Primero se importe todos los crates (librerias) que iba a necesitar
-Declaro el numero aleatorio a adivinar
-Creo un loop infinito con la keyword "loop{}""
-Inicializo como string mutable el numero que va a ingresar el usuario
-Lo leo con io::stdin().read_line() como un string, tiro un expect en caso de lo que ingrese el usuario no es un numero
-Lo convierto a u32 (numero pequeno), primeramente usando .trim() para limpiar el input del usuario 
luego .parse para convertirlo, en caso de que si sea parseable a u32, retorna ok con el valor de la nueva variable,
en caso de que no sea parseable el input, retorna Err(_) pero ignora el error continunando el programa y dejando volver a ingresar un numero sin crashearlo
-Luego uso math y el crate ordening para saber como va en el juego el jugador usando la variable guess(el input del jugador)
y comparandolo con el numero secreto que inicializamos al inicio,
en caso de ser mas chico (Less) imprime "To small!", si es mas grande(Greater) "Too big!"
y si es igual(Equal) "You win!" y aparte acaba el loop infinito con un "break"
*/
fn main() {
    println!("Guess the Number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("Your guessed: {guess} ");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
