+++
title = "Dibujo técnico y CAD"
summary = "Proyecciones ortogonales, vistas, cortes y secciones, acotación, escalas, tolerancias y acabados en el plano, roscas, planos de conjunto y modelado paramétrico en CAD."
branch = "Mecanismos y tecnología mecánica"
order = 6
level = "Básico"
tags = ["dibujo técnico", "vistas", "acotación", "CAD", "planos", "normas IRAM"]
+++

## El lenguaje de la ingeniería

Un plano comunica, sin ambigüedad, la **forma**, las **dimensiones** y las **especificaciones** de una pieza a quien la fabrica o la controla. Por eso está **normalizado**: en Argentina rigen las normas **IRAM**, alineadas en general con las **ISO**.

| Tipo de plano | Contenido |
|---|---|
| De pieza (de despiece) | una sola pieza, con todo lo necesario para fabricarla |
| De conjunto | cómo se ensamblan las piezas; lista de materiales |
| De montaje | instrucciones de armado o instalación |
| Esquema | funcionamiento, con símbolos (eléctrico, neumático, hidráulico) |
| Croquis | dibujo a mano alzada, sin escala rigurosa |

## Formatos y escalas

Los formatos de la serie A guardan la proporción $1 : \sqrt{2}$; cada uno es la mitad del anterior.

| Formato | Medidas (mm) |
|---|---|
| A0 | 841 × 1189 |
| A1 | 594 × 841 |
| A2 | 420 × 594 |
| A3 | 297 × 420 |
| A4 | 210 × 297 |

El **rótulo**, en el ángulo inferior derecho, indica la denominación, el número de plano, la escala, el material, las fechas, las firmas y las revisiones.

$$\text{escala} = \frac{\text{medida en el dibujo}}{\text{medida real}}$$

| Tipo | Escalas normalizadas |
|---|---|
| Natural | 1:1 |
| De reducción | 1:2, 1:5, 1:10, 1:20, 1:50, 1:100 |
| De ampliación | 2:1, 5:1, 10:1, 20:1 |

> [!IMPORTANT]
> Las cotas indican siempre la **medida real** de la pieza, cualquiera sea la escala. Nunca se mide sobre el plano para obtener una dimensión.

## Tipos de línea

| Línea | Uso |
|---|---|
| Continua gruesa | aristas y contornos visibles |
| Continua fina | líneas de cota y auxiliares, rayados, fondos de rosca |
| De trazos | aristas y contornos ocultos |
| De trazo y punto fina | ejes de simetría y de revolución |
| De trazo y punto gruesa en los extremos | traza del plano de corte |
| Continua fina a mano alzada o en zigzag | límite de vistas y cortes parciales |

## Proyecciones ortogonales

Una pieza tridimensional se representa con **vistas**: proyecciones perpendiculares sobre planos.

| Vista | Se observa desde |
|---|---|
| Anterior (alzado) | el frente; la principal |
| Superior (planta) | arriba |
| Lateral izquierda | la izquierda |
| Lateral derecha | la derecha |
| Inferior | abajo |
| Posterior | atrás |

### Dos métodos

| Método | Disposición | Uso |
|---|---|---|
| **ISO E** (primer diedro) | la vista superior se dibuja **debajo** del alzado; la lateral izquierda, a la **derecha** | Europa, Argentina (IRAM) |
| **ISO A** (tercer diedro) | la vista superior va **arriba**; la lateral izquierda, a la **izquierda** | Estados Unidos |

El método se indica en el rótulo con el símbolo del tronco de cono.

> [!WARNING]
> Leer un plano con el método equivocado produce la pieza **espejada**. Verificar siempre el símbolo del rótulo.

### Elección de las vistas

- El alzado es la vista que **más información** da, con la pieza en posición de trabajo o de fabricación.
- Se dibujan las **mínimas** vistas necesarias: una pieza de revolución suele necesitar una sola, con el símbolo de diámetro (⌀).
- Se prefieren las vistas con menos líneas ocultas.

## Perspectivas

Muestran la pieza en una sola imagen. Ayudan a interpretar, pero no reemplazan a las vistas.

| Perspectiva | Ejes | Reducción |
|---|---|---|
| Isométrica | tres ejes a 120° | igual en los tres (0,816; en la práctica se dibuja a 1) |
| Dimétrica | dos ángulos iguales | dos ejes iguales |
| Caballera | un eje a 45° | ½ en el eje de profundidad |
| Cónica | con puntos de fuga | variable; se usa en arquitectura |

## Cortes y secciones

Las líneas ocultas confunden. Un **corte** imagina la pieza cortada por un plano, retira la parte más cercana y dibuja lo que queda: se ve el interior.

| Tipo | Descripción |
|---|---|
| Corte total | el plano atraviesa toda la pieza |
| Semicorte | media vista y medio corte, en piezas simétricas |
| Corte parcial (rotura) | solo una zona |
| Corte por planos paralelos (quebrado) | el plano cambia de nivel para pasar por varios detalles |
| Sección | solo la superficie cortada, sin lo que hay detrás |
| Sección abatida | la sección se gira y se dibuja sobre la misma vista |

La superficie cortada se **raya** con líneas finas a 45°. En un conjunto, las piezas contiguas llevan rayados de distinta inclinación o separación.

> [!NOTE]
> **No se cortan longitudinalmente**, aunque el plano pase por ellos: ejes macizos, tornillos, tuercas, arandelas, pasadores, chavetas, remaches, bolas y rodillos de rodamientos, nervios y rayos de ruedas. Se dibujan en vista para no confundir.

## Acotación

Las **cotas** dan las dimensiones. Elementos: línea de cota, líneas auxiliares, flechas y cifra.

### Reglas

| Regla | |
|---|---|
| Cada medida se acota **una sola vez** | y en la vista que mejor la muestra |
| No debe faltar ni sobrar ninguna cota | el operario no debe calcular |
| No se acota sobre aristas ocultas | hacer un corte |
| Las cifras se leen desde abajo y desde la derecha | |
| Las cotas se ubican fuera de la figura cuando es posible | |
| Unidades | milímetros, sin escribir la unidad |
| No cerrar la cadena de cotas | se omite una, la menos importante |

### Símbolos

| Símbolo | Significado |
|---|---|
| ⌀ | diámetro |
| R | radio |
| □ | lado de un cuadrado |
| S⌀, SR | diámetro y radio de una esfera |
| M | rosca métrica |
| 2 × 45° | chaflán |

### Sistemas de acotación

| Sistema | Idea | Efecto en los errores |
|---|---|---|
| En serie (en cadena) | cada cota parte de la anterior | se **acumulan** |
| En paralelo | todas desde una **referencia** común | no se acumulan |
| Combinada | mezcla ambas | la más usual |
| Por coordenadas | una tabla de posiciones X, Y | práctica para control numérico |

> [!TIP]
> Acotar pensando en la **función** y en la **fabricación**: las cotas funcionales (las que determinan que la pieza trabaje) se indican directamente y con su tolerancia; las demás, desde las superficies de referencia que se usarán al mecanizar.

## Tolerancias y acabado superficial

Ninguna pieza sale con la medida exacta. La **tolerancia** indica la variación admisible.

| Indicación | Significado |
|---|---|
| $25 \pm 0{,}1$ | entre 24,9 y 25,1 mm |
| $25^{+0,021}_{\ 0}$ | entre 25,000 y 25,021 mm |
| ⌀25 H7 | agujero según el sistema ISO de ajustes |
| ⌀25 H7/g6 | ajuste de agujero y eje |

Las cotas sin tolerancia propia se rigen por la **tolerancia general** del rótulo (por ejemplo, ISO 2768-m).

### Tolerancias geométricas

Limitan errores de forma y de posición. Se indican en un **rectángulo de tolerancia** con el símbolo, el valor y la referencia.

| Grupo | Características |
|---|---|
| Forma | rectitud, planitud, redondez, cilindricidad |
| Orientación | paralelismo, perpendicularidad, inclinación |
| Posición | posición, concentricidad, simetría |
| Oscilación | oscilación circular y total |

### Rugosidad

Se indica con el símbolo de acabado y el valor de $R_a$ (rugosidad media aritmética) en micrómetros.

| $R_a$ (μm) | Proceso típico |
|---|---|
| 12,5 a 25 | desbaste, fundición |
| 3,2 a 6,3 | torneado, fresado |
| 0,8 a 1,6 | torneado fino, rectificado |
| 0,1 a 0,4 | rectificado fino, bruñido |
| menos de 0,1 | lapeado, pulido |

Un acabado más fino cuesta más: se especifica solo donde la función lo exige.

## Elementos normalizados

Se representan de forma **simplificada**, no como se ven.

| Elemento | Representación | Designación |
|---|---|---|
| Rosca exterior | cresta en línea gruesa; fondo en línea fina | M12 × 1,75 |
| Rosca interior en corte | fondo en línea gruesa; cresta en línea fina | M12 |
| Tornillo de cabeza hexagonal | | ISO 4014 – M10 × 50 – 8.8 |
| Engranaje | circunferencia primitiva en trazo y punto | módulo y número de dientes en una tabla |
| Resorte | simplificado, con datos en tabla | |
| Rodamiento | contorno con una cruz | 6205 |
| Soldadura | símbolo sobre una línea de referencia con flecha | tipo, tamaño y longitud |

En la designación del tornillo, «8.8» es la **clase de resistencia**: 800 MPa de resistencia a la tracción y límite de fluencia del 80 % de ese valor.

## Plano de conjunto

Muestra todas las piezas montadas. Cada una lleva un **número de posición** (globo) que remite a la **lista de materiales**:

| Pos. | Cant. | Denominación | Material / Norma |
|---|---|---|---|
| 1 | 1 | Carcasa | fundición gris EN-GJL-250 |
| 2 | 1 | Eje | acero SAE 1045 |
| 3 | 2 | Rodamiento 6205 | ISO 15 |
| 4 | 4 | Tornillo M8 × 25 | ISO 4762 – 8.8 |

Solo se acotan las medidas generales y las de montaje.

## Diseño asistido por computadora (CAD)

| Generación | Idea |
|---|---|
| CAD 2D | reemplaza al tablero: se dibujan líneas |
| CAD 3D **paramétrico** | se construye un **modelo sólido**; los planos se generan a partir de él |

### Modelado paramétrico

1. Dibujar un **croquis** 2D sobre un plano.
2. Aplicar **restricciones** geométricas (paralela, tangente, concéntrica) y **cotas** hasta definirlo por completo.
3. Crear una **operación**: extrusión, revolución, barrido.
4. Agregar más operaciones: agujeros, redondeos, chaflanes, patrones.

El **árbol de operaciones** guarda el historial. Cambiar una cota **regenera** el modelo, y con él los planos y los conjuntos.

| Entorno | Función |
|---|---|
| Pieza | modelar cada componente |
| Ensamblaje | relacionar piezas, detectar interferencias, simular movimientos |
| Plano | generar vistas, cortes y cotas a partir del modelo |
| Chapa metálica | desarrollos y plegados |
| Simulación | elementos finitos, movimiento, flujo |

> [!TIP]
> Un buen modelo captura la **intención de diseño**: si un agujero debe estar siempre centrado, se lo restringe al centro en lugar de acotarlo con un número. Así el modelo sigue siendo correcto cuando cambian las dimensiones.

### Del modelo a la fabricación

| Sigla | Significado | Uso |
|---|---|---|
| CAD | diseño asistido | modelar y documentar |
| CAE | ingeniería asistida | simular: tensiones, temperaturas, fluidos |
| CAM | fabricación asistida | generar las trayectorias y el código G para control numérico |
| PDM / PLM | gestión de datos y del ciclo de vida | versiones, revisiones, aprobaciones |

| Formato | Uso |
|---|---|
| STEP (`.step`, `.stp`) | intercambio neutro de sólidos |
| IGES | intercambio de superficies (antiguo) |
| STL, 3MF | malla de triángulos para impresión 3D |
| DXF, DWG | dibujos 2D; corte por láser |
| PDF | distribución de planos |

Programas habituales: SolidWorks, Inventor, Fusion, Solid Edge, Creo, CATIA, NX; AutoCAD en 2D; y los libres FreeCAD y LibreCAD.

> [!WARNING]
> El plano sigue siendo, en la mayoría de los talleres, el documento **contractual**: lo que no figura en él (tolerancias, material, tratamiento, acabado) no puede exigirse. El modelo 3D no reemplaza esa información.
