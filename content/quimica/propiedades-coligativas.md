+++
title = "Propiedades coligativas"
summary = "Descenso de la presión de vapor, ascenso ebulloscópico, descenso crioscópico y presión osmótica; factor de van't Hoff y determinación de masas molares."
branch = "Estados de la materia y soluciones"
order = 4
level = "Intermedio"
tags = ["propiedades coligativas", "ley de Raoult", "ósmosis", "crioscopía", "van t Hoff"]
+++

## Qué son

Las **propiedades coligativas** de una solución dependen de la **cantidad** de partículas de soluto disueltas, no de su naturaleza. Son cuatro:

| Propiedad | Efecto del soluto |
|---|---|
| Presión de vapor | disminuye |
| Punto de ebullición | aumenta |
| Punto de congelación | disminuye |
| Presión osmótica | aparece |

Las fórmulas que siguen valen para soluciones **diluidas** de solutos **no volátiles**.

## Descenso de la presión de vapor

La **ley de Raoult** establece que la presión de vapor del solvente sobre la solución es proporcional a su fracción molar:

$$P = x_{\text{solvente}} \, P^{\circ}$$

donde $P^{\circ}$ es la presión de vapor del solvente puro. El descenso es

$$\Delta P = x_{\text{soluto}} \, P^{\circ}$$

Las partículas de soluto ocupan parte de la superficie y reducen la tendencia del solvente a evaporarse.

> [!NOTE]
> **Ejemplo.** Se disuelven 90 g de glucosa ($M = 180$ g/mol) en 450 g de agua a 25 °C ($P^{\circ} = 23{,}8$ mmHg).
>
> $n_{\text{glucosa}} = 0{,}5$ mol; $n_{\text{agua}} = 450/18 = 25$ mol
>
> $x_{\text{soluto}} = 0{,}5 / 25{,}5 = 0{,}0196$
>
> $\Delta P = 0{,}0196 \times 23{,}8 = 0{,}47$ mmHg, y $P = 23{,}3$ mmHg.

## Ascenso ebulloscópico y descenso crioscópico

Como la presión de vapor baja, hace falta más temperatura para que iguale a la presión exterior: el punto de ebullición **sube**. Y el punto de congelación **baja**.

$$\Delta T_e = K_e \, m \qquad \Delta T_c = K_c \, m$$

$m$ es la **molalidad** (moles de soluto por kilogramo de solvente). $K_e$ y $K_c$ son constantes de cada **solvente**.

| Solvente | $T_e$ (°C) | $K_e$ (°C·kg/mol) | $T_c$ (°C) | $K_c$ (°C·kg/mol) |
|---|---|---|---|---|
| Agua | 100,0 | 0,512 | 0,0 | 1,86 |
| Benceno | 80,1 | 2,53 | 5,5 | 5,12 |
| Etanol | 78,4 | 1,22 | −114,6 | 1,99 |
| Ácido acético | 117,9 | 3,07 | 16,6 | 3,90 |
| Alcanfor | 204 | 5,61 | 178 | 39,7 |

> [!NOTE]
> **Ejemplo.** Para la solución anterior: $m = 0{,}5 / 0{,}450 = 1{,}11$ mol/kg.
>
> $\Delta T_e = 0{,}512 \times 1{,}11 = 0{,}57$ °C → hierve a 100,57 °C.
>
> $\Delta T_c = 1{,}86 \times 1{,}11 = 2{,}07$ °C → congela a −2,07 °C.

Se usa la molalidad, y no la molaridad, porque no cambia con la temperatura.

## Presión osmótica

Una **membrana semipermeable** deja pasar el solvente pero no el soluto. La **ósmosis** es el paso neto de solvente desde la solución más diluida hacia la más concentrada.

La **presión osmótica** $\Pi$ es la presión que hay que aplicar sobre la solución para detener ese flujo:

$$\Pi = M R T$$

con $M$ la molaridad, $R = 0{,}0821$ L·atm/(mol·K) y $T$ en kelvin. Tiene la misma forma que la ecuación de los gases ideales.

> [!NOTE]
> **Ejemplo.** Solución 0,10 M de sacarosa a 25 °C:
> $$\Pi = 0{,}10 \times 0{,}0821 \times 298 = 2{,}45\ \text{atm}$$
> Equivale a una columna de agua de unos 25 m: es, con mucho, la propiedad coligativa más sensible.

| Solución, respecto de otra | Presión osmótica | Efecto sobre una célula |
|---|---|---|
| Isotónica | igual | ninguno |
| Hipotónica | menor | la célula se hincha y puede estallar |
| Hipertónica | mayor | la célula pierde agua y se contrae |

El suero fisiológico (NaCl al 0,9 %) es isotónico con la sangre.

En la **ósmosis inversa** se aplica una presión mayor que $\Pi$ y el solvente fluye hacia el lado diluido: así se desaliniza el agua de mar.

## Electrolitos: factor de van't Hoff

Un electrolito se disocia y aporta más partículas. El **factor de van't Hoff** $i$ es el número de partículas por unidad de fórmula disuelta:

$$\Delta T_c = i\, K_c\, m \qquad \Delta T_e = i\, K_e\, m \qquad \Pi = i\, M R T$$

| Soluto | Disociación | $i$ ideal |
|---|---|---|
| Glucosa, sacarosa, urea | no se disocian | 1 |
| NaCl, KBr | 2 iones | 2 |
| $\mathrm{CaCl_2}$, $\mathrm{Na_2SO_4}$ | 3 iones | 3 |
| $\mathrm{AlCl_3}$ | 4 iones | 4 |

El valor real es algo menor que el ideal, porque los iones de carga opuesta se atraen y forman **pares iónicos**; la diferencia crece con la concentración. Para NaCl 0,1 m, $i \approx 1{,}87$.

Para un electrolito débil que se disocia en $\nu$ iones con grado de disociación $\alpha$:

$$i = 1 + \alpha(\nu - 1)$$

> [!NOTE]
> **Ejemplo.** Punto de congelación de una solución 0,50 m de $\mathrm{CaCl_2}$ (con $i = 3$):
> $$\Delta T_c = 3 \times 1{,}86 \times 0{,}50 = 2{,}79\ \text{°C} \quad\Rightarrow\quad T_c = -2{,}79\ \text{°C}$$

## Determinación de masas molares

Midiendo una propiedad coligativa se obtiene la cantidad de moles de soluto y, con la masa, su masa molar.

> [!NOTE]
> **Crioscopía.** 1,20 g de un compuesto disueltos en 50,0 g de benceno bajan el punto de congelación 0,96 °C.
>
> $m = \dfrac{\Delta T_c}{K_c} = \dfrac{0{,}96}{5{,}12} = 0{,}1875$ mol/kg
>
> $n = 0{,}1875 \times 0{,}0500 = 9{,}38 \times 10^{-3}$ mol
>
> $M = \dfrac{1{,}20}{9{,}38 \times 10^{-3}} = 128$ g/mol

> [!NOTE]
> **Osmometría.** 1,00 g de una proteína en 100 mL de solución da $\Pi = 3{,}72 \times 10^{-3}$ atm a 25 °C.
>
> $M_{\text{molar}} = \dfrac{\Pi}{RT} = \dfrac{3{,}72 \times 10^{-3}}{0{,}0821 \times 298} = 1{,}52 \times 10^{-4}$ mol/L
>
> $n = 1{,}52 \times 10^{-5}$ mol, y la masa molar es $1{,}00 / 1{,}52 \times 10^{-5} \approx 66\,000$ g/mol.

La presión osmótica es el método indicado para **macromoléculas**: el descenso crioscópico de esa misma solución sería de apenas 0,0003 °C, imposible de medir.

## Soluciones de dos líquidos volátiles

Si ambos componentes son volátiles y la solución es **ideal**, cada uno cumple la ley de Raoult:

$$P_{\text{total}} = x_A P_A^{\circ} + x_B P_B^{\circ}$$

El vapor resulta **más rico en el componente más volátil** que el líquido. En eso se basa la **destilación fraccionada**.

| Desviación | Causa | Consecuencia |
|---|---|---|
| Positiva | las interacciones A–B son más débiles que A–A y B–B | presión mayor que la ideal; azeótropo de mínimo punto de ebullición (etanol-agua, 95,6 %) |
| Negativa | las interacciones A–B son más fuertes | presión menor; azeótropo de máximo (ácido nítrico-agua) |

Un **azeótropo** hierve a temperatura constante y con composición constante: no puede separarse por destilación simple.

## Aplicaciones

| Aplicación | Propiedad |
|---|---|
| Anticongelante del radiador (etilenglicol) | descenso crioscópico y ascenso ebulloscópico |
| Sal en rutas heladas | descenso crioscópico |
| Conservación de alimentos con sal o azúcar | ósmosis: deshidrata a los microorganismos |
| Sueros intravenosos | isotonicidad |
| Desalinización y purificación de agua | ósmosis inversa |
| Diálisis | membranas semipermeables |
| Ascenso de la savia; turgencia de las plantas | presión osmótica |

> [!TIP]
> En los problemas, contar siempre las **partículas**: 1 mol de NaCl produce el efecto de 2 mol de glucosa. Y verificar qué unidad pide cada fórmula: fracción molar (Raoult), molalidad (ebulloscopía y crioscopía) o molaridad (ósmosis).
