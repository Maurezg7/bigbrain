+++
title = "Electroquímica"
summary = "Celdas galvánicas, potenciales estándar de reducción, ecuación de Nernst, pilas y baterías, corrosión y electrólisis."
branch = "Equilibrio químico"
order = 3
level = "Avanzado"
tags = ["electroquímica", "pila", "potencial de reducción", "Nernst", "electrólisis", "corrosión"]
+++

## Electricidad y reacciones químicas

La electroquímica estudia la conversión entre energía química y eléctrica mediante reacciones **redox**.

| | Celda galvánica (pila) | Celda electrolítica |
|---|---|---|
| Reacción | espontánea | no espontánea |
| Energía | química → eléctrica | eléctrica → química |
| $\Delta G$ | negativo | positivo |
| Ejemplo | batería | cromado, obtención de aluminio |

> [!IMPORTANT]
> En **toda** celda: en el **ánodo** ocurre la **oxidación** y en el **cátodo** la **reducción**.

## Celdas galvánicas

Si las dos semirreacciones se separan en recipientes distintos, los electrones deben pasar por un cable externo y producen corriente.

### Pila de Daniell

| Electrodo | Semirreacción | Polo |
|---|---|---|
| Ánodo (cinc) | Zn(s) → Zn²⁺(ac) + 2 e⁻ | negativo |
| Cátodo (cobre) | Cu²⁺(ac) + 2 e⁻ → Cu(s) | positivo |

- Los **electrones** circulan por el cable del ánodo al cátodo.
- El **puente salino** cierra el circuito: sus iones mantienen neutras las soluciones.
- El electrodo de cinc se disuelve; sobre el de cobre se deposita metal.

**Notación de celda**: Zn(s) | Zn²⁺(ac) || Cu²⁺(ac) | Cu(s). El ánodo se escribe a la izquierda; la doble barra es el puente salino.

## Potenciales estándar

El **potencial estándar de reducción** $E^0$ mide la tendencia de una especie a reducirse, en condiciones estándar (1 M, 1 bar, 25 °C). Se toma como referencia el **electrodo estándar de hidrógeno**, con $E^0 = 0$ V.

| Semirreacción de reducción | $E^0$ (V) |
|---|---|
| F₂ + 2 e⁻ → 2 F⁻ | +2,87 |
| MnO₄⁻ + 8 H⁺ + 5 e⁻ → Mn²⁺ + 4 H₂O | +1,51 |
| Cl₂ + 2 e⁻ → 2 Cl⁻ | +1,36 |
| O₂ + 4 H⁺ + 4 e⁻ → 2 H₂O | +1,23 |
| Ag⁺ + e⁻ → Ag | +0,80 |
| Cu²⁺ + 2 e⁻ → Cu | +0,34 |
| 2 H⁺ + 2 e⁻ → H₂ | 0,00 |
| Fe²⁺ + 2 e⁻ → Fe | −0,44 |
| Zn²⁺ + 2 e⁻ → Zn | −0,76 |
| Al³⁺ + 3 e⁻ → Al | −1,66 |
| Li⁺ + e⁻ → Li | −3,04 |

- Cuanto **mayor** $E^0$, más fuerte es el **oxidante** (flúor).
- Cuanto **menor** $E^0$, más fuerte es el **reductor** (litio).

### Potencial de celda

> [!IMPORTANT]
> $$E^0_{celda} = E^0_{cátodo} - E^0_{ánodo}$$
> usando ambos potenciales de **reducción**. Si $E^0_{celda} > 0$, la reacción es espontánea.

> [!NOTE]
> Pila de Daniell: $E^0 = 0{,}34 - (-0{,}76) = 1{,}10$ V.

> [!NOTE]
> ¿Puede el cobre reducir al Ag⁺? Cátodo: plata ($+0{,}80$). Ánodo: cobre ($+0{,}34$).
>
> $E^0 = 0{,}80 - 0{,}34 = +0{,}46$ V. Sí: Cu + 2 Ag⁺ → Cu²⁺ + 2 Ag.

> [!WARNING]
> Los potenciales **no** se multiplican por los coeficientes al balancear: son propiedades intensivas.

## Potencial, energía libre y equilibrio

$$\Delta G^0 = -nFE^0_{celda} \qquad E^0_{celda} = \frac{RT}{nF}\ln K$$

con $n$ los moles de electrones transferidos y $F = 96\,485$ C/mol (**constante de Faraday**).

| $E^0_{celda}$ | $\Delta G^0$ | $K$ | Reacción |
|---|---|---|---|
| positivo | negativo | > 1 | espontánea |
| cero | cero | 1 | en equilibrio |
| negativo | positivo | < 1 | no espontánea |

> [!NOTE]
> Pila de Daniell: $\Delta G^0 = -2 \cdot 96\,485 \cdot 1{,}10 = -212$ kJ.

## Ecuación de Nernst

Para concentraciones no estándar, a 25 °C:

> [!IMPORTANT]
> $$E = E^0 - \frac{0{,}0592}{n}\log Q$$

A medida que la pila funciona, $Q$ crece y $E$ disminuye. Cuando $E = 0$, se alcanzó el equilibrio: la pila está agotada.

> [!NOTE]
> Pila de Daniell con $[\text{Zn}^{2+}] = 0{,}010$ M y $[\text{Cu}^{2+}] = 1{,}0$ M.
>
> $Q = \dfrac{0{,}010}{1{,}0} = 0{,}010$. $E = 1{,}10 - \dfrac{0{,}0592}{2}\log(0{,}010) = 1{,}16$ V.

Una **pila de concentración** tiene los mismos electrodos con soluciones de distinta concentración. El medidor de pH se basa en este principio.

## Pilas y baterías

| Tipo | Reacción | Tensión | Recargable |
|---|---|---|---|
| Alcalina | Zn y MnO₂ | 1,5 V | no |
| Plomo-ácido | Pb y PbO₂ en H₂SO₄ | 2 V por celda | sí |
| Ion-litio | litio entre grafito y un óxido metálico | ~3,7 V | sí |
| Celda de combustible | H₂ y O₂ → H₂O | ~1 V (teórico 1,23) | alimentación continua |

Las de ion-litio almacenan mucha energía por unidad de masa porque el litio es liviano y tiene un potencial muy negativo.

## Corrosión

Es la oxidación espontánea de un metal. En el hierro:

- Zona anódica: Fe → Fe²⁺ + 2 e⁻
- Zona catódica: O₂ + 2 H₂O + 4 e⁻ → 4 OH⁻

El producto final es el **herrumbre**, un óxido de hierro(III) hidratado. Necesita agua **y** oxígeno; las sales y los ácidos la aceleran.

| Protección | Cómo funciona |
|---|---|
| Pintura, recubrimientos | aíslan del agua y el oxígeno |
| Galvanizado | capa de cinc, que se oxida primero |
| Ánodo de sacrificio | un metal más activo (Mg, Zn) conectado se corroe en lugar del hierro |
| Aleación (acero inoxidable) | el cromo forma una capa de óxido protectora |
| Protección catódica por corriente | una fuente fuerza al metal a ser cátodo |

El aluminio no se corroe a fondo porque su óxido forma una capa adherente e impermeable (**pasivación**).

## Electrólisis

Una fuente externa fuerza una reacción no espontánea. El ánodo es ahora el polo **positivo** y el cátodo el **negativo**, pero la oxidación sigue ocurriendo en el ánodo.

| Proceso | Productos |
|---|---|
| Electrólisis del agua | H₂ en el cátodo y O₂ en el ánodo |
| NaCl fundido | sodio metálico y cloro |
| Salmuera (NaCl en agua) | hidrógeno, cloro e hidróxido de sodio |
| Proceso Hall-Héroult | aluminio a partir de alúmina fundida |
| Galvanoplastia | se deposita un metal sobre un objeto (cromado, plateado) |
| Refinación del cobre | cobre de alta pureza |

### Leyes de Faraday

La masa depositada es proporcional a la carga que circula:

> [!IMPORTANT]
> $$m = \frac{I\,t\,M}{n\,F}$$

con $I$ la corriente (A), $t$ el tiempo (s), $M$ la masa molar y $n$ los electrones por ion.

> [!NOTE]
> ¿Cuánto cobre se deposita con $2{,}00$ A durante $1{,}00$ h en una solución de Cu²⁺?
>
> Carga: $2{,}00 \cdot 3600 = 7200$ C. Moles de electrones: $\dfrac{7200}{96\,485} = 0{,}0746$.
>
> Moles de cobre: $0{,}0373$. Masa: $0{,}0373 \cdot 63{,}55 = 2{,}37$ g.

> [!TIP]
> Para armar una pila con dos semirreacciones: la de mayor $E^0$ es el cátodo (se reduce) y la otra se invierte y es el ánodo.
