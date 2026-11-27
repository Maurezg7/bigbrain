+++
title = "Análisis dimensional, semejanza y capa límite"
summary = "Teorema Pi de Buckingham, números adimensionales, ensayos con modelos, capa límite laminar y turbulenta, separación, arrastre y sustentación."
branch = "Mecánica de los fluidos"
order = 3
level = "Avanzado"
tags = ["análisis dimensional", "Buckingham", "Reynolds", "capa límite", "arrastre", "semejanza"]
+++

## Por qué adimensionalizar

Muchos problemas de fluidos no tienen solución analítica: se resuelven **experimentando**. Si la caída de presión en un tubo depende de cinco variables, ensayar diez valores de cada una exige $10^5$ experimentos.

El **análisis dimensional** agrupa las variables en unos pocos **números adimensionales**. Así se reducen los experimentos y los resultados valen para cualquier fluido y tamaño.

## Homogeneidad dimensional

Toda ecuación física debe tener las mismas dimensiones en cada término. Las dimensiones fundamentales en mecánica son masa (M), longitud (L) y tiempo (T).

| Magnitud | Símbolo | Dimensiones |
|---|---|---|
| Velocidad | $V$ | $L T^{-1}$ |
| Densidad | $\rho$ | $M L^{-3}$ |
| Fuerza | $F$ | $M L T^{-2}$ |
| Presión, tensión | $p$, $\tau$ | $M L^{-1} T^{-2}$ |
| Viscosidad dinámica | $\mu$ | $M L^{-1} T^{-1}$ |
| Viscosidad cinemática | $\nu$ | $L^2 T^{-1}$ |
| Tensión superficial | $\sigma$ | $M T^{-2}$ |
| Caudal | $Q$ | $L^3 T^{-1}$ |
| Potencia | $P$ | $M L^2 T^{-3}$ |

## Teorema Pi de Buckingham

> [!IMPORTANT]
> Si un fenómeno depende de $n$ variables que involucran $k$ dimensiones fundamentales, puede describirse con $n - k$ grupos adimensionales independientes ($\Pi_1, \Pi_2, \dots$).

### Procedimiento

1. Listar las $n$ variables.
2. Escribir sus dimensiones y contar $k$.
3. Elegir $k$ **variables repetidas** que contengan entre todas las $k$ dimensiones y no formen por sí solas un grupo adimensional.
4. Combinar cada variable restante con las repetidas para formar un grupo adimensional.

> [!NOTE]
> **Arrastre sobre una esfera.** La fuerza $F$ depende de $D$, $V$, $\rho$ y $\mu$: $n = 5$, $k = 3$, luego hay $5 - 3 = 2$ grupos.
>
> Variables repetidas: $\rho$, $V$, $D$.
>
> $\Pi_1 = F \rho^a V^b D^c$. Igualando exponentes de M, L y T resulta $a = -1$, $b = -2$, $c = -2$:
> $$\Pi_1 = \frac{F}{\rho V^2 D^2}$$
>
> Del mismo modo, con $\mu$:
> $$\Pi_2 = \frac{\mu}{\rho V D} = \frac{1}{Re}$$
>
> Entonces
> $$\frac{F}{\rho V^2 D^2} = f(Re)$$
>
> Una sola curva describe el arrastre de cualquier esfera en cualquier fluido.

## Números adimensionales

| Número | Definición | Cociente de | Importa en |
|---|---|---|---|
| **Reynolds** | $Re = \dfrac{\rho V L}{\mu}$ | fuerzas de inercia y viscosas | casi todo flujo |
| **Froude** | $Fr = \dfrac{V}{\sqrt{g L}}$ | inercia y gravedad | superficie libre: canales, barcos, vertederos |
| **Mach** | $Ma = \dfrac{V}{c}$ | velocidad del flujo y del sonido | flujo compresible |
| **Euler** | $Eu = \dfrac{\Delta p}{\rho V^2}$ | presión e inercia | caídas de presión, cavitación |
| **Weber** | $We = \dfrac{\rho V^2 L}{\sigma}$ | inercia y tensión superficial | gotas, atomización |
| **Strouhal** | $St = \dfrac{f L}{V}$ | oscilación y flujo medio | desprendimiento de vórtices |
| Coeficiente de arrastre | $C_D = \dfrac{F_D}{\frac{1}{2} \rho V^2 A}$ | arrastre y fuerza dinámica | cuerpos sumergidos |
| Coeficiente de sustentación | $C_L = \dfrac{F_L}{\frac{1}{2} \rho V^2 A}$ | sustentación y fuerza dinámica | perfiles |

## Semejanza

Para que un ensayo con **modelo** a escala represente al **prototipo** real hacen falta:

| Semejanza | Condición |
|---|---|
| Geométrica | mismas formas; todas las longitudes en la misma escala |
| Cinemática | velocidades proporcionales y con la misma dirección |
| Dinámica | fuerzas proporcionales: **iguales números adimensionales** |

Si los grupos independientes coinciden, el grupo dependiente también coincide.

> [!NOTE]
> **Ejemplo.** Se ensaya en túnel de viento un modelo a escala 1:5 de un automóvil que circula a 100 km/h. Para igualar el Reynolds con el mismo aire:
> $$V_m L_m = V_p L_p \quad\Rightarrow\quad V_m = 5 \times 100 = 500\ \text{km/h}$$
>
> Si a esa velocidad el arrastre del modelo es 1200 N, como $C_D$ es igual y $F \propto V^2 L^2$:
> $$F_p = F_m \left( \frac{V_p}{V_m} \right)^2 \left( \frac{L_p}{L_m} \right)^2 = 1200 \times \frac{1}{25} \times 25 = 1200\ \text{N}$$

A 500 km/h ($Ma \approx 0{,}4$) empiezan los efectos de compresibilidad. Por eso se usan túneles presurizados, o se aprovecha que $C_D$ varía poco con el Reynolds cuando este es alto.

### Semejanza incompleta

No siempre pueden igualarse todos los números. En un modelo de barco con agua, igualar Froude exige $V_m = V_p \sqrt{L_m / L_p}$, e igualar Reynolds, $V_m = V_p \, L_p / L_m$: son incompatibles.

Se iguala el número **dominante** (Froude, para las olas) y se corrige el resto por cálculo.

| Problema | Se iguala |
|---|---|
| Flujo en tuberías, cuerpos sumergidos | Reynolds |
| Canales, vertederos, barcos | Froude |
| Alta velocidad en gases | Mach (y Reynolds) |
| Turbomáquinas | coeficientes de caudal y de altura |

## Capa límite

Un fluido real se **adhiere** a las paredes: sobre la superficie, su velocidad es cero (**condición de no deslizamiento**). Prandtl mostró en 1904 que los efectos viscosos se concentran en una zona delgada junto a la pared: la **capa límite**. Fuera de ella, el flujo se comporta como ideal.

El **espesor** $\delta$ es la distancia a la pared donde la velocidad alcanza el 99 % de la exterior $U$.

### Placa plana

El Reynolds local, a una distancia $x$ del borde de ataque:

$$Re_x = \frac{\rho U x}{\mu}$$

| Régimen | Condición | Espesor | Coeficiente de fricción medio |
|---|---|---|---|
| Laminar | $Re_x < 5 \times 10^5$ | $\dfrac{\delta}{x} = \dfrac{5{,}0}{\sqrt{Re_x}}$ | $C_f = \dfrac{1{,}328}{\sqrt{Re_L}}$ |
| Turbulento | $Re_x > 5 \times 10^5$ | $\dfrac{\delta}{x} \approx \dfrac{0{,}37}{Re_x^{1/5}}$ | $C_f \approx \dfrac{0{,}074}{Re_L^{1/5}}$ |

La solución laminar es la de **Blasius**. La fuerza de fricción sobre una cara de la placa, de área $A$:

$$F = C_f \, \tfrac{1}{2} \rho U^2 A$$

> [!NOTE]
> **Ejemplo.** Aire ($\nu = 1{,}5 \times 10^{-5}\ \text{m}^2/\text{s}$) a 10 m/s sobre una placa. A $x = 0{,}5$ m:
> $$Re_x = \frac{10 \times 0{,}5}{1{,}5 \times 10^{-5}} = 3{,}3 \times 10^5 \quad \text{(laminar)}$$
> $$\delta = \frac{5 \times 0{,}5}{\sqrt{3{,}3 \times 10^5}} = 4{,}3\ \text{mm}$$

| | Laminar | Turbulenta |
|---|---|---|
| Perfil de velocidades | suave | más lleno cerca de la pared |
| Fricción en la pared | menor | mayor |
| Crecimiento del espesor | lento | rápido |
| Resistencia a la separación | baja | alta |

La **transición** se adelanta con la rugosidad, la turbulencia de la corriente y los gradientes de presión adversos.

## Separación

Donde la presión **aumenta** en el sentido del flujo (gradiente adverso), el fluido cercano a la pared, que tiene poca energía cinética, se frena, se detiene y retrocede. La capa límite se **separa** de la superficie y deja una **estela** de baja presión.

Consecuencias:

- gran aumento del arrastre,
- pérdida de sustentación en un perfil (**entrada en pérdida**),
- pérdidas de carga en difusores y ensanchamientos.

## Arrastre

$$F_D = C_D \, \tfrac{1}{2} \rho V^2 A$$

$A$ es el área frontal (o el área en planta, para perfiles).

| Componente | Origen | Domina en |
|---|---|---|
| De **fricción** | tensiones de corte en la pared | cuerpos fuselados, placas alineadas |
| De **forma** (de presión) | diferencia de presión entre el frente y la estela | cuerpos romos |

### Coeficientes típicos

| Cuerpo | $C_D$ |
|---|---|
| Placa plana normal al flujo | 1,2 a 2,0 |
| Cilindro largo ($Re \approx 10^5$) | 1,2 |
| Esfera ($Re \approx 10^5$) | 0,47 |
| Esfera con capa límite turbulenta | 0,1 a 0,2 |
| Automóvil moderno | 0,25 a 0,35 |
| Camión | 0,6 a 1,0 |
| Perfil fuselado | 0,04 |

### La crisis del arrastre

Para una esfera, cerca de $Re \approx 3 \times 10^5$ el $C_D$ **cae bruscamente**, de 0,47 a cerca de 0,1. La capa límite se vuelve turbulenta antes de separarse, resiste mejor el gradiente adverso, se separa más atrás y la estela se angosta.

> [!NOTE]
> Los hoyuelos de una pelota de golf provocan la turbulencia a Reynolds más bajos: reducen el arrastre y la pelota llega más lejos.

Para Reynolds muy bajos ($Re < 1$) vale la **ley de Stokes** para la esfera:

$$F_D = 3 \pi \mu V D \qquad C_D = \frac{24}{Re}$$

### Potencia para vencer el arrastre

$$P = F_D V = \tfrac{1}{2} \rho \, C_D A \, V^3$$

Crece con el **cubo** de la velocidad.

> [!NOTE]
> **Ejemplo.** Automóvil con $C_D = 0{,}30$ y $A = 2{,}2\ \text{m}^2$ a 120 km/h (33,3 m/s), con $\rho = 1{,}2\ \text{kg/m}^3$:
> $$F_D = 0{,}30 \times 0{,}5 \times 1{,}2 \times 33{,}3^2 \times 2{,}2 = 440\ \text{N} \qquad P = 440 \times 33{,}3 = 14{,}7\ \text{kW}$$
> A 90 km/h la potencia baja a 6,2 kW.

## Sustentación

Es la fuerza perpendicular a la corriente:

$$F_L = C_L \, \tfrac{1}{2} \rho V^2 A$$

En un **perfil aerodinámico**, $C_L$ crece casi linealmente con el **ángulo de ataque** hasta unos 12° a 16°. Más allá, la capa límite se separa del extradós y el perfil **entra en pérdida**.

La **fineza** $C_L / C_D$ mide la eficiencia del perfil. Estas ideas se aplican a alas, palas de aerogeneradores, hélices y álabes de turbomáquinas.

## Desprendimiento de vórtices

Detrás de un cilindro, los vórtices se desprenden alternadamente de cada lado (**calle de von Kármán**), con frecuencia

$$f = St \, \frac{V}{D} \qquad St \approx 0{,}2$$

para un amplio rango de Reynolds.

> [!WARNING]
> Si esa frecuencia coincide con una **frecuencia natural** de la estructura, hay resonancia. Afecta a chimeneas, cables de líneas eléctricas (vibración eólica), tubos de intercambiadores y puentes. Se combate con amortiguadores o con aletas helicoidales que desordenan el desprendimiento.

> [!TIP]
> Antes de resolver cualquier problema de fluidos, calcular el número de Reynolds: indica si el flujo es laminar o turbulento y qué correlaciones corresponden.
