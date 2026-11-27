+++
title = "Corrosión y protección"
summary = "Mecanismo electroquímico, serie galvánica, formas de corrosión, velocidad de corrosión, protección catódica, recubrimientos, inhibidores y diseño."
branch = "Materiales"
order = 5
level = "Intermedio"
tags = ["corrosión", "protección catódica", "galvanizado", "serie galvánica", "pasivación"]
+++

## Qué es

La **corrosión** es el deterioro de un metal por reacción con su ambiente. Los metales se obtienen de sus minerales gastando energía; la corrosión es el camino de regreso al estado estable (óxidos, sulfuros).

Su costo ronda el 3 % del producto bruto de un país, y causa fallas de estructuras, fugas y contaminación.

| Tipo | Medio | Ejemplo |
|---|---|---|
| Húmeda (electroquímica) | un electrolito | herrumbre del acero |
| Seca | gases a alta temperatura | cascarilla en hornos |

## Mecanismo electroquímico

Para que haya corrosión húmeda hacen falta cuatro elementos:

| Elemento | Función |
|---|---|
| **Ánodo** | donde el metal se oxida y se disuelve |
| **Cátodo** | donde ocurre la reducción |
| **Electrolito** | conduce iones entre ambos |
| **Conexión eléctrica** | conduce los electrones por el metal |

Si falta uno, la corrosión se detiene. Todos los métodos de protección actúan sobre alguno.

Para el hierro en agua con oxígeno:

$$\text{ánodo:}\quad \mathrm{Fe \to Fe^{2+} + 2e^-}$$

$$\text{cátodo:}\quad \mathrm{O_2 + 2H_2O + 4e^- \to 4OH^-}$$

Los productos se combinan y se oxidan hasta formar la **herrumbre**, un óxido hidratado ($\mathrm{Fe_2O_3 \cdot nH_2O}$) poroso que no protege.

En medios ácidos, la reacción catódica es el desprendimiento de hidrógeno:

$$\mathrm{2H^+ + 2e^- \to H_2}$$

## Serie galvánica

Ordena los metales según su potencial en un medio dado (aquí, agua de mar).

| Extremo | Metales |
|---|---|
| **Activo** (anódico: se corroe) | magnesio, zinc, aluminio, acero al carbono, fundición |
| Intermedio | plomo, estaño, latón, cobre, bronce |
| **Noble** (catódico: protegido) | acero inoxidable pasivo, titanio, plata, grafito, oro, platino |

> [!IMPORTANT]
> Cuando dos metales distintos están en contacto dentro de un electrolito, el **más activo se corroe** más rápido y protege al más noble. Cuanto más separados están en la serie, mayor es el efecto.

## Pasivación

Algunos metales forman una película de óxido delgada, continua y adherente que frena la corrosión.

| Metal | Película |
|---|---|
| Aluminio | $\mathrm{Al_2O_3}$ |
| Acero inoxidable | óxido de cromo (requiere al menos 10,5 % de Cr) |
| Titanio | $\mathrm{TiO_2}$ |

La película se regenera con oxígeno. Los **cloruros** pueden romperla localmente.

## Formas de corrosión

| Forma | Descripción | Ejemplo |
|---|---|---|
| **Uniforme** | ataque parejo en toda la superficie | acero a la intemperie |
| **Galvánica** | entre dos metales distintos en contacto | tornillo de acero en chapa de cobre |
| **Picado** (*pitting*) | perforaciones localizadas y profundas | inoxidable en agua con cloruros |
| **En rendijas** | en espacios estrechos sin renovación de oxígeno | bajo juntas y arandelas |
| **Intergranular** | a lo largo de los bordes de grano | inoxidable sensibilizado junto a una soldadura |
| **Bajo tensión** | fisuras por tensión de tracción más un medio específico | latón en amoníaco; inoxidable en cloruros calientes |
| Corrosión-fatiga | cargas cíclicas en medio corrosivo | ejes de bombas |
| Erosión-corrosión | un fluido veloz arranca la capa protectora | codos, impulsores |
| Cavitación | el colapso de burbujas daña la superficie | hélices, bombas |
| Selectiva | se disuelve un componente de la aleación | deszincado del latón |
| Fragilización por hidrógeno | el hidrógeno atómico difunde y fragiliza | aceros de alta resistencia |
| Microbiológica | bacterias que alteran el medio | tuberías enterradas |

> [!WARNING]
> Las formas **localizadas** (picado, rendijas, bajo tensión) son las más peligrosas: pierden poco material, son difíciles de detectar y producen fallas repentinas.

### Relación de áreas

En un par galvánico, la corriente anódica se concentra en el área del ánodo.

| Combinación | Resultado |
|---|---|
| Ánodo pequeño y cátodo grande | ataque muy rápido del ánodo |
| Ánodo grande y cátodo pequeño | ataque lento y repartido |

Por eso se usan remaches de un metal **más noble** que la chapa, nunca al revés.

## Velocidad de corrosión

Con la pérdida de masa $\Delta m$ de una probeta de área $A$ y densidad $\rho$, expuesta un tiempo $t$:

$$v = \frac{\Delta m}{\rho \, A \, t}$$

Se expresa en mm/año.

| Velocidad (mm/año) | Calificación |
|---|---|
| menos de 0,05 | excelente |
| 0,05 a 0,5 | aceptable |
| 0,5 a 1,3 | pobre |
| más de 1,3 | inaceptable |

La ley de Faraday relaciona la velocidad con la densidad de corriente $i$:

$$v = \frac{i \, M}{n \, F \, \rho}$$

con $M$ la masa molar, $n$ los electrones intercambiados y $F = 96\,485$ C/mol.

> [!NOTE]
> **Ejemplo.** Hierro ($M = 55{,}85$ g/mol, $n = 2$, $\rho = 7{,}87$ g/cm³) con $i = 10\ \mu\text{A/cm}^2$:
> $$v = \frac{10^{-5} \times 55{,}85}{2 \times 96\,485 \times 7{,}87} = 3{,}68 \times 10^{-10}\ \text{cm/s} \approx 0{,}116\ \text{mm/año}$$

El **sobreespesor de corrosión** se calcula como la velocidad por la vida útil prevista.

### Factores

| Factor | Efecto |
|---|---|
| Oxígeno disuelto | en general, acelera |
| Temperatura | acelera las reacciones |
| pH | los ácidos atacan al acero; los álcalis fuertes, al aluminio y al zinc |
| Cloruros | rompen las películas pasivas |
| Velocidad del fluido | renueva el oxígeno y erosiona |
| Humedad | por encima de 60 a 70 % de humedad relativa comienza la corrosión atmosférica |

## Métodos de protección

### Selección del material

| Ambiente | Material |
|---|---|
| Atmósfera | acero pintado o galvanizado; acero patinable |
| Agua dulce | acero revestido, cobre, plásticos |
| Agua de mar | cuproníquel, bronce al aluminio, titanio, inoxidable dúplex |
| Ácidos oxidantes | acero inoxidable |
| Alimentos | inoxidable AISI 304 o 316 |

El AISI 316 contiene **molibdeno**, que mejora la resistencia al picado frente al 304.

### Recubrimientos

| Tipo | Ejemplos | Cómo protege |
|---|---|---|
| Orgánicos | pinturas, epoxi, poliuretano | barrera |
| Metálicos anódicos | **galvanizado** (zinc sobre acero) | barrera y **sacrificio** |
| Metálicos catódicos | cromado, niquelado, estañado | solo barrera |
| De conversión | fosfatado, anodizado | base para pintura; engrosa el óxido |

> [!IMPORTANT]
> En el **galvanizado**, el zinc es más activo que el acero: si el recubrimiento se raya, el zinc se corroe y protege al acero expuesto. En cambio, un recubrimiento **más noble** (estaño, cromo) con un poro acelera el ataque del acero, que queda como ánodo pequeño.

Un sistema de pintura típico tiene imprimación (con zinc o inhibidores), capa intermedia y terminación. La **preparación de la superficie** (arenado o granallado) determina la mayor parte de su duración.

### Protección catódica

Convierte toda la estructura en **cátodo**.

| Método | Funcionamiento | Usos |
|---|---|---|
| **Ánodos de sacrificio** | se conectan bloques de zinc, aluminio o magnesio, que se corroen en lugar del acero | cascos de barcos, termotanques, tuberías cortas |
| **Corriente impresa** | una fuente de corriente continua y ánodos inertes fuerzan la corriente | gasoductos, tanques, muelles |

Se verifica midiendo el potencial de la estructura: para el acero, más negativo que $-0{,}85$ V respecto del electrodo de cobre/sulfato de cobre.

Se combina siempre con un **revestimiento**: este protege casi toda la superficie y la corriente cubre solo sus fallas.

### Otros métodos

| Método | Idea |
|---|---|
| **Inhibidores** | sustancias que, en baja concentración, frenan la reacción (circuitos cerrados de agua, anticongelantes) |
| Control del medio | eliminar el oxígeno (desaireación del agua de calderas), ajustar el pH, deshumidificar |
| Protección anódica | mantener el metal en la zona pasiva con una corriente controlada |

## Diseño contra la corrosión

| Regla | Motivo |
|---|---|
| Evitar rendijas: soldar a tope en lugar de solapar | no retener electrolito |
| Permitir el drenaje y la ventilación | no acumular agua |
| Aislar los metales distintos con juntas y arandelas no conductoras | cortar el par galvánico |
| Evitar ánodos pequeños junto a cátodos grandes | no concentrar el ataque |
| Evitar cantos vivos | la pintura adelgaza en los bordes |
| Dejar acceso para inspeccionar y repintar | mantenimiento |
| Aliviar las tensiones residuales | prevenir la corrosión bajo tensión |
| Evitar cambios bruscos de dirección en tuberías | reducir la erosión |

## Oxidación a alta temperatura

El metal reacciona directamente con el gas. Que la capa de óxido proteja depende de la **relación de Pilling-Bedworth**: el volumen del óxido dividido por el del metal consumido.

| Relación | Capa |
|---|---|
| menor que 1 | porosa; no protege (magnesio) |
| entre 1 y 2 | continua y protectora (aluminio, cromo) |
| mayor que 2 | se agrieta y se desprende (hierro a alta temperatura) |

Por eso las aleaciones para alta temperatura contienen **cromo** y **aluminio**.

> [!TIP]
> Ante una falla por corrosión, identificar primero la **forma** de ataque: indica el mecanismo y, con él, el remedio. Cambiar por un material «mejor» sin diagnóstico puede empeorar el problema.
