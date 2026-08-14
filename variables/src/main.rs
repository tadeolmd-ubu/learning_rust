use std::io;

fn main() {
    let mut nombre = String::new();
    let mut edad = String::new();
    let mut peso = String::new();
    let mut altura = String::new();

    println!("==Bienvenido a la calculadora de estadistica de corredores==");

    println!("Ingrese su Nombre:");

    io::stdin()
        .read_line(&mut nombre)
        .expect("Error a la hora de capturar el nombre");

    let nombre = nombre.trim();

    println!("Ingrese su edad");

    io::stdin()
        .read_line(&mut edad)
        .expect("Error a la hora de leer la edad");

    let edad: i32 = edad.trim().parse().expect("Ingrese un numero como edad");

    println!("Ingrese su peso (con decimales)");

    io::stdin()
        .read_line(&mut peso)
        .expect("Error a la hora de leer la edad");

    let peso: f64 = peso
        .trim()
        .parse()
        .expect("Ingrese un valor valido numerico para su peso");

    println!("Ingrese su altura (en metros con decimales)");

    io::stdin()
        .read_line(&mut altura)
        .expect("Error a la hora de leer la altura");

    let altura: f64 = altura
        .trim()
        .parse()
        .expect("Ingrese un valor valido numerico para su altura");

    println!("Datos de la persona, nombre: {nombre}, edad: {edad}, peso: {peso}, altura: {altura}");

    let mut km1 = String::new();
    let mut km2 = String::new();
    let mut km3 = String::new();
    let mut km4 = String::new();
    let mut km5 = String::new();
    let mut km6 = String::new();
    let mut km7 = String::new();

    let mut kms = [0.0; 7];
    println!("Ingrese los km's corridos por dia en la semana");
    println!("Lunes: ");

    io::stdin().read_line(&mut km1).expect("Ingrese un numero");

    let km: f64 = km1.trim().parse().expect("");

    kms[0] = km;
    println!("Martes: ");

    io::stdin().read_line(&mut km2).expect("Ingrese un numero");

    let km2: f64 = km2.trim().parse().expect("");

    kms[1] = km2;

    println!("Miercoles: ");
    io::stdin().read_line(&mut km3).expect("");

    let km3: f64 = km3.trim().parse().expect("");

    kms[2] = km3;

    println!("Jueves: ");
    io::stdin().read_line(&mut km4).expect("");

    let km4: f64 = km4.trim().parse().expect("");

    kms[3] = km4;

    println!("Viernes: ");
    io::stdin().read_line(&mut km5).expect("");

    let km5: f64 = km5.trim().parse().expect("");

    kms[4] = km5;

    println!("Sabado: ");
    io::stdin().read_line(&mut km6).expect("");

    let km6: f64 = km6.trim().parse().expect("");

    kms[5] = km6;

    println!("Domingo: ");
    io::stdin().read_line(&mut km7).expect("");

    let km7: f64 = km7.trim().parse().expect("");

    kms[6] = km7;
    println!("kms del Lunes: {} km", kms[0]);
    println!("kms del Martes: {} km", kms[1]);
    println!("kms del Miercoles: {} km", kms[2]);
    println!("kms del Jueves: {} km", kms[3]);
    println!("kms del Viernes: {} km", kms[4]);
    println!("kms del Sabado: {} km", kms[5]);
    println!("kms del Domingo: {} km", kms[6]);

    let total = kms[0] + kms[1] + kms[2] + kms[3] + kms[4] + kms[5] + kms[6];
    let prom = total / 7.0;
    let imc = peso / (altura * altura);

    println!("Total de kms de la semana: {total}");
    println!("Promedio de km diarios en la semana: {prom}");
    println!("IMC: {imc}");

        let meta: bool;

    if total >= 25.0 {
        meta = true;
    } else {
        meta = false;
    }
    println!("Se cumplio la meta de 25 kms: {meta}");
}
