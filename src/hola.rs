use std::io; // 1. Importamos la librería de entrada/salida

fn hola() {
    println!("Dime tu nombre:");

    // 2. Creamos una variable mutable para guardar el texto
    // String::new() crea una cadena de texto vacía
    let mut nombre = String::new();

    // 3. Leemos la línea desde el teclado
    io::stdin()
        .read_line(&mut nombre) // Guardamos lo que el usuario escribe en 'nombre'
        .expect("Error al leer la línea"); // Por si algo sale muy mal

    // 4. Saludamos usando el valor guardado
    // .trim() quita el "Enter" que queda al final de la cadena
    println!("¡Hola, {}! Bienvenido a Rust.", nombre.trim());
}

/*Comentario 
multilinea */
//Comentario normal
///Comentario PRO para la documentación
mod saludos; // Esto le dice a Rust: "Busca un archivo llamado saludos.rs"
fn main() {
/*    println!("Hola Mundo");
let _entero: i32 = 42;             // Enteros (i32, i64, u32, etc.)
let _flotante: f64 = 3.1416;       // Decimales
let _booleano: bool = true;        // true o false
let _caracter: char = 'Z';         // Un solo carácter (usa comillas simples)
let _texto: &str = "Hola Rust";    // Cadena de texto (string slice)

let edad = 18;

if edad >= 18 {
    println!("Eres mayor de edad.");
} else if edad == 17 {
    println!("Casi eres mayor.");
} else {
    println!("Eres menor de edad.");
}


let condicion = true;
let numero = if condicion { 5 } else { 6 }; // numero será 5

println!("El valor es: {}", numero);

    let mut contador = 10; // mutable para poder restarle
    let meta = 5;

    if contador > meta {
        println!("El contador es mayor");
        contador = contador - 1; // Podemos cambiarlo porque es 'mut'
    } else {
        println!("Llegaste a la meta");
    }
    
    println!("Valor final: {}", contador);

*/
    // src/main.rs






    let mi_nombre = "Hayato";
    
    // Llamamos a la función usando el nombre del módulo
    saludos::decir_hola(mi_nombre);



}
