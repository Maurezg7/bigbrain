+++
title = "Fundición y conformado plástico"
summary = "Moldeo en arena y a presión, laminación, forja, extrusión, trefilado, trabajo de chapa y manufactura aditiva."
branch = "Mecanismos y tecnología mecánica"
order = 4
level = "Intermedio"
tags = ["fundición", "forja", "laminación", "estampado", "impresión 3D"]
+++

## Cómo se le da forma a una pieza

| Familia | Principio | Ejemplos |
|---|---|---|
| Fundición | se vierte metal líquido en un molde | blocks de motor, carcasas, válvulas |
| Conformado plástico | se deforma el metal sólido | perfiles, bielas, carrocerías |
| Arranque de viruta | se quita material | ejes, alojamientos precisos |
| Unión | se ensamblan partes | estructuras soldadas |
| Aditiva | se agrega material capa por capa | prototipos, piezas complejas |

La elección depende de la forma, el material, la cantidad de piezas y la precisión requerida.

## Fundición

Permite formas complejas y huecas en una sola operación.

| Proceso | Molde | Características |
|---|---|---|
| En arena | arena compactada sobre un modelo; se destruye | cualquier tamaño y aleación; terminación basta |
| En coquilla | metálico, permanente | series medianas, buena terminación |
| A presión (inyección) | metálico, llenado a alta presión | grandes series en aluminio y zamak; paredes finas |
| A la cera perdida | cerámico sobre un modelo de cera | gran precisión; álabes de turbina |
| Centrífuga | molde giratorio | tubos y camisas |

El molde incluye el **bebedero** y los canales por donde entra el metal y los **montantes** o mazarotas, depósitos que alimentan a la pieza mientras se contrae. El modelo se hace más grande para compensar la contracción (del orden del 1 % en fundición gris y 2 % en acero) y con ángulos de salida para poder extraerlo.

El tiempo de solidificación sigue la **regla de Chvorinov**:

$$t = B\left(\frac{V}{A}\right)^2$$

Las partes macizas (mucho volumen y poca superficie) solidifican al final, y ahí aparecen los **rechupes**. Por eso se diseña con espesores uniformes y transiciones suaves.

| Defecto | Causa |
|---|---|
| Rechupe | contracción sin alimentación |
| Porosidad | gases disueltos o atrapados |
| Junta fría | el metal llega demasiado frío |
| Inclusiones | arena o escoria arrastradas |

## Conformado plástico

Deformar el metal más allá de su límite de fluencia mejora sus propiedades: afina el grano y orienta las fibras siguiendo la forma de la pieza.

| | En caliente | En frío |
|---|---|---|
| Temperatura | por encima de la de recristalización | ambiente |
| Fuerzas | menores | mayores |
| Endurecimiento | no | sí, por acritud |
| Precisión y terminación | bajas, con cascarilla | altas |

| Proceso | Descripción | Productos |
|---|---|---|
| Laminación | el material pasa entre rodillos | chapas, perfiles, rieles |
| Forja | compresión con martillo o prensa, libre o con matriz | bielas, cigüeñales, herramientas |
| Extrusión | se empuja a través de una matriz | perfiles de aluminio, tubos |
| Trefilado | se tira a través de una hilera | alambres, barras calibradas |

La fuerza necesaria se estima con la tensión de fluencia media $\bar\sigma_f$ del material; en forja,

$$F\approx K\,\bar\sigma_f\,A$$

con $A$ el área proyectada y $K$ un factor de forma entre 3 y 10 para piezas con rebaba.

## Trabajo de chapa

| Operación | Descripción |
|---|---|
| Corte y punzonado | cizallado entre punzón y matriz |
| Doblado | deformación a lo largo de una línea |
| Embutido | formación de piezas huecas |
| Repujado | conformado sobre un molde giratorio |

La fuerza de corte es el perímetro por el espesor por la resistencia al corte:

$$F = L\,e\,\tau_R$$

> [!NOTE]
> Punzonar un agujero de 20 mm en chapa de acero de 3 mm con $\tau_R = 320$ MPa.
>
> $F = \pi\cdot 20\cdot 3\cdot 320\approx 60{,}3$ kN, unas 6 toneladas.

Al doblar, la chapa se recupera elásticamente un poco (**retorno elástico**), por lo que se dobla de más. El radio interior no debe ser menor que el espesor para que no se fisure.

## Pulvimetalurgia

Polvo metálico compactado en una matriz y luego **sinterizado** a alta temperatura sin llegar a fundir. Da piezas terminadas sin desperdicio: bujes autolubricados, engranajes pequeños, plaquitas de metal duro.

## Polímeros

| Proceso | Productos |
|---|---|
| Inyección | piezas complejas en grandes series |
| Extrusión | caños, perfiles, aislación de cables |
| Soplado | botellas y envases |
| Termoformado | bandejas, carcasas |

## Manufactura aditiva

La impresión 3D construye la pieza capa por capa a partir del modelo digital, sin herramental.

| Tecnología | Material |
|---|---|
| Deposición de filamento (FDM) | termoplásticos |
| Estereolitografía (SLA) | resinas fotosensibles |
| Sinterizado o fusión selectiva por láser | polvos de polímero o de metal |

Conviene para prototipos, piezas únicas y geometrías imposibles por otros medios, como canales internos de refrigeración. Para grandes series sigue siendo más lenta y cara que los procesos tradicionales.
