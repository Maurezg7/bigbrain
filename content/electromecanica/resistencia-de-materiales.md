+++
title = "Esfuerzo y deformación"
summary = "Cómo responde un material a una carga: ley de Hooke, diagrama tensión-deformación y factor de seguridad."
branch = "Mecánica y materiales"
order = 1
level = "Intermedio"
tags = ["tensión", "módulo de Young", "fluencia", "factor de seguridad"]
+++

## Esfuerzo y deformación unitaria

Una misma fuerza hace más daño a una barra fina que a una gruesa. Por eso no se trabaja con fuerzas sino con **esfuerzo** (o tensión mecánica), la fuerza por unidad de área:

$$\sigma = \frac{F}{A}$$

Se mide en pascales; en la práctica, en MPa (1 MPa = 1 N/mm²). De la misma manera, el alargamiento se expresa en forma relativa como **deformación unitaria**, adimensional:

$$\varepsilon = \frac{\Delta L}{L_0}$$

## Ley de Hooke

Mientras la carga es moderada, el esfuerzo es proporcional a la deformación y el material recupera su forma al descargarlo (**comportamiento elástico**):

$$\sigma = E\,\varepsilon$$

$E$ es el **módulo de Young** o de elasticidad, una medida de la rigidez del material.

| Material | $E$ (GPa) | Límite de fluencia (MPa) |
|---|---|---|
| Acero estructural | 200 | 250 |
| Aluminio 6061-T6 | 69 | 275 |
| Cobre | 110 | 70 |
| Hormigón (compresión) | 30 | — |

Combinando las tres ecuaciones se obtiene el alargamiento de una barra bajo carga axial:

$$\Delta L = \frac{F\,L_0}{A\,E}$$

## Diagrama tensión-deformación

El **ensayo de tracción** estira una probeta hasta romperla, registrando $\sigma$ contra $\varepsilon$.

![Curva tensión-deformación de un acero dúctil con sus zonas](/media/electromecanica/tension-deformacion.svg)

*Curva típica de un acero dúctil.*

- **Zona elástica**: recta de pendiente $E$. La deformación es reversible.
- **Límite de fluencia** $\sigma_y$: a partir de aquí la deformación es permanente (**plástica**).
- **Resistencia última** $\sigma_u$: el máximo esfuerzo que soporta el material.
- **Rotura**: tras la estricción (el cuello que se forma en la probeta), la pieza se separa.

Los materiales **dúctiles** (acero, cobre) se deforman mucho antes de romperse; los **frágiles** (fundición, vidrio) rompen casi sin aviso.

## Factor de seguridad

Las piezas se diseñan para trabajar muy por debajo de la fluencia:

$$\sigma_{adm} = \frac{\sigma_y}{FS}$$

El **factor de seguridad** $FS$ (típicamente entre 1,5 y 4) cubre incertidumbres en las cargas, el material y la fabricación.

> [!NOTE]
> Una barra de acero de 2 m de largo y 20 mm de diámetro soporta una carga de 30 kN.
>
> Área: $A = \pi \cdot 10^2 \approx 314\ \text{mm}^2$. Esfuerzo: $\sigma = \dfrac{30\,000}{314} \approx 95{,}5$ MPa.
>
> Alargamiento: $\Delta L = \dfrac{30\,000 \cdot 2000}{314 \cdot 200\,000} \approx 0{,}95$ mm.
>
> Factor de seguridad respecto de la fluencia: $FS = \dfrac{250}{95{,}5} \approx 2{,}6$. La barra trabaja en zona elástica con buen margen.

## Otros esfuerzos

Además de tracción y compresión, una pieza puede estar sometida a **corte** ($\tau = F/A$, con la fuerza paralela a la sección), **flexión** (vigas) y **torsión** (ejes). En todos los casos el criterio es el mismo: comparar el esfuerzo máximo con el admisible del material.
