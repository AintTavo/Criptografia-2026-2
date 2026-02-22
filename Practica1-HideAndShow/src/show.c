#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// constantes de eror
#define OK          0   // Termino correctamente la ejecución
#define ARG_ERR     1   // Argumentos de entrada incorrectos
#define FILE_ERR    3   // Archivos sin contenido o inexistentes
#define LENG_ERR    4   // Longitud del archivo BMP insuficiente para ocultar el mensaje


int main ( int argc, char *argv[] ) {
    FILE *file_bmp = NULL;

    // -------------------------- Manejo de argumentos ------------------------------
    if ( argc == 2 ) {
        if( strstr(argv[1], ".bmp") != NULL){
            file_bmp = fopen(argv[1], "rb");
        }
        else{
            perror("Error(Arg) : El programa debe recibir unicamente un archivo BMP\n");
            return ARG_ERR;
        }
    }
    else {
        perror("Error(Arg) : El programa requiere el uso de 1 argumento: Un archivo bmp\n");
        return ARG_ERR;
    }



    return OK;
}