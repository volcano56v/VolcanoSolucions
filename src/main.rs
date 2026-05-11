fn main() {
    /*Dato pro: Si necesitas manejar números astronómicamente grandes (más que u128), existen librerías externas (crates) como num-bigint. */

    //esto son los posibles para usar como numeros enteros
    let entero: i8 = -8;
    let entero: i16 = -16;
    let entero: i32 = -32;
    let entero: i64 = -64;
    let entero: i128 = -128;

    let entero: u8 = 8;
    let entero: u16 = 16;
    let entero: u32 = 32;
    let entero: u64 = 64;
    let entero: u128 = 128;

    //supongo que esto es para saber el largo
    let hola: isize;
    let hola: usize;

    //estos son los float
    let flotante: f32 = 0.32;
    let flotante: f64 = 0.64;

    //los booleanos
    let booleano: bool = true;
    let booleano: bool = false;

    //caracteres
    let caracter: char = 'H';

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




    let meses = ["Enero", "Febrero", "Marzo"];
    let numeros: [i32; 5] = [1, 2, 3, 4, 5]; // [tipo; cantidad]


}