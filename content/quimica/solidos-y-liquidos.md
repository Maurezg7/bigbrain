+++
title = "Sólidos, líquidos y cambios de fase"
summary = "Propiedades de los líquidos, presión de vapor y ecuación de Clausius-Clapeyron, diagramas de fases, tipos de sólidos, celdas unitarias y cálculo de densidades."
branch = "Estados de la materia y soluciones"
order = 3
level = "Intermedio"
tags = ["líquidos", "sólidos cristalinos", "diagrama de fases", "presión de vapor", "celda unitaria"]
+++

## Los tres estados

| | Gas | Líquido | Sólido |
|---|---|---|---|
| Forma | la del recipiente | la del recipiente | propia |
| Volumen | el del recipiente | propio | propio |
| Compresibilidad | alta | casi nula | casi nula |
| Orden de las partículas | ninguno | de corto alcance | de largo alcance (si es cristalino) |
| Movimiento | libre | se deslizan | vibran en su sitio |

El estado resulta de la competencia entre la **energía cinética** de las partículas, que crece con la temperatura, y las **fuerzas intermoleculares**, que las mantienen juntas.

## Propiedades de los líquidos

| Propiedad | Definición | Aumenta con |
|---|---|---|
| **Tensión superficial** | energía necesaria para aumentar la superficie | fuerzas intermoleculares fuertes |
| **Viscosidad** | resistencia a fluir | fuerzas fuertes, moléculas largas; baja al calentar |
| **Capilaridad** | ascenso o descenso en tubos finos | adhesión al tubo mayor que la cohesión |

Las moléculas de la superficie son atraídas hacia el interior, por lo que el líquido tiende a minimizar su área: por eso las gotas son esféricas.

El agua moja el vidrio y forma un menisco **cóncavo** (la adhesión supera a la cohesión); el mercurio no lo moja y forma uno **convexo**.

### El agua

Sus puentes de hidrógeno le dan propiedades anómalas:

- punto de ebullición muy alto para su masa molar;
- calor específico y calor de vaporización elevados: modera el clima;
- el hielo es **menos denso** que el líquido, y por eso flota;
- densidad máxima a 4 °C.

## Presión de vapor

En un recipiente cerrado, las moléculas se evaporan y condensan hasta alcanzar un **equilibrio dinámico**. La presión del vapor en ese equilibrio es la **presión de vapor**.

- Aumenta con la **temperatura**.
- Es mayor cuanto más **débiles** son las fuerzas intermoleculares (líquidos volátiles).
- No depende de la cantidad de líquido ni del volumen del recipiente.

Un líquido **hierve** cuando su presión de vapor iguala a la presión exterior. El **punto de ebullición normal** corresponde a 1 atm.

> [!NOTE]
> En La Paz (unos 3600 m), la presión es de 0,65 atm y el agua hierve a 88 °C. En una olla a presión, a 2 atm, hierve a 120 °C.

### Ecuación de Clausius-Clapeyron

$$\ln P = -\frac{\Delta H_{\text{vap}}}{R} \cdot \frac{1}{T} + C$$

Graficar $\ln P$ en función de $1/T$ da una recta de pendiente $-\Delta H_{\text{vap}}/R$. Entre dos temperaturas:

$$\ln \frac{P_2}{P_1} = -\frac{\Delta H_{\text{vap}}}{R} \left( \frac{1}{T_2} - \frac{1}{T_1} \right)$$

con $R = 8{,}314$ J/(mol·K) y temperaturas en kelvin.

> [!NOTE]
> **Ejemplo.** El agua hierve a 373 K a 1 atm, con $\Delta H_{\text{vap}} = 40{,}7$ kJ/mol. ¿Cuál es su presión de vapor a 363 K (90 °C)?
> $$\ln \frac{P_2}{1} = -\frac{40\,700}{8{,}314} \left( \frac{1}{363} - \frac{1}{373} \right) = -0{,}3616$$
> $$P_2 = e^{-0{,}3616} = 0{,}70\ \text{atm}$$

## Cambios de fase

| Cambio | De → a | Calor |
|---|---|---|
| Fusión | sólido → líquido | absorbe |
| Vaporización | líquido → gas | absorbe |
| Sublimación | sólido → gas | absorbe |
| Solidificación | líquido → sólido | libera |
| Condensación | gas → líquido | libera |
| Deposición | gas → sólido | libera |

$$\Delta H_{\text{sub}} = \Delta H_{\text{fus}} + \Delta H_{\text{vap}}$$

### Curva de calentamiento

Al calentar a presión constante, la temperatura sube dentro de cada fase y se **mantiene constante** durante cada cambio de fase: el calor se usa para vencer las fuerzas intermoleculares.

| Tramo | Calor |
|---|---|
| Calentar una fase | $q = m\, c\, \Delta T$ |
| Cambiar de fase | $q = n\, \Delta H$ |

> [!NOTE]
> **Ejemplo.** Calor para llevar 18,0 g de hielo a −10 °C hasta vapor a 100 °C.
>
> Datos: $c_{\text{hielo}} = 2{,}09$ y $c_{\text{agua}} = 4{,}18$ J/(g·°C); $\Delta H_{\text{fus}} = 6{,}01$ y $\Delta H_{\text{vap}} = 40{,}7$ kJ/mol.
>
> | Etapa | Cálculo | Calor (kJ) |
> |---|---|---|
> | Hielo de −10 a 0 °C | $18{,}0 \times 2{,}09 \times 10$ | 0,38 |
> | Fusión | $1 \times 6{,}01$ | 6,01 |
> | Agua de 0 a 100 °C | $18{,}0 \times 4{,}18 \times 100$ | 7,52 |
> | Vaporización | $1 \times 40{,}7$ | 40,7 |
> | **Total** | | **54,6** |
>
> La vaporización se lleva tres cuartas partes.

## Diagramas de fases

Muestran qué fase es estable a cada presión y temperatura.

| Elemento | Significado |
|---|---|
| Curvas | condiciones en que dos fases coexisten en equilibrio |
| **Punto triple** | coexisten las tres fases |
| **Punto crítico** | por encima, líquido y gas son indistinguibles: **fluido supercrítico** |

| Sustancia | Punto triple | Punto crítico |
|---|---|---|
| Agua | 0,01 °C; 0,006 atm | 374 °C; 218 atm |
| $\mathrm{CO_2}$ | −56,6 °C; 5,11 atm | 31,1 °C; 72,9 atm |

- El **$\mathrm{CO_2}$** tiene su punto triple por encima de 1 atm: a presión atmosférica no puede ser líquido, y el hielo seco **sublima**.
- En el **agua**, la curva sólido-líquido tiene pendiente **negativa**: aumentar la presión funde el hielo, porque el líquido es más denso. En casi todas las demás sustancias la pendiente es positiva.

El $\mathrm{CO_2}$ supercrítico se usa como solvente para descafeinar el café.

## Tipos de sólidos

Un sólido **cristalino** tiene sus partículas ordenadas en una red que se repite; funde a una temperatura definida. Uno **amorfo** (vidrio, muchos plásticos) carece de orden de largo alcance y se ablanda gradualmente.

| Sólido | Partículas | Unión | Propiedades | Ejemplos |
|---|---|---|---|---|
| **Iónico** | cationes y aniones | atracción electrostática | duro, frágil, alto punto de fusión; conduce fundido o disuelto | NaCl, CaO |
| **Molecular** | moléculas | fuerzas intermoleculares | blando, bajo punto de fusión, no conduce | hielo, $\mathrm{CO_2}$, azúcar |
| **Covalente** (de red) | átomos | enlaces covalentes en toda la red | muy duro, punto de fusión muy alto | diamante, $\mathrm{SiO_2}$, SiC |
| **Metálico** | cationes en un mar de electrones | enlace metálico | conduce, maleable, dúctil, brillo | Fe, Cu, Na |

El **diamante** y el **grafito** son ambos carbono (alótropos). En el diamante cada átomo se une a otros cuatro en una red tridimensional; en el grafito, a tres, formando láminas que se deslizan y con electrones deslocalizados que conducen.

## Celdas unitarias

La **celda unitaria** es la porción más pequeña que, repetida, reproduce el cristal. Las tres celdas cúbicas:

| Celda | Átomos por celda | Número de coordinación | Relación arista–radio | Empaquetamiento | Ejemplos |
|---|---|---|---|---|---|
| Cúbica simple | 1 | 6 | $a = 2r$ | 52 % | Po |
| Cúbica centrada en el cuerpo | 2 | 8 | $a = \dfrac{4r}{\sqrt{3}}$ | 68 % | Fe, Na, W |
| Cúbica centrada en las caras | 4 | 12 | $a = 2\sqrt{2}\, r$ | 74 % | Cu, Al, Au, Ag |

Para contar los átomos de una celda:

| Posición | Fracción dentro de la celda |
|---|---|
| Vértice | 1/8 |
| Arista | 1/4 |
| Cara | 1/2 |
| Interior | 1 |

Centrada en las caras: $8 \times \tfrac{1}{8} + 6 \times \tfrac{1}{2} = 4$ átomos.

### Densidad

$$\rho = \frac{Z \, M}{N_A \, a^3}$$

con $Z$ los átomos por celda, $M$ la masa molar, $N_A$ el número de Avogadro y $a$ la arista.

> [!NOTE]
> **Ejemplo.** El cobre cristaliza en una red cúbica centrada en las caras con $a = 361$ pm $= 3{,}61 \times 10^{-8}$ cm, y $M = 63{,}55$ g/mol.
> $$\rho = \frac{4 \times 63{,}55}{6{,}022 \times 10^{23} \times (3{,}61 \times 10^{-8})^3} = 8{,}97\ \text{g/cm}^3$$
> El valor medido es 8,96 g/cm³. El radio atómico resulta $r = a / (2\sqrt{2}) = 128$ pm.

### Cristales iónicos

| Estructura | Disposición | Coordinación |
|---|---|---|
| Cloruro de sodio | aniones en red centrada en las caras; cationes en los huecos octaédricos | 6 : 6 |
| Cloruro de cesio | anión en los vértices; catión en el centro | 8 : 8 |
| Blenda de cinc (ZnS) | cationes en la mitad de los huecos tetraédricos | 4 : 4 |

La estructura depende de la relación entre los radios del catión y del anión.

### Difracción de rayos X

La estructura de los cristales se determina con rayos X, cuya longitud de onda es comparable a la distancia entre átomos. Los planos de átomos reflejan con interferencia constructiva cuando se cumple la **ley de Bragg**:

$$n \lambda = 2 d \sin\theta$$

con $d$ la distancia entre planos y $\theta$ el ángulo de incidencia.

## Sólidos especiales

| Material | Característica |
|---|---|
| **Semiconductores** | conductividad intermedia, que aumenta con la temperatura y el dopado |
| **Cristales líquidos** | fluyen como líquidos, pero con moléculas orientadas: pantallas |
| **Vidrios** | sólidos amorfos; líquidos sobreenfriados |
| **Aleaciones** | sustitucionales (latón) o intersticiales (acero) |
| **Nanomateriales** | grafeno, nanotubos, fullerenos: propiedades que dependen del tamaño |

> [!TIP]
> Para predecir propiedades, identificar primero el **tipo de sólido**: qué partículas ocupan la red y qué las une. Eso basta para ordenar puntos de fusión, durezas y conductividades.
