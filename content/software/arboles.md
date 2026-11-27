+++
title = "Árboles"
summary = "Árboles binarios, recorridos, árboles binarios de búsqueda, árboles balanceados, montículos y colas de prioridad."
branch = "Algoritmos y estructuras de datos"
order = 5
level = "Intermedio"
tags = ["árboles", "árbol binario de búsqueda", "recorridos", "montículo", "cola de prioridad"]
+++

## Terminología

Un **árbol** es una estructura jerárquica de **nodos** conectados, sin ciclos.

| Término | Significado |
|---|---|
| Raíz | el nodo superior, sin padre |
| Hijo, padre | nodos directamente conectados |
| Hoja | nodo sin hijos |
| Subárbol | un nodo con todos sus descendientes |
| Profundidad de un nodo | cantidad de aristas desde la raíz |
| Altura del árbol | profundidad de la hoja más lejana |

En un **árbol binario**, cada nodo tiene a lo sumo dos hijos: izquierdo y derecho.

```python
class Nodo:
    def __init__(self, valor):
        self.valor = valor
        self.izq = None
        self.der = None
```

Un árbol binario de altura $h$ tiene como máximo $2^{h+1} - 1$ nodos. Si está **balanceado**, su altura es del orden de $\log_2 n$.

## Recorridos

### En profundidad

| Recorrido | Orden | Uso |
|---|---|---|
| **Preorden** | raíz, izquierdo, derecho | copiar el árbol |
| **Inorden** | izquierdo, raíz, derecho | en un árbol de búsqueda, da los valores ordenados |
| **Postorden** | izquierdo, derecho, raíz | borrar el árbol, evaluar expresiones |

```python
def inorden(nodo):
    if nodo is None:
        return
    inorden(nodo.izq)
    print(nodo.valor)
    inorden(nodo.der)
```

### Por niveles

Visita los nodos nivel por nivel, usando una **cola**:

```python
from collections import deque

def por_niveles(raiz):
    cola = deque([raiz])
    while cola:
        nodo = cola.popleft()
        print(nodo.valor)
        if nodo.izq: cola.append(nodo.izq)
        if nodo.der: cola.append(nodo.der)
```

> [!NOTE]
> Para el árbol
>
> ```text
>         8
>        / \
>       3   10
>      / \    \
>     1   6    14
> ```
>
> - Preorden: 8, 3, 1, 6, 10, 14
> - Inorden: 1, 3, 6, 8, 10, 14
> - Postorden: 1, 6, 3, 14, 10, 8
> - Por niveles: 8, 3, 10, 1, 6, 14

## Árbol binario de búsqueda

> [!IMPORTANT]
> En un **árbol binario de búsqueda** (ABB), para cada nodo: todos los valores del subárbol izquierdo son **menores** y todos los del derecho son **mayores**.

Buscar es como la búsqueda binaria: en cada nodo se descarta un subárbol.

```python
def buscar(nodo, valor):
    if nodo is None or nodo.valor == valor:
        return nodo
    if valor < nodo.valor:
        return buscar(nodo.izq, valor)
    return buscar(nodo.der, valor)

def insertar(nodo, valor):
    if nodo is None:
        return Nodo(valor)
    if valor < nodo.valor:
        nodo.izq = insertar(nodo.izq, valor)
    elif valor > nodo.valor:
        nodo.der = insertar(nodo.der, valor)
    return nodo
```

**Borrar** tiene tres casos:

1. El nodo es una hoja: se quita.
2. Tiene un hijo: el hijo ocupa su lugar.
3. Tiene dos hijos: se reemplaza por su **sucesor** (el menor del subárbol derecho) y se borra ese.

| Operación | Árbol balanceado | Árbol degenerado |
|---|---|---|
| Buscar | $O(\log n)$ | $O(n)$ |
| Insertar | $O(\log n)$ | $O(n)$ |
| Borrar | $O(\log n)$ | $O(n)$ |

> [!WARNING]
> Si se insertan valores ya ordenados (1, 2, 3, 4...), el ABB degenera en una lista enlazada y todo cuesta $O(n)$.

## Árboles balanceados

Se reorganizan solos, mediante **rotaciones**, para mantener la altura en $O(\log n)$.

| Árbol | Característica | Dónde se usa |
|---|---|---|
| **AVL** | las alturas de los dos subárboles difieren a lo sumo en 1 | búsquedas muy frecuentes |
| **Rojo-negro** | balance menos estricto, menos rotaciones | `TreeMap` de Java, `std::map` de C++ |
| **Árbol B** | muchos hijos por nodo, poca altura | bases de datos, sistemas de archivos, `BTreeMap` de Rust |

Los **árboles B** minimizan los accesos a disco: cada nodo ocupa un bloque y contiene muchas claves.

## Montículos

Un **montículo** (*heap*) binario es un árbol **completo** (lleno nivel por nivel, de izquierda a derecha) que cumple:

- **Montículo de mínimos**: cada nodo es menor o igual que sus hijos. La raíz es el mínimo.
- **Montículo de máximos**: cada nodo es mayor o igual que sus hijos.

Por ser completo, se guarda en un **arreglo** sin punteros. Para el nodo en la posición $i$ (desde 0):

| Nodo | Índice |
|---|---|
| Hijo izquierdo | $2i + 1$ |
| Hijo derecho | $2i + 2$ |
| Padre | $\lfloor (i - 1)/2 \rfloor$ |

| Operación | Costo | Cómo |
|---|---|---|
| Ver el mínimo | $O(1)$ | es la raíz |
| Insertar | $O(\log n)$ | se agrega al final y **sube** intercambiándose con su padre |
| Extraer el mínimo | $O(\log n)$ | el último pasa a la raíz y **baja** |
| Construir desde un arreglo | $O(n)$ | |

### Cola de prioridad

Es el tipo abstracto que entrega siempre el elemento de mayor prioridad. El montículo es su implementación habitual.

```python
import heapq

tareas = []
heapq.heappush(tareas, (2, "escribir informe"))
heapq.heappush(tareas, (1, "apagar incendio"))
heapq.heappush(tareas, (3, "ordenar escritorio"))

heapq.heappop(tareas)      # (1, 'apagar incendio')
```

Usos: planificación de procesos, algoritmo de Dijkstra, simulación de eventos, hallar los $k$ mayores, y el ordenamiento **heapsort** ($O(n \log n)$, sin memoria extra).

## Otros árboles

| Árbol | Para qué sirve |
|---|---|
| **Trie** (árbol de prefijos) | autocompletado, diccionarios: buscar una palabra cuesta según su longitud |
| **Árbol de segmentos** | consultas y actualizaciones sobre rangos en $O(\log n)$ |
| **Árbol de sintaxis** (AST) | representa la estructura de un programa en compiladores |
| **Árbol de decisión** | clasificación en aprendizaje automático |
| **DOM** | la estructura de una página web |

Los sistemas de archivos, los organigramas y el formato JSON también son árboles.

## Recursión sobre árboles

Casi todo problema de árboles se resuelve con el mismo esquema: resolver para los subárboles y combinar.

```python
def altura(nodo):
    if nodo is None:
        return -1
    return 1 + max(altura(nodo.izq), altura(nodo.der))

def cantidad(nodo):
    if nodo is None:
        return 0
    return 1 + cantidad(nodo.izq) + cantidad(nodo.der)
```

> [!TIP]
> Para elegir: si hacen falta las claves **en orden** o búsquedas por rango, árbol balanceado; si solo importa el mínimo o el máximo, montículo; si solo importa buscar por clave exacta, tabla hash.
