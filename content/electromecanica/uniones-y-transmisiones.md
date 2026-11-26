+++
title = "Uniones, resortes y transmisiones flexibles"
summary = "Tornillos y precarga, uniones soldadas, resortes, correas, cadenas, acoplamientos, frenos y embragues."
branch = "Mecánica y elementos de máquinas"
order = 6
level = "Intermedio"
tags = ["tornillos", "resortes", "correas", "cadenas", "acoplamientos"]
+++

## Uniones roscadas

Un tornillo es un plano inclinado enrollado: convierte un par de apriete en una gran fuerza axial. La rosca métrica se designa por su diámetro y paso (M12 × 1,75) y la resistencia por su **clase**:

| Clase | Resistencia a la tracción | Límite de fluencia |
|---|---|---|
| 4.6 | 400 MPa | 240 MPa |
| 8.8 | 800 MPa | 640 MPa |
| 10.9 | 1000 MPa | 900 MPa |
| 12.9 | 1200 MPa | 1080 MPa |

El primer número por 100 es la resistencia; el producto de ambos por 10, la fluencia.

### Precarga

Un tornillo bien apretado queda estirado como un resorte y comprime a las piezas. Esa **precarga** $F_i$ es lo que hace funcionar la unión: impide que las piezas se separen o deslicen y reduce la variación de tensión en el tornillo, protegiéndolo de la fatiga. El par de apriete necesario es aproximadamente

$$T\approx K\,F_i\,d$$

con $K\approx 0{,}2$ para roscas secas y $0{,}15$ si están lubricadas.

> [!NOTE]
> Tornillo M12 clase 8.8 (sección resistente $84{,}3\ \text{mm}^2$) precargado al 70 % de la fluencia.
>
> $F_i = 0{,}7\cdot 640\cdot 84{,}3\approx 37{,}8$ kN y $T\approx 0{,}2\cdot 37\,800\cdot 0{,}012\approx 91$ N·m.

> [!WARNING]
> Solo alrededor del 10 % del par de apriete estira el tornillo; el resto vence rozamientos. Lubricar una rosca y aplicar el par previsto para rosca seca puede romperlo.

## Uniones soldadas

En una soldadura de filete se calcula la tensión de corte en la **garganta**, de espesor $a\approx 0{,}707$ veces el cateto:

$$\tau = \frac{F}{a\,l}$$

Las soldaduras resisten bien cargas estáticas pero son sensibles a la fatiga, por las entallas y tensiones residuales que dejan.

## Resortes

Almacenan energía elástica: $F = k\,x$ y $E = \tfrac12 k x^2$. Para un resorte helicoidal de alambre de diámetro $d$, diámetro medio $D$ y $N$ espiras activas:

$$k = \frac{G\,d^4}{8\,D^3 N},\qquad \tau = K_w\,\frac{8\,F\,D}{\pi d^3}$$

donde $K_w$ es el factor de Wahl, que corrige por la curvatura. Resortes en paralelo suman rigideces; en serie, suman las inversas.

## Correas

Transmiten potencia entre ejes alejados, absorben golpes y son silenciosas. Las planas y trapeciales trabajan por **fricción**:

$$\frac{F_1}{F_2} = e^{\mu\theta},\qquad P = (F_1 - F_2)\,v$$

con $F_1$ y $F_2$ las tensiones de los ramales tenso y flojo y $\theta$ el ángulo abrazado. La correa **trapecial** se acuña en la garganta de la polea y multiplica el rozamiento efectivo. Deslizan entre un 1 y un 2 %; las **dentadas** (sincrónicas) no deslizan y mantienen la relación exacta.

## Cadenas

No deslizan y soportan grandes cargas a baja velocidad, pero necesitan lubricación y hacen ruido. La velocidad de la cadena oscila levemente (efecto poligonal), más cuanto menos dientes tiene el piñón; se recomiendan 17 o más.

| | Correa trapecial | Correa dentada | Cadena | Engranajes |
|---|---|---|---|---|
| Relación exacta | no | sí | sí | sí |
| Distancia entre ejes | grande | grande | media | chica |
| Lubricación | no | no | sí | sí |
| Absorbe golpes | sí | algo | poco | no |
| Rendimiento | 94 a 97 % | 97 a 99 % | 96 a 98 % | 96 a 99 % |

## Acoplamientos

Unen dos ejes alineados. Los **rígidos** exigen alineación perfecta; los **flexibles** (de quijada con elastómero, de láminas, de engranajes) toleran pequeños errores y amortiguan; la junta **cardan** admite ángulos grandes. Una buena alineación, hecha con reloj comparador o con láser, alarga la vida de rodamientos y sellos.

## Frenos y embragues

Ambos transmiten par por rozamiento entre superficies; el embrague une dos ejes que giran y el freno une uno con una parte fija. Para un disco con fuerza axial $F$, radio medio $r_m$ y $z$ caras de fricción:

$$T = \mu\,F\,r_m\,z$$

La energía de cada frenada se convierte en calor, por lo que la capacidad térmica y la disipación limitan la frecuencia de maniobras.
