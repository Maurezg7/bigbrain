+++
title = "Programación lineal"
summary = "Formulación de problemas de optimización lineal, método gráfico, forma estándar, método simplex, dualidad, análisis de sensibilidad y programación entera."
branch = "Álgebra lineal"
order = 7
level = "Avanzado"
tags = ["programación lineal", "simplex", "optimización", "dualidad", "investigación operativa"]
+++

## El problema

La **programación lineal** busca el máximo o el mínimo de una función lineal, sujeta a restricciones lineales. Es la herramienta central de la investigación operativa: planificación de producción, mezclas, transporte, asignación de recursos.

| Elemento | Significado |
|---|---|
| **Variables de decisión** | lo que se puede elegir: $x_1, \dots, x_n$ |
| **Función objetivo** | lo que se quiere maximizar o minimizar: $z = c_1 x_1 + \dots + c_n x_n$ |
| **Restricciones** | limitaciones: $a_{i1} x_1 + \dots + a_{in} x_n \le b_i$ |
| No negatividad | $x_j \ge 0$ |

En forma matricial:

$$\max\ z = \mathbf{c}^T \mathbf{x} \qquad \text{sujeto a}\quad A\mathbf{x} \le \mathbf{b},\quad \mathbf{x} \ge \mathbf{0}$$

## Formulación

> [!NOTE]
> **Problema.** Un taller fabrica mesas y sillas. Cada mesa da \$40 de ganancia y cada silla, \$30. Una mesa requiere 2 h de carpintería y 1 h de terminación; una silla, 1 h y 1 h. Hay 100 h de carpintería y 80 h de terminación por semana, y no se venden más de 40 mesas.
>
> Variables: $x$ = mesas, $y$ = sillas.
>
> $$\max\ z = 40x + 30y$$
>
> $$\begin{aligned} 2x + y &\le 100 \quad \text{(carpintería)} \\ x + y &\le 80 \quad \text{(terminación)} \\ x &\le 40 \quad \text{(demanda)} \\ x,\ y &\ge 0 \end{aligned}$$

Pasos para formular: definir las variables con sus unidades, escribir el objetivo, y traducir cada limitación en una desigualdad.

## Método gráfico

Con dos variables, cada restricción es un semiplano. Su intersección es la **región factible**: un polígono convexo.

> [!IMPORTANT]
> **Teorema fundamental.** Si un problema de programación lineal tiene solución óptima, esta se alcanza en un **vértice** de la región factible.

Basta entonces evaluar el objetivo en los vértices.

> [!NOTE]
> **Vértices del ejemplo.**
>
> | Vértice | Origen | $z = 40x + 30y$ |
> |---|---|---|
> | $(0, 0)$ | ejes | 0 |
> | $(40, 0)$ | demanda y eje $x$ | 1600 |
> | $(40, 20)$ | demanda y carpintería | 2200 |
> | $(20, 60)$ | carpintería y terminación | **2600** |
> | $(0, 80)$ | terminación y eje $y$ | 2400 |
>
> Óptimo: 20 mesas y 60 sillas, con ganancia de \$2600.

Geométricamente, se desplaza la recta $40x + 30y = k$ en la dirección de su gradiente hasta el último punto en que toca la región.

### Casos especiales

| Caso | Descripción |
|---|---|
| Solución única | un vértice |
| Óptimos múltiples | la recta objetivo es paralela a un lado: todo el lado es óptimo |
| No acotado | la región se extiende indefinidamente en la dirección de mejora |
| Infactible | las restricciones son incompatibles: región vacía |

## Forma estándar

El método simplex trabaja con igualdades. Cada desigualdad se convierte agregando una **variable de holgura** no negativa, que representa el recurso sin usar:

$$2x + y + s_1 = 100 \qquad x + y + s_2 = 80 \qquad x + s_3 = 40$$

| Conversión | Cómo |
|---|---|
| Restricción $\le$ | sumar una holgura |
| Restricción $\ge$ | restar una variable de exceso (y agregar una artificial) |
| Minimizar $z$ | maximizar $-z$ |
| Variable libre | reemplazar por $x = x^+ - x^-$ |

Con $m$ ecuaciones y $n$ variables, una **solución básica** se obtiene anulando $n - m$ variables (las **no básicas**) y despejando las otras $m$ (las **básicas**). Las soluciones básicas factibles corresponden a los vértices.

## Método simplex

Ideado por George Dantzig en 1947. Recorre vértices **adyacentes**, mejorando el objetivo en cada paso, hasta que no pueda mejorar.

### Algoritmo

1. Partir de una solución básica factible (las holguras como básicas).
2. **Variable entrante**: la no básica con el coeficiente más negativo en la fila de $z$. Si no hay ninguna negativa, la solución es **óptima**.
3. **Variable saliente**: la de la fila con el menor cociente entre el término independiente y el coeficiente (positivo) de la columna entrante.
4. **Pivotar**: operaciones de fila para que la columna entrante quede con un 1 en la fila pivote y ceros en las demás.
5. Volver al paso 2.

> [!NOTE]
> **Tabla inicial** del ejemplo (la fila $z$ representa $z - 40x - 30y = 0$):
>
> | Base | $x$ | $y$ | $s_1$ | $s_2$ | $s_3$ | b |
> |---|---|---|---|---|---|---|
> | $s_1$ | 2 | 1 | 1 | 0 | 0 | 100 |
> | $s_2$ | 1 | 1 | 0 | 1 | 0 | 80 |
> | $s_3$ | 1 | 0 | 0 | 0 | 1 | 40 |
> | $z$ | −40 | −30 | 0 | 0 | 0 | 0 |
>
> Entra $x$ (−40). Cocientes: $100/2 = 50$, $80/1 = 80$, $40/1 = 40$. Sale $s_3$.
>
> | Base | $x$ | $y$ | $s_1$ | $s_2$ | $s_3$ | b |
> |---|---|---|---|---|---|---|
> | $s_1$ | 0 | 1 | 1 | 0 | −2 | 20 |
> | $s_2$ | 0 | 1 | 0 | 1 | −1 | 40 |
> | $x$ | 1 | 0 | 0 | 0 | 1 | 40 |
> | $z$ | 0 | −30 | 0 | 0 | 40 | 1600 |
>
> Entra $y$. Cocientes: $20/1 = 20$, $40/1 = 40$. Sale $s_1$.
>
> | Base | $x$ | $y$ | $s_1$ | $s_2$ | $s_3$ | b |
> |---|---|---|---|---|---|---|
> | $y$ | 0 | 1 | 1 | 0 | −2 | 20 |
> | $s_2$ | 0 | 0 | −1 | 1 | 1 | 20 |
> | $x$ | 1 | 0 | 0 | 0 | 1 | 40 |
> | $z$ | 0 | 0 | 30 | 0 | −20 | 2200 |
>
> Entra $s_3$. Cocientes: solo $20/1$ y $40/1$ (el −2 no cuenta). Sale $s_2$.
>
> | Base | $x$ | $y$ | $s_1$ | $s_2$ | $s_3$ | b |
> |---|---|---|---|---|---|---|
> | $y$ | 0 | 1 | −1 | 2 | 0 | 60 |
> | $s_3$ | 0 | 0 | −1 | 1 | 1 | 20 |
> | $x$ | 1 | 0 | 1 | −1 | 0 | 20 |
> | $z$ | 0 | 0 | 10 | 20 | 0 | **2600** |
>
> No quedan coeficientes negativos: óptimo en $x = 20$, $y = 60$, $z = 2600$. El recorrido fue $(0,0) \to (40,0) \to (40,20) \to (20,60)$.

La holgura $s_3 = 20$ indica que la restricción de demanda no está activa: se fabrican 20 mesas menos que el tope.

Cuando el origen no es factible (restricciones $\ge$ o de igualdad), se usa el método de la **gran M** o el de **dos fases** para hallar un vértice inicial.

En la práctica el simplex es muy rápido, aunque su peor caso es exponencial. Los **métodos de punto interior** tienen garantía polinómica y compiten con él en problemas grandes.

## Dualidad

A todo problema lineal (el **primal**) le corresponde otro (el **dual**):

| Primal | Dual |
|---|---|
| $\max\ \mathbf{c}^T \mathbf{x}$ | $\min\ \mathbf{b}^T \mathbf{y}$ |
| $A\mathbf{x} \le \mathbf{b}$ | $A^T \mathbf{y} \ge \mathbf{c}$ |
| $\mathbf{x} \ge \mathbf{0}$ | $\mathbf{y} \ge \mathbf{0}$ |

Hay una variable dual por cada restricción del primal.

> [!NOTE]
> **Dual del ejemplo:**
> $$\min\ w = 100 y_1 + 80 y_2 + 40 y_3$$
> $$2y_1 + y_2 + y_3 \ge 40 \qquad y_1 + y_2 \ge 30 \qquad y_i \ge 0$$
>
> Su solución es $y_1 = 10$, $y_2 = 20$, $y_3 = 0$, con $w = 1000 + 1600 = 2600$.

| Teorema | Enunciado |
|---|---|
| Dualidad débil | toda solución factible del dual acota a toda solución factible del primal: $\mathbf{c}^T\mathbf{x} \le \mathbf{b}^T\mathbf{y}$ |
| **Dualidad fuerte** | si uno tiene óptimo, el otro también, y los valores **coinciden** |
| Holgura complementaria | si una restricción tiene holgura, su variable dual es cero |

### Precios sombra

El valor óptimo de cada variable dual es el **precio sombra** del recurso: cuánto mejora el objetivo por cada unidad adicional de ese recurso. Aparecen en la fila $z$ de la tabla final, bajo las columnas de las holguras.

| Recurso | Precio sombra | Interpretación |
|---|---|---|
| Carpintería | 10 | una hora más aumenta la ganancia en \$10 |
| Terminación | 20 | una hora más la aumenta en \$20 |
| Demanda de mesas | 0 | no es limitante |

Conviene pagar una hora extra de terminación si cuesta menos de \$20 por encima del costo normal.

## Análisis de sensibilidad

Estudia cuánto pueden cambiar los datos sin que cambie la solución.

| Cambio | Pregunta |
|---|---|
| Coeficientes del objetivo | ¿en qué rango sigue siendo óptimo el mismo vértice? |
| Términos independientes | ¿en qué rango siguen valiendo los precios sombra? |

> [!NOTE]
> En el ejemplo, el vértice $(20, 60)$ sigue siendo óptimo mientras la pendiente del objetivo esté entre las de las dos restricciones que lo forman: $1 \le c_x / c_y \le 2$. Con $c_y = 30$, la ganancia por mesa puede variar entre \$30 y \$60.

## Programación entera

Si las variables deben ser **enteras** (no se fabrican 2,5 máquinas) o binarias (hacer o no hacer), el problema es de **programación entera**.

> [!WARNING]
> Redondear la solución continua puede dar un punto infactible o lejos del óptimo. Y el problema entero es **NP-difícil**: mucho más costoso que el lineal.

El método de **ramificación y acotación** resuelve la relajación continua; si alguna variable resulta fraccionaria ($x = 2{,}5$), abre dos subproblemas ($x \le 2$ y $x \ge 3$) y descarta las ramas que no pueden mejorar la mejor solución entera hallada.

Las variables binarias modelan decisiones lógicas:

| Condición | Restricción |
|---|---|
| Elegir a lo sumo uno de varios proyectos | $x_1 + x_2 + x_3 \le 1$ |
| El proyecto 2 requiere el 1 | $x_2 \le x_1$ |
| Costo fijo si se produce | $x \le M\,y$, con $y$ binaria |

## Problemas clásicos

| Problema | Descripción |
|---|---|
| Mezcla (dieta) | combinar ingredientes al mínimo costo cumpliendo requisitos |
| **Transporte** | enviar desde orígenes a destinos al mínimo costo |
| **Asignación** | asignar tareas a personas, una a una |
| Flujo máximo, camino mínimo | problemas en redes |
| Planificación de la producción | cuánto fabricar en cada período |
| Corte de material | minimizar el desperdicio |

## En la práctica

```python
from scipy.optimize import linprog

# linprog minimiza: se cambia el signo del objetivo
res = linprog(
    c=[-40, -30],
    A_ub=[[2, 1], [1, 1], [1, 0]],
    b_ub=[100, 80, 40],
    bounds=[(0, None), (0, None)],
)
print(res.x, -res.fun)          # [20. 60.] 2600.0
```

Resolvedores: HiGHS, GLPK y CBC (libres); Gurobi y CPLEX (comerciales). Lenguajes de modelado: PuLP, Pyomo, OR-Tools, JuMP.

> [!TIP]
> La dificultad real no está en resolver, sino en **formular**: definir bien las variables y no olvidar restricciones. Verificar siempre que la solución tenga sentido en el problema original.
