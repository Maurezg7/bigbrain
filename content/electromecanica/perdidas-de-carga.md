+++
title = "Flujo en tuberías y pérdidas de carga"
summary = "Número de Reynolds, flujo laminar y turbulento, ecuación de Darcy-Weisbach, diagrama de Moody, accesorios y redes."
branch = "Mecánica de los fluidos"
order = 2
level = "Intermedio"
tags = ["Reynolds", "Darcy-Weisbach", "Moody", "pérdida de carga", "tuberías"]
+++

## Laminar o turbulento

El régimen del flujo depende del **número de Reynolds**, cociente entre fuerzas de inercia y viscosas:

$$Re = \frac{\rho\,v\,D}{\mu} = \frac{v\,D}{\nu}$$

| $Re$ en tuberías | Régimen | Características |
|---|---|---|
| Menor que 2300 | laminar | capas ordenadas, perfil parabólico |
| 2300 a 4000 | transición | inestable |
| Mayor que 4000 | turbulento | mezcla intensa, perfil más plano |

Casi todos los flujos industriales de agua y aire son turbulentos; los de aceites viscosos suelen ser laminares.

> [!NOTE]
> Agua a 20 °C a 2 m/s por un caño de 50 mm: $Re = \dfrac{2\cdot 0{,}05}{10^{-6}} = 100\,000$. Turbulento.

## Pérdidas por fricción

El rozamiento con las paredes disipa energía, que se manifiesta como una caída de presión. La ecuación de **Darcy-Weisbach** da la pérdida en metros de columna de fluido:

$$h_f = f\,\frac{L}{D}\,\frac{v^2}{2g}$$

$f$ es el factor de fricción, que depende de $Re$ y de la rugosidad relativa $\varepsilon/D$.

| Régimen | Factor de fricción |
|---|---|
| Laminar | $f = \dfrac{64}{Re}$ |
| Turbulento | diagrama de Moody o ecuación de Colebrook |

$$\frac{1}{\sqrt f} = -2\log_{10}\left(\frac{\varepsilon/D}{3{,}7} + \frac{2{,}51}{Re\sqrt f}\right)$$

Como Colebrook es implícita, se usa la aproximación explícita de Swamee-Jain:

$$f = \frac{0{,}25}{\left[\log_{10}\left(\dfrac{\varepsilon/D}{3{,}7} + \dfrac{5{,}74}{Re^{0{,}9}}\right)\right]^2}$$

| Material | Rugosidad $\varepsilon$ (mm) |
|---|---|
| PVC, cobre, vidrio | 0,0015 |
| Acero comercial | 0,045 |
| Hierro galvanizado | 0,15 |
| Fundición | 0,26 |
| Hormigón | 0,3 a 3 |

![Diagrama de Moody simplificado con la recta laminar y las curvas turbulentas para distintas rugosidades](/media/electromecanica/moody.svg)

*Diagrama de Moody: factor de fricción en función del número de Reynolds y la rugosidad relativa.*

## Pérdidas localizadas

Codos, válvulas, tes y cambios de sección producen pérdidas adicionales, proporcionales a la altura de velocidad:

$$h_s = K\,\frac{v^2}{2g}$$

| Accesorio | $K$ |
|---|---|
| Codo de 90° normal | 0,9 |
| Codo de 90° de radio largo | 0,4 |
| Te, paso lateral | 1,8 |
| Válvula esclusa abierta | 0,2 |
| Válvula globo abierta | 10 |
| Válvula de retención | 2,5 |
| Entrada desde un tanque | 0,5 |
| Salida a un tanque | 1,0 |

También se expresan como **longitud equivalente** de caño recto que produce la misma pérdida.

## Ejemplo completo

> [!NOTE]
> Se bombean 10 L/s de agua por 100 m de caño de acero de 80 mm con dos codos normales y una válvula globo.
>
> $v = \dfrac{0{,}010}{\pi\cdot 0{,}04^2}\approx 1{,}99$ m/s y $\dfrac{v^2}{2g}\approx 0{,}202$ m.
>
> $Re = \dfrac{1{,}99\cdot 0{,}08}{10^{-6}}\approx 159\,000$ y $\varepsilon/D = 0{,}00056$; Swamee-Jain da $f\approx 0{,}0195$.
>
> Fricción: $h_f = 0{,}0195\cdot\dfrac{100}{0{,}08}\cdot 0{,}202\approx 4{,}9$ m. Accesorios: $(2\cdot 0{,}9 + 10)\cdot 0{,}202\approx 2{,}4$ m.
>
> Pérdida total: unos 7,3 m de columna de agua, es decir 0,72 bar.

## Cómo varían las pérdidas

En régimen turbulento las pérdidas crecen con el **cuadrado** del caudal, y para un mismo caudal son inversamente proporcionales a la **quinta potencia** del diámetro:

$$h_f\propto\frac{Q^2}{D^5}$$

Aumentar el diámetro un 20 % reduce las pérdidas a menos de la mitad. La elección del diámetro es un compromiso entre el costo de la cañería y el de la energía de bombeo. Velocidades habituales: 1 a 3 m/s en agua y 10 a 20 m/s en aire comprimido y gases.

## Curva del sistema

La altura que debe vencer una bomba es la suma de una parte estática (desnivel y diferencia de presiones) y las pérdidas:

$$H_{sistema} = H_{est} + k\,Q^2$$

Su intersección con la curva de la bomba da el **punto de funcionamiento**.

## Tuberías en serie y en paralelo

| Disposición | Caudal | Pérdida |
|---|---|---|
| Serie | el mismo en todos los tramos | se suman |
| Paralelo | se suman | la misma en todas las ramas |

Las redes malladas se resuelven por iteración con el método de Hardy Cross o con programas de cálculo.

## Golpe de ariete

Cerrar una válvula de golpe detiene la columna de líquido y genera una onda de sobrepresión que viaja a la velocidad del sonido en el fluido $a$ (unos 1000 a 1400 m/s en agua). Para un cierre instantáneo, la fórmula de Joukowsky da

$$\Delta p = \rho\,a\,\Delta v$$

> [!WARNING]
> Detener agua que circula a 2 m/s genera unos 20 bar de sobrepresión, suficientes para reventar una cañería. Se evita con cierres lentos, válvulas de alivio, chimeneas de equilibrio o tanques hidroneumáticos.
