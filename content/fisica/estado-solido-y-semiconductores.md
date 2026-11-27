+++
title = "Estado sólido y semiconductores"
summary = "Redes cristalinas, electrones libres, bandas de energía, conductores, aislantes y semiconductores, dopado, unión p-n, dispositivos y superconductividad."
branch = "Física moderna"
order = 6
level = "Avanzado"
tags = ["bandas de energía", "semiconductores", "dopado", "unión p-n", "superconductividad", "nivel de Fermi"]
+++

## Sólidos cristalinos

En un **cristal**, los átomos se ordenan en una **red** periódica. Las propiedades eléctricas, térmicas y ópticas de los sólidos surgen del comportamiento cuántico de los electrones en esa red.

| Enlace | Electrones | Ejemplos | Conducción |
|---|---|---|---|
| Iónico | transferidos | NaCl | aislante |
| Covalente | compartidos y localizados | diamante, silicio | aislante o semiconductor |
| Metálico | compartidos por toda la red | cobre, hierro | conductor |
| Molecular | cada molécula conserva los suyos | hielo, argón sólido | aislante |

## Electrones libres en un metal

### Modelo clásico de Drude

Trata a los electrones de conducción como un gas que choca con los iones. Explica la ley de Ohm:

$$\sigma = \frac{n e^2 \tau}{m}$$

con $n$ la densidad de electrones y $\tau$ el tiempo medio entre choques. Pero predice mal el calor específico: según él, los electrones deberían aportar tanto como los iones, y casi no aportan.

### Modelo cuántico: gas de Fermi

Los electrones son **fermiones**: por el principio de exclusión no pueden estar todos en el nivel más bajo, y llenan los estados de menor a mayor energía. A temperatura cero, el nivel más alto ocupado es la **energía de Fermi**:

$$E_F = \frac{\hbar^2}{2m} (3\pi^2 n)^{2/3}$$

> [!NOTE]
> **Ejemplo.** El cobre tiene $n = 8{,}47 \times 10^{28}$ electrones por m³.
> $$E_F = \frac{(1{,}055 \times 10^{-34})^2}{2 \times 9{,}11 \times 10^{-31}} \left( 3\pi^2 \times 8{,}47 \times 10^{28} \right)^{2/3} = 1{,}13 \times 10^{-18}\ \text{J} = 7{,}0\ \text{eV}$$
> La **temperatura de Fermi** equivalente es $T_F = E_F / k_B \approx 82\,000$ K, y la velocidad de Fermi, $1{,}6 \times 10^6$ m/s: aun a 0 K los electrones se mueven rapidísimo.

A temperatura $T$, la probabilidad de ocupación de un estado de energía $E$ es la **distribución de Fermi-Dirac**:

$$f(E) = \frac{1}{e^{(E - E_F)/k_B T} + 1}$$

| Energía | Ocupación |
|---|---|
| muy por debajo de $E_F$ | 1 |
| $E = E_F$ | ½ |
| muy por encima de $E_F$ | 0 |

La transición ocurre en una franja de ancho del orden de $k_B T$, que a temperatura ambiente vale 0,025 eV: muy poco frente a $E_F$. Solo los electrones cercanos al nivel de Fermi pueden excitarse, y por eso contribuyen tan poco al calor específico.

## Bandas de energía

Cuando $N$ átomos se acercan para formar un sólido, cada nivel atómico se desdobla en $N$ niveles muy próximos, que forman una **banda** prácticamente continua. Entre las bandas quedan intervalos sin estados: las **bandas prohibidas** (*gaps*).

De otro modo: un electrón en un potencial **periódico** solo puede tener ciertas energías (teorema de Bloch).

| Banda | Definición |
|---|---|
| **De valencia** | la más alta que está ocupada a 0 K |
| **De conducción** | la siguiente, donde los electrones pueden moverse |
| **Prohibida** | la separación entre ambas, $E_g$ |

Una banda **completamente llena** no conduce: no hay estados vacíos cercanos a los que puedan pasar los electrones al aplicar un campo.

| Material | Estructura de bandas | $E_g$ | Resistividad (Ω·m) |
|---|---|---|---|
| **Conductor** | banda parcialmente llena, o bandas solapadas | 0 | $10^{-8}$ |
| **Semiconductor** | banda de valencia llena; *gap* pequeño | 0,1 a 3 eV | $10^{-4}$ a $10^{4}$ |
| **Aislante** | banda de valencia llena; *gap* grande | más de 4 eV | más de $10^{10}$ |

| Material | $E_g$ (eV) |
|---|---|
| Germanio | 0,67 |
| **Silicio** | 1,12 |
| Arseniuro de galio | 1,42 |
| Nitruro de galio | 3,4 |
| Diamante | 5,5 |

| Al aumentar la temperatura | La resistividad | Porque |
|---|---|---|
| Metal | **aumenta** | los iones vibran más y dispersan más a los electrones |
| Semiconductor | **disminuye** | más electrones saltan a la banda de conducción |

## Semiconductores intrínsecos

En un semiconductor puro, la agitación térmica hace saltar algunos electrones a la banda de conducción. Cada uno deja en la banda de valencia un **hueco**, que se comporta como un portador de carga **positiva**.

Conducen tanto los electrones como los huecos. La concentración de cada uno es

$$n_i \propto T^{3/2}\, e^{-E_g / 2 k_B T}$$

Para el silicio a 300 K, $n_i \approx 10^{16}\ \text{m}^{-3}$: muy pocos, frente a los $5 \times 10^{28}$ átomos por m³.

> [!NOTE]
> El factor exponencial domina. Al pasar de 300 K a 350 K en silicio, $E_g / 2k_B$ vale 6500 K y
> $$\frac{n_i(350)}{n_i(300)} \approx \left( \frac{350}{300} \right)^{3/2} e^{6500 \left( \frac{1}{300} - \frac{1}{350} \right)} \approx 1{,}26 \times 22 \approx 28$$
> La concentración se multiplica casi por 30.

Un fotón con energía mayor que $E_g$ también crea un par electrón-hueco: es la base de las celdas solares y de los fotodetectores. El silicio absorbe hasta $\lambda = 1240 / 1{,}12 = 1107$ nm.

## Dopado

Agregar una cantidad ínfima de impurezas (una parte por millón) cambia la conductividad en varios órdenes de magnitud.

| Tipo | Impureza | Aporta | Portadores mayoritarios |
|---|---|---|---|
| **n** | **donadora**, con 5 electrones de valencia (P, As, Sb) | un electrón que queda casi libre | electrones |
| **p** | **aceptora**, con 3 (B, Al, Ga) | un hueco | huecos |

El electrón sobrante del donador ocupa un nivel a unos 0,05 eV por debajo de la banda de conducción: a temperatura ambiente está prácticamente siempre ionizado.

El material dopado sigue siendo **eléctricamente neutro**.

En equilibrio se cumple la **ley de acción de masas**:

$$n\,p = n_i^2$$

> [!NOTE]
> **Ejemplo.** Silicio dopado con $10^{22}$ átomos de fósforo por m³:
>
> $n \approx 10^{22}\ \text{m}^{-3}$ (un millón de veces más que en el intrínseco).
>
> $p = n_i^2 / n = 10^{32} / 10^{22} = 10^{10}\ \text{m}^{-3}$.

El dopado desplaza el **nivel de Fermi**: hacia la banda de conducción en el tipo n, y hacia la de valencia en el tipo p.

La conductividad suma las dos contribuciones:

$$\sigma = e\,(n\,\mu_n + p\,\mu_p)$$

con $\mu$ la **movilidad** de cada portador.

El **efecto Hall** permite medir el signo y la concentración de los portadores: en un campo magnético perpendicular a la corriente aparece una tensión transversal cuyo signo depende del tipo de portador.

## La unión p-n

Al unir una región p con una n:

1. los electrones **difunden** de n hacia p, y los huecos de p hacia n;
2. se recombinan, y cerca de la unión queda una **zona de agotamiento** sin portadores libres, con iones fijos: positivos del lado n y negativos del lado p;
3. esos iones crean un **campo eléctrico** que se opone a la difusión, y una **barrera de potencial** de unos 0,7 V en silicio.

| Polarización | Conexión | Efecto | Corriente |
|---|---|---|---|
| **Directa** | positivo al lado p | baja la barrera | grande; crece exponencialmente |
| **Inversa** | positivo al lado n | sube la barrera y ensancha la zona de agotamiento | casi nula |

$$I = I_0 \left( e^{eV / k_B T} - 1 \right)$$

A 300 K, $k_B T / e = 0{,}0259$ V. La unión conduce en un solo sentido: es un **diodo**.

## Dispositivos

| Dispositivo | Principio |
|---|---|
| **Diodo rectificador** | deja pasar la corriente en un sentido |
| **LED** | en polarización directa, los electrones y huecos se recombinan emitiendo fotones de energía cercana a $E_g$ |
| **Celda solar** | un fotón crea un par electrón-hueco, y el campo de la unión los separa |
| **Fotodiodo** | lo mismo, usado como detector |
| **Láser de diodo** | emisión estimulada en la unión |
| **Transistor bipolar** | dos uniones (npn o pnp): una corriente de base pequeña controla una grande |
| **MOSFET** | una tensión en la compuerta crea o cierra un canal conductor |

El color de un LED lo fija el *gap*: $\lambda = hc / E_g$.

> [!NOTE]
> Un LED de nitruro de galio e indio con $E_g = 2{,}7$ eV emite en $\lambda = 1240 / 2{,}7 = 459$ nm: azul. El LED azul eficiente (premio Nobel de 2014) hizo posible la iluminación LED blanca.

Los **circuitos integrados** contienen miles de millones de MOSFET en un chip de silicio, fabricados por fotolitografía, con dimensiones de pocos nanómetros.

## Vibraciones de la red

Los átomos del cristal vibran alrededor de sus posiciones de equilibrio. Esas vibraciones están cuantizadas: sus cuantos son los **fonones**.

Explican el **calor específico** de los sólidos:

| Temperatura | Calor específico molar |
|---|---|
| Alta | $3R \approx 25$ J/(mol·K) (ley de Dulong y Petit) |
| Baja | proporcional a $T^3$ (modelo de Debye) |

Los fonones transportan calor en los aislantes, y sus choques con los electrones originan la resistencia eléctrica de los metales.

## Superconductividad

Por debajo de una **temperatura crítica** $T_c$, algunos materiales pierden **toda** resistencia eléctrica. La descubrió Kamerlingh Onnes en 1911, en mercurio a 4,2 K.

| Propiedad | Descripción |
|---|---|
| Resistencia nula | una corriente circula indefinidamente sin pérdidas |
| **Efecto Meissner** | el superconductor expulsa el campo magnético de su interior: un imán levita sobre él |
| Campo crítico | un campo magnético suficientemente intenso destruye el estado |

La **teoría BCS** (1957) la explica: los electrones se unen de a dos en **pares de Cooper**, atraídos indirectamente a través de las vibraciones de la red. Los pares se comportan como bosones y se mueven todos en un mismo estado cuántico, sin dispersarse.

| Material | $T_c$ (K) |
|---|---|
| Mercurio | 4,2 |
| Niobio | 9,3 |
| Niobio-titanio | 10 |
| $\mathrm{Nb_3Sn}$ | 18 |
| $\mathrm{YBa_2Cu_3O_7}$ (cerámico) | 93 |

Los superconductores de **alta temperatura** superan los 77 K del nitrógeno líquido, mucho más barato que el helio. Su mecanismo aún no se comprende del todo.

Aplicaciones: imanes de resonancia magnética y de aceleradores de partículas, trenes de levitación, sensores magnéticos ultrasensibles (SQUID) y qubits de computadoras cuánticas.

## Propiedades magnéticas

| Comportamiento | Causa | Ejemplos |
|---|---|---|
| **Diamagnetismo** | corrientes inducidas se oponen al campo; débil | cobre, agua, bismuto |
| **Paramagnetismo** | momentos magnéticos atómicos se alinean parcialmente con el campo | aluminio, oxígeno |
| **Ferromagnetismo** | los momentos se alinean espontáneamente en **dominios** | hierro, cobalto, níquel |

Por encima de la **temperatura de Curie** (770 °C en el hierro), un ferromagneto se vuelve paramagnético.

## Materiales recientes

| Material | Característica |
|---|---|
| **Grafeno** | una sola capa de átomos de carbono; conductor excelente, muy resistente |
| Puntos cuánticos | nanocristales cuyo color depende del tamaño |
| Perovskitas | celdas solares eficientes y baratas |
| Semiconductores de banda ancha (SiC, GaN) | electrónica de potencia eficiente |
| Aislantes topológicos | aislantes por dentro, conductores en la superficie |

> [!TIP]
> Para entender un material, dibujar su diagrama de bandas: dónde está el nivel de Fermi y cuánto vale el *gap* explican si conduce, cómo responde a la temperatura y qué luz absorbe o emite.
