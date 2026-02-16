#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char *argv[]) {
    
    FILE *file_txt = NULL;
    FILE *file_bmp = NULL;

    // Manejo de argumentos
    if (argc != 2) {
        for (int i = 1; i < argc; i++){
            if (strstr(argv[i],".txt") != NULL){
                printf("Archivo txt: [ %d, %s]\n", i, argv[i]);
            }
            else if (strstr(argv[i], ".bmp") != NULL){
                printf("Archivo bmp: [ %d, %s]\n", i, argv[i]);
            }
        }
    }
    else {
        perror("El program requiere del uso de dos argumentos:\n");
        perror("\t1. Un archivo txt\n");
        perror("\t2. Una imagen bmp\n");
        return 1;
    }

    fclose(file_txt);
    fclose(file_bmp);
    return 0;
}
