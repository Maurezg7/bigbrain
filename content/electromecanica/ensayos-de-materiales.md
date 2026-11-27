+++
title = "Ensayos de materiales"
summary = "Ensayo de tracción y curva tensión-deformación, dureza Brinell, Rockwell y Vickers, impacto Charpy, fatiga, termofluencia y ensayos no destructivos."
branch = "Materiales"
order = 3
level = "Intermedio"
tags = ["ensayo de tracción", "dureza", "Charpy", "fatiga", "ensayos no destructivos"]
+++

## Para qué se ensaya

Las propiedades mecánicas de un material no se calculan: **se miden**. Los ensayos, hechos según normas (ISO, ASTM, IRAM), permiten

- obtener los datos para el diseño,
- controlar la calidad de un lote,
- analizar por qué falló una pieza.

| Tipo | La pieza | Ejemplos |
|---|---|---|
| Destructivo | queda inutilizada | tracción, dureza, impacto, fatiga |
| No destructivo | sigue en servicio | ultrasonido, radiografía, líquidos penetrantes |

## Ensayo de tracción

Se estira una **probeta** normalizada a velocidad constante hasta romperla, registrando la fuerza $F$ y el alargamiento $\Delta L$.

$$\sigma = \frac{F}{A_0} \qquad \varepsilon = \frac{\Delta L}{L_0}$$

Son la tensión y la deformación **de ingeniería**, referidas a la sección $A_0$ y la longitud $L_0$ iniciales.

### La curva tensión-deformación

Para un acero dúctil:

| Zona | Qué ocurre |
|---|---|
| Elástica | la deformación es proporcional a la tensión y se recupera al descargar |
| Fluencia | el material se deforma plásticamente sin que aumente la tensión |
| Endurecimiento | hace falta más tensión para seguir deformando |
| Estricción | la sección se reduce localmente; la fuerza baja |
| Rotura | |

### Propiedades que se obtienen

| Propiedad | Símbolo | Definición |
|---|---|---|
| Módulo de elasticidad | $E$ | pendiente de la zona elástica: $\sigma = E \varepsilon$ |
| Límite de fluencia | $\sigma_y$, $R_e$ | tensión donde comienza la deformación plástica |
| Límite convencional | $R_{p0,2}$ | tensión que deja 0,2 % de deformación permanente |
| Resistencia a la tracción | $\sigma_u$, $R_m$ | tensión máxima de la curva |
| Alargamiento de rotura | $A$ | $\dfrac{L_f - L_0}{L_0} \times 100$ |
| Estricción | $Z$ | $\dfrac{A_0 - A_f}{A_0} \times 100$ |

Los materiales sin fluencia marcada (aluminio, aceros templados) usan el límite convencional $R_{p0,2}$: se traza una paralela a la recta elástica desde $\varepsilon = 0{,}002$.

> [!NOTE]
> **Ejemplo.** Probeta de 10 mm de diámetro y $L_0 = 50$ mm. Fluye con 27,5 kN, soporta un máximo de 39,3 kN y rompe con $L_f = 62$ mm.
>
> $A_0 = \pi (5)^2 = 78{,}5\ \text{mm}^2$
>
> $\sigma_y = 27\,500 / 78{,}5 = 350$ MPa
>
> $\sigma_u = 39\,300 / 78{,}5 = 500$ MPa
>
> $A = (62 - 50)/50 = 24\ \%$

| Propiedad | Se mide por |
|---|---|
| **Ductilidad** | el alargamiento y la estricción |
| **Resiliencia** | el área bajo la zona elástica: $u_r = \sigma_y^2 / (2E)$ |
| **Tenacidad** | el área bajo toda la curva |

### Tensión y deformación verdaderas

Referidas a la sección instantánea:

$$\sigma_v = \sigma (1 + \varepsilon) \qquad \varepsilon_v = \ln(1 + \varepsilon)$$

Valen hasta el comienzo de la estricción. La curva verdadera sigue creciendo hasta la rotura; en la zona plástica suele ajustarse con $\sigma_v = K \varepsilon_v^n$, donde $n$ es el exponente de endurecimiento.

### Valores típicos

| Material | $E$ (GPa) | $\sigma_y$ (MPa) | $\sigma_u$ (MPa) | $A$ (%) |
|---|---|---|---|---|
| Acero de bajo carbono | 207 | 250 | 400 | 25 |
| Acero aleado templado y revenido | 207 | 900 | 1050 | 12 |
| Acero inoxidable austenítico | 193 | 210 | 520 | 40 |
| Aluminio 6061-T6 | 69 | 275 | 310 | 12 |
| Cobre recocido | 117 | 70 | 220 | 45 |
| Fundición gris | 100 | — | 200 | menos de 1 |

## Dureza

Es la resistencia a la **penetración** localizada. El ensayo es rápido, barato y casi no daña la pieza.

| Ensayo | Penetrador | Se mide | Uso |
|---|---|---|---|
| **Brinell** (HB) | bola de carburo de 10 mm | diámetro de la huella | fundiciones, aceros blandos, no ferrosos |
| **Rockwell** (HRC, HRB) | cono de diamante (C) o bola (B) | profundidad de la huella | control en producción; aceros templados (C) |
| **Vickers** (HV) | pirámide de diamante de 136° | diagonales de la huella | toda la gama; capas delgadas |
| Knoop | pirámide alargada | diagonal mayor | microdureza, materiales frágiles |
| Shore | punzón con resorte | profundidad | elastómeros y plásticos |

$$HB = \frac{2F}{\pi D \left( D - \sqrt{D^2 - d^2} \right)} \qquad HV = 1{,}854\, \frac{F}{d^2}$$

con $F$ en kgf, y los diámetros o la diagonal en mm.

> [!TIP]
> Para aceros al carbono y de baja aleación, la resistencia a la tracción se estima con $\sigma_u \approx 3{,}45 \times HB$ (en MPa). Un acero de 200 HB resiste unos 690 MPa.

## Ensayo de impacto

Mide la energía que absorbe una probeta **entallada** al romperse de un golpe. En el ensayo **Charpy**, un péndulo cae desde una altura $h_0$, rompe la probeta y sube hasta $h_1$:

$$E = m g (h_0 - h_1)$$

### Transición dúctil-frágil

Los metales de estructura cúbica centrada en el cuerpo (aceros ferríticos) se vuelven **frágiles** por debajo de cierta temperatura.

| Temperatura | Comportamiento | Energía absorbida |
|---|---|---|
| Alta | dúctil; fractura fibrosa | alta |
| De transición | mixto | cae bruscamente |
| Baja | frágil; fractura cristalina y brillante | baja |

Los metales cúbicos centrados en las caras (aluminio, cobre, acero inoxidable austenítico) no presentan transición: sirven para servicio criogénico.

> [!WARNING]
> Un acero puede ser dúctil en el ensayo de tracción y romperse de forma frágil en servicio si se combinan **baja temperatura**, **entalla** y **carga de impacto**. Así fallaron los buques Liberty en la Segunda Guerra Mundial.

## Fatiga

Bajo cargas **variables y repetidas**, un material rompe con tensiones muy inferiores a su resistencia estática. Es la causa de la mayoría de las fallas mecánicas.

El ensayo somete probetas a ciclos de tensión de amplitud $\sigma_a$ y cuenta los ciclos $N$ hasta la rotura. El resultado es la **curva S-N** (de Wöhler).

| Material | Comportamiento |
|---|---|
| Aceros | la curva se hace horizontal: existe un **límite de fatiga** $\sigma_e$, por debajo del cual la vida es infinita |
| Aluminio y no ferrosos | no hay límite: se define la resistencia para una vida dada ($5 \times 10^8$ ciclos) |

Para aceros, $\sigma_e \approx 0{,}5\, \sigma_u$ en probeta pulida (hasta unos 700 MPa).

La fractura por fatiga muestra una zona lisa con **marcas de playa** (el avance lento de la fisura) y una zona rugosa (la rotura final brusca).

| Reduce la vida a fatiga | La aumenta |
|---|---|
| entallas y cambios bruscos de sección | radios de acuerdo generosos |
| superficie rugosa | pulido |
| tensiones residuales de tracción | granallado, que deja compresión superficial |
| corrosión | tratamientos superficiales |

## Termofluencia

A temperaturas superiores a $0{,}4\, T_f$ (con $T_f$ la temperatura absoluta de fusión), un material bajo carga **constante** se deforma progresivamente con el tiempo (*creep*).

| Etapa | Velocidad de deformación |
|---|---|
| Primaria | decreciente |
| Secundaria | constante y mínima; es el dato de diseño |
| Terciaria | creciente, hasta la rotura |

Es crítica en álabes de turbinas, tuberías de vapor y calderas.

## Mecánica de fractura

Toda pieza real contiene fisuras. El **factor de intensidad de tensiones** describe el estado en la punta de una fisura de longitud $a$:

$$K_I = Y \sigma \sqrt{\pi a}$$

$Y$ es un factor geométrico. La fisura se propaga de modo inestable cuando $K_I$ alcanza la **tenacidad a la fractura** $K_{Ic}$, una propiedad del material.

> [!NOTE]
> **Ejemplo.** Una placa de acero con $K_{Ic} = 50\ \text{MPa}\sqrt{\text{m}}$ trabaja a 300 MPa. Con $Y = 1$, la fisura crítica es
> $$a_c = \frac{1}{\pi} \left( \frac{K_{Ic}}{\sigma} \right)^2 = \frac{1}{\pi} \left( \frac{50}{300} \right)^2 = 8{,}8\ \text{mm}$$

## Ensayos no destructivos

| Método | Principio | Detecta | Limitación |
|---|---|---|---|
| Inspección visual | observación directa, con lupa o endoscopio | defectos superficiales visibles | depende del inspector |
| **Líquidos penetrantes** | un líquido coloreado entra en las fisuras por capilaridad | fisuras abiertas a la superficie | solo superficiales |
| **Partículas magnéticas** | las fisuras desvían el campo y atraen las partículas | fisuras superficiales y cercanas a la superficie | solo materiales ferromagnéticos |
| **Ultrasonido** | los defectos reflejan las ondas | defectos internos; mide espesores | requiere un operador entrenado |
| **Radiografía** (rayos X o gamma) | los defectos absorben distinto la radiación | poros, inclusiones, falta de fusión | protección radiológica; costosa |
| Corrientes inducidas | los defectos alteran las corrientes parásitas | fisuras superficiales; mide espesores de recubrimiento | solo conductores |
| Termografía | diferencias de temperatura superficial | puntos calientes en tableros, fallas de aislación | |

> [!TIP]
> En soldaduras, lo habitual es combinar métodos: inspección visual de todo, líquidos penetrantes o partículas magnéticas para la superficie, y ultrasonido o radiografía para el interior de las juntas críticas.

## Otros ensayos

| Ensayo | Mide |
|---|---|
| Compresión | resistencia de materiales frágiles (hormigón, fundición) |
| Flexión | módulo de rotura de cerámicos; ductilidad de soldaduras (plegado) |
| Torsión | módulo de corte $G$ y resistencia al corte |
| Metalografía | la microestructura, con microscopio, tras pulir y atacar |
| Espectrometría | la composición química |
