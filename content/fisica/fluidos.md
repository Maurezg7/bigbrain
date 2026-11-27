+++
title = "Fluidos"
summary = "Densidad y presión, principio de Pascal, empuje de Arquímedes, ecuación de continuidad, Bernoulli y viscosidad."
branch = "Mecánica"
order = 9
level = "Intermedio"
tags = ["presión", "Arquímedes", "Bernoulli", "hidrostática"]
+++

## Densidad y presión

Un **fluido** (líquido o gas) no tiene forma propia: fluye.

$$\rho = \frac{m}{V} \qquad p = \frac{F}{A}$$

| Sustancia | Densidad (kg/m³) |
|---|---|
| Aire | 1,2 |
| Aceite | 900 |
| Agua | 1000 |
| Agua de mar | 1025 |
| Acero | 7850 |
| Mercurio | 13 600 |

La presión se mide en **pascales** (Pa = N/m²). $1\ \text{atm} = 101\,325\ \text{Pa} = 760\ \text{mmHg} \approx 1{,}013\ \text{bar}$.

La presión es un escalar: en un punto de un fluido en reposo actúa igual en todas las direcciones, y empuja perpendicularmente a cualquier superficie.

## Hidrostática

> [!IMPORTANT]
> $$p = p_0 + \rho gh$$

La presión aumenta con la profundidad y solo depende de ella, no de la forma del recipiente.

> [!NOTE]
> A $10$ m bajo el agua: $p = 101\,325 + 1000 \cdot 9{,}8 \cdot 10 = 199\,325$ Pa. Cada $10$ m de agua suman cerca de una atmósfera.

- **Presión absoluta**: la total. **Presión manométrica**: la que excede a la atmosférica (la que marca un manómetro de neumáticos).
- Un **barómetro** de mercurio sostiene una columna de $760$ mm a nivel del mar.

### Principio de Pascal

Un cambio de presión en un fluido encerrado se transmite íntegramente a todos sus puntos.

**Prensa hidráulica**: $\dfrac{F_1}{A_1} = \dfrac{F_2}{A_2}$. Una fuerza pequeña sobre un pistón chico produce una grande en el pistón grande. Así funcionan los frenos y los gatos hidráulicos.

> [!NOTE]
> Pistones de $5\ \text{cm}^2$ y $500\ \text{cm}^2$: con $100$ N se levantan $10\,000$ N. A cambio, el pistón grande se mueve cien veces menos.

## Principio de Arquímedes

> [!IMPORTANT]
> Todo cuerpo sumergido recibe un **empuje** hacia arriba igual al peso del fluido que desaloja:
> $$E = \rho_{fluido}\,g\,V_{sumergido}$$

| Situación | Resultado |
|---|---|
| $\rho_{cuerpo} < \rho_{fluido}$ | flota |
| $\rho_{cuerpo} = \rho_{fluido}$ | queda en equilibrio a cualquier profundidad |
| $\rho_{cuerpo} > \rho_{fluido}$ | se hunde |

Un cuerpo que flota se sumerge hasta que el empuje iguala a su peso. La fracción sumergida es $\dfrac{\rho_{cuerpo}}{\rho_{fluido}}$.

> [!NOTE]
> El hielo ($917\ \text{kg/m}^3$) en agua de mar ($1025\ \text{kg/m}^3$) tiene sumergido el $\dfrac{917}{1025} = 89\,\%$. Solo se ve el 11 % del témpano.

Un barco de acero flota porque su forma hueca desaloja mucha agua: su densidad **media** es menor que la del agua.

## Fluidos en movimiento

Para un **fluido ideal** (incompresible y sin viscosidad) en régimen estacionario:

### Ecuación de continuidad

La masa se conserva: lo que entra, sale.

$$A_1v_1 = A_2v_2 = Q$$

$Q$ es el **caudal** (m³/s). Donde el tubo se angosta, el fluido va más rápido: por eso se tapa parte de la manguera para que el chorro llegue más lejos.

### Ecuación de Bernoulli

> [!IMPORTANT]
> $$p + \tfrac{1}{2}\rho v^2 + \rho gh = \text{constante}$$

Es la conservación de la energía por unidad de volumen. A igual altura, **donde la velocidad es mayor, la presión es menor**.

| Aplicación | Explicación |
|---|---|
| Tubo de Venturi | la caída de presión en el estrechamiento mide el caudal |
| Tubo de Pitot | mide la velocidad de un avión |
| Sustentación | el aire va más rápido sobre el ala: menor presión arriba |
| Pulverizador | el aire rápido aspira el líquido |
| Efecto de un tiro con comba | la rotación altera las velocidades a cada lado |

**Teorema de Torricelli**: la velocidad de salida por un orificio a profundidad $h$ es $v = \sqrt{2gh}$, igual a la de un cuerpo que cae esa altura.

> [!NOTE]
> Por un caño horizontal circula agua a $2$ m/s con $150$ kPa. El caño se angosta a la mitad del área.
>
> Continuidad: $v_2 = 4$ m/s.
>
> Bernoulli: $p_2 = 150\,000 + \tfrac{1}{2}(1000)(2^2 - 4^2) = 144\,000$ Pa.

## Fluidos reales

Los fluidos reales tienen **viscosidad** ($\eta$): rozamiento interno que disipa energía.

- **Flujo laminar**: capas ordenadas. **Flujo turbulento**: remolinos caóticos.
- El **número de Reynolds** $Re = \dfrac{\rho vD}{\eta}$ predice el régimen: en un caño, laminar si $Re < 2300$ y turbulento si $Re > 4000$.
- **Ley de Poiseuille** (flujo laminar en un tubo): $Q = \dfrac{\pi r^4\Delta p}{8\eta L}$. El caudal depende de la **cuarta potencia** del radio: reducir el radio a la mitad lo divide por 16.

**Tensión superficial**: las moléculas de la superficie de un líquido se atraen y forman una especie de membrana. Explica las gotas, los insectos que caminan sobre el agua y la capilaridad.

> [!WARNING]
> Bernoulli solo vale a lo largo de una línea de corriente, sin viscosidad ni bombas en el medio. En cañerías reales hay que agregar las pérdidas de carga.

> [!TIP]
> Continuidad dice cómo cambia la velocidad; Bernoulli, cómo cambia la presión. Se usan siempre juntas.
