+++
title = "Corriente eléctrica y circuitos"
summary = "Corriente, resistencia y ley de Ohm, potencia, fuerza electromotriz, asociación de resistencias y leyes de Kirchhoff."
branch = "Electromagnetismo"
order = 3
level = "Básico"
tags = ["corriente", "ley de Ohm", "resistencia", "Kirchhoff", "potencia eléctrica"]
+++

## Corriente eléctrica

> [!IMPORTANT]
> $$I = \frac{dQ}{dt}$$

La **corriente** es la carga que atraviesa una sección por unidad de tiempo. Se mide en **amperes** (A = C/s).

- El **sentido convencional** es el de las cargas positivas: del polo positivo al negativo por el circuito exterior. En los metales los electrones van al revés.
- Los electrones avanzan muy lento (velocidad de deriva de fracciones de mm/s), pero el campo eléctrico se establece casi a la velocidad de la luz: la lámpara enciende al instante.

## Resistencia y ley de Ohm

> [!IMPORTANT]
> $$V = IR$$

La **resistencia** se mide en **ohms** ($\Omega$). Para un conductor de longitud $L$ y sección $A$:

$$R = \rho\,\frac{L}{A}$$

| Material | Resistividad $\rho$ ($\Omega$·m) |
|---|---|
| Plata | $1{,}6 \times 10^{-8}$ |
| Cobre | $1{,}7 \times 10^{-8}$ |
| Aluminio | $2{,}8 \times 10^{-8}$ |
| Nicrom | $1{,}1 \times 10^{-6}$ |
| Silicio | $\sim 10^{3}$ |
| Vidrio | $\sim 10^{12}$ |

En los metales la resistencia **aumenta** con la temperatura: $R = R_0[1 + \alpha(T - T_0)]$. En los semiconductores disminuye.

> [!NOTE]
> Un cable de cobre de $100$ m y $2{,}5\ \text{mm}^2$: $R = 1{,}7 \times 10^{-8} \cdot \dfrac{100}{2{,}5 \times 10^{-6}} = 0{,}68\ \Omega$.

No todos los componentes cumplen la ley de Ohm: los diodos y las lámparas no son **óhmicos**.

## Potencia

$$P = VI = I^2R = \frac{V^2}{R}$$

En una resistencia, la energía eléctrica se convierte en calor (**efecto Joule**).

La energía consumida es $E = Pt$. La unidad comercial es el **kilowatt-hora**: $1\ \text{kWh} = 3{,}6 \times 10^6$ J.

> [!NOTE]
> Una estufa de $2000$ W a $220$ V: $I = \dfrac{2000}{220} = 9{,}1$ A y $R = \dfrac{220^2}{2000} = 24{,}2\ \Omega$.
>
> Encendida $5$ horas consume $10$ kWh.

Las líneas de transmisión usan **alta tensión** porque, para la misma potencia, la corriente es menor y las pérdidas $I^2R$ caen con el cuadrado.

## Fuerza electromotriz

Una fuente (pila, batería, generador) entrega energía a las cargas. Su **fem** $\varepsilon$ es la energía por unidad de carga. Una fuente real tiene **resistencia interna** $r$:

$$V = \varepsilon - Ir$$

La tensión en bornes cae cuando se le pide más corriente.

## Asociación de resistencias

| | Serie | Paralelo |
|---|---|---|
| Resistencia equivalente | $R_{eq} = R_1 + R_2 + \dots$ | $\dfrac{1}{R_{eq}} = \dfrac{1}{R_1} + \dfrac{1}{R_2} + \dots$ |
| Igual en todas | la corriente | la tensión |
| Se reparte | la tensión | la corriente |

- En serie, la equivalente es **mayor** que la mayor.
- En paralelo, es **menor** que la menor. Para dos: $R_{eq} = \dfrac{R_1R_2}{R_1 + R_2}$.

Las instalaciones domiciliarias se conectan en **paralelo**: cada aparato recibe la tensión completa y funciona independientemente.

> [!NOTE]
> $R_1 = 4\ \Omega$ en serie con el paralelo de $R_2 = 6\ \Omega$ y $R_3 = 3\ \Omega$, con una batería de $12$ V.
>
> Paralelo: $\dfrac{6 \cdot 3}{9} = 2\ \Omega$. Total: $6\ \Omega$. Corriente: $I = 2$ A.
>
> Tensión en el paralelo: $2 \cdot 2 = 4$ V. Entonces $I_2 = \dfrac{4}{6} = 0{,}67$ A e $I_3 = \dfrac{4}{3} = 1{,}33$ A.

**Divisor de tensión**: en dos resistencias en serie, $V_2 = V\,\dfrac{R_2}{R_1 + R_2}$.

## Leyes de Kirchhoff

Para circuitos que no se reducen a serie y paralelo.

> [!IMPORTANT]
> 1. **Ley de nodos**: la suma de las corrientes que entran a un nodo es igual a la suma de las que salen. (Conservación de la carga.)
> 2. **Ley de mallas**: la suma de las diferencias de potencial a lo largo de cualquier camino cerrado es cero. (Conservación de la energía.)

### Procedimiento

1. Asignar un sentido a la corriente de cada rama (arbitrario).
2. Escribir la ley de nodos.
3. Recorrer cada malla sumando tensiones: al atravesar una resistencia en el sentido de la corriente, $-IR$; al pasar una fuente de $-$ a $+$, $+\varepsilon$.
4. Resolver el sistema. Si una corriente da negativa, su sentido real es el opuesto.

> [!NOTE]
> Dos mallas con una rama común. Malla izquierda: fuente de $10$ V y $R_1 = 2\ \Omega$; rama central $R_3 = 4\ \Omega$; malla derecha: fuente de $6$ V y $R_2 = 1\ \Omega$. Ambas fuentes empujan corriente hacia el nodo superior.
>
> Nodos: $I_1 + I_2 = I_3$.
>
> Malla izquierda: $10 - 2I_1 - 4I_3 = 0$. Malla derecha: $6 - I_2 - 4I_3 = 0$.
>
> Resolviendo: $I_3 = \dfrac{11}{7} = 1{,}57$ A, $I_1 = 1{,}86$ A, $I_2 = -0{,}29$ A.
>
> $I_2$ es negativa: la fuente de $6$ V está siendo **cargada** por la otra.

## Instrumentos

| Instrumento | Mide | Se conecta | Resistencia ideal |
|---|---|---|---|
| Amperímetro | corriente | en serie | cero |
| Voltímetro | tensión | en paralelo | infinita |
| Óhmetro | resistencia | con el circuito sin tensión | — |

## Seguridad

| Corriente por el cuerpo | Efecto |
|---|---|
| 1 mA | cosquilleo |
| 10–20 mA | contracción muscular: no se puede soltar |
| 50–100 mA | fibrilación ventricular |

Lo peligroso es la **corriente**, no la tensión en sí. El **disyuntor diferencial** corta al detectar una fuga de 30 mA; la **puesta a tierra** deriva las fallas; la **llave termomagnética** protege los cables contra sobrecargas y cortocircuitos.

> [!TIP]
> Antes de plantear Kirchhoff, simplificar todo lo que se pueda con serie y paralelo.
