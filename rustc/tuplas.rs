fn main()
{ 
    let tupla = (24, "Mohamed", false);
    let (edad,nombre,hace_ejercicio) = tupla;
    println!("Datos tupla: ({}, {}, {}) ",edad, nombre, hace_ejercicio);

    let (_,nombre,_) = tupla; //Ignoramos el 1º y el 3º valor.
    println!("Nombre: {}",nombre);

    println!("hace ejercicio? : {}", tupla.2);
}
