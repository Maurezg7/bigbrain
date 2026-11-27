+++
title = "Funciones y modularidad"
summary = "Parámetros, valores de retorno, ámbito de las variables, paso por valor y por referencia, y cómo dividir un programa en módulos."
branch = "Fundamentos de programación"
order = 2
level = "Básico"
tags = ["funciones", "ámbito", "parámetros", "módulos", "modularidad"]
+++

## Por qué usar funciones

Una **función** es un bloque de código con nombre que realiza una tarea. Sirve para:

- **Reutilizar**: escribir una vez, usar muchas.
- **Abstraer**: usar algo sabiendo qué hace, sin recordar cómo.
- **Dividir** un problema grande en partes pequeñas que se pueden probar por separado.

```python
def area_rectangulo(base, altura):
    return base * altura

print(area_rectangulo(3, 4))   # 12
```

| Término | Significado |
|---|---|
| **Parámetro** | el nombre que aparece en la definición (`base`) |
| **Argumento** | el valor que se pasa al llamar (`3`) |
| **Valor de retorno** | lo que la función devuelve con `return` |
| **Firma** | el nombre, los parámetros y el tipo de retorno |

## Parámetros

```python
def saludar(nombre, saludo="Hola"):      # valor por defecto
    return f"{saludo}, {nombre}"

saludar("Ana")                       # 'Hola, Ana'
saludar("Ana", "Buen día")           # argumento posicional
saludar(saludo="Chau", nombre="Ana") # argumentos con nombre
```

Una función puede devolver varios valores agrupados:

```python
def dividir(a, b):
    return a // b, a % b

cociente, resto = dividir(17, 5)     # 3, 2
```

## Ámbito de las variables

El **ámbito** (*scope*) es la región del programa donde una variable existe.

- Una variable **local** se crea dentro de una función y desaparece cuando esta termina.
- Una variable **global** se define fuera y es visible en todo el archivo.

```python
total = 0                 # global

def sumar(x):
    resultado = x + 1     # local
    return resultado

sumar(5)
print(resultado)          # Error: resultado no existe aquí
```

> [!WARNING]
> Evitar que las funciones modifiquen variables globales: cualquier parte del programa puede cambiarlas y los errores se vuelven difíciles de rastrear. Es mejor pasar datos por parámetros y devolverlos con `return`.

## Paso de argumentos

| Mecanismo | Qué recibe la función | Modificar el parámetro afecta al original |
|---|---|---|
| Por **valor** | una copia | no |
| Por **referencia** | acceso al dato original | sí |

Depende del lenguaje:

- **C** pasa todo por valor; para modificar el original se pasa un puntero.
- **Python**, **Java** y **JavaScript** pasan una copia de la **referencia** al objeto: no se puede reemplazar la variable del llamador, pero sí **mutar** el objeto al que apunta.

```python
def agregar(lista):
    lista.append(4)        # muta el objeto original

def reasignar(lista):
    lista = [0]            # solo cambia la variable local

numeros = [1, 2, 3]
agregar(numeros)           # numeros es [1, 2, 3, 4]
reasignar(numeros)         # numeros sigue siendo [1, 2, 3, 4]
```

## Funciones puras y efectos secundarios

Una **función pura**:

1. Devuelve siempre el mismo resultado para los mismos argumentos.
2. No tiene **efectos secundarios**: no modifica nada fuera de sí misma.

```python
def al_cuadrado(x):        # pura
    return x * x

contador = 0
def incrementar():         # impura: modifica estado externo
    global contador
    contador += 1
```

Las funciones puras son más fáciles de probar, de entender y de reutilizar. Leer un archivo, imprimir o consultar la hora son efectos: conviene concentrarlos en pocas funciones.

## La pila de llamadas

Cada llamada crea un **marco** (*stack frame*) con sus parámetros y variables locales, que se apila sobre el de quien la llamó. Al retornar, el marco se descarta.

```text
main()
 └─ procesar()
     └─ validar()     ← en ejecución
```

Cuando ocurre un error, la **traza** (*stack trace*) muestra esta cadena de llamadas. Una recursión sin caso base llena la pila: desbordamiento de pila.

## Diseñar buenas funciones

| Principio | Significado |
|---|---|
| Una sola tarea | si para describirla hace falta «y», son dos funciones |
| Nombre claro | un verbo que diga qué hace: `calcular_promedio`, no `proc2` |
| Corta | que entre en una pantalla |
| Pocos parámetros | más de tres o cuatro sugiere agruparlos en una estructura |
| Sin sorpresas | que no haga nada que su nombre no anuncie |

> [!NOTE]
> Antes:
>
> ```python
> def procesar(notas):
>     total = 0
>     for n in notas:
>         total += n
>     promedio = total / len(notas)
>     if promedio >= 6:
>         print("Aprobado")
>     else:
>         print("Desaprobado")
> ```
>
> Después, separando cálculo, decisión y salida:
>
> ```python
> def promedio(notas):
>     return sum(notas) / len(notas)
>
> def aprobo(notas, minimo=6):
>     return promedio(notas) >= minimo
>
> print("Aprobado" if aprobo(notas) else "Desaprobado")
> ```
>
> Ahora `promedio` y `aprobo` se pueden probar y reutilizar.

## Documentación

```python
def imc(peso_kg: float, altura_m: float) -> float:
    """Devuelve el índice de masa corporal.

    Lanza ValueError si la altura no es positiva.
    """
    if altura_m <= 0:
        raise ValueError("la altura debe ser positiva")
    return peso_kg / altura_m ** 2
```

Las **anotaciones de tipo** y un comentario breve dicen qué espera la función y qué devuelve.

## Módulos

Un **módulo** agrupa funciones y datos relacionados en un archivo. Los módulos se agrupan en **paquetes** y **bibliotecas**.

```python
# geometria.py
import math

def area_circulo(r):
    return math.pi * r ** 2
```

```python
# principal.py
from geometria import area_circulo

print(area_circulo(2))
```

| Concepto | Qué busca |
|---|---|
| **Cohesión alta** | que todo lo que hay en un módulo trate de lo mismo |
| **Acoplamiento bajo** | que los módulos dependan poco unos de otros |
| **Interfaz** | lo que el módulo expone a los demás |
| **Encapsulamiento** | ocultar los detalles internos |

Un buen módulo se puede modificar por dentro sin tocar el resto del programa, mientras su interfaz no cambie.

> [!IMPORTANT]
> **No repetirse** (principio DRY): si la misma lógica aparece copiada en varios lugares, extraerla a una función. Un error corregido en una copia queda vivo en las otras.

> [!TIP]
> Si cuesta ponerle nombre a una función, probablemente hace demasiadas cosas.
