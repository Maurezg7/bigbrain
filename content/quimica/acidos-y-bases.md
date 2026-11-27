+++
title = "Ácidos, bases y pH"
summary = "Qué hace ácida o básica a una solución, cómo se calcula el pH y cómo funciona una neutralización."
branch = "Equilibrio químico"
order = 1
level = "Intermedio"
tags = ["pH", "neutralización", "equilibrio", "buffer"]
+++

## Equilibrio químico

Muchas reacciones son reversibles y llegan a un estado en que las concentraciones dejan de cambiar. Para $a\text{A} + b\text{B} \rightleftharpoons c\text{C} + d\text{D}$, la **constante de equilibrio** es

$$K = \frac{[\text{C}]^c[\text{D}]^d}{[\text{A}]^a[\text{B}]^b}$$

Según el **principio de Le Chatelier**, si se perturba un sistema en equilibrio (cambiando concentración, presión o temperatura), este se desplaza en el sentido que contrarresta el cambio.

## Qué es un ácido y qué una base

Según **Brønsted y Lowry**, un ácido es una especie que cede un protón ($\text{H}^+$) y una base, una que lo acepta:

$$\text{HCl} + \text{H}_2\text{O} \longrightarrow \text{H}_3\text{O}^+ + \text{Cl}^-$$

Los ácidos y bases **fuertes** (HCl, HNO₃, NaOH) se ionizan por completo; los **débiles** (ácido acético, amoníaco) solo en parte, y se caracterizan por su constante $K_a$ o $K_b$.

## La escala de pH

El agua se autoioniza muy poco: $[\text{H}_3\text{O}^+][\text{OH}^-] = K_w = 1{,}0 \times 10^{-14}$ a 25 °C. Para no trabajar con potencias de diez se define

$$\text{pH} = -\log[\text{H}_3\text{O}^+] \qquad \text{pOH} = -\log[\text{OH}^-] \qquad \text{pH} + \text{pOH} = 14$$

![Escala de pH de 0 a 14 con ejemplos cotidianos](/media/quimica/escala-ph.svg)

*Escala de pH a 25 °C. Cada unidad representa un factor 10 en la concentración de $\text{H}_3\text{O}^+$.*

> [!NOTE]
> pH de una solución de HCl 0,01 M: como es un ácido fuerte, $[\text{H}_3\text{O}^+] = 0{,}01 = 10^{-2}$ M y el pH es $2$.
>
> pH de NaOH 0,001 M: $[\text{OH}^-] = 10^{-3}$, pOH $= 3$ y pH $= 14 - 3 = 11$.

## Ácidos débiles

Para un ácido débil HA de concentración inicial $C$, si se ioniza poco:

$$K_a = \frac{[\text{H}_3\text{O}^+][\text{A}^-]}{[\text{HA}]} \approx \frac{x^2}{C} \;\Rightarrow\; [\text{H}_3\text{O}^+] \approx \sqrt{K_a\,C}$$

> [!NOTE]
> Ácido acético 0,1 M ($K_a = 1{,}8 \times 10^{-5}$): $[\text{H}_3\text{O}^+] \approx \sqrt{1{,}8 \times 10^{-6}} \approx 1{,}34 \times 10^{-3}$ M, pH $\approx 2{,}87$. Mucho menos ácido que el HCl de igual concentración (pH 1).

## Neutralización y titulación

Un ácido y una base reaccionan formando sal y agua: $\text{HCl} + \text{NaOH} \to \text{NaCl} + \text{H}_2\text{O}$. En una **titulación** se agrega una solución de concentración conocida hasta el **punto de equivalencia**, donde los moles de ácido y de base se igualan. Para una relación 1:1:

$$M_a V_a = M_b V_b$$

## Soluciones amortiguadoras

Un **buffer** es una mezcla de un ácido débil y su base conjugada que resiste los cambios de pH. Su pH se calcula con la ecuación de Henderson-Hasselbalch:

$$\text{pH} = \text{p}K_a + \log\frac{[\text{A}^-]}{[\text{HA}]}$$

La sangre se mantiene en pH 7,4 gracias al buffer ácido carbónico/bicarbonato.
