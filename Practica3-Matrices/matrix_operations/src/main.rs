use colored::Colorize;


fn main() {
    let matrix_0 = [5 , 17, 20, 9, 23, 3, 2, 11, 13];
    print_matrix(&matrix_0);
    let determinant = matrix_determinant(&matrix_0);
    println!("{} {}","Determinante 1:".blue(), determinant);

    println!();
    let matrix_1 = [0, 1, 2, 3];
    print_matrix(&matrix_1);
    let determinant = matrix_determinant(&matrix_1);
    println!("{} {}", "Determinante 2:".blue(), determinant);

}

// Funcion para determinar la determinante de una matriz 
fn matrix_determinant( matrix : &[i32] ) -> i32 {
    let size = matrix.len() as f64;
    let size = size.sqrt();

    // corrección de errores :
    // En caso de que su raiz cuadrada tenga decimales (no es un numero cuadrado) regresa un error
    if size != size.trunc() {
        println!("{} {}","Error:".red().bold(), "This function only accept square matrices".red());
        return -1;
    }

    let size = size as u32;

    if size > 3 {
        println!("{} {}","Error:".red().bold(), "The function only acceps square matrices of 1 to 2".red());
        return -1;
    }

    if size == 2 {
        let a = matrix[0];
        let b = matrix[1];
        let c = matrix[2];
        let d = matrix[3];
        return (a * d) - (b * c);
    }

    // cambia el float
    
    let _size = size as i32;
    let mut determinant : i32 = 0;
    let mut tmp_det : i32;

    let x : i32 = 0;
    for i in 0.._size {
        tmp_det = 1;
        for j in 0.._size {
            tmp_det *= find_in_matrix( matrix, size, i + j, x + j);
            
        }
        determinant += tmp_det;
    }

    let x : i32 = _size - 1;
    for i in 0.._size {
        tmp_det = 1;
        for j in 0.._size {
            tmp_det *= find_in_matrix( matrix, size, i + j, x - j);
        }
        determinant -= tmp_det;
    }

    return determinant;
}

fn find_in_matrix( matrix: &[i32] , size : u32 , row : i32 , column : i32 ) -> i32 {
    let tmp_row : u32 = module(row, size as u32);
    let tmp_col : u32 = module(column, size as u32);
    return matrix[((size * tmp_row) + tmp_col) as usize];
} 

fn module( a : i32, n : u32) -> u32 {
    let _n = n as i32; 
    return (((a % _n) + _n) % _n ) as u32;
}

fn print_matrix(matrix: &[i32]) {
    let size = matrix.len() as f64;
    let size = size.sqrt();

    if size != size.trunc() {
        println!("{} {}","Error:".red().bold(), "This function only accept square matrices".red());
        return;
    }
    
    let size = size as i32;

    for i in 0..size {
        print!("{}","|".yellow().bold());
        for j in 0..size {
            print!("\t{}\t", find_in_matrix(matrix, size as u32, i, j));
        }
        println!("{}","|".yellow().bold());
    }
}