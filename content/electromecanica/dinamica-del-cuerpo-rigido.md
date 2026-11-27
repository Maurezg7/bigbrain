+++
title = "Dinámica del cuerpo rígido"
summary = "Rotación, momento de inercia, par y potencia, energía cinética, volantes y reducción de inercias en accionamientos."
branch = "Mecánica y elementos de máquinas"
order = 1
level = "Intermedio"
tags = ["rotación", "momento de inercia", "par", "volante", "accionamiento"]
+++

## Traslación y rotación

Las máquinas giran. Las leyes de la rotación tienen la misma forma que las de la traslación, cambiando cada magnitud por su análoga:

| Traslación | Rotación |
|---|---|
| Posición $x$ | Ángulo $\theta$ |
| Velocidad $v$ | Velocidad angular $\omega$ |
| Aceleración $a$ | Aceleración angular $\alpha$ |
| Masa $m$ | Momento de inercia $J$ |
| Fuerza $F$ | Par o momento $T$ |
| $F = m\,a$ | $T = J\,\alpha$ |
| $E_c = \tfrac12 m v^2$ | $E_c = \tfrac12 J\omega^2$ |
| $P = F\,v$ | $P = T\,\omega$ |

Un punto a distancia $r$ del eje tiene velocidad $v = \omega r$, aceleración tangencial $a_t = \alpha r$ y aceleración centrípeta $a_n = \omega^2 r$.

## Momento de inercia

Mide la resistencia de un cuerpo a cambiar su velocidad de giro; depende de la masa y de cuán lejos del eje está:

$$J = \int r^2\,dm$$

| Cuerpo | $J$ respecto de su eje |
|---|---|
| Cilindro macizo o disco | $\tfrac12 m r^2$ |
| Aro o tubo delgado | $m r^2$ |
| Cilindro hueco | $\tfrac12 m (r_e^2 + r_i^2)$ |
| Varilla, eje por el centro | $\tfrac{1}{12} m L^2$ |
| Esfera maciza | $\tfrac25 m r^2$ |

Para un eje paralelo a distancia $d$ del baricéntrico vale el teorema de Steiner: $J = J_G + m d^2$.

## Par, velocidad y potencia

$$P = T\,\omega,\qquad \omega = \frac{2\pi n}{60}$$

Con $P$ en kW y $n$ en rpm resulta la fórmula práctica

$$T\ [\text{N·m}] = \frac{9550\,P}{n}$$

> [!NOTE]
> Un motor de 11 kW a 1460 rpm entrega $T = \dfrac{9550\cdot 11}{1460}\approx 72$ N·m.

## Ecuación del accionamiento

El motor acelera a la máquina mientras su par supere al resistente:

$$T_{motor} - T_{carga} = J_{total}\,\frac{d\omega}{dt}$$

Con par acelerador constante, el tiempo de arranque es $t = \dfrac{J\,\omega}{T_{motor} - T_{carga}}$.

## Reducción de inercias

Si entre motor y carga hay un reductor de relación $i = n_{motor}/n_{carga}$, la carga gira más lento y con más par. Vista desde el eje del motor:

$$T_{carga}' = \frac{T_{carga}}{i\,\eta},\qquad J_{carga}' = \frac{J_{carga}}{i^2}$$

> [!NOTE]
> Tambor de $J = 50\ \text{kg·m}^2$ movido a través de un reductor $i = 10$ por un motor con $J = 0{,}1\ \text{kg·m}^2$.
>
> Inercia total en el eje del motor: $0{,}1 + \dfrac{50}{100} = 0{,}6\ \text{kg·m}^2$.
>
> Con un par acelerador de 30 N·m hasta 1450 rpm (152 rad/s): $t = \dfrac{0{,}6\cdot 152}{30}\approx 3$ s.

## Volantes de inercia

Un volante almacena energía cinética y la devuelve cuando la máquina la necesita, emparejando la marcha de motores alternativos, prensas y punzonadoras. La irregularidad se mide con el **grado de irregularidad**:

$$\delta = \frac{\omega_{max} - \omega_{min}}{\omega_{med}},\qquad \Delta E = J\,\omega_{med}^2\,\delta$$

De aquí se despeja la inercia necesaria para una fluctuación de energía $\Delta E$ dada.

## Trabajo, energía e impulso

- **Teorema de la energía**: el trabajo de todas las fuerzas es igual a la variación de energía cinética.
- **Momento cinético**: $L = J\omega$ se conserva si no hay par exterior.
- **Efecto giroscópico**: al cambiar la orientación del eje de un rotor aparece un par perpendicular, $T = J\,\omega\,\Omega$, que carga los rodamientos de máquinas montadas en vehículos.

## Balanceo

Una masa $m$ desplazada una distancia $e$ del eje genera una fuerza centrífuga giratoria

$$F = m\,e\,\omega^2$$

que crece con el cuadrado de la velocidad y produce vibraciones. El **balanceo estático** corrige el centro de gravedad; el **dinámico**, en dos planos, corrige además el par que aparece en rotores largos.
