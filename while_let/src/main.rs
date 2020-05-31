fn main() 
{
    let mut op = Some(0);

    //Lanzar dardo hasta obtener un 5 
    while let Some(i) = op {
        if i == 5 {
            println!("Premio!");
            op = None;
        } else {
            println!("`i` no es 5, es {:?}. Volver a tirar.", i);
            op = Some(i + 1);
        }     
    }    
}

