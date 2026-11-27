+++
title = "Espectroscopía"
summary = "Interacción de la radiación con la materia, ley de Lambert-Beer, espectroscopía UV-visible, infrarroja, resonancia magnética nuclear, espectrometría de masas y elucidación de estructuras."
branch = "Química analítica y aplicada"
order = 3
level = "Avanzado"
tags = ["espectroscopía", "Lambert-Beer", "infrarrojo", "RMN", "espectrometría de masas"]
+++

## Radiación y materia

La **espectroscopía** estudia cómo la materia absorbe, emite o dispersa radiación electromagnética. Como los niveles de energía están **cuantizados**, una molécula solo absorbe los fotones cuya energía coincide con la diferencia entre dos niveles:

$$\Delta E = h\nu = \frac{hc}{\lambda}$$

Cada sustancia tiene un **espectro** característico: una huella que permite identificarla y cuantificarla.

| Región | Longitud de onda | Transición | Técnica |
|---|---|---|---|
| Rayos X | 0,01 – 10 nm | electrones internos | difracción, fluorescencia de rayos X |
| Ultravioleta y visible | 190 – 800 nm | electrones de valencia | UV-visible |
| Infrarrojo | 2,5 – 25 µm | vibraciones de los enlaces | IR |
| Microondas | 1 mm – 30 cm | rotaciones moleculares | de microondas |
| Radiofrecuencia | metros | espín nuclear en un campo magnético | RMN |

En infrarrojo se usa el **número de onda**, $\tilde{\nu} = 1/\lambda$, en $\text{cm}^{-1}$: es proporcional a la energía.

## Ley de Lambert-Beer

Al atravesar una muestra, la intensidad de la luz disminuye de $I_0$ a $I$.

| Magnitud | Definición |
|---|---|
| **Transmitancia** | $T = I / I_0$ |
| **Absorbancia** | $A = -\log T = \log (I_0 / I)$ |

$$A = \varepsilon \, b \, c$$

| Símbolo | Significado | Unidad |
|---|---|---|
| $\varepsilon$ | absortividad molar, propia de la sustancia y de la longitud de onda | L/(mol·cm) |
| $b$ | camino óptico (ancho de la cubeta) | cm |
| $c$ | concentración | mol/L |

La absorbancia es **proporcional a la concentración**: es la base del análisis cuantitativo.

> [!NOTE]
> **Ejemplo.** Una solución de permanganato ($\varepsilon = 2\,455$ L/(mol·cm) a 525 nm) en una cubeta de 1,00 cm da $A = 0{,}491$.
> $$c = \frac{A}{\varepsilon b} = \frac{0{,}491}{2\,455 \times 1{,}00} = 2{,}00 \times 10^{-4}\ \text{M}$$
> La transmitancia es $T = 10^{-0{,}491} = 0{,}323$: pasa el 32,3 % de la luz.

> [!WARNING]
> La ley se cumple en soluciones **diluidas** y con luz monocromática. Conviene trabajar con absorbancias entre 0,1 y 1,0: por debajo, el error relativo es grande; por encima, llega muy poca luz al detector.

Se mide a la longitud de onda de **máxima absorción** ($\lambda_{\text{máx}}$), donde la sensibilidad es mayor.

## Espectroscopía UV-visible

Promueve electrones a orbitales de mayor energía.

| Transición | Aparece en | Región |
|---|---|---|
| $\sigma \to \sigma^*$ | enlaces simples | ultravioleta lejano |
| $n \to \pi^*$ | grupos con pares libres y enlaces múltiples (C=O) | 270 – 300 nm, débil |
| $\pi \to \pi^*$ | enlaces dobles y anillos aromáticos | intensa |
| $d \to d$ | complejos de metales de transición | visible |

Un **cromóforo** es el grupo responsable de la absorción.

> [!IMPORTANT]
> La **conjugación** (enlaces dobles alternados) reduce la diferencia de energía entre orbitales y desplaza la absorción hacia longitudes de onda **mayores**. Con suficientes enlaces conjugados, el compuesto absorbe en el visible y es coloreado.

| Compuesto | Enlaces dobles conjugados | $\lambda_{\text{máx}}$ (nm) | Color |
|---|---|---|---|
| Eteno | 1 | 171 | incoloro |
| 1,3-Butadieno | 2 | 217 | incoloro |
| Benceno | 3 (aromático) | 255 | incoloro |
| β-Caroteno | 11 | 455 | naranja |

Usos: determinar concentraciones, seguir la velocidad de una reacción, medir proteínas y ácidos nucleicos (el ADN absorbe a 260 nm).

## Espectroscopía infrarroja

La radiación infrarroja excita las **vibraciones** de los enlaces: estiramientos y flexiones. Un enlace se comporta como un resorte, cuya frecuencia es

$$\tilde{\nu} = \frac{1}{2\pi c} \sqrt{\frac{k}{\mu}}$$

con $k$ la constante de fuerza y $\mu$ la masa reducida. Los enlaces **más fuertes** y los átomos **más livianos** vibran a mayor frecuencia.

Para que una vibración absorba, debe cambiar el **momento dipolar**: por eso $\mathrm{N_2}$ y $\mathrm{O_2}$ no absorben en el infrarrojo, y $\mathrm{CO_2}$ y $\mathrm{H_2O}$ sí (son gases de efecto invernadero).

### Bandas características

| Enlace | Grupo | Número de onda ($\text{cm}^{-1}$) | Aspecto |
|---|---|---|---|
| O–H | alcohol | 3200 – 3600 | ancha e intensa |
| O–H | ácido carboxílico | 2500 – 3300 | muy ancha |
| N–H | amina, amida | 3300 – 3500 | media; dos picos en $\mathrm{-NH_2}$ |
| C–H | alcano ($sp^3$) | 2850 – 2960 | intensa |
| C–H | alqueno, aromático ($sp^2$) | 3000 – 3100 | media |
| C–H | alquino ($sp$) | 3300 | aguda |
| C≡N | nitrilo | 2210 – 2260 | media |
| C≡C | alquino | 2100 – 2260 | débil |
| **C=O** | carbonilo | 1650 – 1800 | **muy intensa** |
| C=C | alqueno | 1620 – 1680 | variable |
| C=C | aromático | 1450 – 1600 | varias |
| C–O | alcohol, éter, éster | 1000 – 1300 | intensa |

La posición del carbonilo distingue el grupo funcional:

| Compuesto | C=O ($\text{cm}^{-1}$) |
|---|---|
| Éster | 1735 – 1750 |
| Aldehído | 1720 – 1740 |
| Cetona | 1705 – 1725 |
| Ácido carboxílico | 1700 – 1725 |
| Amida | 1640 – 1690 |

La zona por debajo de $1500\ \text{cm}^{-1}$ es la **huella dactilar**: compleja y única para cada compuesto.

> [!NOTE]
> **Interpretación.** Un espectro con una banda ancha en $3350\ \text{cm}^{-1}$ y ninguna entre 1650 y 1800 indica un **alcohol**. Si aparece una banda muy ancha de 2500 a 3300 junto con otra intensa en 1710, es un **ácido carboxílico**.

## Resonancia magnética nuclear

Los núcleos con espín, como $^1\mathrm{H}$ y $^{13}\mathrm{C}$, se comportan como pequeños imanes. En un campo magnético intenso adoptan dos orientaciones de distinta energía, y absorben radiofrecuencia al pasar de una a otra.

La frecuencia exacta depende del **entorno químico**: los electrones que rodean al núcleo lo **apantallan** del campo. Es la técnica más poderosa para determinar estructuras orgánicas.

Un espectro de $^1\mathrm{H}$ da cuatro datos:

| Dato | Informa |
|---|---|
| **Número de señales** | cuántos tipos de hidrógeno distintos hay |
| **Desplazamiento químico** ($\delta$, en ppm) | el entorno de cada tipo |
| **Integración** (área) | cuántos hidrógenos hay de cada tipo |
| **Multiplicidad** | cuántos hidrógenos vecinos tiene |

### Desplazamiento químico

Se mide respecto del tetrametilsilano (TMS), al que se asigna $\delta = 0$. Los átomos electronegativos y los enlaces $\pi$ **desapantallan** y aumentan $\delta$.

| Tipo de hidrógeno | $\delta$ (ppm) |
|---|---|
| Alquilo, $\mathrm{R-CH_3}$ | 0,9 |
| Alquilo, $\mathrm{R_2CH_2}$ | 1,3 |
| Vecino a un carbonilo, $\mathrm{-CO-CH_3}$ | 2,0 – 2,5 |
| Unido a carbono con oxígeno, $\mathrm{-O-CH}$ | 3,3 – 4,0 |
| Vinílico, $\mathrm{C=CH}$ | 4,5 – 6,5 |
| Aromático | 6,5 – 8,0 |
| Aldehído, $\mathrm{-CHO}$ | 9 – 10 |
| Ácido carboxílico, $\mathrm{-COOH}$ | 10 – 12 |
| Alcohol, $\mathrm{-OH}$ | 1 – 5 (variable) |

### Acoplamiento

Una señal se desdobla por efecto de los hidrógenos de los carbonos **vecinos**. Con $n$ vecinos equivalentes aparece en $n + 1$ picos (**regla $n + 1$**), con intensidades dadas por el triángulo de Pascal.

| Vecinos | Señal | Intensidades |
|---|---|---|
| 0 | singlete | 1 |
| 1 | doblete | 1 : 1 |
| 2 | triplete | 1 : 2 : 1 |
| 3 | cuartete | 1 : 3 : 3 : 1 |

> [!NOTE]
> **Etanol, $\mathrm{CH_3-CH_2-OH}$.**
>
> | Hidrógenos | $\delta$ | Integración | Multiplicidad |
> |---|---|---|---|
> | $\mathrm{CH_3}$ | 1,2 | 3 | triplete (2 vecinos) |
> | $\mathrm{CH_2}$ | 3,7 | 2 | cuartete (3 vecinos) |
> | $\mathrm{OH}$ | variable | 1 | singlete ancho |
>
> El patrón triplete-cuartete es la firma de un grupo etilo.

El espectro de $^{13}\mathrm{C}$ muestra una señal por cada tipo de carbono, entre 0 y 220 ppm; los carbonilos aparecen entre 160 y 220.

La misma física, aplicada a los hidrógenos del agua del cuerpo, da la **resonancia magnética** de uso médico.

## Espectrometría de masas

No usa radiación: **ioniza** las moléculas, las fragmenta y separa los iones según su relación **masa/carga** ($m/z$).

| Señal | Informa |
|---|---|
| **Ion molecular** ($\mathrm{M^+}$) | la masa molecular |
| **Pico base** | el fragmento más abundante (se le asigna 100 %) |
| Fragmentos | la estructura: los enlaces se rompen de manera característica |
| Picos isotópicos | qué elementos hay |

| Patrón isotópico | Indica |
|---|---|
| M y M+2 en relación 3 : 1 | un átomo de **cloro** ($^{35}\mathrm{Cl}$ y $^{37}\mathrm{Cl}$) |
| M y M+2 en relación 1 : 1 | un átomo de **bromo** ($^{79}\mathrm{Br}$ y $^{81}\mathrm{Br}$) |
| M+1 pequeño | $^{13}\mathrm{C}$: alrededor de 1,1 % por cada carbono |

| Pérdida desde M | Fragmento perdido |
|---|---|
| 15 | $\mathrm{CH_3}$ |
| 18 | $\mathrm{H_2O}$ (alcoholes) |
| 28 | CO o $\mathrm{C_2H_4}$ |
| 29 | $\mathrm{C_2H_5}$ o CHO |
| 45 | $\mathrm{OC_2H_5}$ o COOH |

**Regla del nitrógeno**: una masa molecular impar indica un número impar de átomos de nitrógeno.

La espectrometría de **alta resolución** mide la masa con cuatro decimales y permite deducir la fórmula molecular.

## Espectroscopía atómica

Analiza **elementos**, no moléculas. La muestra se atomiza en una llama, un horno o un plasma.

| Técnica | Principio | Uso |
|---|---|---|
| Absorción atómica | los átomos absorben la luz de una lámpara del mismo elemento | metales en aguas, suelos, alimentos |
| Emisión atómica (ICP) | los átomos excitados en un plasma emiten | muchos elementos a la vez |
| ICP-masas | los iones del plasma se separan por masa | trazas y ultratrazas |

El ensayo a la llama es su versión cualitativa: el sodio da amarillo; el potasio, violeta; el cobre, verde.

## Elucidación de estructuras

Las técnicas se combinan.

### Grado de insaturación

A partir de la fórmula molecular $\mathrm{C_cH_hN_nO_oX_x}$:

$$\text{GI} = \frac{2c + 2 + n - h - x}{2}$$

Cada enlace doble o anillo cuenta 1; cada triple, 2; un anillo bencénico, 4.

### Procedimiento

1. **Masas**: masa molecular y fórmula.
2. **Grado de insaturación**.
3. **Infrarrojo**: grupos funcionales.
4. **RMN**: el esqueleto y cómo se conectan los fragmentos.
5. Proponer la estructura y verificar que explique **todos** los datos.

> [!NOTE]
> **Ejemplo.** Compuesto $\mathrm{C_4H_8O_2}$ ($M = 88$).
>
> **Grado de insaturación**: $(8 + 2 - 8)/2 = 1$.
>
> **IR**: banda intensa en $1740\ \text{cm}^{-1}$; ninguna por encima de 3000. Es un **éster**.
>
> **RMN de $^1\mathrm{H}$**:
>
> | $\delta$ | Integración | Multiplicidad | Asignación |
> |---|---|---|---|
> | 1,3 | 3 | triplete | $\mathrm{CH_3}$ junto a un $\mathrm{CH_2}$ |
> | 2,0 | 3 | singlete | $\mathrm{CH_3}$ junto al carbonilo, sin vecinos |
> | 4,1 | 2 | cuartete | $\mathrm{CH_2}$ unido a oxígeno, junto a un $\mathrm{CH_3}$ |
>
> Estructura: $\mathrm{CH_3-CO-O-CH_2-CH_3}$, **acetato de etilo**.

## Resumen

| Técnica | Qué revela | Muestra |
|---|---|---|
| UV-visible | conjugación; concentración | solución diluida |
| Infrarrojo | grupos funcionales | sólido, líquido o gas |
| RMN | conectividad de los átomos | unos miligramos en solvente deuterado |
| Masas | masa molecular y fragmentos | microgramos o menos |
| Atómica | elementos y su concentración | solución |
| Difracción de rayos X | estructura tridimensional | un monocristal |

> [!TIP]
> Empezar por lo que cada técnica dice con seguridad: el infrarrojo confirma o descarta grupos funcionales en segundos (¿hay C=O?, ¿hay O–H?), y la integración de la RMN debe sumar los hidrógenos de la fórmula. Recién después, armar el rompecabezas.
