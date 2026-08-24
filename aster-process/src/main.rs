struct Process {
    pid: u32,
    name: String,
    memory_mb: u32,
    active: bool,
}
fn main() {
    let fireFox = Process {
        pid: 101,
        name: String::from("fireFox"),
        memory_mb: 401,
        active: true,
    };
    let vsCode = Process {
        pid: 102,
        name: String::from("vsCode"),
        memory_mb: 402,
        active: true,
    };
    let spotify = Process {
        pid: 103,
        name: String::from("spotify"),
        memory_mb: 403,
        active: true,
    };

    let fireFox = suspender_proceso(fireFox);

    finalizar_proceso(fireFox);

    let vsCode = suspender_proceso(vsCode);

    finalizar_proceso(vsCode);

    let spotify = suspender_proceso(spotify);

    finalizar_proceso(spotify);
}

fn finalizar_proceso(process: Process) {
    let nombre = process.name;
    println!("Finalizando {nombre}...");
}
fn suspender_proceso(mut process: Process) -> Process {
    process.active = false;

    let estado = process.active;
    let nombre = process.name.clone();

    println!("El estado de {nombre} ahora es: {estado}");

    process
}
