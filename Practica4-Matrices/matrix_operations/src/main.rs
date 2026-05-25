use colored::Colorize;
use wasm_bindgen::prelude::*;

/*  
    --------------------------------------------------------------------------------
    Test Area : Funcion Main
    --------------------------------------------------------------------------------
*/

fn main() {
    let matrix_0 = [5, 17, 20, 9, 23, 3, 2, 11, 13];
    println!("{}", "Matriz original 0:".blue());
    print_matrix(&matrix_0);

    println!("{}", "Matriz 0 transpuesta:".blue());
    print_matrix(&matrix_transposed(&matrix_0));

    println!("{} {}", "Determinante 0:".blue(), matrix_determinant(&matrix_0));

    println!("{}", "Cofactor (0,0) de Matriz 0:".blue());
    print_matrix(&matrix_cofactor(&matrix_0, 0, 0));

    println!("{}", "Inversa modular de Matriz 0 (mod 27):".blue());
    print_matrix(&matrix_inverse_module(&matrix_0, 27));

    println!("{}", "Verificación: Matriz 0 * Inversa mod 27 = Identidad:".blue());
    let inv0 = matrix_inverse_module(&matrix_0, 27);
    if !inv0.is_empty() {
        print_matrix(&matrix_multiplication_matrix_module(&matrix_0, &inv0, 27));
    }

    println!();

    let matrix_1 = [0, 1, 2, 3];
    println!("{}", "Matriz original 1 (2x2):".blue());
    print_matrix(&matrix_1);
    println!("{}", "Inversa modular de Matriz 1 (mod 256):".blue());
    print_matrix(&matrix_inverse_module(&matrix_1, 256));
}

/*  
    --------------------------------------------------------------------------------
    Main code : Algebra lineal pura
    --------------------------------------------------------------------------------
*/

/// Calcula la matriz inversa real (f64) de una matriz cuadrada 2x2 o 3x3.
#[wasm_bindgen]
pub fn matrix_inverse(matrix: &[i32]) -> Vec<f64> {
    let det = matrix_determinant(matrix);
    if det == 0 {
        error("The inverse matrix does not exist. The determinant is equal to 0.");
        return Vec::new();
    }
    let adjugate = matrix_adjugate(matrix);
    let inv_det = 1.0 / det as f64;
    matrix_multiplication_escalar_f(&adjugate, inv_det)
}

/// Calcula el determinante de una matriz cuadrada de 1x1, 2x2 o 3x3.
#[wasm_bindgen]
pub fn matrix_determinant(matrix: &[i32]) -> i32 {
    let len = matrix.len();
    let size_f = (len as f64).sqrt();

    if size_f != size_f.trunc() {
        error("This function only accepts square matrices");
        return -1;
    }

    let size = size_f as u32;

    if size > 3 {
        error("The function only accepts square matrices of size 1 to 3");
        return -1;
    }

    if size == 1 {
        return matrix[0];
    }

    if size == 2 {
        return matrix[0] * matrix[3] - matrix[1] * matrix[2];
    }

    // Regla de Sarrus para 3x3
    let s = size as i32;
    let mut det = 0i32;

    for i in 0..s {
        let mut pos = 1i32;
        for j in 0..s {
            pos *= find_in_matrix(matrix, size, j, (i + j) % s);
        }
        det += pos;

        let mut neg = 1i32;
        for j in 0..s {
            neg *= find_in_matrix(matrix, size, j, (i - j).rem_euclid(s));
        }
        det -= neg;
    }

    det
}

/// Calcula la matriz transpuesta de una matriz cuadrada 2x2 o 3x3.
#[wasm_bindgen]
pub fn matrix_transposed(matrix: &[i32]) -> Vec<i32> {
    let size = matrix.len();
    if !matrix_validation(size) {
        return vec![-1];
    }
    let n = (size as f64).sqrt() as u32;
    let s = n as i32;
    let mut result = Vec::with_capacity(size);
    for i in 0..s {
        for j in 0..s {
            result.push(find_in_matrix(matrix, n, j, i));
        }
    }
    result
}

/// Calcula la matriz adjunta (transpuesta de la matriz de cofactores).
///
/// BUG CORREGIDO: La versión anterior construía la cofactor-matrix y luego
/// transponía por separado, lo que resultaba en una doble transposición cuando
/// se usaba desde matrix_inverse_module.  Ahora se construye directamente en
/// orden transpuesto: adjugate[col][row] = sign(row,col) * det(minor(row,col)).
#[wasm_bindgen]
pub fn matrix_adjugate(matrix: &[i32]) -> Vec<i32> {
    let size = matrix.len();
    if !matrix_validation(size) {
        error("The size of the matrix is not valid for a square matrix");
        return vec![-1];
    }
    let n = (size as f64).sqrt() as i32;
    let mut result = Vec::with_capacity(size);

    for col in 0..n {
        for row in 0..n {
            let minor = matrix_cofactor(matrix, row, col);
            let minor_det = matrix_determinant(&minor);
            let sign = if (row + col) % 2 == 0 { 1 } else { -1 };
            result.push(sign * minor_det);
        }
    }
    result
}

/// Extrae el menor (submatriz) eliminando la fila i_coef y la columna j_coef.
///
/// BUG CORREGIDO: find_in_matrix recibe (matrix, size, ROW, COL) pero la
/// versión original pasaba (matrix, size, j, i) — filas y columnas invertidas.
pub fn matrix_cofactor(matrix: &[i32], i_coef: i32, j_coef: i32) -> Vec<i32> {
    let size = matrix.len();
    if !matrix_validation(size) {
        error("The size of the matrix is not valid for a square matrix");
        return vec![-1];
    }
    let n = (size as f64).sqrt() as i32;
    let un = n as u32;

    if i_coef >= n || j_coef >= n {
        error("The cofactor index is out of bounds.");
        return vec![-1];
    }

    let mut result = Vec::with_capacity((n as usize - 1).pow(2));
    for i in 0..n {
        if i == i_coef { continue; }
        for j in 0..n {
            if j == j_coef { continue; }
            // CORREGIDO: orden correcto (row=i, col=j)
            result.push(find_in_matrix(matrix, un, i, j));
        }
    }
    result
}

/// Suma elemento a elemento dos matrices del mismo tamaño.
#[wasm_bindgen]
pub fn matrix_addition(matrix_a: &[i32], matrix_b: &[i32]) -> Vec<i32> {
    if matrix_a.len() != matrix_b.len() {
        error("The dimensions of the matrices must be the same.");
        return vec![-1];
    }
    matrix_a.iter().zip(matrix_b.iter()).map(|(a, b)| a + b).collect()
}

/// Multiplica dos matrices cuadradas 2x2 o 3x3.
#[wasm_bindgen]
pub fn matrix_multiplication_matrix(matrix_a: &[i32], matrix_b: &[i32]) -> Vec<i32> {
    if matrix_a.len() != matrix_b.len() {
        error("The dimensions of the matrices must be the same.");
        return vec![-1];
    }
    if !matrix_validation(matrix_a.len()) {
        return vec![-1];
    }
    let n = (matrix_a.len() as f64).sqrt() as u32;
    let mut result = Vec::with_capacity(matrix_a.len());

    for row in 0..n {
        for col in 0..n {
            let sum: i32 = (0..n)
                .map(|k| {
                    find_in_matrix(matrix_a, n, row as i32, k as i32)
                        * find_in_matrix(matrix_b, n, k as i32, col as i32)
                })
                .sum();
            result.push(sum);
        }
    }
    result
}

/// Multiplica todos los elementos de una matriz por un escalar entero.
#[wasm_bindgen]
pub fn matrix_multiplication_escalar(matrix: &[i32], escalar: i32) -> Vec<i32> {
    matrix.iter().map(|&x| x * escalar).collect()
}

/// Multiplica todos los elementos de una matriz por un escalar f64.
#[wasm_bindgen]
pub fn matrix_multiplication_escalar_f(matrix: &[i32], escalar: f64) -> Vec<f64> {
    matrix.iter().map(|&x| x as f64 * escalar).collect()
}

/*  
    --------------------------------------------------------------------------------
    Main Code : Algebra lineal modular
    --------------------------------------------------------------------------------
*/

/// Aplica módulo m a cada elemento de la matriz (resultado siempre positivo).
#[wasm_bindgen]
pub fn matrix_module(matrix: &[i32], m: u32) -> Vec<i32> {
    matrix.iter().map(|&x| module(x, m) as i32).collect()
}

/// Calcula la matriz inversa modular: A^{-1} mod m.
///
/// BUGS CORREGIDOS:
///   1. Se eliminó la doble transposición — matrix_adjugate ya devuelve la adjunta.
///   2. La identidad de verificación ahora se genera según el tamaño real de la matriz.
#[wasm_bindgen]
pub fn matrix_inverse_module(matrix: &[i32], m: u32) -> Vec<i32> {
    let raw_det = matrix_determinant(matrix);
    if raw_det == 0 {
        error("The inverse matrix does not exist. The determinant is equal to 0.");
        return Vec::new();
    }

    let det_mod = module(raw_det, m) as i32;
    let inv_det = euclid_extended(m as i32, det_mod);

    if inv_det == 0 {
        error("The determinant is not invertible module m. The inverse matrix does not exist.");
        return Vec::new();
    }

    if module(inv_det * det_mod, m) != 1 {
        error("The inverse is uncalculable, the key values are not coprime. Please verify the inputs.");
        return Vec::new();
    }


    let adjugate = matrix_adjugate(matrix);
    let result = matrix_multiplication_escalar_module(&adjugate, inv_det, m);

    // Verificación dinámica (funciona para 2x2 y 3x3)
    let product = matrix_multiplication_matrix_module(&result, matrix, m);
    let identity = identity_matrix(matrix.len());

    println!("{}", "Debug: Product of matrix and its inverse mod m:".yellow().bold());
    print_matrix(&product);
    println!("{}", "Debug: Expected identity matrix:".yellow().bold());
    print_matrix(&identity);

    if product != identity {
        error("The calculated inverse is not correct. Please verify the inputs.");
        return Vec::new();
    }

    result
}

/// Suma dos matrices y aplica módulo m al resultado.
#[wasm_bindgen]
pub fn matrix_addition_module(matrix_a: &[i32], matrix_b: &[i32], m: u32) -> Vec<i32> {
    let sum = matrix_addition(matrix_a, matrix_b);
    matrix_module(&sum, m)
}

/// Multiplica dos matrices y aplica módulo m al resultado.
#[wasm_bindgen]
pub fn matrix_multiplication_matrix_module(matrix_a: &[i32], matrix_b: &[i32], m: u32) -> Vec<i32> {
    let product = matrix_multiplication_matrix(matrix_a, matrix_b);
    matrix_module(&product, m)
}

/// Multiplica una matriz por un escalar y aplica módulo m al resultado.
#[wasm_bindgen]
pub fn matrix_multiplication_escalar_module(matrix: &[i32], escalar: i32, m: u32) -> Vec<i32> {
    let scaled = matrix_multiplication_escalar(matrix, escalar);
    matrix_module(&scaled, m)
}

/*  
    --------------------------------------------------------------------------------
    Tool functions
    --------------------------------------------------------------------------------
*/

/// Devuelve el elemento [row][column] de la matriz, con coordenadas módulo `size`.
fn find_in_matrix(matrix: &[i32], size: u32, row: i32, column: i32) -> i32 {
    let r = module(row, size) as u32;
    let c = module(column, size) as u32;
    matrix[(size * r + c) as usize]
}

/// Módulo siempre positivo.
fn module(a: i32, n: u32) -> u32 {
    let n = n as i32;
    (((a % n) + n) % n) as u32
}

/// Genera la matriz identidad para un slice de `total_elements` elementos.
fn identity_matrix(total_elements: usize) -> Vec<i32> {
    let n = (total_elements as f64).sqrt() as usize;
    let mut id = vec![0i32; total_elements];
    for i in 0..n {
        id[i * n + i] = 1;
    }
    id
}

/// Valida que el slice represente una matriz cuadrada de tamaño 1 a 3.
fn matrix_validation(size: usize) -> bool {
    let sqrt = (size as f64).sqrt();
    if sqrt != sqrt.trunc() {
        error("This function only accepts square matrices");
        return false;
    }
    if (sqrt as u32) > 3 {
        error("The function only accepts square matrices of size 1 to 3");
        return false;
    }
    true
}

/// Inverso multiplicativo de `alpha` en Z_{n_length} vía Euclides extendido.
fn euclid_extended(n_length: i32, alpha: i32) -> i32 {
    let (a, b) = if n_length > alpha {
        (n_length, alpha)
    } else {
        (alpha, n_length)
    };

    let (_d, x, y) = xgcd_rec(a, b);
    let out = if a == alpha { x } else { y };

    if out > 0 { out } else { module(out, n_length as u32) as i32 }
}

fn xgcd_rec(a: i32, b: i32) -> (i32, i32, i32) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (d, x1, y1) = xgcd_rec(b, a % b);
    let q = a / b;
    let (x, y) = (y1, x1 - y1 * q);
    debug_assert!(a * x + b * y == d, "Bezout identity failed");
    (d, x, y)
}

/// Imprime una matriz cuadrada formateada en terminal.
fn print_matrix(matrix: &[i32]) {
    let size_f = (matrix.len() as f64).sqrt();
    if size_f != size_f.trunc() {
        error("print_matrix only accepts square matrices");
        return;
    }
    let n = size_f as i32;
    for i in 0..n {
        print!("{}", "|".yellow().bold());
        for j in 0..n {
            print!("\t{}\t", find_in_matrix(matrix, n as u32, i, j));
        }
        println!("{}", "|".yellow().bold());
    }
}

fn print_matrix_f(matrix: &[f64]) {
    let size_f = (matrix.len() as f64).sqrt();
    if size_f != size_f.trunc() {
        error("print_matrix_f only accepts square matrices");
        return;
    }
    let n = size_f as i32;
    for i in 0..n {
        print!("{}", "|".yellow().bold());
        for j in 0..n {
            let r = module(i, n as u32) as u32;
            let c = module(j, n as u32) as u32;
            print!("\t{:.4}\t", matrix[(n as u32 * r + c) as usize]);
        }
        println!("{}", "|".yellow().bold());
    }
}

fn error(message: &str) {
    println!("{} {}", "Error:".red().bold(), message.red());
}

fn debug(label: &str, message: &str) {
    print!("{}", "Debug [".yellow().bold());
    print!("{}", label.yellow().italic());
    print!("{} ", "]:".yellow().bold());
    println!("{}", message.yellow());
}