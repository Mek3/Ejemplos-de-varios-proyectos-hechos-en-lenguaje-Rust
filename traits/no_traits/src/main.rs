#[derive(Debug)]

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

// Implementando funciones
impl Rectangulo {
	fn mostrar_puntos(&self){
		// Accediendo a los puntos
		print!("Listado de puntos: ({}, {}), ", self.p1.x, self.p1.y);
		print!("({}, {}), ", self.p2.x, self.p2.y);
		print!("({}, {}), ", self.p3.x, self.p3.y);
		println!(" ({}, {})", self.p4.x, self.p4.y);
	}
}

fn main() {   
    // Inicializar estructuras
    
    let p1: Punto = Punto { x: 0.0, y: 0.0 };
	let p2: Punto = Punto { x: 0.0, y: 1.0 };
	let p3: Punto = Punto { x: 2.0, y: 1.0 };
	let p4: Punto = Punto { x: 2.0, y: 0.0 };
		
    let _rectangulo = Rectangulo {p1, p2, p3, p4};
    
    _rectangulo.mostrar_puntos();

    //Asignar estructura de unidad
    let _nil = Nil;

    // Inicializar tupla
    let pareja = Pareja(1, 0.1);

    // Accediendo a los campos de pareja
    println!("La pareja contiene {:?} y {:?}", pareja.0, pareja.1);
}

