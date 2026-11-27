+++
title = "Física nuclear"
summary = "Estructura del núcleo, energía de enlace, radiactividad, ley de desintegración, fisión, fusión y efectos de la radiación."
branch = "Física moderna"
order = 4
level = "Avanzado"
tags = ["núcleo", "radiactividad", "semivida", "fisión", "fusión"]
+++

## El núcleo

El núcleo concentra casi toda la masa del átomo en una región cien mil veces menor. Está formado por **nucleones**: protones y neutrones.

| Símbolo | Nombre | Significado |
|---|---|---|
| $Z$ | número atómico | cantidad de protones (define el elemento) |
| $N$ | número de neutrones | |
| $A = Z + N$ | número másico | total de nucleones |

Un núclido se escribe $^A_Z\text{X}$, por ejemplo $^{235}_{92}\text{U}$.

Los **isótopos** son átomos del mismo elemento (igual $Z$) con distinta cantidad de neutrones: $^{12}\text{C}$, $^{13}\text{C}$ y $^{14}\text{C}$.

El radio del núcleo es aproximadamente $r = r_0A^{1/3}$, con $r_0 \approx 1{,}2$ fm ($1\ \text{fm} = 10^{-15}$ m). Su densidad es enorme: unos $2{,}3 \times 10^{17}\ \text{kg/m}^3$.

### Fuerza nuclear fuerte

Los protones se repelen eléctricamente. Los mantiene unidos la **fuerza nuclear fuerte**: atractiva, muy intensa, pero de alcance cortísimo (unos pocos fm), y actúa igual entre protones y neutrones.

Los núcleos livianos estables tienen $N \approx Z$; los pesados necesitan más neutrones para compensar la repulsión. Por encima del plomo ($Z = 82$) no hay núcleos estables.

## Energía de enlace

La masa de un núcleo es **menor** que la suma de las masas de sus nucleones. Esa diferencia es el **defecto de masa**:

> [!IMPORTANT]
> $$E_b = \Delta m\,c^2 = \left[Zm_p + Nm_n - m_{núcleo}\right]c^2$$

Es la energía necesaria para separar el núcleo en sus componentes. Se usa la **unidad de masa atómica**: $1\ \text{u} = 931{,}5\ \text{MeV}/c^2$.

> [!NOTE]
> Helio-4 (2 protones y 2 neutrones), usando masas atómicas: $2(1{,}007825) + 2(1{,}008665) - 4{,}002603 = 0{,}030377$ u.
>
> $E_b = 0{,}030377 \cdot 931{,}5 = 28{,}3$ MeV, o $7{,}07$ MeV por nucleón.

La **energía de enlace por nucleón** es máxima cerca del hierro-56 (unos $8{,}8$ MeV). Por eso se libera energía de dos maneras:

- **Fusión**: unir núcleos livianos.
- **Fisión**: partir núcleos pesados.

## Radiactividad

Los núcleos inestables se transforman espontáneamente emitiendo radiación.

| Tipo | Emite | Cambio en el núcleo | Lo detiene |
|---|---|---|---|
| Alfa ($\alpha$) | núcleo de helio-4 | $Z - 2$, $A - 4$ | una hoja de papel |
| Beta menos ($\beta^-$) | electrón y antineutrino | $Z + 1$, $A$ igual | unos mm de aluminio |
| Beta más ($\beta^+$) | positrón y neutrino | $Z - 1$, $A$ igual | unos mm de aluminio |
| Gamma ($\gamma$) | fotón de alta energía | sin cambio | varios cm de plomo |

En toda desintegración se conservan la carga y el número de nucleones.

$$^{238}_{92}\text{U} \to\ ^{234}_{90}\text{Th} +\ ^4_2\text{He} \qquad\qquad ^{14}_{6}\text{C} \to\ ^{14}_{7}\text{N} + e^- + \bar{\nu}$$

### Ley de desintegración

La desintegración es un proceso al azar: no se puede predecir cuándo decaerá un núcleo, pero sí cuántos lo harán de un conjunto grande.

> [!IMPORTANT]
> $$N(t) = N_0\,e^{-\lambda t} \qquad T_{1/2} = \frac{\ln 2}{\lambda}$$

- $\lambda$ es la **constante de desintegración**.
- $T_{1/2}$ es la **semivida** (o período de semidesintegración): el tiempo en que se desintegra la mitad de los núcleos. Tras $n$ semividas queda $\left(\tfrac{1}{2}\right)^n$.
- La **actividad** $A = \lambda N$ se mide en **becquerels** (1 Bq = 1 desintegración por segundo).

| Núclido | Semivida |
|---|---|
| Yodo-131 | 8 días |
| Cobalto-60 | 5,27 años |
| Carbono-14 | 5730 años |
| Uranio-235 | 704 millones de años |
| Uranio-238 | 4470 millones de años |

> [!NOTE]
> Una muestra de yodo-131 tiene una actividad de $800$ Bq. A los $24$ días (3 semividas): $800 \cdot \left(\tfrac{1}{2}\right)^3 = 100$ Bq.

> [!NOTE]
> **Datación con carbono-14**: un hueso tiene el 25 % del $^{14}\text{C}$ de un organismo vivo. Pasaron dos semividas: unos $11\,460$ años.

## Fisión

Un núcleo pesado se parte en dos fragmentos al absorber un neutrón:

$$n +\ ^{235}_{92}\text{U} \to\ ^{141}_{56}\text{Ba} +\ ^{92}_{36}\text{Kr} + 3n + \sim 200\ \text{MeV}$$

Los neutrones liberados pueden provocar nuevas fisiones: una **reacción en cadena**.

- Si es **controlada** (en promedio, un neutrón por fisión causa otra): **reactor nuclear**.
- Si es descontrolada: explosión.

Para que se sostenga hace falta una **masa crítica**. En un reactor, el **moderador** (agua, agua pesada o grafito) frena los neutrones y las **barras de control** absorben los sobrantes.

La fisión de $1$ kg de uranio-235 libera tanta energía como quemar unas $2500$ toneladas de carbón. No emite dióxido de carbono, pero genera residuos radiactivos de larga duración.

## Fusión

Núcleos livianos se unen para formar uno más pesado:

$$^2_1\text{H} +\ ^3_1\text{H} \to\ ^4_2\text{He} + n + 17{,}6\ \text{MeV}$$

Es la fuente de energía del **Sol** y las estrellas. Por unidad de masa libera varias veces más que la fisión, con combustible abundante y pocos residuos.

La dificultad: los núcleos se repelen, así que se necesitan temperaturas de más de cien millones de kelvin y confinar el plasma con campos magnéticos (tokamak) o con láseres. Aún no hay reactores comerciales.

## Radiación y materia viva

La radiación **ionizante** arranca electrones y puede dañar las células y el ADN.

| Magnitud | Unidad | Mide |
|---|---|---|
| Actividad | becquerel (Bq) | desintegraciones por segundo |
| Dosis absorbida | gray (Gy = J/kg) | energía depositada |
| Dosis equivalente | sievert (Sv) | efecto biológico |

| Fuente | Dosis aproximada |
|---|---|
| Radiación natural | 2–3 mSv por año |
| Radiografía de tórax | 0,1 mSv |
| Tomografía | 5–10 mSv |
| Límite laboral | 20 mSv por año |

La protección se basa en tres factores: menos **tiempo**, más **distancia** y **blindaje**.

## Aplicaciones

| Área | Uso |
|---|---|
| Medicina | diagnóstico por imágenes (PET, centellograma), radioterapia |
| Energía | centrales nucleares |
| Industria | medición de espesores, radiografía de soldaduras, esterilización |
| Arqueología y geología | datación |
| Agricultura | trazadores, conservación de alimentos |

## Partículas elementales

Protones y neutrones no son elementales: están hechos de **quarks**. El **modelo estándar** describe la materia con quarks y leptones (como el electrón y los neutrinos), y las interacciones mediante partículas mediadoras: el fotón (electromagnética), los gluones (fuerte) y los bosones W y Z (débil).

> [!WARNING]
> «Radiactivo» no equivale a «peligroso» sin más: importan el tipo de radiación, la dosis y el tiempo. La irradiación de un objeto, además, no lo vuelve radiactivo.

> [!TIP]
> En toda reacción nuclear, verificar que se conserven la suma de los números másicos y la de los atómicos.
