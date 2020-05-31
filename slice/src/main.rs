fn analiza_slice(slice: &[i32]) 
{
    println!("El primer elemento del slice: {}", slice[0]);
    println!("El slice tiene {} elementos", slice.len());
}

fn main()
{   
    let lista: [i32; 5] = [1, 2, 3, 4, 5];

    // Convertir un array en un slice
    analiza_slice(&lista);

    // Analizar solo una sección.
    analiza_slice(&lista[1 .. 4]);
}
