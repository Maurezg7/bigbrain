+++
title = "Variables, condicionales, bucles y funciones"
summary = "Los cuatro ladrillos con los que se construye cualquier programa, en cualquier lenguaje."
branch = "Fundamentos de programación"
order = 1
level = "Básico"
tags = ["variables", "tipos", "bucles", "funciones"]
+++

## Qué es un programa

Un programa es una secuencia de instrucciones que una computadora ejecuta para transformar datos de entrada en resultados. Un **algoritmo** es la idea (los pasos); el programa es esa idea escrita en un lenguaje concreto. Los ejemplos de esta página están en Python, pero los conceptos son los mismos en todos los lenguajes.

## Variables y tipos

Una **variable** es un nombre que se refiere a un valor guardado en memoria. Cada valor tiene un **tipo**, que determina qué operaciones admite.

```python
edad = 20              # entero (int)
promedio = 8.75        # decimal (float)
nombre = "Lucía"       # texto (str)
aprobado = True        # booleano (bool)
notas = [7, 9, 10]     # lista
```

| Tipo | Ejemplos | Operaciones típicas |
|---|---|---|
| Entero | `42`, `-7` | `+ - * // %` |
| Decimal | `3.14` | `+ - * /` |
| Texto | `"hola"` | concatenar, cortar, buscar |
| Booleano | `True`, `False` | `and`, `or`, `not` |

## Condicionales

Permiten que el programa tome decisiones según una condición:

```python
if nota >= 7:
    print("Promocionado")
elif nota >= 4:
    print("Aprobado, rinde final")
else:
    print("Desaprobado")
```

Las condiciones se arman con operadores de comparación (`==`, `!=`, `<`, `<=`, `>`, `>=`) y lógicos (`and`, `or`, `not`).

## Bucles

Repiten un bloque de código. El `for` recorre una colección o un rango conocido; el `while` repite mientras se cumpla una condición.

```python
suma = 0
for nota in notas:        # una vuelta por cada elemento
    suma += nota
print(suma / len(notas))  # 8.666...

n = 10
while n > 0:              # repite hasta que n llegue a 0
    n -= 3
```

![Diagrama de flujo de un bucle while](/media/software/bucle.svg)

*Un bucle `while`: se evalúa la condición, se ejecuta el cuerpo y se vuelve a evaluar.*

> [!WARNING]
> Si la condición de un `while` nunca se vuelve falsa, el programa queda en un **bucle infinito**. Asegurate de que el cuerpo modifique algo de lo que depende la condición.

## Funciones

Una **función** agrupa instrucciones bajo un nombre, recibe **parámetros** y devuelve un resultado. Sirven para no repetir código y para dividir un problema grande en partes pequeñas que se pueden probar por separado.

```python
def promedio(valores):
    """Devuelve el promedio de una lista de números."""
    return sum(valores) / len(valores)

print(promedio([7, 9, 10]))   # 8.666...
print(promedio([4, 6]))       # 5.0
```

Las variables creadas dentro de una función son **locales**: solo existen mientras la función se ejecuta.

> [!NOTE]
> **FizzBuzz**, un clásico: imprimir los números del 1 al 15, pero "Fizz" si es múltiplo de 3, "Buzz" si lo es de 5 y "FizzBuzz" si lo es de ambos.
>
> ```python
> for i in range(1, 16):
>     if i % 15 == 0:
>         print("FizzBuzz")
>     elif i % 3 == 0:
>         print("Fizz")
>     elif i % 5 == 0:
>         print("Buzz")
>     else:
>         print(i)
> ```
>
> El orden importa: si se preguntara primero por el 3, el 15 nunca llegaría a "FizzBuzz".

> [!TIP]
> Antes de escribir código, resolvé el problema a mano con un ejemplo chico y anotá los pasos. Si no podés explicarlo en palabras, todavía no lo podés programar.
