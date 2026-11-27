+++
title = "Teoría de grafos"
summary = "Vértices y aristas, grado, caminos y ciclos, árboles, grafos eulerianos y hamiltonianos, y coloreo."
branch = "Matemática discreta"
order = 4
level = "Intermedio"
tags = ["grafos", "árboles", "caminos", "Euler"]
+++

## Qué es un grafo

Un **grafo** $G = (V, E)$ es un conjunto de **vértices** (o nodos) unidos por **aristas**. Modela cualquier situación donde hay objetos y relaciones entre pares de ellos.

| Situación | Vértices | Aristas |
|---|---|---|
| Red vial | ciudades | rutas |
| Red social | personas | amistades |
| Internet | páginas | enlaces |
| Circuito eléctrico | nodos | componentes |
| Plan de estudios | materias | correlatividades |

### Tipos

| Tipo | Característica |
|---|---|
| No dirigido | las aristas no tienen sentido |
| Dirigido (digrafo) | las aristas son flechas |
| Ponderado | cada arista tiene un peso (distancia, costo) |
| Simple | sin lazos ni aristas repetidas |
| Completo $K_n$ | todos los vértices unidos entre sí |
| Bipartito | los vértices se dividen en dos grupos y solo hay aristas entre grupos |

## Grado

El **grado** de un vértice es la cantidad de aristas que llegan a él. En un digrafo se distingue grado de entrada y de salida.

> [!IMPORTANT]
> **Lema del apretón de manos**: $\displaystyle\sum_{v \in V}\text{gr}(v) = 2|E|$

Cada arista aporta 2 a la suma. Consecuencia: la cantidad de vértices de grado impar es siempre **par**.

> [!NOTE]
> ¿Puede haber una reunión de 7 personas donde cada una salude exactamente a 3? No: la suma de grados sería 21, que es impar.

El grafo completo $K_n$ tiene $\dbinom{n}{2} = \dfrac{n(n - 1)}{2}$ aristas.

## Caminos y conexidad

- **Camino**: sucesión de vértices unidos por aristas.
- **Ciclo**: camino que empieza y termina en el mismo vértice.
- **Grafo conexo**: hay un camino entre cualquier par de vértices.
- **Componentes conexas**: los «pedazos» de un grafo no conexo.
- **Distancia**: longitud del camino más corto entre dos vértices.

## Representación

Para un grafo de $n$ vértices:

| | Matriz de adyacencia | Lista de adyacencia |
|---|---|---|
| Qué guarda | $a_{ij} = 1$ si hay arista de $i$ a $j$ | para cada vértice, sus vecinos |
| Memoria | $n^2$ | $n + |E|$ |
| Conviene para | grafos densos | grafos con pocas aristas |

En un grafo no dirigido la matriz es simétrica. Además, el elemento $(i, j)$ de $A^k$ cuenta los caminos de longitud $k$ entre $i$ y $j$.

## Árboles

> [!IMPORTANT]
> Un **árbol** es un grafo conexo y sin ciclos.

Para un grafo de $n$ vértices son equivalentes:

- Es un árbol.
- Es conexo y tiene $n - 1$ aristas.
- No tiene ciclos y tiene $n - 1$ aristas.
- Entre cada par de vértices hay **un único** camino.

Un **árbol generador** de un grafo conexo es un subgrafo que es árbol y contiene todos los vértices. El de **peso mínimo** conecta todo con el menor costo (algoritmos de Kruskal y de Prim): es el problema de tender una red con el mínimo de cable.

## Grafos eulerianos

El problema de los **siete puentes de Königsberg** (Euler, 1736) dio origen a la teoría: ¿se puede recorrer la ciudad pasando por cada puente exactamente una vez?

> [!IMPORTANT]
> Un grafo conexo tiene un **ciclo euleriano** (recorre cada **arista** una vez y vuelve al inicio) si y solo si **todos** los vértices tienen grado par.
>
> Tiene un **camino euleriano** (sin volver al inicio) si y solo si tiene exactamente **dos** vértices de grado impar.

En Königsberg los cuatro vértices tenían grado impar: es imposible.

Es el problema de dibujar una figura sin levantar el lápiz, o el recorrido de un camión recolector.

## Grafos hamiltonianos

Un **ciclo hamiltoniano** pasa por cada **vértice** exactamente una vez.

A diferencia del caso euleriano, **no se conoce** una condición simple ni un algoritmo eficiente para decidirlo. El **problema del viajante** (hallar el ciclo hamiltoniano más corto) es uno de los problemas difíciles más famosos de la computación.

| | Euleriano | Hamiltoniano |
|---|---|---|
| Recorre una vez cada | arista | vértice |
| Condición | grados pares | no hay una simple |
| Dificultad | fácil | muy difícil (NP-completo) |

## Grafos planos

Un grafo es **plano** si se puede dibujar sin que se crucen aristas.

> [!IMPORTANT]
> **Fórmula de Euler**: en un grafo plano conexo, $V - E + F = 2$, donde $F$ es la cantidad de caras (incluida la exterior).

Vale también para los poliedros: un cubo tiene $8 - 12 + 6 = 2$.

$K_5$ y $K_{3,3}$ no son planos. Importa en el diseño de circuitos impresos.

## Coloreo

Colorear un grafo es asignar colores a los vértices de modo que dos vecinos no compartan color. El mínimo necesario es el **número cromático** $\chi(G)$.

- Un grafo es bipartito si y solo si $\chi = 2$, y eso ocurre cuando no tiene ciclos de longitud impar.
- **Teorema de los cuatro colores**: todo grafo plano se colorea con 4 colores; por eso alcanzan para cualquier mapa.

Aplicación: armar horarios de exámenes. Las materias son vértices, se unen las que comparten alumnos, y cada color es un turno.

## Problemas clásicos

| Problema | Algoritmo |
|---|---|
| Camino más corto | Dijkstra, Bellman-Ford |
| Recorrer todo el grafo | búsqueda en anchura y en profundidad |
| Árbol generador mínimo | Kruskal, Prim |
| Ordenar tareas con dependencias | orden topológico |
| Flujo máximo en una red | Ford-Fulkerson |

> [!TIP]
> Muchos problemas se vuelven simples cuando se reconoce el grafo que hay detrás: ¿qué son los vértices y qué las aristas?
