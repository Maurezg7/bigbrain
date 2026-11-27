+++
title = "Python"
summary = "Sintaxis esencial, tipos y colecciones, funciones, clases, módulos, manejo de archivos y el ecosistema de Python."
branch = "Lenguajes"
order = 1
level = "Básico"
tags = ["Python", "listas", "diccionarios", "comprensiones", "pip"]
+++

## Características

**Python** es un lenguaje de alto nivel, **interpretado**, de tipado **dinámico** y fuerte, con administración automática de memoria. Prioriza la legibilidad: los bloques se marcan con **sangría**, no con llaves.

| Fortalezas | Debilidades |
|---|---|
| fácil de leer y aprender | más lento que los lenguajes compilados |
| enorme biblioteca estándar y ecosistema | los errores de tipo aparecen al ejecutar |
| dominante en ciencia de datos e IA | el GIL limita el paralelismo con hilos |
| ideal para automatizar y prototipar | distribución de aplicaciones más compleja |

```python
print("Hola, mundo")
```

## Tipos básicos

| Tipo | Ejemplo | Mutable |
|---|---|---|
| `int` | `42` (precisión arbitraria) | no |
| `float` | `3.14` | no |
| `bool` | `True`, `False` | no |
| `str` | `"hola"` | no |
| `NoneType` | `None` | no |
| `list` | `[1, 2, 3]` | sí |
| `tuple` | `(1, 2)` | no |
| `dict` | `{"a": 1}` | sí |
| `set` | `{1, 2, 3}` | sí |

```python
x = 7
7 / 2        # 3.5   división real
7 // 2       # 3     división entera
7 % 2        # 1     resto
2 ** 10      # 1024  potencia
```

### Cadenas

```python
nombre = "Ana"
edad = 20
f"{nombre} tiene {edad} años"     # f-string
f"{3.14159:.2f}"                  # '3.14'

s = "universidad"
s[0]          # 'u'
s[-1]         # 'd'       índices negativos: desde el final
s[0:4]        # 'univ'    rebanada: desde 0 hasta 4 sin incluirlo
s[::-1]       # 'dadisrevinu'
s.upper(), s.replace("u", "U"), s.split("i")
", ".join(["a", "b", "c"])        # 'a, b, c'
```

## Control de flujo

```python
if nota >= 7:
    estado = "promociona"
elif nota >= 4:
    estado = "regulariza"
else:
    estado = "libre"

for i in range(3):                # 0, 1, 2
    print(i)

for i, valor in enumerate(["a", "b"]):
    print(i, valor)

for nombre, nota in zip(nombres, notas):
    print(nombre, nota)

while n > 0:
    n -= 1
```

Valores que se consideran falsos: `False`, `None`, `0`, `""`, y las colecciones vacías.

## Colecciones

### Listas

```python
numeros = [3, 1, 2]
numeros.append(4)                 # agregar al final
numeros.sort()                    # ordena en el lugar
ordenada = sorted(numeros, reverse=True)   # devuelve una nueva
len(numeros), sum(numeros), max(numeros)
numeros[1:3]                      # rebanada
3 in numeros                      # True
```

### Diccionarios

```python
notas = {"Ana": 9, "Luis": 6}
notas["Eva"] = 8
notas.get("Juan", 0)              # 0 si no existe
for nombre, nota in notas.items():
    print(nombre, nota)
```

### Tuplas y conjuntos

```python
punto = (3, 4)
x, y = punto                      # desempaquetado
a, b = b, a                       # intercambio

a = {1, 2, 3}; b = {3, 4}
a | b      # unión        {1, 2, 3, 4}
a & b      # intersección {3}
a - b      # diferencia   {1, 2}
```

### Comprensiones

Construyen colecciones en una línea:

```python
cuadrados = [n * n for n in range(10)]
pares = [n for n in range(10) if n % 2 == 0]
longitudes = {p: len(p) for p in ["sol", "luna"]}
unicos = {c.lower() for c in "Banana"}
```

> [!WARNING]
> Asignar una lista **no la copia**: `b = a` hace que ambos nombres apunten a la misma lista. Para copiarla: `b = a.copy()` o `b = a[:]`.

## Funciones

```python
def potencia(base, exponente=2):
    """Eleva base al exponente."""
    return base ** exponente

potencia(3)                # 9
potencia(2, exponente=5)   # 32

def sumar(*numeros, **opciones):  # cantidad variable de argumentos
    return sum(numeros)

doble = lambda x: x * 2
```

> [!WARNING]
> No usar objetos mutables como valor por defecto: `def f(lista=[])` comparte la **misma** lista entre todas las llamadas. Usar `None` y crearla adentro.

### Anotaciones de tipo

Son opcionales y no se verifican al ejecutar, pero las revisan herramientas como `mypy` o `pyright`:

```python
def promedio(valores: list[float]) -> float:
    return sum(valores) / len(valores)
```

## Clases

```python
from dataclasses import dataclass

@dataclass
class Punto:
    x: float
    y: float

    def distancia_al_origen(self) -> float:
        return (self.x ** 2 + self.y ** 2) ** 0.5

p = Punto(3, 4)
p.distancia_al_origen()    # 5.0
print(p)                   # Punto(x=3, y=4)
```

`@dataclass` genera el constructor, la representación y la comparación.

Los **métodos especiales** (`__init__`, `__str__`, `__len__`, `__eq__`, `__add__`) integran las clases con la sintaxis del lenguaje.

## Excepciones y archivos

```python
try:
    with open("datos.txt", encoding="utf-8") as archivo:
        for linea in archivo:
            print(linea.strip())
except FileNotFoundError:
    print("No existe el archivo")
```

`with` cierra el archivo aunque ocurra un error.

```python
import json

with open("config.json") as f:
    config = json.load(f)
```

## Módulos

```python
import math
from collections import Counter, defaultdict
from pathlib import Path

math.sqrt(16)
Counter("banana").most_common(1)      # [('a', 3)]
Path("datos").glob("*.csv")
```

```python
if __name__ == "__main__":
    main()        # solo si el archivo se ejecuta directamente
```

Módulos útiles de la biblioteca estándar: `math`, `random`, `datetime`, `pathlib`, `json`, `csv`, `re`, `collections`, `itertools`, `functools`, `argparse`, `unittest`.

## Generadores

Producen valores de a uno, sin armar toda la lista en memoria:

```python
def pares_hasta(n):
    for i in range(0, n, 2):
        yield i

suma = sum(x * x for x in range(1_000_000))   # expresión generadora
```

## Entornos y paquetes

Cada proyecto debería tener su **entorno virtual**, con sus propias dependencias:

```bash
python -m venv .venv
source .venv/bin/activate
pip install requests
pip freeze > requirements.txt
```

Herramientas modernas como `uv` o `poetry` gestionan el entorno y las dependencias desde `pyproject.toml`.

## Ecosistema

| Área | Bibliotecas |
|---|---|
| Cálculo numérico | NumPy, SciPy |
| Datos | pandas, Polars |
| Gráficos | Matplotlib, Plotly |
| Aprendizaje automático | scikit-learn, PyTorch, TensorFlow |
| Web | Django, Flask, FastAPI |
| Automatización | requests, Beautiful Soup, Playwright |
| Pruebas | pytest |
| Calidad | ruff, mypy |

## Estilo

La guía **PEP 8** fija las convenciones:

| Elemento | Convención |
|---|---|
| Variables y funciones | `minusculas_con_guiones` |
| Clases | `PalabrasCapitalizadas` |
| Constantes | `MAYUSCULAS` |
| Sangría | 4 espacios |

El «Zen de Python» (`import this`) resume su filosofía: *lo explícito es mejor que lo implícito; lo simple es mejor que lo complejo; la legibilidad cuenta*.

> [!TIP]
> Antes de escribir un bucle para una tarea común, buscar en la biblioteca estándar: `sum`, `max`, `sorted`, `any`, `all`, `zip`, `enumerate` y `collections` ya resuelven la mayoría.
