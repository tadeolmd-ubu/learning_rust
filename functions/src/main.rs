use std::io;


/*
-Como practica del libro en el capitulo de funciones hice un colector de estadisticas semanales de un corredor
-Primero se colectan los datos con la funcion "colect_data()" que retorna una tupla con todos los datos del corredor,
-En main lo destructuro para poder asignar a variables solas el valor de cada uno que se recolecto
-Hago separacion de responsabilidades para cada cosa que va a hacer el proyecto
-Hago un Calculador del IMC de la persona con la funcion "calcular_imc(peso: f64, altura: f64)"
-Calculo todos las estadisticas del corredor en funciones separadas, cada una aceptando como parametro
lo que sale de la tupla de collect_data(), llamandolos una por una en la "fn main()" obtengo todos las
estadisticas que queria obtener
*/
fn main() {
    let (nombre, edad, peso, altura, distancia, tiempo) = collect_data();

    let imc = calcular_imc(peso, altura);

    let ritmo = calcular_ritmo(distancia, tiempo);

    let velocidad = calcular_velocidad(distancia, tiempo);

    let meta = cumplio_meta(distancia);

    println!("==ESTADISTICAS SEMANELES DEL CORREDOR==");
    println!("Nombre: {nombre}");
    println!("Edad; {edad}");

    println!("Imc: {imc}");
    println!("Ritmo medio por km: {ritmo}, min/km");
    println!("Velocidad media: {velocidad}, km/h");
    println!("Cumplio la meta de 25 km en la semana; {meta}");
}

fn collect_data() -> (String, i32, f64, f64, f64, f64) {
    let mut nombre = String::new();
    let mut edad = String::new();
    let mut peso = String::new();
    let mut altura = String::new();
    let mut distancia = String::new();
    let mut tiempo = String::new();
    println!("Ingrese su nombre: ");

    io::stdin().read_line(&mut nombre).expect("");

    println!("Ingrese su edad: ");
    io::stdin().read_line(&mut edad).expect("");

    let edad: i32 = edad.trim().parse().expect("");

    println!("Ingrese su peso en kg: ");
    io::stdin().read_line(&mut peso).expect("");

    let peso: f64 = peso.trim().parse().expect("");

    println!("Ingrese su altura en metros: ");
    io::stdin().read_line(&mut altura).expect("");

    let altura: f64 = altura.trim().parse().expect("");

    println!("Ingrese los kms que corre en la semana: ");
    io::stdin().read_line(&mut distancia).expect("");

    let distancia: f64 = distancia.trim().parse().expect("");

    println!("Ingrese el tiempo que corre en la semana en minutos: ");
    io::stdin().read_line(&mut tiempo).expect("");

    let tiempo: f64 = tiempo.trim().parse().expect("");

    (nombre, edad, peso, altura, distancia, tiempo)
}

fn calcular_imc(peso: f64, altura: f64) -> f64 {
    peso / (altura * altura)
}

fn calcular_ritmo(distancia: f64, tiempo: f64) -> f64 {
    tiempo / distancia
}

fn calcular_velocidad(distancia: f64, tiempo: f64) -> f64 {
    let tiempo_horas = tiempo / 60.0;

    distancia / tiempo_horas
}

fn cumplio_meta(distancia: f64) -> bool {
    distancia >= 25.0
}
