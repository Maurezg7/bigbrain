+++
title = "C"
summary = "Tipos, punteros, arreglos, cadenas, memoria dinámica, estructuras y el proceso de compilación en C."
branch = "Lenguajes"
order = 2
level = "Intermedio"
tags = ["C", "punteros", "malloc", "memoria", "compilación"]
+++

## Características

**C** es un lenguaje **compilado**, de tipado estático y de bajo nivel: da acceso directo a la memoria y casi no agrega costo sobre lo que hace la máquina. Con él están escritos los sistemas operativos, los intérpretes de otros lenguajes y el software de los microcontroladores.

| Fortalezas | Debilidades |
|---|---|
| muy rápido y predecible | la memoria se administra a mano |
| corre en cualquier procesador | no verifica límites de arreglos |
| control total del hardware | los errores de memoria son fallas de seguridad |
| lenguaje pequeño | biblioteca estándar mínima |

```c
#include <stdio.h>

int main(void) {
    printf("Hola, mundo\n");
    return 0;
}
```

## Compilación

```bash
gcc -Wall -Wextra -std=c17 -o programa programa.c
./programa
```

| Etapa | Qué hace | Salida |
|---|---|---|
| Preprocesador | resuelve `#include` y `#define` | código expandido |
| Compilador | traduce a código máquina | archivo objeto (`.o`) |
| Enlazador | une los objetos y las bibliotecas | ejecutable |

Los archivos de **cabecera** (`.h`) declaran funciones y tipos; los `.c` los definen.

## Tipos

| Tipo | Tamaño típico | Formato en `printf` |
|---|---|---|
| `char` | 1 byte | `%c` |
| `int` | 4 bytes | `%d` |
| `long long` | 8 bytes | `%lld` |
| `float` | 4 bytes | `%f` |
| `double` | 8 bytes | `%f` |
| puntero | 8 bytes (en 64 bits) | `%p` |

Los tamaños dependen de la plataforma. Para tamaños exactos se usa `<stdint.h>`: `int32_t`, `uint8_t`, `uint64_t`. `sizeof` devuelve el tamaño en bytes.

> [!WARNING]
> Una variable local sin inicializar contiene **basura**. Y `5 / 2` da `2`: la división entre enteros es entera.

## Punteros

Un **puntero** es una variable que guarda la **dirección de memoria** de otra.

| Operador | Significado |
|---|---|
| `&x` | dirección de `x` |
| `*p` | el valor al que apunta `p` (desreferenciar) |

```c
int x = 10;
int *p = &x;      // p apunta a x

*p = 20;          // modifica x a través de p
printf("%d\n", x);   // 20
```

`NULL` es el puntero que no apunta a nada. Desreferenciarlo hace caer el programa.

### Paso por referencia

C pasa los argumentos **por valor**. Para que una función modifique una variable del llamador, se le pasa su dirección:

```c
void intercambiar(int *a, int *b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}

int x = 1, y = 2;
intercambiar(&x, &y);     // x = 2, y = 1
```

## Arreglos

```c
int notas[5] = {7, 8, 6, 9, 10};

for (int i = 0; i < 5; i++) {
    printf("%d\n", notas[i]);
}
```

- Los índices van de `0` a `n − 1`.
- El tamaño es fijo.
- C **no verifica los límites**: `notas[5]` lee memoria ajena sin avisar.

El nombre de un arreglo se convierte en un puntero a su primer elemento: `notas[i]` equivale a `*(notas + i)`. Por eso, al pasar un arreglo a una función hay que pasar también su tamaño:

```c
int suma(const int *valores, size_t n) {
    int total = 0;
    for (size_t i = 0; i < n; i++) {
        total += valores[i];
    }
    return total;
}
```

## Cadenas

Una cadena es un arreglo de `char` terminado en el carácter nulo `'\0'`.

```c
char nombre[] = "Ana";     // 4 bytes: 'A', 'n', 'a', '\0'
```

| Función (`<string.h>`) | Qué hace |
|---|---|
| `strlen(s)` | longitud, sin contar el `'\0'` |
| `strcmp(a, b)` | compara; 0 si son iguales |
| `strcpy(dst, src)` | copia (sin verificar el espacio) |
| `snprintf(buf, n, ...)` | escribe con formato, limitado a `n` bytes |

> [!WARNING]
> Las cadenas se comparan con `strcmp`, no con `==` (que compara direcciones). Y copiar a un búfer más chico que el texto produce un **desbordamiento de búfer**: la vulnerabilidad clásica de C. Usar siempre las variantes con límite (`snprintf`, `fgets`).

## Memoria

| Región | Qué guarda | Duración |
|---|---|---|
| **Pila** (*stack*) | variables locales y parámetros | hasta que la función retorna |
| **Montón** (*heap*) | memoria pedida con `malloc` | hasta que se libera con `free` |
| Datos estáticos | variables globales y `static` | todo el programa |

### Memoria dinámica

```c
#include <stdlib.h>

int *valores = malloc(n * sizeof *valores);
if (valores == NULL) {
    return 1;                 // no había memoria
}

for (size_t i = 0; i < n; i++) {
    valores[i] = 0;
}

free(valores);
valores = NULL;
```

| Función | Qué hace |
|---|---|
| `malloc(bytes)` | reserva memoria sin inicializar |
| `calloc(n, tamaño)` | reserva e inicializa en cero |
| `realloc(p, bytes)` | cambia el tamaño de un bloque |
| `free(p)` | libera |

> [!IMPORTANT]
> Cada `malloc` debe tener exactamente un `free`.

### Errores de memoria

| Error | Descripción |
|---|---|
| Fuga de memoria | no liberar lo reservado |
| Uso después de liberar | usar un puntero tras `free` |
| Doble liberación | llamar a `free` dos veces |
| Desbordamiento de búfer | escribir fuera de un arreglo |
| Puntero colgante | devolver la dirección de una variable local |
| Desreferenciar `NULL` | usar un puntero nulo |

Casi todos son **comportamiento indefinido**: el programa puede fallar, dar resultados erróneos o parecer que funciona.

Herramientas para detectarlos:

```bash
gcc -g -fsanitize=address,undefined programa.c -o programa
valgrind ./programa
```

## Estructuras

```c
typedef struct {
    char nombre[50];
    int edad;
    double promedio;
} Estudiante;

Estudiante e = {"Ana", 20, 8.5};
printf("%s\n", e.nombre);

Estudiante *p = &e;
p->edad = 21;              // equivale a (*p).edad
```

Con estructuras y punteros se construyen listas enlazadas y árboles:

```c
typedef struct Nodo {
    int valor;
    struct Nodo *siguiente;
} Nodo;
```

Otros tipos: `enum` (constantes con nombre) y `union` (varios campos que comparten la misma memoria).

## Entrada, salida y archivos

```c
int edad;
if (scanf("%d", &edad) != 1) {     // devuelve cuántos valores leyó
    fprintf(stderr, "Entrada inválida\n");
}

FILE *f = fopen("datos.txt", "r");
if (f == NULL) {
    perror("fopen");
    return 1;
}
char linea[256];
while (fgets(linea, sizeof linea, f) != NULL) {
    printf("%s", linea);
}
fclose(f);
```

No hay excepciones: los errores se informan por el **valor de retorno**, y hay que revisarlo siempre.

## Preprocesador y modularidad

```c
#define PI 3.14159
#define MAX(a, b) ((a) > (b) ? (a) : (b))
```

Una cabecera se protege contra la inclusión doble:

```c
#ifndef GEOMETRIA_H
#define GEOMETRIA_H

double area_circulo(double radio);

#endif
```

`static` en una función o variable global la hace visible solo dentro de su archivo. `const` declara que un valor no se modifica.

Para proyectos con varios archivos se usa **make** o **CMake**.

## Operaciones con bits

| Operador | Significado | Ejemplo |
|---|---|---|
| `&` | Y | `x & 1` (¿es impar?) |
| <code>&#124;</code> | O | activar un bit |
| `^` | O exclusivo | invertir un bit |
| `~` | negación | |
| `<<`, `>>` | desplazamientos | `1 << 3` es 8 |

Son esenciales para programar microcontroladores y protocolos.

## C y sus descendientes

| Lenguaje | Relación con C |
|---|---|
| C++ | agrega clases, plantillas y gestión automática de recursos |
| Rust | el mismo rendimiento, con seguridad de memoria verificada al compilar |
| Go, Zig | alternativas modernas para sistemas |

> [!TIP]
> Compilar siempre con `-Wall -Wextra` y tratar cada advertencia como un error: en C, las advertencias suelen ser fallas reales.
