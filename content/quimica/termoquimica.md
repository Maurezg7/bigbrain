+++
title = "Termoquímica"
summary = "Energía, calor y trabajo en las reacciones, entalpía, calorimetría, ley de Hess, entalpías de formación y energías de enlace."
branch = "Termoquímica y cinética"
order = 1
level = "Intermedio"
tags = ["entalpía", "ley de Hess", "calorimetría", "exotérmica", "endotérmica"]
+++

## Energía en las reacciones

Toda reacción química intercambia energía con su entorno, casi siempre como calor.

- **Sistema**: la parte que se estudia (los reactivos y productos).
- **Entorno**: todo lo demás.

**Primera ley de la termodinámica**: la energía no se crea ni se destruye.

$$\Delta U = q + w$$

con $q$ el calor **absorbido** por el sistema y $w$ el trabajo realizado **sobre** él (convención usual en química).

## Entalpía

A presión constante (la condición habitual en el laboratorio), el calor intercambiado es el cambio de **entalpía**:

> [!IMPORTANT]
> $$\Delta H = q_p = H_{productos} - H_{reactivos}$$

| | Exotérmica | Endotérmica |
|---|---|---|
| Signo | $\Delta H < 0$ | $\Delta H > 0$ |
| Calor | lo libera | lo absorbe |
| El entorno | se calienta | se enfría |
| Ejemplos | combustión, neutralización | fusión del hielo, fotosíntesis |

La entalpía es una **función de estado**: su variación depende solo de los estados inicial y final.

### Ecuaciones termoquímicas

$$\text{CH}_4(g) + 2\,\text{O}_2(g) \to \text{CO}_2(g) + 2\,\text{H}_2\text{O}(l) \qquad \Delta H = -890\ \text{kJ}$$

Reglas:

1. $\Delta H$ es proporcional a la cantidad: si se duplican los coeficientes, se duplica.
2. Si se **invierte** la reacción, $\Delta H$ cambia de signo.
3. Hay que indicar los **estados** de agregación, porque afectan el valor.

> [!NOTE]
> ¿Cuánto calor libera la combustión de $100$ g de metano ($16{,}04$ g/mol)?
>
> $n = \dfrac{100}{16{,}04} = 6{,}23$ mol; $q = 6{,}23 \cdot 890 = 5550$ kJ liberados.

## Calorimetría

Mide el calor de una reacción a partir del cambio de temperatura:

$$q = m\,c\,\Delta T$$

$c$ es el **calor específico**: para el agua, $4{,}184$ J/(g·°C).

En un calorímetro aislado, el calor que libera la reacción lo absorbe la solución: $q_{reacción} = -q_{solución}$.

> [!NOTE]
> Se mezclan $50{,}0$ mL de HCl $1{,}00$ M con $50{,}0$ mL de NaOH $1{,}00$ M. La temperatura sube de $25{,}0$ a $31{,}8$ °C. Se toma la solución como $100$ g de agua.
>
> $q_{solución} = 100 \cdot 4{,}184 \cdot 6{,}8 = 2845$ J.
>
> Reaccionaron $0{,}0500$ mol: $\Delta H = -\dfrac{2{,}845}{0{,}0500} = -57$ kJ/mol.

- **Calorímetro a presión constante** (vaso aislado): mide $\Delta H$.
- **Bomba calorimétrica** (volumen constante): mide $\Delta U$; se usa para combustiones y para el valor energético de los alimentos.

## Ley de Hess

> [!IMPORTANT]
> Si una reacción puede expresarse como suma de varias etapas, su $\Delta H$ es la suma de los $\Delta H$ de esas etapas.

Permite calcular entalpías que no se pueden medir directamente.

> [!NOTE]
> Hallar $\Delta H$ de C(grafito) + ½ O₂(g) → CO(g), sabiendo:
>
> (1) C(grafito) + O₂(g) → CO₂(g), $\Delta H = -393{,}5$ kJ
>
> (2) CO(g) + ½ O₂(g) → CO₂(g), $\Delta H = -283{,}0$ kJ
>
> Se suma (1) con la **inversa** de (2):
>
> $\Delta H = -393{,}5 + 283{,}0 = -110{,}5$ kJ.

## Entalpía estándar de formación

$\Delta H_f^0$ es el cambio de entalpía al formar **un mol** de un compuesto a partir de sus elementos en su estado estándar (1 bar; por convención se tabula a 25 °C).

La de un **elemento en su forma más estable es cero**: O₂(g), C(grafito), Fe(s).

| Sustancia | $\Delta H_f^0$ (kJ/mol) |
|---|---|
| H₂O(l) | −285,8 |
| H₂O(g) | −241,8 |
| CO₂(g) | −393,5 |
| CO(g) | −110,5 |
| CH₄(g) | −74,8 |
| C₂H₅OH(l) | −277,7 |
| NH₃(g) | −45,9 |
| NaCl(s) | −411,2 |

> [!IMPORTANT]
> $$\Delta H^0_{reacción} = \sum n\,\Delta H_f^0(\text{productos}) - \sum n\,\Delta H_f^0(\text{reactivos})$$

> [!NOTE]
> Combustión del metano: CH₄(g) + 2 O₂(g) → CO₂(g) + 2 H₂O(l)
>
> $\Delta H^0 = [-393{,}5 + 2(-285{,}8)] - [-74{,}8 + 0] = -890{,}3$ kJ.

## Energías de enlace

La **energía de enlace** es la necesaria para romper un mol de enlaces en fase gaseosa. Romper enlaces **absorbe** energía; formarlos la **libera**.

$$\Delta H \approx \sum E(\text{enlaces rotos}) - \sum E(\text{enlaces formados})$$

| Enlace | Energía (kJ/mol) |
|---|---|
| H–H | 436 |
| C–H | 413 |
| C–C | 347 |
| C=C | 614 |
| O–H | 463 |
| O=O | 498 |
| C=O (en CO₂) | 799 |
| N≡N | 945 |

> [!NOTE]
> H₂(g) + ½ O₂(g) → H₂O(g)
>
> Rotos: $436 + \tfrac{1}{2}(498) = 685$. Formados: $2(463) = 926$.
>
> $\Delta H \approx 685 - 926 = -241$ kJ, muy cerca del valor tabulado.

Es una **estimación**, porque las energías tabuladas son promedios, y solo vale para gases.

## Combustibles

| Combustible | Calor de combustión (kJ/g) |
|---|---|
| Hidrógeno | 142 |
| Gas natural (metano) | 55 |
| Nafta | ~47 |
| Etanol | 30 |
| Carbón | ~30 |
| Madera | ~18 |

En nutrición: los carbohidratos y las proteínas aportan cerca de $17$ kJ/g (4 kcal/g), y las grasas $38$ kJ/g (9 kcal/g). $1\ \text{kcal} = 4{,}184$ kJ.

> [!WARNING]
> El orden en las fórmulas es opuesto: con entalpías de formación es **productos menos reactivos**; con energías de enlace es **rotos (reactivos) menos formados (productos)**.

> [!TIP]
> Para aplicar la ley de Hess, ubicar cada sustancia de la reacción buscada en las ecuaciones dato, e invertirlas o multiplicarlas hasta que al sumar se cancele todo lo demás.
