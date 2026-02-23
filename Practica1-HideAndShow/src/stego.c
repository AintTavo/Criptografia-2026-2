#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "stego.h"

// ##########################################################################################################################################################
//                                                                      Hide
// ##########################################################################################################################################################

int hide(char* path_txt, char* path_bmp){
    FILE *file_txt = NULL;
    FILE *file_bmp = NULL;

    // ------------------------ Manejo de argumentos ------------------------------
    if ( strstr( path_txt, ".txt" ) != NULL ) {
        file_txt = fopen( path_txt,"rb" );
    }
    else{
        return ARG_ERR;
    }

    if ( strstr( path_bmp, ".bmp" ) != NULL ) {
        file_bmp = fopen( path_bmp, "rb" );
    }
    else{
        return ARG_ERR;
    }


    // ------------------------- Manejo de errores en argumentos ---------------------------------
    if ( file_txt == NULL ){
        return FILE_ERR;
    }
    if( file_bmp == NULL ){
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
            return LENG_ERR;
        }
        fputc(bmp_char, file_secret);
    }

    // Grabado de header del secreto
    txt_char = 0x03;
    for( int i = 0 ; i < 8 ; i++ ){

        bmp_char = fgetc(file_bmp);
        if ( bmp_char == EOF ) {
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

    // ---------------------- cierre punteros a archivos ---------------------------------
    fclose(file_txt);
    fclose(file_bmp);
    fclose(file_secret);
    return OK;
}


// ##########################################################################################################################################################
//                                                                      Show
// ##########################################################################################################################################################

char* show(char* path_bmp) {
    FILE *file_bmp = NULL;

    // -------------------------- Manejo de argumentos ------------------------------
    if ( strstr( path_bmp, ".bmp" ) != NULL ) {
        file_bmp = fopen( path_bmp, "rb" );
    }
    else{
        return "El archivo no era un BMP";
    }


    // ------------------------ Manejo de errores en los argumentos -------------------------

    if ( file_bmp == NULL ) {
        return "Archivo vacio o inexistente";
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
                pos_secret++;
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

    string_secret[pos_secret] = '\0';

    // ------------------------- Cierre de puntes y archivos ------------------------------
    fclose(file_bmp);

    return string_secret;
}