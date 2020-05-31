#[warn(dead_code)]
/// trait que incluye dentro una funcion que calcula el area de una figura dada.
trait Area {
    fn obtener_area(&self) -> f64;
}

/// Estructura básica que representa a un rectagulo.
struct Rectangulo {
    a: f64,
    b: f64,
}

/// Implemntación de la función del trait para el calculo del area de un rectagulo
impl Area for Rectangulo {
    fn obtener_area(&self) -> f64 {
        std::f64::consts::PI * (self.a * self.b)
    }
}

/// Estructura básica que representa a un cuadrado.
struct Cuadrado {
    lado: f64,
}

/// Implemntación de la función del trait para el calculo del area de cuadrado.
impl  Area for Cuadrado {
    fn obtener_area(&self) -> f64 {
        self.lado * self.lado
    }
}

fn main()
{
    ///Creación del rectangulo.
    let r = Rectangulo {
        a: 2.0f64,
        b: 7.0f64,
    };
    ///Creación del cuadrado.
    let c = Cuadrado{  
        lado: 4.0f64,
    };

	///Llamadas a la función definida en el trait.
    println!("El area del rectandgulo es: {}", r.obtener_area());
    println!("El area del cuadrado es:  {}", c.obtener_area());
}
