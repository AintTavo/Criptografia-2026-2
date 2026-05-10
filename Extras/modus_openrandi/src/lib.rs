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

// -> Modo de operación Electronic Code Book (ECB) con Hill  cipher: Se aplica el cifrado directamente en cada bloque
// C = Ek(p)
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

// -> Modo de operación Cipher Block Chaining (CBC) :
// Cn = Ek(p xor Cn-1)
#[wasm_bindgen]
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
// Cn = Ek(Cn-1) xor p
#[wasm_bindgen]
pub fn modus_cfb_hc(
    msg : &[i32],       // Mensaje a encriptar
    block_size : usize, // Tamaño de bloque
    c_0 :&[i32],        // Vector de inicialización (IV)
    key : &[i32],       // Llave
    m : i32             // Modulo
) -> Vec<i32> {
    let mut cipher_text: Vec<i32> = Vec::new(); // Variable de retorno

    // # Corrección de errores CFB(1):
    // Verificación de integridad de la llave para el cifrado Hill
    if key.len() != (block_size * block_size) {
        error("The key does not correspond with the block size for hill cipher");
        cipher_text.push(1);
        return cipher_text;
    }

    // # Corrección de errores CFB(2):
    // Verificación de que el IV tenga el tamaño de bloque correspondiente
    if c_0.len() != block_size {
        error("The initial block is not the size of a normal block");
        cipher_text.push(1);
        return cipher_text;
    } 

    let mut _tmp_msg: Vec<i32> = Vec::new();                            // Variable para reajustar el mensaje
    let padding = ( block_size - ( msg.len() % block_size ) ) % block_size;                // Cálculo del relleno necesario

    _tmp_msg = msg.to_vec().clone();                                    // Clonación del mensaje original

    // # Corrección de errores CFB(3):
    // Relleno de bloques incompletos con 1's si no es múltiplo del tamaño de bloque
    _tmp_msg.extend(repeat(1).take(padding));

    // División del mensaje en bloques del tamaño especificado
    let msg_blocks = _tmp_msg.chunks(block_size);
    let mut tmp_block : Vec<i32> = c_0.to_vec().clone();                // Registro temporal para encadenamiento

    // # Ciclo : Main Loop de Cipher Feedback
    for i in msg_blocks {
        let block = i.to_vec();

        let _c_ek = hill_cipher(&tmp_block, &key, m);                   // Se cifra el bloque anterior o IV
        let _cipher_block = block_xor(&block, &_c_ek);                  // XOR entre texto plano y salida del cifrador
        tmp_block = _cipher_block.to_vec().clone();                     // La salida cifrada alimenta el siguiente bloque

        cipher_text.extend(_cipher_block);                              // Se anexa al resultado final
    }

    // Eliminación de los datos de holgura (padding) del mensaje final
    let _cipher_size = cipher_text.len();                           // Se calcula el tamaño del mensaje final
    cipher_text.drain(( _cipher_size - padding ).._cipher_size);    // Se le recortan los datos de holgura

    return cipher_text;
}


// -> Modo de operación Output Feedback (OFB) :
// C = Ek(Co) xor p
#[wasm_bindgen]
pub fn modus_ofb_hc(
    msg : &[i32],       // Mensaje a encriptar
    block_size : usize, // Tamaño de bloque
    c_0 :&[i32],        // Vector de inicialización (IV)
    key : &[i32],       // Llave
    m : i32             // Modulo
) -> Vec<i32> {
    let mut cipher_text: Vec<i32> = Vec::new(); // Variable de retorno

    // # Corrección de errores OFB(1):
    // Verificación de tamaño de llave
    if key.len() != (block_size * block_size) {
        error("The key does not correspond with the block size for hill cipher");
        cipher_text.push(1);
        return cipher_text;
    }

    // # Corrección de errores OFB(2):
    // Verificación de tamaño del bloque inicial
    if c_0.len() != block_size {
        error("The initial block is not the size of a normal block");
        cipher_text.push(1);
        return cipher_text;
    } 

    let mut _tmp_msg: Vec<i32> = Vec::new();                            // Variable auxiliar para el mensaje
    let padding = ( block_size - ( msg.len() % block_size ) ) % block_size;                // Variable de ajuste de tamaño

    _tmp_msg = msg.to_vec().clone();

    // Relleno manual de bloques si el mensaje no es múltiplo del bloque
    _tmp_msg.extend(repeat(1).take(padding));

    let msg_blocks = _tmp_msg.chunks(block_size);
    let mut tmp_block : Vec<i32> = c_0.to_vec().clone();                // Bloque para retroalimentación de salida

    // # Ciclo : Main Loop de Output Feedback
    for i in msg_blocks {
        let block = i.to_vec();

        let _c_0_ek = hill_cipher(&tmp_block, &key, m);                 // Se cifra el flujo de salida independientemente del mensaje
        let _cipher_block = block_xor(&_c_0_ek, &block);                // XOR entre el flujo cifrado y el texto plano
        
        tmp_block = _c_0_ek.clone();                                    // La salida del cifrador (antes de XOR) se retroalimenta

        cipher_text.extend(_cipher_block);
    }

    // Recorte de padding para recuperar tamaño original
    let _cipher_size = cipher_text.len();                           // Se calcula el tamaño del mensaje final
    cipher_text.drain(( _cipher_size - padding ).._cipher_size);    // Se le recortan los datos de holgura

    return cipher_text;
}

// -> Modo de operación Propagating Cipher Block Chaining (PCBC)
// n = 0    |   C = Ek ( p xor Co )
// n = 1    |   C = Ek ( (p n-1 xor C n-1) xor p )
#[wasm_bindgen]
pub fn modus_pcbc_hc(
    msg : &[i32],       // Mensaje a encriptar
    block_size : usize, // Tamaño de bloque
    c_0 :&[i32],        // Vector de inicialización (IV)
    key : &[i32],       // Llave
    m : i32             // Módulo
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

    // # Ciclo : Main Loop de Propagating Cipher Block Chaining 
    for i in msg_blocks {
        let block = i.to_vec();                                 // i es un apuntador entonce se convierte a un vector

        let _xor_result = block_xor(&block, &tmp_block);        // Se aplica una xor con el bloque anterior 
        let _cipher_block = hill_cipher(&_xor_result, &key, m); // Se aplica el cifrado al bloque
        tmp_block = block_xor(&block, &_cipher_block);          // Se aplica la funcion de propagación

        cipher_text.extend(_cipher_block);                      // Se guarda el bloque cifrado en el texto final
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