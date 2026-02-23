// stego.h
#ifndef STEGO_H
#define STEGO_H

#define OK 0
#define ARG_ERR 2
#define FILE_ERR 3
#define LENG_ERR 4

int hide(char* path_txt, char* path_bmp);
char* show(char* path_bmp);

#endif