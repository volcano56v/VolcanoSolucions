// src/saludos.rs
pub fn decir_hola(nombre: &str) {
    println!("¡Hola, {} desde otro archivo!", nombre);
}

/*
Sí, aunque let es lo que usarás el 99% del tiempo para variables locales, 
Rust tiene otras formas de definir valores dependiendo de dónde los necesites y 
cómo quieras que se comporten.Aquí tienes las alternativas:1. const (Constantes)
Se usan para valores que nunca cambian y cuyo valor se conoce antes de que el 
programa se ejecute (en tiempo de compilación).Obligatorio: Siempre debes escribir 
el tipo de dato.Nomenclatura: Por convención, se escriben siempre en MAYUSCULAS_CON_GUIONES.Uso: 
Se pueden declarar fuera del main (ámbito global).rustconst PI: f64 = 3.141592;
const SEGUNDOS_EN_UN_MINUTO: u32 = 60;
Usa el código con precaución.2. static (Variables Estáticas)Parecidas a las constantes, pero 
representan una posición fija en la memoria.Duran todo el tiempo que el programa esté encendido.A 
diferencia de const, una variable static puede ser mutable (usando static mut), pero esto se 
considera "inseguro" y requiere bloques especiales de código (unsafe).ruststatic NOMBRE_APP: &str = "Mi Super App";
Usa el código con precaución.3. Diferencia clave: let vs constCaracterísticaletconstInferenciaRust 
adivina el tipo.Debes decir el tipo sí o sí.LugarSolo dentro de funciones.En cualquier lado 
(global o local).MutabilidadPuede ser mut.Jamás puede ser mutable.ValorPuede calcularse al ejecutarse.Debe 
ser un valor fijo al compilar.4. static mut (Solo si eres valiente)Rust es famoso por la seguridad, y las 
variables globales mutables son peligrosas porque varios hilos podrían intentar cambiarlas a la vez. Por eso, 
para usarlas, Rust te obliga a usar la palabra unsafe:ruststatic mut CONTADOR_GLOBAL: u32 = 0;

fn main() {
    unsafe {
        CONTADOR_GLOBAL += 1;
    }
}
Usa el código con precaución.5. Bonus: fn y struct / enumAunque no son variables, son las otras palabras clave 
para definir "cosas" en Rust:fn: Define funciones (bloques de lógica).struct: Define estructuras (agrupación de datos).enum: Define 
enumeraciones (opciones de tipos).¿Te gustaría ver cómo crear un proyecto pequeño que use una constante y una variable mutable 
para aplicar todo lo que hemos visto? Es la mejor forma de que se te quede grabado.
*/