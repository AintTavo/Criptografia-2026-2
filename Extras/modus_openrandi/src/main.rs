// Librerias estandar
use std::collections::HashMap;
use std::time::Instant;
use std::iter::*;

// Crates externas
use colored::Colorize;
use matrix_operations::matrix_inverse_module;

/*  
    --------------------------------------------------------------------------------
    Test Area : Funcion Main
    --------------------------------------------------------------------------------
*/  
fn main() {
    let block = [10, 50, 9];
    let block_2 = [14, 100, 197];
    let init = [9, 99, 11];
    let key = [ 1, 2, 3, 4, 5, 6, 11, 9, 8];
    let m = 256;
    let mesage = [10, 50, 9, 10, 50, 9, 10, 50, 9, 10, 50, 9, 10, 50, 9, 10, 50, 9, 10, 50, 9];

    let cipher_text = hill_cipher(&block, &key, m);
    debug_block("cipher Text", &cipher_text);

    let inicio = Instant::now();
    let cipher_text_ecb = modus_ecb_hc(&mesage, 3, &key, m);
    let duracion = inicio.elapsed();
    debug_block("cipher Text ECB", &cipher_text_ecb);
    println!("Tiempo transcurrido: {:?}", duracion);
    
    let cipher_text_cbc = modus_cbc_hc(&mesage, 3, &init, &key, m);
    debug_block("cipher Text CBC", &cipher_text_cbc);

    let cipher_text_cfb = modus_cfb_hc(&mesage, 3, &init, &key, m);
    debug_block("cipher Text CFB", &cipher_text_cfb);

    let cipher_text_ofb = modus_ofb_hc(&mesage, 3, &init, &key, m);
    debug_block("cipher Text OFB", &cipher_text_ofb);

    let tmp_block = block_xor(&block, &block_2);
    debug_block("Xor", &tmp_block);

}

/*  
    --------------------------------------------------------------------------------
    Main : Función principal del programa, aquí se ejecuta el código principal del programa, y se llama a las funciones necesarias para el desarrollo del programa.
    --------------------------------------------------------------------------------
*/  

// -> Hill Cipher : Aplicacion del cifrador en hill basado en algebra lineal.
fn hill_cipher( 
    block: &[i32], 
    key: &[i32], 
    m: i32
) -> Vec<i32> {

    // Variable de retorno
    let mut cipher_text: Vec<i32> = Vec::new();

    // # Correción de errores HC(1):
    // La clave tiene que ser al menos del tamaño del blocke al cuadrado, sino el cifrado es imposible de realizar
    if key.len() != (block.len() * block.len()) {
        cipher_text.push(1);
        error("The key must be a square matrix of the same size as the block");
        return cipher_text;
    }

    // Ciclo: Realiza una multiplicación de matrices
    for i in 0..block.len() {
        let mut sum = 0;

        for j in 0..block.len() {
            sum += block[j] * key[j * block.len() + i];
        }
        sum = module(sum, m);
        cipher_text.push(sum);
    }

    // Regresa el texto cifrado
    return cipher_text;
}


// -> Modo de operación Electronic Code Book (ECB) con Hill  cipher: Se aplica el cifrado directamente en cada bloque
// C = Ek(p)
pub fn modus_ecb_hc(
    msg : &[i32],       // Mensaje a encriptar
    block_size : usize, // Tamaño de bloque a encriptar
    key : &[i32],       // Llave
    m : i32             // Modulo
) -> Vec<i32> {
    let mut cipher_text: Vec<i32> = Vec::new();                            // Variable de retorno

    // # Correción de errores ECB(1):
    // Si la llave no corresponde con el tamaño correcto para el cifrado hill retorna.
    if key.len() != (block_size * block_size) {
        error("The key does not correspond with the block size for hill cipher");
        cipher_text.push(1);
        return cipher_text;
    }

    let mut _tmp_msg: Vec<i32> = Vec::new();                               // Variable para copiar y reajustar el mensaje a cifrar
    let padding = (block_size - (msg.len() % block_size)) % block_size;    // Variable para determinar si el ultimo bloque tiene suficiente tamaño

    _tmp_msg = msg.to_vec().clone();        // Se clona el mensaje original

    // # Correción de errores ECB(2):
    // En caso de que el bloque final no tenga suficiente tamaño, se le añaden 1's para rellenarlo
    _tmp_msg.extend(repeat(1).take(padding));

    // Se rompe el mensaje en un arreglo de arreglos del tamaño del bloque
    let msg_blocks = _tmp_msg.chunks(block_size);

    // Mapa para guardar datos calculados y ahorrar operaciónes
    let mut cipher_map : HashMap< Vec<i32>, Vec<i32> > = HashMap::new(); 
    
    // Ciclo : Electronic CodeBook main loop, da una pasada por todos los bloques del cifrado
    for i in msg_blocks {
        let block = i.to_vec();                         // i es un apuntador entonces se tiene que pasar a vector
        let cipher_block : Vec<i32>;                    // Se crea un cipher block temporal

        match cipher_map.get(&block) {                  // Se busca en el hash map si ya se ha calculado el valor y si:
            Some(p) => cipher_block = p.to_vec(),       // Si existe se pasa directo como cipher_block
            None => {                                   // Si no existe, se calcula y añade a la tabla hash
                cipher_block = hill_cipher(&block, key, m);
                cipher_map.entry(block).or_insert(cipher_block.clone());
            }
        }
        cipher_text.extend(cipher_block);   // Se agrega el bloque cifrado al final del texto cifado
    }

    let _cipher_size = cipher_text.len();                           // Se calcula el tamaño del mensaje final
    cipher_text.drain(( _cipher_size - padding ).._cipher_size);    // Se le recortan los datos de holgura

    return cipher_text;
}


// -> Modo de operación Cipher Block Chaining (CBC) :
// Cn = Ek(p xor Cn-1)
pub fn modus_cbc_hc( 
    msg : &[i32],           // Mensaje a encriptar
    block_size : usize,     // Tamaño del bloque
    c_0 :&[i32],            // Bloque de cifrado original
    key : &[i32],           // Llave
    m : i32                 // Modulo
) -> Vec<i32> {

    let mut cipher_text: Vec<i32> = Vec::new();     // Varaible de retorno

    // Correción de errores CBC (1):
    // Si la llave para el cifrado hill no es el cuadrado del tamaño del bloque retorna 1.
    if key.len() != (block_size * block_size) {
        error("The key does not correspond with the block size for hill cipher");
        cipher_text.push(1);
        return cipher_text;
    }

    // Correción de errores CBC (2):
    // Si el bloque de cifrado inicial no corresponde con el tamaño correcto retorna 1.
    if c_0.len() != block_size {
        error("The initial block is not the size of a normal block");
        cipher_text.push(1);
        return cipher_text;
    } 

    let mut _tmp_msg: Vec<i32> = Vec::new();                               // Variable para copiar y reajustar el mensaje a cifrar
    let padding = (block_size - (msg.len() % block_size)) % block_size;    // Variable para determinar si el ultimo bloque tiene suficiente tamaño

    _tmp_msg = msg.to_vec().clone();        // Se clona el mensaje original

    // # Correción de errores CBC (2):
    // En caso de que el bloque final no tenga suficiente tamaño, se le añaden 1's para rellenarlo
    _tmp_msg.extend(repeat(1).take(padding));

    // Se rompe el mensaje en un arreglo de arreglos del tamaño del bloque
    let msg_blocks = _tmp_msg.chunks(block_size);

    let mut tmp_block : Vec<i32> = c_0.to_vec(); // Se inicializa el valor del bloque temporal con el del bloque inicial

    // # Ciclo : Main Loop de Cipher Block Chaining 
    for i in msg_blocks {
        let block = i.to_vec();                                 // i es un apuntador entonce se convierte a un vector

        let _xor_result = block_xor(&block, &tmp_block);        // Se aplica una xor con el bloque anterior 
        let _cipher_block = hill_cipher(&_xor_result, &key, m); // Se aplica el cifrado al bloque
        tmp_block = _cipher_block.clone();                      // se pasa el nuevo bloque al bloque actual

        cipher_text.extend(_cipher_block);                      // Se guarda el bloque cifrado en el texto final
    }

    
    let _cipher_size = cipher_text.len();                           // Se calcula el tamaño del mensaje final
    cipher_text.drain(( _cipher_size - padding ).._cipher_size);    // Se le recortan los datos de holgura

    return cipher_text;
}


// -> Modo de operación Cipher Feedback (CFB) : 
pub fn modus_cfb_hc(
    msg : &[i32], 
    block_size : usize, 
    c_0 :&[i32], 
    key : &[i32], 
    m : i32 
) -> Vec<i32> {
    let mut cipher_text: Vec<i32> = Vec::new();

    if key.len() != (block_size * block_size) {
        error("The key does not correspond with the block size for hill cipher");
        cipher_text.push(1);
        return cipher_text;
    }

    if c_0.len() != block_size {
        error("The initial block is not the size of a normal block");
        cipher_text.push(1);
        return cipher_text;
    } 

    let mut _tmp_msg: Vec<i32> = Vec::new();
    let padding = block_size - (msg.len() % block_size);

    _tmp_msg = msg.to_vec().clone();

    if msg.len() % block_size != 0 {
        for _ in 0..padding {
            _tmp_msg.push(1);
        }
    }

    let msg_blocks = _tmp_msg.chunks(block_size);
    let mut tmp_block : Vec<i32> = c_0.to_vec().clone();

    for i in msg_blocks {
        let block = i.to_vec();

        let _cipher_block = hill_cipher(&tmp_block, &key, m);
        let _xor_result = block_xor(&block, &_cipher_block);
        tmp_block = _xor_result.to_vec().clone();

        cipher_text.extend(_xor_result);
    }

    
    cipher_text.drain(( cipher_text.len() - padding )..cipher_text.len());

    return cipher_text;
}


// -> Modo de operación Cipher Feedback (OFB) :
pub fn modus_ofb_hc(
    msg : &[i32], 
    block_size : usize, 
    c_0 :&[i32], 
    key : &[i32], 
    m : i32 
) -> Vec<i32> {
    let mut cipher_text: Vec<i32> = Vec::new();

    if key.len() != (block_size * block_size) {
        error("The key does not correspond with the block size for hill cipher");
        cipher_text.push(1);
        return cipher_text;
    }

    if c_0.len() != block_size {
        error("The initial block is not the size of a normal block");
        cipher_text.push(1);
        return cipher_text;
    } 

    let mut _tmp_msg: Vec<i32> = Vec::new();
    let padding = block_size - (msg.len() % block_size);

    _tmp_msg = msg.to_vec().clone();

    if msg.len() % block_size != 0 {
        for _ in 0..padding {
            _tmp_msg.push(1);
        }
    }

    let msg_blocks = _tmp_msg.chunks(block_size);
    let mut tmp_block : Vec<i32> = c_0.to_vec().clone();

    for i in msg_blocks {
        let block = i.to_vec();

        let _c_0_ek = hill_cipher(&tmp_block, &key, m);
        let _cipher_block = block_xor(&_c_0_ek, &block);
        
        tmp_block = _c_0_ek.to_vec().clone();

        cipher_text.extend(_cipher_block);
    }

    
    cipher_text.drain(( cipher_text.len() - padding )..cipher_text.len());

    return cipher_text;
}

pub fn modus_pcbc_hc() {

}

pub fn modus_ctr_hc() {

}

/*  
    --------------------------------------------------------------------------------
    Tool functions : Funciones que sirven principalmente como herramientas para el resto del desarrollo.
    --------------------------------------------------------------------------------
*/  

// -> Función Modulo, pero siempre positivo
#[inline(always)]   // Hace que en donde este en el codigo el compilador sustituya esa parte de codigo por directamente la función
fn module( a: i32 , m : i32 ) -> i32 {
    return ( ( a % m ) + m ) % m ;  // Al sumar un modulo y aplicarlo nuevamente y darle modulo hace que sean siempre positivos.
}

fn block_xor( block_1 : &[i32], block_2 : &[i32]) -> Vec<i32> {
    let  result : Vec<i32> = block_1.to_vec().iter()
        .zip(block_2.to_vec().iter())
        .map(|(x , y)| x ^ y)
        .collect();
    return result;
}

// -> Función para imprimir una saldia formateada para error
fn error( message : &str ) {
    println!("{} {}", "Error:".red().bold(), message.red());
} 


fn debug_block(label : &str, block : &[i32]) {
    print!("{}", "Debug [".yellow().bold());
    print!("{}", label.yellow().italic());
    print!("{} ", "]: ".yellow().bold());
    print!("{}", "[".yellow());
    for i in block {
        print!("\t{}", i.to_string());
    }
    println!("{}", "]".yellow());
}
