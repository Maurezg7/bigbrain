+++
title = "Computabilidad y complejidad"
summary = "Máquina de Turing, tesis de Church-Turing, problema de la parada, reducciones, clases P y NP, problemas NP-completos y estrategias para problemas intratables."
branch = "Teoría de la computación"
order = 2
level = "Avanzado"
tags = ["máquina de Turing", "problema de la parada", "P vs NP", "NP-completo", "decidibilidad"]
+++

## Dos preguntas

| Pregunta | Rama |
|---|---|
| ¿Qué problemas **pueden** resolverse con un algoritmo? | computabilidad |
| De los que pueden, ¿cuáles se resuelven con recursos **razonables**? | complejidad |

## Máquina de Turing

Es el modelo matemático de un algoritmo, propuesto por Alan Turing en 1936.

| Componente | Descripción |
|---|---|
| Cinta | infinita, dividida en celdas con un símbolo cada una |
| Cabezal | lee y escribe una celda, y se mueve una posición a izquierda o derecha |
| Estados | un conjunto finito, con uno inicial y estados de aceptación y de rechazo |
| Función de transición | según el estado y el símbolo leído: qué escribir, hacia dónde moverse y a qué estado pasar |

$$\delta(q, a) = (q', b, D) \qquad D \in \{\text{izquierda}, \text{derecha}\}$$

Ante una entrada, la máquina puede **aceptar**, **rechazar** o **no detenerse nunca**.

> [!NOTE]
> **Ejemplo.** Para reconocer $a^n b^n$: marcar la primera $a$, avanzar hasta la primera $b$ sin marcar y marcarla, volver al inicio y repetir. Si al terminar no quedan símbolos sin marcar, aceptar.

### Variantes

Varias cintas, cinta infinita en ambos sentidos, no determinismo: todas reconocen **los mismos** lenguajes que la máquina básica. Cambia la velocidad, no el poder.

Una **máquina universal** recibe la descripción de otra máquina y su entrada, y la simula. Es la idea del computador de programa almacenado.

## Tesis de Church-Turing

> [!IMPORTANT]
> Todo lo que puede calcularse con un procedimiento efectivo puede calcularse con una máquina de Turing.

No es un teorema (la noción de «procedimiento efectivo» es informal), pero todos los modelos propuestos resultaron equivalentes: el cálculo lambda, las funciones recursivas y cualquier lenguaje de programación habitual. Un lenguaje con ese poder se llama **Turing-completo**.

## Decidibilidad

| Un lenguaje es | si existe una máquina que |
|---|---|
| **Decidible** | se detiene siempre, aceptando o rechazando |
| **Reconocible** (recursivamente enumerable) | acepta las cadenas del lenguaje; con las demás puede no detenerse |

Hay problemas indecidibles: existen **más lenguajes que máquinas**. Las máquinas se pueden numerar (son descripciones finitas), pero los lenguajes forman un conjunto no numerable.

## El problema de la parada

> Dado un programa $P$ y una entrada $x$, ¿se detiene $P$ al ejecutarse con $x$?

**Es indecidible.**

> [!NOTE]
> **Demostración por contradicción.** Supongamos que existe `para(P, x)`, que responde siempre correctamente. Construimos:
>
> ```python
> def paradoja(P):
>     if para(P, P):          # ¿P se detiene con su propio código?
>         while True: pass    # entonces, no detenerse
>     else:
>         return              # si no, detenerse
> ```
>
> ¿Qué hace `paradoja(paradoja)`?
>
> - Si se detiene, `para` dijo que sí, y entonces entra en el bucle: no se detiene.
> - Si no se detiene, `para` dijo que no, y entonces retorna: se detiene.
>
> Ambas opciones son contradictorias. Luego `para` no puede existir.

El problema de la parada es reconocible (basta ejecutar el programa y esperar), pero no decidible.

### Consecuencias

**Teorema de Rice**: toda propiedad no trivial sobre el **comportamiento** de un programa es indecidible. No existe un algoritmo general que determine si un programa

- se detiene siempre,
- es equivalente a otro,
- contiene código que nunca se ejecuta,
- está libre de errores.

Por eso los compiladores y analizadores estáticos dan respuestas **aproximadas**: rechazan algunos programas correctos o dejan pasar algunos errores.

### Reducciones

Un problema $A$ se **reduce** a $B$ si una solución de $B$ permite resolver $A$. Entonces:

- si $B$ es decidible, $A$ también;
- si $A$ es indecidible, $B$ también.

Así se prueba que un problema nuevo es indecidible: reduciendo a él el problema de la parada.

## Complejidad

Entre los problemas decidibles, importa cuántos recursos consumen en función del tamaño $n$ de la entrada.

| Crecimiento | $n = 20$ | $n = 50$ | $n = 100$ |
|---|---|---|---|
| $n^2$ | 400 | 2500 | 10 000 |
| $n^3$ | 8000 | 125 000 | $10^6$ |
| $2^n$ | $10^6$ | $10^{15}$ | $10^{30}$ |
| $n!$ | $2{,}4 \times 10^{18}$ | $3 \times 10^{64}$ | $9 \times 10^{157}$ |

Un algoritmo **polinómico** se considera tratable; uno **exponencial**, intratable para entradas grandes.

## P y NP

| Clase | Problemas de decisión que |
|---|---|
| **P** | se **resuelven** en tiempo polinómico |
| **NP** | tienen soluciones que se **verifican** en tiempo polinómico |

> [!NOTE]
> **Sudoku generalizado.** Encontrar la solución de una grilla de $n^2 \times n^2$ puede ser muy difícil; comprobar que una solución propuesta es correcta es inmediato. Está en NP.

Todo problema de P está en NP: $P \subseteq NP$.

> [!IMPORTANT]
> **¿P = NP?** ¿Todo problema cuya solución se verifica rápido también se resuelve rápido? Es el problema abierto más importante de la informática. Se cree que **no**, pero nadie lo ha demostrado.

NP no significa «no polinómico», sino polinómico **no determinista**: lo resolvería en tiempo polinómico una máquina capaz de adivinar la solución correcta.

## NP-completos

Un problema es **NP-difícil** si todo problema de NP se reduce a él en tiempo polinómico. Es **NP-completo** si además pertenece a NP.

Son los más difíciles de NP: si **uno solo** tuviera un algoritmo polinómico, todos lo tendrían y sería P = NP.

**Teorema de Cook-Levin** (1971): la satisfacibilidad booleana (SAT) es NP-completa.

| Problema | Pregunta |
|---|---|
| SAT | ¿hay una asignación de valores que haga verdadera esta fórmula booleana? |
| Viajante (versión de decisión) | ¿hay un recorrido por todas las ciudades de longitud menor que $k$? |
| Coloreo de grafos | ¿se puede colorear con $k$ colores sin vecinos del mismo color? |
| Mochila | ¿hay un subconjunto de objetos con peso a lo sumo $W$ y valor al menos $V$? |
| Suma de subconjuntos | ¿algún subconjunto suma exactamente $t$? |
| Clique | ¿hay $k$ vértices todos conectados entre sí? |
| Cubrimiento de vértices | ¿hay $k$ vértices que tocan todas las aristas? |
| Ciclo hamiltoniano | ¿hay un ciclo que pasa una vez por cada vértice? |

Para probar que un problema nuevo es NP-completo: mostrar que está en NP y **reducir** a él un problema NP-completo conocido.

### Problemas parecidos, dificultad distinta

| Fácil (en P) | Difícil (NP-completo) |
|---|---|
| camino más corto | camino más largo |
| ciclo euleriano (pasa por cada arista) | ciclo hamiltoniano (pasa por cada vértice) |
| 2-SAT | 3-SAT |
| colorear con 2 colores | colorear con 3 colores |
| programación lineal | programación lineal entera |

## Otras clases

| Clase | Definición |
|---|---|
| co-NP | el complemento está en NP (se verifica rápido la respuesta «no») |
| PSPACE | se resuelven con memoria polinómica |
| EXPTIME | se resuelven en tiempo exponencial |
| BPP | tiempo polinómico con algoritmos aleatorizados, con error acotado |
| BQP | tiempo polinómico en una computadora cuántica |

$$P \subseteq NP \subseteq PSPACE \subseteq EXPTIME$$

Se sabe que $P \neq EXPTIME$, de modo que alguna de esas inclusiones es estricta; no se sabe cuál.

La **factorización de enteros** está en NP, pero no se sabe si está en P ni si es NP-completa. El algoritmo cuántico de Shor la resuelve en tiempo polinómico.

## Qué hacer ante un problema NP-difícil

| Estrategia | Idea |
|---|---|
| Algoritmos de **aproximación** | garantizan una solución cercana al óptimo (por ejemplo, a lo sumo el doble) |
| **Heurísticas** | búsqueda local, recocido simulado, algoritmos genéticos: buenas soluciones sin garantía |
| Casos **particulares** | el problema puede ser fácil en árboles, grafos planos o con parámetros pequeños |
| Búsqueda exacta con poda | ramificación y acotación; funciona para tamaños moderados |
| Resolvedores | los de SAT y de programación entera resuelven casos reales con millones de variables |
| Programación dinámica | algoritmos pseudopolinómicos (mochila en $O(nW)$) |

> [!TIP]
> Reconocer que un problema es NP-difícil es útil: indica que hay que dejar de buscar el algoritmo exacto y eficiente, y elegir entre aproximar, restringir el problema o aceptar tiempos exponenciales en casos pequeños.

> [!WARNING]
> NP-completo se refiere al **peor caso**. Muchos casos prácticos se resuelven bien; la dificultad no implica que toda entrada sea intratable.
