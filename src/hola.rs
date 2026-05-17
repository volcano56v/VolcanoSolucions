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
/*
use std::io;
fn main() {
    /*Dato pro: Si necesitas manejar números astronómicamente grandes (más que u128), existen librerías externas (crates) como num-bigint. */

    //esto son los posibles para usar como numeros enteros
    let _entero: i8 = -8;
    let _entero: i16 = -16;
    let _entero: i32 = -32;
    let _entero: i64 = -64;
    let _entero: i128 = -128;

    let _entero: u8 = 8;
    let _entero: u16 = 16;
    let _entero: u32 = 32;
    let _entero: u64 = 64;
    let _entero: u128 = 128;

    //supongo que esto es para saber el largo
    let _hola: isize;
    let _hola: usize;

    //estos son los float
    let _flotante: f32 = 0.32;
    let _flotante: f64 = 0.64;

    //los booleanos
    let _booleano: bool = true;
    let _booleano: bool = false;

    //caracteres
    let _caracter: char = 'H';

    //texto
    let texto: &str = "Hola Mundo";
    print!("{}",texto);
    print!("{texto}");


    let mi_tupla: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = mi_tupla; // Desestructuración para obtener los valores
    println!("El valor de y es: {}", y);
    //Esto es más la forma tradicional
    println!("El valor de x es: {}, el valor de y es: {}, el valor de z es: {}. Esto es todo, esto es solo un mensaje extra", x, y, z);
    //truco para rust 1.58+, esto solo fucniona desde la version antentes mencionada desde adelante
    println!("X es: {x}, Y es: {y}, Z es: {z}. Mensaje extra.");
    println!("Numero es: {x}");




    let _meses = ["Enero", "Febrero", "Marzo"];
    println!("{:?}", _meses);
    println!("{:#?}", _meses);
    let _numeros: [i32; 5] = [1, 2, 3, 4, 5]; // [tipo; cantidad]


    //esta es una estructura condicional
    let edad = 18;

    if edad >= 18 {
        println!("Eres mayor de edad");
    } else {
        println!("Eres menor de edad");
    }


    println!("Escribe algo:");

    let mut entrada = String::new(); // Creamos una variable variable (mut) vacía

    io::stdin()
        .read_line(&mut entrada) // Leemos la línea y la guardamos en 'entrada'
        .expect("Error al leer la línea"); // Manejo de errores obligatorio

    println!("Escribiste: {}", entrada);

    if entrada.trim() == "Hola" {
    println!("Tú también dijiste hola");
    }

    //esta es otra forma
    let mut entrada = String::new();
    std::io::stdin().read_line(&mut entrada).unwrap(); 
    println!("Dijo: {entrada}");

    println!("Nombre:");
    let nombre = leer_consola();
    println!("Hola {nombre}!");


}

fn leer_consola() -> String {
    let mut buffer = String::new();
    
    // El match revisa el resultado de la lectura
    match std::io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            buffer.trim().to_string()
        }
        Err(e) => {
            // Si hay error, imprimimos el porqué y devolvemos algo vacío
            println!("Hubo un error real: {e}");
            String::new() 
        }
    }
}*/