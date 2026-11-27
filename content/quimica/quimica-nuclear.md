+++
title = "Química nuclear"
summary = "Estabilidad nuclear, tipos de desintegración, ecuaciones nucleares, semivida, fisión, fusión y aplicaciones de los radioisótopos."
branch = "Estructura de la materia"
order = 3
level = "Intermedio"
tags = ["radiactividad", "isótopos", "semivida", "fisión", "fusión"]
+++

## Reacciones nucleares y químicas

| | Reacción química | Reacción nuclear |
|---|---|---|
| Qué cambia | los electrones de valencia | el núcleo |
| Los elementos | se conservan | pueden transformarse en otros |
| Energía | ~ cientos de kJ/mol | ~ $10^{8}$ a $10^{9}$ kJ/mol |
| Influye la temperatura o la presión | sí | prácticamente no |

## Núclidos e isótopos

Un núclido se representa $^A_Z\text{X}$: $A$ es el **número másico** (protones + neutrones) y $Z$ el **número atómico** (protones).

Los **isótopos** tienen igual $Z$ y distinto $A$. Tienen las mismas propiedades químicas, pero distinta masa y distinta estabilidad nuclear.

| Isótopo | Protones | Neutrones | Estabilidad |
|---|---|---|---|
| Carbono-12 | 6 | 6 | estable |
| Carbono-13 | 6 | 7 | estable |
| Carbono-14 | 6 | 8 | radiactivo |

### Estabilidad

- Los núcleos livianos estables tienen cerca de un neutrón por protón; los pesados, hasta 1,5.
- Todos los núclidos con $Z > 83$ son radiactivos.
- Son especialmente estables los núcleos con cantidades pares de protones y de neutrones.

## Tipos de desintegración

| Tipo | Partícula emitida | Efecto en el núcleo | Poder de penetración |
|---|---|---|---|
| Alfa | $^4_2\text{He}$ | $Z - 2$, $A - 4$ | bajo (lo frena el papel) |
| Beta | $^{\ 0}_{-1}e$ | $Z + 1$ | medio (aluminio) |
| Positrón | $^{\ 0}_{+1}e$ | $Z - 1$ | medio |
| Captura electrónica | absorbe un electrón | $Z - 1$ | — |
| Gamma | fotón | sin cambio | alto (plomo, hormigón) |

En la emisión beta, un neutrón se convierte en protón; en la de positrón, un protón en neutrón.

## Ecuaciones nucleares

> [!IMPORTANT]
> En una ecuación nuclear se conservan la **suma de los números másicos** y la **suma de los números atómicos**.

> [!NOTE]
> Desintegración alfa del radio-226:
>
> $$^{226}_{\ 88}\text{Ra} \to\ ^{222}_{\ 86}\text{Rn} +\ ^4_2\text{He}$$
>
> Másicos: $226 = 222 + 4$. Atómicos: $88 = 86 + 2$.

> [!NOTE]
> Desintegración beta del yodo-131:
>
> $$^{131}_{\ 53}\text{I} \to\ ^{131}_{\ 54}\text{Xe} +\ ^{\ 0}_{-1}e$$

> [!NOTE]
> Completar: $^{238}_{\ 92}\text{U} \to\ ? +\ ^4_2\text{He}$.
>
> $A = 238 - 4 = 234$; $Z = 92 - 2 = 90$, que es torio: $^{234}_{\ 90}\text{Th}$.

Un núclido pesado decae en varios pasos (**serie radiactiva**) hasta llegar a un isótopo estable del plomo.

## Cinética de la desintegración

Es un proceso de **primer orden**: la velocidad es proporcional a la cantidad de núcleos presentes.

> [!IMPORTANT]
> $$N = N_0\,e^{-kt} \qquad \ln\frac{N_0}{N} = kt \qquad t_{1/2} = \frac{0{,}693}{k}$$

La **semivida** $t_{1/2}$ es el tiempo en que se desintegra la mitad de la muestra. Es constante y propia de cada núclido.

| Semividas transcurridas | Fracción que queda |
|---|---|
| 1 | 1/2 |
| 2 | 1/4 |
| 3 | 1/8 |
| 4 | 1/16 |
| 10 | ~1/1000 |

> [!NOTE]
> El tecnecio-99m, usado en diagnóstico, tiene $t_{1/2} = 6$ h. De una dosis de $80$ mg, a las $24$ h (4 semividas) quedan $80 \cdot \tfrac{1}{16} = 5$ mg.

> [!NOTE]
> Una muestra de madera tiene el 60 % de la actividad de carbono-14 de la madera viva ($t_{1/2} = 5730$ años).
>
> $k = \dfrac{0{,}693}{5730} = 1{,}21 \times 10^{-4}\ \text{año}^{-1}$; $t = \dfrac{\ln(1/0{,}60)}{k} = 4220$ años.

### Datación

| Método | Semivida | Sirve para |
|---|---|---|
| Carbono-14 | 5730 años | restos orgánicos de hasta ~50 000 años |
| Potasio-40 / argón-40 | 1250 millones de años | rocas volcánicas |
| Uranio-238 / plomo-206 | 4470 millones de años | rocas antiguas, edad de la Tierra |

## Transmutación

Bombardeando núcleos con partículas se obtienen otros elementos. Así se fabrican los radioisótopos de uso médico y los elementos **transuránicos** ($Z > 92$), que no existen en la naturaleza.

$$^{14}_{\ 7}\text{N} +\ ^4_2\text{He} \to\ ^{17}_{\ 8}\text{O} +\ ^1_1\text{H}$$

## Energía nuclear

La masa de un núcleo es menor que la de sus nucleones separados. Ese **defecto de masa** equivale a la energía de enlace:

$$\Delta E = \Delta m\,c^2$$

### Fisión

Un núcleo pesado se divide en dos medianos al absorber un neutrón, y libera más neutrones:

$$^1_0n +\ ^{235}_{\ 92}\text{U} \to\ ^{141}_{\ 56}\text{Ba} +\ ^{92}_{36}\text{Kr} + 3\,^1_0n$$

Esos neutrones sostienen una **reacción en cadena**. En un reactor se la controla con barras que absorben neutrones; el calor produce vapor que mueve una turbina.

### Fusión

Núcleos livianos se unen:

$$^2_1\text{H} +\ ^3_1\text{H} \to\ ^4_2\text{He} +\ ^1_0n$$

Es la fuente de energía de las estrellas. Requiere temperaturas de millones de grados y todavía no se logró aprovecharla comercialmente.

| | Fisión | Fusión |
|---|---|---|
| Combustible | uranio, plutonio | isótopos del hidrógeno |
| Residuos | radiactivos de larga duración | pocos |
| Estado | en uso comercial | experimental |

## Efectos y unidades

La radiación ionizante rompe enlaces y forma radicales libres en los tejidos.

| Magnitud | Unidad |
|---|---|
| Actividad | becquerel (Bq), curie (Ci) |
| Dosis absorbida | gray (Gy) |
| Dosis equivalente | sievert (Sv) |

## Aplicaciones

| Radioisótopo | Uso |
|---|---|
| Tecnecio-99m | diagnóstico por imágenes |
| Yodo-131 | tratamiento de tiroides |
| Cobalto-60 | radioterapia, esterilización |
| Flúor-18 | tomografía PET |
| Americio-241 | detectores de humo |
| Carbono-14 | datación, trazador en reacciones |

Los **trazadores** permiten seguir el camino de un átomo en una reacción o en un organismo.

> [!WARNING]
> La semivida no depende de la temperatura, la presión ni del compuesto en que esté el átomo. No hay forma química de «apagar» la radiactividad.

> [!TIP]
> Para balancear una ecuación nuclear, sumar arriba (másicos) y abajo (atómicos) a cada lado. El $Z$ que falta indica el elemento en la tabla periódica.
