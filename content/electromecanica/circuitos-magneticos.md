+++
title = "Circuitos magnéticos"
summary = "Fuerza magnetomotriz, reluctancia, curva de magnetización, histéresis, entrehierros e inductancia."
branch = "Electromagnetismo"
order = 2
level = "Intermedio"
tags = ["reluctancia", "flujo", "histéresis", "entrehierro", "inductancia"]
+++

## Analogía con el circuito eléctrico

Transformadores, motores, relés y contactores guían el flujo magnético por un núcleo de hierro, igual que un cable guía la corriente. Esa semejanza permite calcularlos como si fueran circuitos.

| Circuito eléctrico | Circuito magnético |
|---|---|
| Tensión $V$ (V) | Fuerza magnetomotriz $\mathcal F = N I$ (A·vuelta) |
| Corriente $I$ (A) | Flujo $\Phi$ (Wb) |
| Resistencia $R = \dfrac{l}{\sigma A}$ | Reluctancia $\mathcal R = \dfrac{l}{\mu A}$ |
| $V = I R$ | $\mathcal F = \Phi\,\mathcal R$ (ley de Hopkinson) |

Las magnitudes locales son la inducción $B = \Phi/A$ (en teslas) y la intensidad de campo $H = NI/l$ (en A/m), relacionadas por $B = \mu H$.

## Curva de magnetización e histéresis

![Lazo de histéresis de un material ferromagnético con saturación, remanencia y campo coercitivo](/media/electromecanica/histeresis.svg)

*Lazo de histéresis: el área encerrada es la energía perdida en cada ciclo.*

En el hierro, $\mu$ no es constante. Al aumentar $H$:

1. la inducción crece rápido (zona lineal, permeabilidad alta);
2. aparece el **codo** de la curva;
3. el material se **satura**: más corriente casi no produce más flujo.

Las máquinas se diseñan para trabajar cerca del codo, entre 1,2 y 1,7 T en chapa de acero al silicio. Al disminuir $H$, la inducción no vuelve por el mismo camino: queda una **remanencia** $B_r$ y hace falta un campo **coercitivo** $H_c$ para anularla.

| Material | Lazo | Uso |
|---|---|---|
| Blando (acero al silicio, ferritas) | angosto | núcleos de transformadores y motores |
| Duro (neodimio, ferrita dura, alnico) | ancho | imanes permanentes |

## Pérdidas en el hierro

- **Histéresis**: proporcionales a la frecuencia y al área del lazo, $P_h \propto f\,B_{max}^{n}$ con $n$ entre 1,6 y 2.
- **Corrientes parásitas**: $P_e \propto f^2 B_{max}^2 e^2$, donde $e$ es el espesor de la chapa. Por eso los núcleos se arman con chapas delgadas (0,35 a 0,5 mm) aisladas entre sí.

## El entrehierro

Un pequeño corte de aire en el núcleo tiene una reluctancia enorme, porque $\mu_0$ es miles de veces menor que la permeabilidad del hierro.

> [!NOTE]
> Núcleo de $l = 40$ cm, $A = 10\ \text{cm}^2$, $\mu_r = 2000$, con un entrehierro de 1 mm y una bobina de 500 vueltas. Se busca $B = 1$ T.
>
> $\mathcal R_{Fe} = \dfrac{0{,}4}{2000\cdot 4\pi\cdot10^{-7}\cdot 10^{-3}} \approx 1{,}59\cdot10^{5}$ y $\mathcal R_{aire} = \dfrac{10^{-3}}{4\pi\cdot10^{-7}\cdot 10^{-3}} \approx 7{,}96\cdot10^{5}$ A·vuelta/Wb.
>
> $\Phi = B A = 10^{-3}$ Wb, entonces $NI = 10^{-3}\cdot 9{,}55\cdot10^{5} \approx 955$ A·vuelta e $I \approx 1{,}9$ A.
>
> Un milímetro de aire pide cinco veces más corriente que 40 cm de hierro.

El entrehierro es inevitable en los motores (entre estator y rotor) y se agrega a propósito en los inductores para que no saturen y almacenen energía.

## Inductancia y fuerza

La inductancia de una bobina sobre un circuito magnético es

$$L = \frac{N\Phi}{I} = \frac{N^2}{\mathcal R}$$

y la energía almacenada, $W = \tfrac12 L I^2$. La fuerza con que un electroimán atrae su armadura a través de un entrehierro de área $A$ es

$$F = \frac{B^2 A}{2\mu_0}$$

Con $B = 1$ T son unos 40 N por cada cm²: de ahí la fuerza de contactores, frenos y electroimanes de izaje.
