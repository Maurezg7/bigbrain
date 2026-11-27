+++
title = "Mecánica analítica"
summary = "Coordenadas generalizadas, principio de mínima acción, ecuaciones de Euler-Lagrange, simetrías y teorema de Noether, formulación de Hamilton y espacio de fases."
branch = "Mecánica"
order = 10
level = "Avanzado"
tags = ["lagrangiano", "hamiltoniano", "Euler-Lagrange", "mínima acción", "Noether", "espacio de fases"]
+++

## Otra forma de hacer mecánica

La mecánica de Newton trabaja con **vectores**: fuerzas y aceleraciones. Sirve bien para problemas simples, pero se complica con ligaduras (un péndulo, una cuenta en un alambre), porque hay que incluir fuerzas de vínculo desconocidas.

La **mecánica analítica** reformula todo con **escalares**: energías. Da las mismas predicciones, con ventajas:

- las fuerzas de vínculo no aparecen;
- vale en cualquier sistema de coordenadas;
- revela la relación entre **simetrías** y **leyes de conservación**;
- es el punto de partida de la mecánica cuántica y de la teoría de campos.

## Coordenadas generalizadas

Las **ligaduras** restringen el movimiento. Los **grados de libertad** son la cantidad de coordenadas independientes necesarias para describir la configuración:

$$n = 3N - k$$

para $N$ partículas con $k$ ligaduras.

Las **coordenadas generalizadas** $q_1, \dots, q_n$ son cualquier conjunto de variables independientes que fije la configuración. No tienen por qué ser longitudes: pueden ser ángulos.

| Sistema | Grados de libertad | Coordenadas generalizadas |
|---|---|---|
| Partícula libre | 3 | $x, y, z$ |
| Péndulo plano | 1 | el ángulo $\theta$ |
| Cuenta en un alambre | 1 | la distancia a lo largo del alambre |
| Péndulo doble | 2 | $\theta_1, \theta_2$ |
| Sólido rígido | 6 | 3 de posición y 3 ángulos |

Las derivadas $\dot{q}_i$ son las **velocidades generalizadas**.

## El lagrangiano

$$L(q, \dot{q}, t) = T - V$$

Es la energía cinética **menos** la potencial, expresadas en coordenadas generalizadas.

## Principio de mínima acción

La **acción** es la integral del lagrangiano a lo largo de un camino entre dos instantes:

$$S = \int_{t_1}^{t_2} L(q, \dot{q}, t)\, dt$$

> [!IMPORTANT]
> **Principio de Hamilton.** De todos los caminos posibles entre dos configuraciones dadas, el sistema sigue aquel que hace **estacionaria** la acción: $\delta S = 0$.

Es una formulación global: en lugar de decir cómo cambia el estado a cada instante, selecciona la trayectoria completa. El análogo en óptica es el principio de Fermat.

## Ecuaciones de Euler-Lagrange

Imponer $\delta S = 0$ conduce, mediante el cálculo de variaciones, a una ecuación por cada coordenada:

$$\frac{d}{dt} \left( \frac{\partial L}{\partial \dot{q}_i} \right) - \frac{\partial L}{\partial q_i} = 0$$

### Receta

1. Elegir las coordenadas generalizadas.
2. Escribir $T$ y $V$ en función de ellas.
3. Formar $L = T - V$.
4. Aplicar la ecuación a cada coordenada.

> [!NOTE]
> **Partícula en una dimensión.** $L = \tfrac{1}{2} m \dot{x}^2 - V(x)$.
> $$\frac{\partial L}{\partial \dot{x}} = m\dot{x} \qquad \frac{\partial L}{\partial x} = -\frac{dV}{dx}$$
> $$m\ddot{x} = -\frac{dV}{dx} = F$$
> Se recupera la segunda ley de Newton.

> [!NOTE]
> **Péndulo simple.** Coordenada: $\theta$. La masa se mueve con velocidad $l\dot{\theta}$ y su altura es $-l\cos\theta$.
> $$L = \tfrac{1}{2} m l^2 \dot{\theta}^2 + m g l \cos\theta$$
> $$\frac{\partial L}{\partial \dot{\theta}} = m l^2 \dot{\theta} \qquad \frac{\partial L}{\partial \theta} = -m g l \sin\theta$$
> $$m l^2 \ddot{\theta} + m g l \sin\theta = 0 \quad\Rightarrow\quad \ddot{\theta} + \frac{g}{l} \sin\theta = 0$$
> La tensión de la cuerda no apareció en ningún momento.

> [!NOTE]
> **Máquina de Atwood.** Dos masas $m_1$ y $m_2$ unidas por una cuerda que pasa por una polea ideal. Una sola coordenada: $x$, lo que baja $m_1$ (y sube $m_2$).
> $$L = \tfrac{1}{2}(m_1 + m_2)\dot{x}^2 + (m_1 - m_2)\, g\, x$$
> $$(m_1 + m_2)\ddot{x} = (m_1 - m_2)\, g \quad\Rightarrow\quad \ddot{x} = \frac{m_1 - m_2}{m_1 + m_2}\, g$$

> [!NOTE]
> **Movimiento en un potencial central**, en coordenadas polares:
> $$L = \tfrac{1}{2} m (\dot{r}^2 + r^2 \dot{\varphi}^2) - V(r)$$
> Para $\varphi$: como $L$ no depende de $\varphi$,
> $$\frac{d}{dt}(m r^2 \dot{\varphi}) = 0$$
> El momento angular se conserva. Para $r$:
> $$m\ddot{r} = m r \dot{\varphi}^2 - \frac{dV}{dr}$$
> El término centrífugo aparece solo.

## Momentos generalizados y conservación

El **momento conjugado** a la coordenada $q_i$ es

$$p_i = \frac{\partial L}{\partial \dot{q}_i}$$

| Coordenada | Momento conjugado |
|---|---|
| cartesiana $x$ | cantidad de movimiento $m\dot{x}$ |
| ángulo $\varphi$ | momento angular $m r^2 \dot{\varphi}$ |

Una coordenada es **cíclica** si no aparece en el lagrangiano (aunque sí aparezca su velocidad). Entonces la ecuación de Euler-Lagrange da $\dot{p}_i = 0$:

> [!IMPORTANT]
> El momento conjugado a una coordenada cíclica **se conserva**.

## Simetrías: teorema de Noether

Que una coordenada sea cíclica significa que el sistema no cambia al desplazarla: tiene una **simetría**.

> [!IMPORTANT]
> **Teorema de Noether (1918).** A cada simetría continua del lagrangiano le corresponde una magnitud conservada.

| Simetría | El sistema no cambia al... | Se conserva |
|---|---|---|
| Traslación espacial | desplazarlo | la cantidad de movimiento |
| Rotación | girarlo | el momento angular |
| Traslación temporal | repetir el experimento más tarde | la energía |

Las leyes de conservación dejan de ser hechos aislados: son consecuencia de las simetrías del espacio y del tiempo. La idea se extiende a la física de partículas, donde la conservación de la carga eléctrica proviene de una simetría interna.

## El hamiltoniano

Se define mediante una transformación de Legendre:

$$H(q, p, t) = \sum_i p_i \dot{q}_i - L$$

expresado en función de las coordenadas y los **momentos**, no de las velocidades.

Si las ligaduras no dependen del tiempo y el potencial no depende de las velocidades, $H$ es la **energía total**:

$$H = T + V$$

Si $L$ no depende explícitamente del tiempo, $H$ se conserva.

### Ecuaciones de Hamilton

$$\dot{q}_i = \frac{\partial H}{\partial p_i} \qquad \dot{p}_i = -\frac{\partial H}{\partial q_i}$$

Son $2n$ ecuaciones de **primer orden**, en lugar de las $n$ de segundo orden de Lagrange.

> [!NOTE]
> **Oscilador armónico.** $L = \tfrac{1}{2} m \dot{x}^2 - \tfrac{1}{2} k x^2$, con $p = m\dot{x}$.
> $$H = p\dot{x} - L = \frac{p^2}{2m} + \frac{1}{2} k x^2$$
> $$\dot{x} = \frac{\partial H}{\partial p} = \frac{p}{m} \qquad \dot{p} = -\frac{\partial H}{\partial x} = -kx$$
> Combinándolas: $m\ddot{x} = -kx$, con $\omega = \sqrt{k/m}$.

| | Lagrange | Hamilton |
|---|---|---|
| Función | $L(q, \dot{q}, t) = T - V$ | $H(q, p, t)$ |
| Variables | coordenadas y velocidades | coordenadas y momentos |
| Ecuaciones | $n$ de segundo orden | $2n$ de primer orden |
| Conviene para | plantear y resolver problemas con ligaduras | estudio teórico, mecánica estadística y cuántica |

## Espacio de fases

Es el espacio de dimensión $2n$ cuyos ejes son las coordenadas y los momentos. Un **punto** representa el estado completo del sistema, y su evolución traza una **trayectoria**.

- Por cada punto pasa una sola trayectoria: no se cruzan.
- En un sistema conservativo, las trayectorias son las curvas de $H$ constante.

> [!NOTE]
> **Oscilador armónico.** La energía constante,
> $$\frac{p^2}{2m} + \frac{1}{2} k x^2 = E$$
> es la ecuación de una **elipse** de semiejes $\sqrt{2E/k}$ y $\sqrt{2mE}$. El sistema la recorre periódicamente.

| Sistema | Retrato de fases |
|---|---|
| Oscilador armónico | elipses concéntricas |
| Péndulo, oscilaciones | curvas cerradas alrededor del equilibrio |
| Péndulo, rotaciones | curvas abiertas |
| Oscilador amortiguado | espirales hacia el origen |

En el péndulo, la curva que separa oscilaciones de rotaciones se llama **separatriz**.

### Teorema de Liouville

Un conjunto de estados iniciales ocupa una región del espacio de fases. Al evolucionar, esa región se deforma pero su **volumen se conserva**. Es la base de la mecánica estadística.

## Corchetes de Poisson

Para dos funciones $f$ y $g$ de las coordenadas y los momentos:

$$\{f, g\} = \sum_i \left( \frac{\partial f}{\partial q_i} \frac{\partial g}{\partial p_i} - \frac{\partial f}{\partial p_i} \frac{\partial g}{\partial q_i} \right)$$

La evolución de cualquier magnitud es

$$\frac{df}{dt} = \{f, H\} + \frac{\partial f}{\partial t}$$

Una magnitud que no depende explícitamente del tiempo se conserva si $\{f, H\} = 0$. Los corchetes fundamentales son

$$\{q_i, p_j\} = \delta_{ij}$$

> [!NOTE]
> **Puente a la mecánica cuántica.** La cuantización reemplaza el corchete de Poisson por el conmutador de operadores:
> $$\{x, p\} = 1 \quad\longrightarrow\quad [\hat{x}, \hat{p}] = i\hbar$$
> De esa relación sale el principio de incertidumbre. Y la formulación de Feynman suma sobre todos los caminos, cada uno con fase $e^{iS/\hbar}$: en el límite clásico sobrevive el de acción estacionaria.

## Pequeñas oscilaciones

Cerca de un equilibrio estable, todo sistema se comporta como un conjunto de osciladores armónicos. Desarrollando el potencial hasta segundo orden, las ecuaciones quedan lineales:

$$M \ddot{\mathbf{q}} + K \mathbf{q} = \mathbf{0}$$

Las frecuencias de los **modos normales** salen de

$$\det(K - \omega^2 M) = 0$$

Un sistema con $n$ grados de libertad tiene $n$ modos normales; en cada uno, todas las partes oscilan con la misma frecuencia. El movimiento general es una superposición de modos.

> [!NOTE]
> **Dos masas iguales unidas entre sí y a las paredes por tres resortes iguales.**
>
> | Modo | Movimiento | Frecuencia |
> |---|---|---|
> | Simétrico | las dos masas se mueven juntas | $\omega_1 = \sqrt{k/m}$ |
> | Antisimétrico | en sentidos opuestos | $\omega_2 = \sqrt{3k/m}$ |

## Más allá

| Tema | Idea |
|---|---|
| Multiplicadores de Lagrange | permiten calcular las fuerzas de vínculo cuando hacen falta |
| Fuerzas no conservativas | se agregan como fuerzas generalizadas; el rozamiento viscoso, con la función de disipación de Rayleigh |
| Partícula en un campo electromagnético | $L = \tfrac{1}{2} m v^2 - q\phi + q\,\mathbf{v} \cdot \mathbf{A}$ |
| Teoría de campos | el lagrangiano se reemplaza por una densidad lagrangiana: así se formulan el electromagnetismo, la relatividad general y el modelo estándar |
| Caos | sistemas como el péndulo doble son deterministas pero imprevisibles a largo plazo |

> [!TIP]
> La elección de coordenadas es la mitad del problema: buscar las que respeten las simetrías del sistema. Cada coordenada cíclica regala una cantidad conservada y una ecuación menos por resolver.
