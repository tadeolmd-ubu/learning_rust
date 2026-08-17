use std::io;

fn main() {
    let mut kms = 0;
    let mut meta = String::new();

    println!("Cuantos kms vas a correr hoy?");

    io::stdin().read_line(&mut meta).expect("");

    let meta: i64 = meta.trim().parse().expect("");

    loop {
        kms += 1;

        println!("Has corrido: {kms} km");
        if kms == meta {
            println!("¡Llegaste a la meta!");
            break;
        }
    }

    let kilometros = [5, 8, 0, 6, 10, 4, 7];

    println!("--- SEMANA ---");

    for index in 0..7 {
        if kilometros[index] == 0 {
            println!("Dia {} de descanso.", index + 1);
        }
        if kilometros[index] < 5 && kilometros[index] > 0 {
            println!(
                "Dia {} Entrenamiento ligero, corriste: {} km",
                index + 1,
                kilometros[index]
            );
        } else if kilometros[index] > 4 && kilometros[index] < 10 {
            println!(
                "Dia {} Entrenamiento normal, corriste: {}",
                index + 1,
                kilometros[index]
            );
        } else if kilometros[index] > 9 {
            println!(
                "Dia {} Entrenamiento largo, corriste: {}",
                index + 1,
                kilometros[index]
            );
        }
    }

    println!("--- PRIMER ENTRENAMIENTO LARGO ---");

    'encontrar_10: for index in 0..7 {
        if kilometros[index] == 10 {
            println!("El primer dia con 10 kms es el dia: {}", index + 1);
            break 'encontrar_10;
        }
    }
}

//Luis Octavio Garcia Bojorquez
