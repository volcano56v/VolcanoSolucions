use std::io::{self, Write};
use std::str::FromStr;
use std::string;
fn main() {
    /*Dato pro: Si necesitas manejar números astronómicamente grandes (más que u128), existen librerías externas (crates) como num-bigint. */

   /* //esto son los posibles para usar como numeros enteros
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

    println!("Nombre:");
    let nombre = leer_consola();
    println!("Hola {nombre}!");

    if nombre.trim() == "Vicente" {
    println!("Tú también dijiste hola");
    }

    let n_i32: i32 = mostrar_por_consola("este es un numero de 32 bit");*/

    println!("=== MENÚ DE OPCIONES ===");
    println!("1. Saludar");
    println!("2. Mostrar versión de Rust");
    println!("3. Salir");
    println!("========================");

    let opcion = pedir_opcion();

    match opcion {
    1 => println!("¡Hola!"),
    2 => {
        println!("Consultando al sistema operativo...");
        
        // Ejecuta e imprime directamente en la consola actual
        std::process::Command::new("rustc")
            .arg("-V")
            .status()
            .expect("Error al ejecutar");
    }
    3 => println!("Saliendo... ¡Adiós!"),
    _ => println!("Opción no válida."),
}

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
}

fn mostrar_por_consola<T: FromStr>(mensaje: &str) -> T {
    loop {
        // 1. Imprimir el mensaje en la misma línea
        print!("{}", mensaje);
        io::stdout().flush().expect("Error al limpiar consola");

        // 2. Leer la línea
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Error al leer");

        // 3. Intentar convertir al tipo T (i8, u64, f32, bool, etc.)
        match buffer.trim().parse::<T>() {
            Ok(valor) => return valor,
            Err(_) => println!("Entrada inválida. Reintenta."),
        }
    }
}

fn pedir_opcion() -> i32 {
    loop {
        print!("Selecciona una opción: ");
        io::stdout().flush().expect("Error al limpiar consola");

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Error al leer");

        match buffer.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => println!("Por favor, introduce un número válido."),
        }
    }
}
