mod my {
    // Estructura publica con un tipo genérico publico.
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // Estructura privada con un elemento genérico privado.
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        pub contents: T,
    }

    impl<T> ClosedBox<T> {
        // Constructor público
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }            
        }
    }
}

fn main()
{
	
    // Public structs with public fields can be constructed as usual
    let open_box = my::OpenBox { contents: "Objeto" };

    // and their fields can be normally accessed.
    println!("La caja contiene: {}", open_box.contents);


    // Inicializar el constructor
    let _closed_box = my::ClosedBox::new("Objeto dos");
    
    println!("La caja contiene: {}", _closed_box.contents);  
   
}

