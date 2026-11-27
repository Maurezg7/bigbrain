+++
title = "Relatividad especial"
summary = "Postulados de Einstein, dilatación del tiempo, contracción de longitudes, simultaneidad, y la equivalencia entre masa y energía."
branch = "Física moderna"
order = 2
level = "Avanzado"
tags = ["relatividad", "dilatación del tiempo", "factor de Lorentz", "E=mc2"]
+++

## El problema

Las ecuaciones de Maxwell predicen que la luz viaja a $c$, pero ¿respecto de qué? El experimento de Michelson y Morley (1887) no detectó ningún «éter»: la velocidad de la luz resultó la misma en todas las direcciones.

En 1905, Einstein resolvió la contradicción cambiando las ideas de espacio y tiempo.

## Postulados

> [!IMPORTANT]
> 1. **Principio de relatividad**: las leyes de la física son las mismas en todos los sistemas de referencia inerciales.
> 2. **Constancia de la velocidad de la luz**: la luz se propaga en el vacío a $c = 299\,792\,458$ m/s para todos los observadores inerciales, sin importar el movimiento de la fuente o del observador.

El segundo postulado contradice la suma habitual de velocidades. Sus consecuencias solo se notan a velocidades cercanas a $c$.

## Factor de Lorentz

$$\gamma = \frac{1}{\sqrt{1 - \dfrac{v^2}{c^2}}}$$

| $v/c$ | $\gamma$ |
|---|---|
| 0,1 | 1,005 |
| 0,5 | 1,155 |
| 0,8 | 1,667 |
| 0,9 | 2,294 |
| 0,99 | 7,09 |
| 0,999 | 22,4 |

A velocidades cotidianas $\gamma \approx 1$ y se recupera la física de Newton.

## Dilatación del tiempo

> [!IMPORTANT]
> $$\Delta t = \gamma\,\Delta t_0$$

$\Delta t_0$ es el **tiempo propio**: el medido por un reloj que está presente en ambos sucesos (en reposo respecto de ellos). Cualquier otro observador mide un intervalo **mayor**: los relojes en movimiento atrasan.

> [!NOTE]
> Los **muones** creados en la alta atmósfera tienen una vida media de $2{,}2\ \mu\text{s}$ en reposo. A $0{,}998c$ recorrerían en promedio solo $660$ m antes de desintegrarse, y sin embargo muchos llegan al suelo.
>
> Con $\gamma = 15{,}8$, desde la Tierra su vida media es $34{,}8\ \mu\text{s}$ y recorren en promedio más de $10$ km.

No es una ilusión ni un defecto de los relojes: es el tiempo mismo. Se ha verificado con relojes atómicos en aviones, y el sistema GPS debe corregirlo.

**Paradoja de los gemelos**: el que viaja a gran velocidad y regresa es más joven que el que se quedó. No hay contradicción: el viajero aceleró para dar la vuelta, así que las situaciones no son simétricas.

## Contracción de longitudes

$$L = \frac{L_0}{\gamma}$$

$L_0$ es la **longitud propia**, medida en el sistema donde el objeto está en reposo. Un objeto en movimiento se mide más corto **en la dirección del movimiento**; las dimensiones perpendiculares no cambian.

> [!NOTE]
> Desde el sistema del muón, su vida sigue siendo $2{,}2\ \mu\text{s}$, pero la atmósfera de $10$ km se contrae a $\dfrac{10}{15{,}8} = 0{,}63$ km. Ambos observadores coinciden en que el muón llega al suelo.

## Relatividad de la simultaneidad

Dos sucesos simultáneos para un observador pueden **no serlo** para otro que se mueve respecto del primero. No existe un «ahora» universal.

Lo que sí es absoluto es el **intervalo espacio-temporal**:

$$\Delta s^2 = c^2\Delta t^2 - \Delta x^2$$

Todos los observadores inerciales miden el mismo valor. Espacio y tiempo forman una sola entidad: el **espacio-tiempo**.

## Transformaciones de Lorentz

Relacionan las coordenadas de un suceso en dos sistemas, uno moviéndose con velocidad $v$ en $x$ respecto del otro:

$$x' = \gamma(x - vt) \qquad t' = \gamma\left(t - \frac{vx}{c^2}\right)$$

Reemplazan a las de Galileo ($x' = x - vt$, $t' = t$).

### Suma de velocidades

$$u' = \frac{u - v}{1 - \dfrac{uv}{c^2}}$$

> [!NOTE]
> Una nave se aleja de la Tierra a $0{,}8c$ y lanza hacia adelante una sonda a $0{,}8c$ respecto de la nave.
>
> Velocidad de la sonda respecto de la Tierra: $\dfrac{0{,}8c + 0{,}8c}{1 + 0{,}64} = 0{,}976c$, no $1{,}6c$.

Ninguna composición de velocidades supera $c$. Y si $u = c$, también $u' = c$.

## Dinámica relativista

**Cantidad de movimiento**:

$$\vec{p} = \gamma m\vec{v}$$

**Energía**:

> [!IMPORTANT]
> $$E = \gamma mc^2 \qquad E_0 = mc^2 \qquad E_c = (\gamma - 1)mc^2$$

- $E_0 = mc^2$ es la **energía en reposo**: la masa es una forma de energía.
- Para $v \ll c$, la energía cinética se reduce a $\tfrac{1}{2}mv^2$.
- Cuando $v \to c$, $\gamma \to \infty$: haría falta energía infinita. **Ningún objeto con masa alcanza la velocidad de la luz.**

Relación entre energía y cantidad de movimiento:

$$E^2 = (pc)^2 + (mc^2)^2$$

Para partículas sin masa, como el fotón, $E = pc$.

> [!NOTE]
> Energía en reposo de un electrón: $E_0 = 9{,}11 \times 10^{-31} \cdot (3 \times 10^8)^2 = 8{,}2 \times 10^{-14}$ J $= 0{,}511$ MeV.

> [!NOTE]
> La energía en reposo de $1$ g de materia: $10^{-3} \cdot 9 \times 10^{16} = 9 \times 10^{13}$ J, equivalente a unas $21$ kilotoneladas de TNT.

### Masa y energía

En toda reacción, la variación de masa corresponde a la energía liberada: $\Delta E = \Delta m\,c^2$.

| Proceso | Fracción de masa convertida |
|---|---|
| Combustión química | ~$10^{-10}$ |
| Fisión nuclear | ~0,1 % |
| Fusión nuclear | ~0,7 % |
| Aniquilación materia-antimateria | 100 % |

El Sol convierte unos 4 millones de toneladas de masa en energía cada segundo.

## Relatividad general

En 1915 Einstein extendió la teoría a sistemas acelerados y a la gravedad.

- **Principio de equivalencia**: los efectos de la gravedad son indistinguibles de los de una aceleración.
- La masa y la energía **curvan** el espacio-tiempo, y los cuerpos siguen las trayectorias más rectas posibles en él.

Predicciones confirmadas: la desviación de la luz por el Sol, el atraso de los relojes en campos gravitatorios intensos, los agujeros negros y las ondas gravitacionales.

> [!WARNING]
> Que «nada supera a la luz» se refiere a objetos, energía e información. No hay sistema de referencia en el que un fotón esté en reposo.

> [!TIP]
> Para identificar el tiempo propio, buscar el reloj que está en los dos sucesos. Para la longitud propia, el observador para quien el objeto está quieto.
