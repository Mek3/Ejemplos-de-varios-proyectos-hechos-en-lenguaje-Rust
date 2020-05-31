fn main() {
    let mut i = 0;

    let resultado = loop {
        
        if i == 5 {
            break i * 14;
        }else {
			i += 1;
		}
    };
    
	println!("el resultado es: {:?}", resultado);
}

