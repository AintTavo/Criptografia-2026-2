// Librerias Estándar
use std::collections::HashMap;
use std::iter::*;

// Crates Externas
use wasm_bindgen::prelude::*;

/*  
    --------------------------------------------------------------------------------
    Functions to Export : Funciones que estaran publicas para JS
    --------------------------------------------------------------------------------
*/

#[wasm_bindgen]
pub fn modus_ecb_hc(
    msg : &[i32],       // Mensaje a encriptar
    block_size : usize, // Tamaño de bloque a encriptar
    key : &[i32],       // Llave
    m : i32             // Modulo
) -> Vec<i32> {
    let mut cipher_text: Vec<i32> = Vec::new();                            // Variable de retorno

    // Correción de errores ECB(1):
    // Si la llave no corresponde con el tamaño correcto para el cifrado hill retorna.
    if key.len() != (block_size * block_size) {
        println!("Error: The key does not correspond with the block size for hill cipher");
        cipher_text.push(1);
        return cipher_text;
    }

    let mut _tmp_msg: Vec<i32> = Vec::new();                               // Variable para copiar y reajustar el mensaje a cifrar
    let padding = (block_size - (msg.len() % block_size)) % block_size;    // Variable para determinar si el ultimo bloque tiene suficiente tamaño

    _tmp_msg = msg.to_vec().clone();        // Se clona el mensaje original

    // Correción de errores ECB(2):
    // En caso de que el bloque final no tenga suficiente tamaño, se le añaden 1's para rellenarlo
    _tmp_msg.extend(repeat(1).take(padding));

    // Se rompe el mensaje en un arreglo de arreglos del tamaño del bloque
    let msg_blocks = _tmp_msg.chunks(block_size);

    // Mapa para guardar datos calculados y ahorrar operaciónes
    let mut cipher_map : HashMap< Vec<i32>, Vec<i32> > = HashMap::new(); 
    
    // -> Electronic CodeBook main loop, da una pasada por todos los bloques del cifrado
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

/*  
    --------------------------------------------------------------------------------
    Private Functions : Funciones necesarias para el funcionamiento de las publicas
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

    // Correción de errores
    // La clave tiene que ser al menos del tamaño del blocke al cuadrado, sino el cifrado es imposible de realizar
    if key.len() != (block.len() * block.len()) {
        cipher_text.push(1);
        println!("Error: The key must be a square matrix of the same size as the block");
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

#[inline(always)]
fn module( a: i32 , m : i32 ) -> i32 {
    return ( ( a % m ) + m ) % m ;
}