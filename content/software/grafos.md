+++
title = "Algoritmos sobre grafos"
summary = "Representación de grafos, recorridos en anchura y en profundidad, caminos mínimos con Dijkstra, orden topológico y árbol de expansión mínima."
branch = "Algoritmos y estructuras de datos"
order = 6
level = "Avanzado"
tags = ["grafos", "BFS", "DFS", "Dijkstra", "orden topológico"]
+++

## Qué es un grafo

Un **grafo** es un conjunto de **vértices** (nodos) unidos por **aristas**. Modela redes: rutas, redes sociales, dependencias, circuitos, páginas web.

| Tipo | Característica |
|---|---|
| No dirigido | las aristas no tienen sentido |
| Dirigido | cada arista va de un vértice a otro |
| Ponderado | cada arista tiene un peso (distancia, costo) |
| Conexo | hay camino entre cualquier par de vértices |
| Acíclico | no tiene ciclos; un dirigido acíclico se abrevia **DAG** |

Se usa $V$ para la cantidad de vértices y $E$ para la de aristas.

## Representación

| | Matriz de adyacencia | Lista de adyacencia |
|---|---|---|
| Estructura | tabla $V \times V$ | para cada vértice, la lista de sus vecinos |
| Memoria | $O(V^2)$ | $O(V + E)$ |
| ¿Hay arista entre $u$ y $v$? | $O(1)$ | $O(\text{grado de } u)$ |
| Recorrer los vecinos | $O(V)$ | $O(\text{grado})$ |
| Conviene para | grafos densos | grafos dispersos (lo habitual) |

```python
grafo = {
    "A": ["B", "C"],
    "B": ["A", "D"],
    "C": ["A", "D"],
    "D": ["B", "C", "E"],
    "E": ["D"],
}
```

## Recorrido en anchura (BFS)

Visita los vértices por **capas**: primero los vecinos, luego los vecinos de los vecinos. Usa una **cola**.

```python
from collections import deque

def bfs(grafo, inicio):
    distancia = {inicio: 0}
    cola = deque([inicio])
    while cola:
        u = cola.popleft()
        for v in grafo[u]:
            if v not in distancia:
                distancia[v] = distancia[u] + 1
                cola.append(v)
    return distancia
```

> [!IMPORTANT]
> En un grafo **sin pesos**, BFS encuentra el camino con **menos aristas** desde el origen a cada vértice.

## Recorrido en profundidad (DFS)

Avanza por un camino hasta no poder seguir y entonces retrocede. Usa recursión (o una **pila**).

```python
def dfs(grafo, u, visitados=None):
    if visitados is None:
        visitados = set()
    visitados.add(u)
    for v in grafo[u]:
        if v not in visitados:
            dfs(grafo, v, visitados)
    return visitados
```

Ambos recorridos cuestan $O(V + E)$.

| | BFS | DFS |
|---|---|---|
| Estructura | cola | pila o recursión |
| Camino más corto sin pesos | sí | no |
| Memoria | puede ser mucha en grafos anchos | proporcional a la profundidad |
| Usos | distancias, niveles | ciclos, orden topológico, componentes, laberintos |

> [!WARNING]
> Siempre hay que marcar los vértices **visitados**. Sin eso, un ciclo produce un bucle infinito.

### Componentes conexas

Se lanza un recorrido desde cada vértice todavía no visitado; cada lanzamiento descubre una componente.

### Detección de ciclos

En un grafo dirigido, hay un ciclo si durante el DFS se llega a un vértice que está **en el camino actual** (en proceso, todavía sin terminar).

## Orden topológico

En un **DAG**, es un orden de los vértices tal que toda arista $u \to v$ va de un vértice anterior a uno posterior. Sirve para ordenar tareas con dependencias: materias correlativas, pasos de compilación, instalación de paquetes.

**Algoritmo de Kahn**:

1. Calcular el **grado de entrada** de cada vértice.
2. Poner en una cola los que tienen grado 0.
3. Sacar uno, agregarlo al orden y restar 1 al grado de sus vecinos; los que llegan a 0 entran a la cola.

```python
from collections import deque

def orden_topologico(grafo):
    grado = {u: 0 for u in grafo}
    for u in grafo:
        for v in grafo[u]:
            grado[v] += 1
    cola = deque(u for u in grafo if grado[u] == 0)
    orden = []
    while cola:
        u = cola.popleft()
        orden.append(u)
        for v in grafo[u]:
            grado[v] -= 1
            if grado[v] == 0:
                cola.append(v)
    if len(orden) < len(grafo):
        raise ValueError("el grafo tiene un ciclo")
    return orden
```

Si quedan vértices sin procesar, hay un ciclo y no existe orden.

## Caminos mínimos

### Dijkstra

Halla el camino de menor peso desde un origen a todos los demás, con pesos **no negativos**. En cada paso toma el vértice no procesado más cercano y **relaja** sus aristas: si pasar por él mejora la distancia a un vecino, la actualiza.

```python
import heapq

def dijkstra(grafo, inicio):          # grafo[u] = [(v, peso), ...]
    dist = {u: float("inf") for u in grafo}
    dist[inicio] = 0
    cola = [(0, inicio)]
    while cola:
        d, u = heapq.heappop(cola)
        if d > dist[u]:
            continue                   # entrada vieja
        for v, peso in grafo[u]:
            if d + peso < dist[v]:
                dist[v] = d + peso
                heapq.heappush(cola, (dist[v], v))
    return dist
```

Con cola de prioridad cuesta $O((V + E)\log V)$.

> [!WARNING]
> Dijkstra **no funciona** con pesos negativos: da por definitiva una distancia que luego podría mejorar.

| Algoritmo | Sirve para | Costo |
|---|---|---|
| BFS | grafos sin pesos | $O(V + E)$ |
| Dijkstra | pesos no negativos | $O((V + E)\log V)$ |
| Bellman-Ford | pesos negativos; detecta ciclos negativos | $O(VE)$ |
| Floyd-Warshall | todos los pares | $O(V^3)$ |
| A* | un destino, con una heurística que guía la búsqueda | depende de la heurística |

## Árbol de expansión mínima

En un grafo conexo y ponderado, es el subconjunto de aristas que conecta todos los vértices con el **menor peso total**, sin ciclos. Tiene $V - 1$ aristas. Sirve para diseñar redes de costo mínimo.

| Algoritmo | Estrategia | Costo |
|---|---|---|
| **Kruskal** | ordenar las aristas por peso y agregar cada una si no forma ciclo | $O(E \log E)$ |
| **Prim** | crecer desde un vértice, agregando siempre la arista más barata que sale del árbol | $O(E \log V)$ |

Kruskal detecta ciclos con la estructura **unión-búsqueda** (*union-find*), que mantiene conjuntos disjuntos con operaciones casi constantes.

Ambos son algoritmos **voraces** (*greedy*): eligen en cada paso la mejor opción local, y en este problema eso garantiza el óptimo global.

## Modelar con grafos

Muchos problemas se resuelven al reconocer el grafo escondido:

| Problema | Vértices | Aristas | Algoritmo |
|---|---|---|---|
| Laberinto | celdas | celdas vecinas libres | BFS |
| Ruta más rápida | esquinas | calles con tiempo | Dijkstra |
| Correlatividades | materias | «es requisito de» | orden topológico |
| Rompecabezas | estados | movimientos válidos | BFS |
| Tendido de fibra | ciudades | costo del enlace | árbol de expansión mínima |

Para una grilla no hace falta construir el grafo: los vecinos de la celda $(f, c)$ son $(f \pm 1, c)$ y $(f, c \pm 1)$.

> [!TIP]
> Elegir el recorrido por lo que se pide: «el mínimo de pasos» apunta a BFS; «¿existe un camino?», «ciclos» o «dependencias» apuntan a DFS.
