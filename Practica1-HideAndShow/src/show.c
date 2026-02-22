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
        if( strstr( argv[1], ".bmp" ) != NULL ){
            file_bmp = fopen( argv[1], "rb" );
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


    // ------------------------ Manejo de errores en los argumentos -------------------------

    if ( file_bmp == NULL ) {
        perror("Error(File) : Documento BMP inexistente\n");
        return FILE_ERR;
    }

    // ------------------------ Obtencion de longitud del archivo ---------------------------

    long unsigned length_bmp = 0;
    fseek(file_bmp, 0, SEEK_END);
    length_bmp =  ftell(file_bmp);
    rewind(file_bmp);

    // ------------------------ Lectura en busca de secretos ------------------------------
    
    char *string_secret = malloc( sizeof(char) * (length_bmp / 8) );

    int bmp_byte;
    char tmp_secret_byte = 0x00;
    int unsigned count = 0;
    int unsigned pos_secret = 0;
    int flag_message = 0;

    fseek(file_bmp, 138, SEEK_SET);

    while ( ( bmp_byte = fgetc(file_bmp) ) != EOF ) {
        tmp_secret_byte = (tmp_secret_byte << 1) | (bmp_byte & 0x01);
        count++;

        if ( count == 8 ){

            if(tmp_secret_byte == 0x03 && flag_message == 1){
                break;
            }
            else if ( tmp_secret_byte == 0x03 ){
                flag_message = 1;
            }
            else if (flag_message == 1){
                string_secret[pos_secret] = tmp_secret_byte;
                pos_secret++;
            }
            tmp_secret_byte = 0x00;
            count = 0;

        }
    }

    printf("%s\n", string_secret);
    // ------------------------- Trimedo de cadena completa -------------------------------
    // ------------------------- Cierre de puntes y archivos ------------------------------
    free(string_secret);
    fclose(file_bmp);
    return OK;
}