# Practica 1

La practica es aplicar conceptos de estenografia para implementar con manipulación de bits el en un bmp de 24 bits ocultar una contraseña. !!!!!Sin sobreescrobir los archivos!!!!.

No guardar el resultado de la and sino aplicarlo directamente a la imagen

Añadir todes para los datos de la contraseña 0x03 y 0xFC

En este caso se ocultara en el valor menos significativo de los colores del BMP (RGB, cada uno 1 byte). Debe tener 
No guardar el resultado 
## Propuesta de solución

Pasos del programa

1. tomar dos argumentos para los archivos txt y bmp
2. Ver que los dos argumentos si sean de la extensión correcta y en el orden correcto.
3. Abrir archivo txt
4. Pasar de ascii a un arreglo de bits(dar directamente a la imagen )
5. Leer bmp y stream de lectura
6. Durante la longitud del arreglo de bits al bmp, y escribir con una and e con cada bit del arreglo.
7. Escribir el resto del archivo bmp