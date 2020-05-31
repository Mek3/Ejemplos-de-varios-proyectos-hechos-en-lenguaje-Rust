

//fn main(){
	/*println!("Hola mundo cruel! {0}, {1}", 40, 52);
	println!("{} of {:b} people know binary, the other half doesn't", 1, 20);
	
	println!("{number:>width$}", number=82, width=10);
	
	println!("My name is {0}, {1} {0}", "Bond");*/
	/*
	println!("Hola mundo");
	format!("Esto que essss?");
}*/
/*
fn main()
{ 
   let tupla = (24, "Mohamed", true);
    let (edad,nombre,hace_ejercicio) = tupla;
    println!("Edad: {}",edad);
    let (_,country,_) = tupla;
    println!("Nombre: {}",nombre);
    let hace_ejercicio = tupla.2;
    println!("hace ejercicio? : {}", hace_ejercicio);
}
*/
/*
#[derive(Debug)]
struct Coche<'a> {
    Marca: &'a str,
    Modelo: &'a str,
    Anyo: &'a str,
}

struct Nil;

struct Pareja(i32, f32);

// Estructura con dos elementos
struct Punto {
    x: f32,
    y: f32,
}

// Estructura reutilizando la estructura punto
#[allow(dead_code)]
struct Rectangulo {
    p1: Punto,
    p2: Punto,
    p3: Punto,
    p4: Punto,
}

fn main() {
    // Inicializar estuctura Coche
    let Marca = "Seat";
    let Modelo = "Ibiza";
    let Anyo = "2018";
    let coche = Coche {Marca, Modelo, Anyo};

    // Imprimir datos del coche
    println!("{:?}", coche); /*Esto nos imprimirá lo siguiente: 
							Coche { Marca: "Seat", Modelo: "Ibiza", Anyo: "2018" }*/


    let p1: Punto = Punto { x: 0.0, y: 0.0 };
    let p2: Punto = Punto { x: 0.0, y: 1.0 };
    let p3: Punto = Punto { x: 2.0, y: 1.0 };
    let p4: Punto = Punto { x: 2.0, y: 0.0 };
    

    // Accediendo a los puntos
    print!("listado de puntos: ({}, {}), ", p1.x, p1.y);
    print!("({}, {}), ", p2.x, p2.y);
    print!("({}, {}), ", p3.x, p3.y);
    println!(" ({}, {})", p4.x, p4.y);
    
    let rectangulo = Rectangulo {p1, p2, p3, p4};

    //Asignar estructura de unidad
    let _nil = Nil;

    // inicializar tupla
    let pareja = Pareja(1, 0.1);

    // Access the fields of a tuple struct
    println!("La pareja contiene {:?} y {:?}", pareja.0, pareja.1);
}*/

// An attribute to hide warnings for unused code.
#![allow(dead_code)]

enum SituacionLaboral {
    Trabajando,
    Parado,
    Estudiante,
}

enum Profesion {
    Medico,
    Policia,
    Informatico,    
}

fn main() {
    // la palabra reservada use sería como un include de c/c++, nos sirve para abreviar.
    use SituacionLaboral::{Trabajando, Parado, Estudiante};
    // usar todos.
    use Profesion::*;

    // Equivalente a `SituacionLaboral::Trabajando`.
    let situacion_laboral = Trabajando;
    // Equivalente a `Profesion::Informatico`.
    let profesion = Informatico;

    match situacion_laboral {
        Trabajando => println!("Estoy trabajando!"),
        Parado => println!("Estoy en el paro"),
        Estudiante => println!("Soy estudiante"),
    }

    match profesion {
        Medico => println!("Mi profesión es Médico!"),
        Policia => println!("Mi profesión es policía"),
        Informatico => println!("Mi profesión es Informático"),
    }
}


