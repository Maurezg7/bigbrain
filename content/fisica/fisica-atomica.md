+++
title = "Física atómica"
summary = "Espectros y modelo de Bohr, átomo de hidrógeno cuántico, números cuánticos, espín, principio de exclusión, átomos multielectrónicos, rayos X y láser."
branch = "Física moderna"
order = 5
level = "Avanzado"
tags = ["átomo de hidrógeno", "modelo de Bohr", "números cuánticos", "espín", "principio de exclusión", "láser"]
+++

## Espectros atómicos

Un gas a baja presión excitado por una descarga no emite todos los colores, sino un conjunto de **líneas** de longitudes de onda definidas: su **espectro de emisión**. El mismo gas absorbe esas mismas longitudes de onda.

Cada elemento tiene un espectro propio. Así se conoce la composición de las estrellas.

Para el hidrógeno, las líneas obedecen la **fórmula de Rydberg**:

$$\frac{1}{\lambda} = R_H \left( \frac{1}{n_1^2} - \frac{1}{n_2^2} \right) \qquad R_H = 1{,}097 \times 10^7\ \text{m}^{-1}$$

| Serie | $n_1$ | Región |
|---|---|---|
| Lyman | 1 | ultravioleta |
| Balmer | 2 | visible |
| Paschen | 3 | infrarrojo |

La física clásica no podía explicar las líneas, ni siquiera la existencia del átomo: un electrón en órbita debería irradiar y caer al núcleo en una fracción de segundo.

## Modelo de Bohr

En 1913, Niels Bohr postuló:

1. El electrón gira en ciertas órbitas **estacionarias** sin irradiar.
2. Solo son posibles las órbitas en que el momento angular es múltiplo de $\hbar$: $L = m v r = n\hbar$.
3. El átomo emite o absorbe un fotón al **saltar** entre órbitas: $h\nu = E_i - E_f$.

De ahí resultan radios y energías cuantizados:

$$r_n = n^2 a_0 \qquad a_0 = 0{,}0529\ \text{nm}$$

$$E_n = -\frac{13{,}6\ \text{eV}}{n^2}$$

$a_0$ es el **radio de Bohr**. La energía es negativa porque el electrón está ligado.

| Nivel | $n$ | Energía (eV) |
|---|---|---|
| Fundamental | 1 | −13,6 |
| Primer excitado | 2 | −3,40 |
| Segundo excitado | 3 | −1,51 |
| | 4 | −0,85 |
| Ionización | $\infty$ | 0 |

La **energía de ionización** del hidrógeno es de 13,6 eV.

> [!NOTE]
> **Ejemplo.** Transición de $n = 3$ a $n = 2$ (primera línea de Balmer).
> $$E = -1{,}51 - (-3{,}40) = 1{,}89\ \text{eV}$$
> $$\lambda = \frac{hc}{E} = \frac{1240\ \text{eV·nm}}{1{,}89\ \text{eV}} = 656\ \text{nm}$$
> Es la línea roja H-alfa.

Para iones con un solo electrón y número atómico $Z$ ($\mathrm{He^+}$, $\mathrm{Li^{2+}}$): $E_n = -13{,}6\, Z^2 / n^2$ eV.

El modelo acierta con las energías del hidrógeno, pero no explica las intensidades de las líneas ni los átomos con más electrones, y la idea de órbita definida contradice el principio de incertidumbre.

## El átomo de hidrógeno cuántico

La ecuación de Schrödinger con el potencial de Coulomb,

$$V(r) = -\frac{e^2}{4\pi\varepsilon_0 r}$$

tiene soluciones solo para ciertas energías, las mismas de Bohr. Pero el electrón ya no describe una órbita: lo representa una **función de onda** $\psi$, y $\lvert \psi \rvert^2$ da la densidad de probabilidad de encontrarlo. La región donde es probable hallarlo es un **orbital**.

### Números cuánticos

Cada solución queda identificada por tres números:

| Número | Símbolo | Valores | Determina |
|---|---|---|---|
| **Principal** | $n$ | 1, 2, 3, … | la energía y el tamaño |
| **Orbital** (azimutal) | $l$ | 0, 1, …, $n - 1$ | el módulo del momento angular; la forma |
| **Magnético** | $m_l$ | $-l, \dots, +l$ | la componente $z$ del momento angular; la orientación |

$$L = \sqrt{l(l + 1)}\,\hbar \qquad L_z = m_l \hbar$$

| $l$ | 0 | 1 | 2 | 3 |
|---|---|---|---|---|
| Letra | s | p | d | f |
| Orbitales ($2l + 1$) | 1 | 3 | 5 | 7 |

Para cada $n$ hay $n^2$ estados con la misma energía (**degeneración**).

> [!NOTE]
> Para $n = 2$: $l = 0$ (el orbital 2s) y $l = 1$ con $m_l = -1, 0, +1$ (los tres 2p). Total: $2^2 = 4$ estados.

El momento angular está cuantizado en módulo y en dirección (**cuantización espacial**): el vector $\mathbf{L}$ nunca apunta exactamente según $z$, porque $L_z$ máximo, $l\hbar$, es menor que $\sqrt{l(l+1)}\,\hbar$.

### El estado fundamental

$$\psi_{100} = \frac{1}{\sqrt{\pi a_0^3}}\, e^{-r/a_0}$$

Tiene simetría esférica y momento angular **nulo**, a diferencia de lo que suponía Bohr.

La probabilidad de hallar al electrón entre $r$ y $r + dr$ es $P(r)\,dr = 4\pi r^2 \lvert \psi \rvert^2\, dr$. Es máxima en $r = a_0$: el radio de Bohr es la distancia **más probable**, no la única.

## Espín

En el experimento de **Stern y Gerlach** (1922), un haz de átomos de plata se dividió en **dos** al atravesar un campo magnético no uniforme. El momento angular orbital solo puede dar un número impar de componentes ($2l + 1$).

El electrón tiene un momento angular **intrínseco**, el **espín**, sin análogo clásico:

| Número cuántico | Valor |
|---|---|
| De espín, $s$ | ½ (fijo) |
| Magnético de espín, $m_s$ | +½ («arriba») o −½ («abajo») |

El estado de un electrón en un átomo queda determinado por **cuatro** números cuánticos: $n$, $l$, $m_l$, $m_s$.

El espín lleva asociado un **momento magnético**: el electrón es un pequeño imán. Es el origen del magnetismo de los materiales.

### Estructura fina

El momento magnético del espín interactúa con el campo magnético que el electrón «ve» por su movimiento alrededor del núcleo: es el **acoplamiento espín-órbita**, que desdobla ligeramente los niveles. La línea amarilla del sodio es en realidad un doblete: 589,0 y 589,6 nm.

En un campo magnético externo, los niveles se desdoblan según $m_l$ y $m_s$: **efecto Zeeman**.

## Principio de exclusión

> [!IMPORTANT]
> **Principio de exclusión de Pauli.** Dos electrones de un mismo átomo no pueden tener los cuatro números cuánticos iguales.

Cada orbital admite a lo sumo dos electrones, con espines opuestos.

| Subcapa | Orbitales | Electrones |
|---|---|---|
| s | 1 | 2 |
| p | 3 | 6 |
| d | 5 | 10 |
| f | 7 | 14 |

La capa $n$ admite $2n^2$ electrones: 2, 8, 18, 32.

El principio vale para todos los **fermiones** (partículas de espín semientero). Los **bosones** (espín entero), como los fotones, pueden compartir un mismo estado.

Sin exclusión, todos los electrones caerían al nivel más bajo: no habría química, y la materia no ocuparía volumen.

## Átomos con muchos electrones

Cada electrón siente al núcleo **apantallado** por los demás: una **carga nuclear efectiva** menor que $Z$.

En el hidrógeno la energía depende solo de $n$. En los demás átomos depende también de $l$: los orbitales s penetran más cerca del núcleo y quedan menos apantallados.

$$E_{ns} < E_{np} < E_{nd} < E_{nf}$$

### Configuración electrónica

Los electrones ocupan los orbitales en orden de energía creciente:

$$1s < 2s < 2p < 3s < 3p < 4s < 3d < 4p < 5s < 4d < 5p < 6s < 4f < 5d < 6p$$

**Regla de Hund**: en una subcapa, los electrones ocupan primero orbitales distintos con espines paralelos.

| Elemento | $Z$ | Configuración |
|---|---|---|
| Hidrógeno | 1 | $1s^1$ |
| Helio | 2 | $1s^2$ |
| Carbono | 6 | $1s^2\, 2s^2\, 2p^2$ |
| Neón | 10 | $1s^2\, 2s^2\, 2p^6$ |
| Sodio | 11 | $[\mathrm{Ne}]\, 3s^1$ |
| Hierro | 26 | $[\mathrm{Ar}]\, 4s^2\, 3d^6$ |

La **tabla periódica** es consecuencia directa: los elementos de una misma columna tienen la misma configuración externa y, por eso, propiedades químicas parecidas. Los gases nobles tienen capas completas.

### Reglas de selección

No todas las transiciones entre niveles ocurren. Como el fotón lleva momento angular, las permitidas cumplen

$$\Delta l = \pm 1 \qquad \Delta m_l = 0, \pm 1$$

## Rayos X

Se producen al frenar electrones rápidos contra un blanco metálico.

| Componente | Origen |
|---|---|
| **Espectro continuo** | radiación de frenado; tiene una longitud de onda mínima |
| **Líneas características** | un electrón incidente arranca un electrón interno, y otro de una capa superior ocupa el hueco |

Toda la energía del electrón, $eV$, puede pasar a un solo fotón:

$$\lambda_{\min} = \frac{hc}{eV}$$

> [!NOTE]
> **Ejemplo.** Con una tensión de 30 kV:
> $$\lambda_{\min} = \frac{1240\ \text{eV·nm}}{30\,000\ \text{eV}} = 0{,}0413\ \text{nm}$$

La **ley de Moseley** relaciona la frecuencia de la línea $K_\alpha$ con el número atómico:

$$\sqrt{f} \propto (Z - 1)$$

Permitió ordenar la tabla periódica por número atómico y predecir elementos aún no descubiertos.

## Emisión estimulada y láser

Un átomo y la radiación interactúan de tres maneras:

| Proceso | Descripción |
|---|---|
| Absorción | un fotón lleva al átomo a un nivel superior |
| **Emisión espontánea** | el átomo excitado decae solo; el fotón sale en dirección y fase al azar |
| **Emisión estimulada** | un fotón induce el decaimiento; el fotón emitido es **idéntico** al incidente |

En equilibrio hay más átomos en el nivel inferior, y domina la absorción. Para amplificar la luz hace falta una **inversión de población**: más átomos en el nivel superior.

Un **láser** tiene tres partes:

| Parte | Función |
|---|---|
| Medio activo | gas, cristal o semiconductor donde ocurre la emisión estimulada |
| Bombeo | energía (eléctrica u óptica) que crea la inversión de población |
| Cavidad resonante | dos espejos que hacen pasar la luz muchas veces; uno deja salir una parte |

La inversión se logra con un nivel **metaestable**, de vida larga, donde los átomos se acumulan.

| Propiedad de la luz láser | Significado |
|---|---|
| Monocromática | una sola longitud de onda |
| **Coherente** | todas las ondas en fase |
| Direccional | haz casi paralelo |
| Intensa | mucha potencia en un área pequeña |

| Láser | Longitud de onda | Uso |
|---|---|---|
| Helio-neón | 633 nm | laboratorio, alineación |
| De diodo | 400 – 1600 nm | lectores, fibra óptica, punteros |
| De $\mathrm{CO_2}$ | 10,6 µm | corte y soldadura |
| Nd:YAG | 1064 nm | industria, cirugía |
| De excímero | ultravioleta | cirugía ocular, litografía |

## Moléculas

Al unirse en moléculas, además de los niveles electrónicos aparecen otros:

| Movimiento | Separación típica | Región |
|---|---|---|
| Electrónico | unos eV | visible y ultravioleta |
| **Vibración** | 0,1 eV | infrarrojo |
| **Rotación** | 0,001 eV | microondas |

Vibración: $E_v = (v + \tfrac{1}{2})\hbar\omega$. Rotación: $E_J = \dfrac{\hbar^2}{2I} J(J + 1)$.

Los espectros moleculares son **bandas** formadas por muchas líneas muy próximas.

> [!TIP]
> Para calcular longitudes de onda a partir de energías en electronvoltios, recordar $hc = 1240$ eV·nm: $\lambda\,(\text{nm}) = 1240 / E\,(\text{eV})$.
