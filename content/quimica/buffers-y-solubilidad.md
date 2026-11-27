+++
title = "Buffers y equilibrios de solubilidad"
summary = "Efecto del ion común, soluciones amortiguadoras, ecuación de Henderson-Hasselbalch, curvas de titulación y producto de solubilidad."
branch = "Equilibrio químico"
order = 2
level = "Avanzado"
tags = ["buffer", "Henderson-Hasselbalch", "titulación", "producto de solubilidad", "ion común"]
+++

## Efecto del ion común

La ionización de un electrolito débil **disminuye** si se agrega un electrolito fuerte que comparte un ion con él. Es una aplicación del principio de Le Châtelier.

> [!NOTE]
> Al agregar acetato de sodio a una solución de ácido acético,
>
> CH₃COOH ⇌ H⁺ + CH₃COO⁻
>
> el exceso de acetato desplaza el equilibrio hacia la izquierda: hay menos H⁺ y el pH sube.

## Soluciones amortiguadoras

Un **buffer** resiste los cambios de pH cuando se le agregan cantidades moderadas de ácido o de base.

> [!IMPORTANT]
> Se forma con un **ácido débil y su base conjugada** (o una base débil y su ácido conjugado), en concentraciones comparables.

| Buffer | Ácido | Base conjugada |
|---|---|---|
| Acetato | CH₃COOH | CH₃COO⁻ |
| Bicarbonato | H₂CO₃ | HCO₃⁻ |
| Fosfato | H₂PO₄⁻ | HPO₄²⁻ |
| Amonio | NH₄⁺ | NH₃ |

**Cómo actúa**:

- Si se agrega ácido, la base conjugada lo consume: A⁻ + H⁺ → HA.
- Si se agrega base, el ácido la consume: HA + OH⁻ → A⁻ + H₂O.

### Ecuación de Henderson-Hasselbalch

> [!IMPORTANT]
> $$\text{pH} = \text{p}K_a + \log\frac{[\text{A}^-]}{[\text{HA}]}$$

Si las concentraciones son iguales, $\text{pH} = \text{p}K_a$.

> [!NOTE]
> Buffer con ácido acético $0{,}10$ M y acetato de sodio $0{,}20$ M ($\text{p}K_a = 4{,}74$).
>
> $\text{pH} = 4{,}74 + \log\dfrac{0{,}20}{0{,}10} = 5{,}04$.

> [!NOTE]
> A $1{,}00$ L de ese buffer se agregan $0{,}010$ mol de HCl.
>
> El acetato baja a $0{,}19$ mol y el ácido sube a $0{,}11$ mol: $\text{pH} = 4{,}74 + \log\dfrac{0{,}19}{0{,}11} = 4{,}98$.
>
> El pH cambió $0{,}06$. En agua pura habría pasado de $7{,}00$ a $2{,}00$.

### Capacidad amortiguadora

- Es mayor cuanto **más concentrados** están los componentes.
- Es máxima cuando $[\text{A}^-] = [\text{HA}]$.
- Un buffer es eficaz en el rango $\text{pH} = \text{p}K_a \pm 1$.

Para preparar un buffer de cierto pH, se elige un ácido con $\text{p}K_a$ cercano.

La **sangre** mantiene su pH entre 7,35 y 7,45 con el sistema ácido carbónico/bicarbonato, regulado por la respiración (que elimina CO₂) y los riñones.

## Curvas de titulación

Muestran el pH en función del volumen de titulante agregado. En el **punto de equivalencia** se agregó la cantidad estequiométrica.

| Titulación | pH en el punto de equivalencia | Indicador adecuado |
|---|---|---|
| Ácido fuerte con base fuerte | 7 | varios (fenolftaleína, azul de bromotimol) |
| Ácido débil con base fuerte | > 7 | fenolftaleína |
| Base débil con ácido fuerte | < 7 | rojo de metilo, naranja de metilo |

En la titulación de un ácido débil con una base fuerte:

1. **Inicio**: pH de un ácido débil.
2. **Antes de la equivalencia**: hay un buffer. A mitad de camino, $\text{pH} = \text{p}K_a$.
3. **Equivalencia**: solo queda la base conjugada, que se hidroliza: pH > 7.
4. **Después**: el exceso de base fuerte fija el pH.

> [!NOTE]
> Se titulan $25{,}0$ mL de ácido acético $0{,}100$ M con NaOH $0{,}100$ M.
>
> Equivalencia a $25{,}0$ mL. Con $12{,}5$ mL (la mitad), $\text{pH} = \text{p}K_a = 4{,}74$.

Los **ácidos polipróticos** (como H₃PO₄) muestran un salto por cada protón.

Un **indicador** es un ácido débil cuyas formas ácida y básica tienen distinto color. Cambia cerca de su $\text{p}K_a$, y se elige para que ese cambio coincida con el salto de la curva.

## Equilibrios de solubilidad

Una sal poco soluble establece un equilibrio entre el sólido y sus iones:

$$\text{A}_m\text{B}_n(s) \rightleftharpoons m\,\text{A}^{n+}(ac) + n\,\text{B}^{m-}(ac)$$

> [!IMPORTANT]
> **Producto de solubilidad**: $K_{ps} = [\text{A}^{n+}]^m[\text{B}^{m-}]^n$

| Sal | $K_{ps}$ (25 °C) |
|---|---|
| AgCl | $1{,}8 \times 10^{-10}$ |
| BaSO₄ | $1{,}1 \times 10^{-10}$ |
| CaCO₃ | $3{,}4 \times 10^{-9}$ |
| PbCl₂ | $1{,}7 \times 10^{-5}$ |
| Mg(OH)₂ | $5{,}6 \times 10^{-12}$ |

### Solubilidad molar

Es la cantidad de moles de sal que se disuelven por litro, $s$.

> [!NOTE]
> AgCl: $K_{ps} = s \cdot s = s^2 \Rightarrow s = \sqrt{1{,}8 \times 10^{-10}} = 1{,}3 \times 10^{-5}$ M.

> [!NOTE]
> Mg(OH)₂ ⇌ Mg²⁺ + 2 OH⁻: $K_{ps} = s(2s)^2 = 4s^3$.
>
> $s = \sqrt[3]{\dfrac{5{,}6 \times 10^{-12}}{4}} = 1{,}1 \times 10^{-4}$ M.

> [!WARNING]
> Solo se pueden comparar solubilidades mirando $K_{ps}$ entre sales del **mismo tipo** (igual proporción de iones). Si no, hay que calcular $s$.

### Predicción de precipitación

Se calcula el producto iónico $Q$ con las concentraciones iniciales:

| Comparación | Resultado |
|---|---|
| $Q < K_{ps}$ | solución insaturada: no precipita |
| $Q = K_{ps}$ | saturada |
| $Q > K_{ps}$ | precipita hasta que $Q = K_{ps}$ |

> [!NOTE]
> Se mezclan volúmenes iguales de AgNO₃ $2{,}0 \times 10^{-4}$ M y NaCl $2{,}0 \times 10^{-4}$ M. Al mezclar, cada concentración se reduce a la mitad.
>
> $Q = (1{,}0 \times 10^{-4})^2 = 1{,}0 \times 10^{-8} > 1{,}8 \times 10^{-10}$: precipita AgCl.

### Factores que afectan la solubilidad

| Factor | Efecto |
|---|---|
| Ion común | **disminuye** la solubilidad |
| pH | las sales de aniones básicos (carbonatos, hidróxidos, fosfatos) son más solubles en medio ácido |
| Formación de complejos | **aumenta** la solubilidad |
| Temperatura | cambia $K_{ps}$ |

> [!NOTE]
> Solubilidad de AgCl en NaCl $0{,}10$ M: $K_{ps} = s(0{,}10 + s) \approx 0{,}10\,s$.
>
> $s = 1{,}8 \times 10^{-9}$ M: unas $7000$ veces menor que en agua pura.

El efecto del pH explica la caries (el ácido disuelve la hidroxiapatita del esmalte), la formación de cavernas en la piedra caliza y el daño de la lluvia ácida al mármol.

**Precipitación selectiva**: agregando lentamente un reactivo, precipita primero la sal menos soluble, lo que permite separar iones.

> [!TIP]
> En un buffer no hace falta la tabla ICE: basta Henderson-Hasselbalch con los moles de ácido y de base conjugada después de la reacción de neutralización.
