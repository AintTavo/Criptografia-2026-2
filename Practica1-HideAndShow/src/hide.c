#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define OK 0
#define ARG_ERR 2
#define FILE_ERR 3

int main(int argc, char *argv[]) {
    
    FILE *file_txt = NULL;
    FILE *file_bmp = NULL;

    char *path_txt;
    char *path_bmp;

    // ------------------------ Manejo de argumentos ------------------------------
    if (argc != 2) {
        for (int i = 1; i < argc; i++){
            if (strstr(argv[i],".txt") != NULL){
                file_txt = fopen(argv[i], "r");
                path_txt = malloc((strlen(argv[i]) + 1) * sizeof(char));
                strcpy(path_txt, argv[i]);
            }
            else if (strstr(argv[i], ".bmp") != NULL){
                file_bmp = fopen(argv[i], "r");
                path_bmp = malloc((strlen(argv[i]) + 1) * sizeof(char));
                strcpy(path_bmp, argv[i]);
            }
            else {
                perror("Error: Uno de los dos documentos no es del tipo soportado\n");
                return ARG_ERR;
            }
        }
    }
    else {
        perror("Error: El programa requiere del uso de dos argumentos:\n");
        perror("\t1. Un archivo txt\n");
        perror("\t2. Una imagen bmp\n");
        return ARG_ERR;
    }

    // ------------------------- Manejo de errores en argumentos ---------------------------------
    if (file_txt == NULL){
        perror("Error: Documento de texto inexistente\n");
        return FILE_ERR;
    }
    if(file_bmp == NULL){
        perror("Error: Documento bmp inexistente\n");
        return FILE_ERR;
    }

    // ------------------------- Creacion de documento con seccion oculta ------------------------
    char *path_new_file = malloc((strlen(path_bmp) + 8) * sizeof(char));
    strcpy(path_new_file, path_bmp); 
    char *last_dot = strrchr(path_new_file, '.');
    *last_dot = '\0';
    sprintf(path_new_file,"%s_secret.bmp", path_new_file);

    FILE *file_secret = fopen(path_new_file, "w+");
    
    
   
    // ----------------------- Cierre memoria dinamica ------------------------------------
    free(path_new_file);
    free(path_txt);
    free(path_bmp);

    // ---------------------- cierre punteros a archivos ---------------------------------
    fclose(file_txt);
    fclose(file_bmp);
    fclose(file_secret);
    return OK;
}
