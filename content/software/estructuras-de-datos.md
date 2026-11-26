+++
title = "Estructuras de datos fundamentales"
summary = "Arreglos, listas enlazadas, pilas, colas, tablas hash y árboles: cuándo conviene cada una."
branch = "Algoritmos y estructuras de datos"
order = 2
level = "Intermedio"
tags = ["arreglos", "pila", "cola", "hash", "árbol"]
+++

## Elegir la estructura correcta

Una **estructura de datos** es una forma de organizar la información en memoria. La elección determina qué operaciones son rápidas y cuáles lentas; muchas veces, elegir bien la estructura *es* resolver el problema.

## Arreglos

Guardan elementos del mismo tipo en posiciones contiguas de memoria. Acceder al elemento $i$ es inmediato, $O(1)$, pero insertar o borrar en el medio obliga a desplazar a los demás, $O(n)$. Los arreglos dinámicos (`Vec` en Rust, `list` en Python, `ArrayList` en Java) crecen solos y agregan al final en $O(1)$ amortizado.

## Listas enlazadas

Cada nodo guarda un dato y una referencia al siguiente. Insertar o borrar junto a un nodo ya ubicado es $O(1)$, pero llegar a la posición $i$ requiere recorrer la lista, $O(n)$.

## Pilas y colas

![Una pila con push y pop por el tope y una cola con entrada y salida por extremos opuestos](/media/software/pila-cola.svg)

*En la pila se entra y se sale por el mismo extremo; en la cola, por extremos opuestos.*

- **Pila** (*stack*), LIFO: el último en entrar es el primero en salir. Operaciones `push` y `pop`. Usos: la pila de llamadas a funciones, deshacer/rehacer, verificar paréntesis balanceados.
- **Cola** (*queue*), FIFO: el primero en entrar es el primero en salir. Usos: tareas pendientes, impresión, recorrido en anchura de grafos.

> [!NOTE]
> Verificar que los paréntesis de una expresión estén balanceados, con una pila:
>
> ```python
> def balanceado(texto):
>     pares = {")": "(", "]": "[", "}": "{"}
>     pila = []
>     for c in texto:
>         if c in "([{":
>             pila.append(c)
>         elif c in pares:
>             if not pila or pila.pop() != pares[c]:
>                 return False
>     return not pila
>
> balanceado("{[()]}")   # True
> balanceado("([)]")     # False
> ```

## Tablas hash

Guardan pares **clave → valor**. Una **función hash** convierte la clave en un índice de un arreglo, de modo que buscar, insertar y borrar cuestan $O(1)$ en promedio. Son los diccionarios de Python, los `HashMap` de Rust y Java, los objetos de JavaScript. No mantienen los elementos ordenados.

## Árboles

Un **árbol** organiza los datos de forma jerárquica: cada nodo tiene hijos, y hay un único nodo raíz. En un **árbol binario de búsqueda**, todo lo que está a la izquierda de un nodo es menor que él y todo lo que está a la derecha es mayor.

![Árbol binario de búsqueda con raíz 8](/media/software/arbol.svg)

*Árbol binario de búsqueda. Para buscar el 6 se baja 8 → 3 → 6: tres comparaciones.*

Si el árbol está **balanceado**, su altura es $\log_2 n$ y buscar, insertar y borrar cuestan $O(\log n)$, con la ventaja sobre la tabla hash de que los datos quedan ordenados.

## Grafos

Un **grafo** es un conjunto de nodos unidos por aristas. Modela redes: mapas de rutas, redes sociales, dependencias entre tareas. Se recorre en anchura (BFS, con una cola) o en profundidad (DFS, con una pila o recursión).

## Resumen

| Estructura | Acceso | Búsqueda | Inserción | Cuándo usarla |
|---|---|---|---|---|
| Arreglo dinámico | $O(1)$ | $O(n)$ | $O(1)$ al final | opción por defecto |
| Lista enlazada | $O(n)$ | $O(n)$ | $O(1)$ con el nodo | muchas inserciones en el medio |
| Tabla hash | — | $O(1)$ prom. | $O(1)$ prom. | buscar por clave |
| Árbol balanceado | — | $O(\log n)$ | $O(\log n)$ | datos ordenados, rangos |
