#include <stdio.h>
#include <stdlib.h>
#include <string.h>


// Constantes de error
#define OK          0   // Termino correctamente la ejecución
#define ARG_ERR     2   // Argumentos de entrada incorrectos
#define FILE_ERR    3   // Archivos sin contenido o inexistentes
#define LENG_ERR    4   // Longitud del archivo BMP insuficiente para ocultar el mensaje

int main ( int argc, char *argv[] ) {
    
    FILE *file_txt = NULL;
    FILE *file_bmp = NULL;

    char *path_bmp;

    // ------------------------ Manejo de argumentos ------------------------------
    if ( argc == 3 ) {
        for ( int i = 1 ; i < argc ; i++ ){
            if ( strstr( argv[i],".txt") != NULL ){
                file_txt = fopen( argv[i], "rb" );
            }
            else if ( strstr( argv[i], ".bmp" ) != NULL ){
                file_bmp = fopen( argv[i], "rb" );
                path_bmp = malloc(( strlen(argv[i]) + 1 ) * sizeof(char) );
                strcpy( path_bmp , argv[i] );
            }
            else {
                perror("Error(Arg) : Uno de los dos documentos no es del tipo soportado\n");
                return ARG_ERR;
            }
        }
    }
    else {
        perror("Error(Arg) : El programa requiere del uso de dos argumentos:\n");
        perror("\t1. Un archivo txt\n");
        perror("\t2. Una imagen bmp\n");
        return ARG_ERR;
    }

    // ------------------------- Manejo de errores en argumentos ---------------------------------
    if ( file_txt == NULL ){
        perror("Error(File) : Documento de texto inexistente\n");
        return FILE_ERR;
    }
    if( file_bmp == NULL ){
        perror("Error(File) : Documento bmp inexistente\n");
        return FILE_ERR;
    }

    // ------------------------- Creacion de documento con seccion oculta ------------------------
    char *path_new_file = malloc((strlen(path_bmp) + 8) * sizeof(char));
    strcpy(path_new_file, path_bmp); 
    char *last_dot = strrchr(path_new_file, '.');
    *last_dot = '\0';
    sprintf(path_new_file,"%s_secret.bmp", path_new_file);

    FILE *file_secret = fopen(path_new_file, "wb+");


    // -------------------------- Lectura y escritura de archivos --------------------------
    int bmp_char;
    int txt_char;

    // Translado de encabezado maximo de un bmp
    for (int  i = 0 ; i < 138 ; i++){
        bmp_char = fgetc(file_bmp);
        if ( bmp_char == EOF ){
            perror("Error(Header) : Longitud del BMP insuficiente\n");
            return LENG_ERR;
        }
        fputc(bmp_char, file_secret);
    }

    // Grabado de header del secreto
    txt_char = 0x03;
    for( int i = 0 ; i < 8 ; i++ ){

        bmp_char = fgetc(file_bmp);
        if ( bmp_char == EOF ) {
            perror("Error(Hiding) : Longitud del BMP insuficiente\n");
            return LENG_ERR;
        }

        if (( txt_char & 0x80 ) == 0x80)
            fputc(( bmp_char | 0x01 ) , file_secret );
        else
            fputc(( bmp_char & 0xFE ) , file_secret );

        txt_char = txt_char << 1;

    }
    
    // Grabado de contraseña
    while (( txt_char = fgetc(file_txt) ) != EOF ){
        for ( int i = 0 ; i < 8 ; i++ ){

            bmp_char = fgetc(file_bmp);
            if ( bmp_char == EOF ) {
                perror("Error(Hiding) : Longitud del BMP insuficiente\n");
                return LENG_ERR;
            }

            if (( txt_char & 0x80 ) == 0x80)
                fputc(( bmp_char | 0x01 ) , file_secret );
            else
                fputc(( bmp_char & 0xFE ) , file_secret );

            txt_char = txt_char << 1;
        }
    }

    // Grabado de footer del secreto
    txt_char = 0x03;
    for( int i = 0 ; i < 8 ; i++ ){

        bmp_char = fgetc(file_bmp);
        if ( bmp_char == EOF ) {
            perror("Error(Hiding) : Longitud del BMP insuficiente\n");
            return LENG_ERR;
        }

        if (( txt_char & 0x80 ) == 0x80)
            fputc(( bmp_char | 0x01 ) , file_secret );
        else
            fputc(( bmp_char & 0xFE ) , file_secret );

        txt_char = txt_char << 1;

    }

    // Translado del resto del archivo bmp
    while (( bmp_char = fgetc(file_bmp) ) != EOF ){
        fputc( bmp_char, file_secret );
    }
   
    // ----------------------- Cierre memoria dinamica ------------------------------------
    free(path_new_file);
    free(path_bmp);

    // ---------------------- cierre punteros a archivos ---------------------------------
    fclose(file_txt);
    fclose(file_bmp);
    fclose(file_secret);
    return OK;
}
