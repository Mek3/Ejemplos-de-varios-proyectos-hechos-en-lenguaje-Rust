/*fn forzar_astring(a: &String)
{
    println!("{} convertido a String", a);
}*/

fn forzar_astr(a: &str) {
    println!("{} convertido a str", a);
}

fn main() {
    let s = String::from("Hello, world!");
    println!("{}", s);

    // Vamos a forzarlo a ser str
    forzar_astr(&s);

    /* let a = "Who are you?";
    s.push_str(a);

       println!("{}", s);*/

    let mut n = String::new();

    n.push_str("Hello, world!");
    n.push_str(" Who are you?");
    let m = "hola".to_string();
	println!("{}", m);
    println!("{}", n);
}
