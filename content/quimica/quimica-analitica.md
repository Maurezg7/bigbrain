+++
title = "Química analítica"
summary = "Etapas del análisis, cifras de mérito, gravimetría, volumetrías ácido-base, redox, de precipitación y complejométricas, curvas de titulación y calibración instrumental."
branch = "Química analítica y aplicada"
order = 1
level = "Intermedio"
tags = ["química analítica", "titulación", "gravimetría", "indicadores", "calibración", "EDTA"]
+++

## Qué estudia

La química analítica determina **qué** contiene una muestra y **cuánto**.

| Análisis | Pregunta |
|---|---|
| **Cualitativo** | ¿qué sustancias hay? |
| **Cuantitativo** | ¿en qué cantidad? |

La sustancia que se determina es el **analito**; el resto de la muestra, la **matriz**.

| Métodos | Miden | Ejemplos |
|---|---|---|
| **Clásicos** | masa o volumen | gravimetría, volumetría |
| **Instrumentales** | una propiedad física | espectroscopía, cromatografía, electroquímica |

## Etapas de un análisis

1. **Definir el problema**: qué se necesita saber y con qué exactitud.
2. **Muestreo**: obtener una porción **representativa**.
3. **Preparación**: secar, moler, disolver, eliminar interferencias.
4. **Medición**.
5. **Cálculo** y tratamiento estadístico.
6. **Informe**, con la incertidumbre.

> [!WARNING]
> El muestreo suele ser la mayor fuente de error. Ningún instrumento corrige una muestra que no representa al material.

## Calidad de un resultado

| Concepto | Definición | Se evalúa con |
|---|---|---|
| **Exactitud** | cercanía al valor verdadero | error relativo; materiales de referencia |
| **Precisión** | concordancia entre repeticiones | desviación estándar |
| **Sensibilidad** | cambio de la señal por unidad de concentración | pendiente de la calibración |
| **Selectividad** | capacidad de distinguir el analito de las interferencias | |
| **Límite de detección** | menor concentración distinguible del blanco | $3 s_{\text{blanco}} /$ pendiente |
| **Límite de cuantificación** | menor concentración medible con confianza | $10 s_{\text{blanco}} /$ pendiente |

| Error | Origen | Efecto | Se reduce |
|---|---|---|---|
| **Sistemático** | instrumento descalibrado, método defectuoso | sesga todos los resultados igual | calibrando; con blancos y patrones |
| **Aleatorio** | fluctuaciones incontrolables | dispersa los resultados | promediando repeticiones |

El resultado se informa como media ± intervalo de confianza:

$$\bar{x} \pm \frac{t \, s}{\sqrt{n}}$$

## Gravimetría

Se determina el analito **pesando** un compuesto de composición conocida. En la gravimetría por precipitación:

1. se precipita el analito con un reactivo en exceso;
2. se filtra y se lava;
3. se seca o calcina;
4. se pesa.

El **factor gravimétrico** convierte la masa pesada en masa de analito:

$$\text{factor} = \frac{M_{\text{analito}}}{M_{\text{precipitado}}} \times \frac{\text{moles de analito}}{\text{moles de precipitado}}$$

> [!NOTE]
> **Ejemplo.** 0,5000 g de una muestra con cloruro se tratan con $\mathrm{AgNO_3}$ y se obtienen 0,7134 g de AgCl ($M = 143{,}32$ g/mol; Cl: 35,45).
> $$m_{\mathrm{Cl}} = 0{,}7134 \times \frac{35{,}45}{143{,}32} = 0{,}1765\ \text{g}$$
> $$\%\,\mathrm{Cl} = \frac{0{,}1765}{0{,}5000} \times 100 = 35{,}29\ \%$$

Un buen precipitado es muy insoluble, puro, fácil de filtrar y de composición definida. Los cristales grandes se favorecen precipitando lentamente, en caliente y desde soluciones diluidas.

## Volumetría

Se mide el **volumen** de una solución de concentración conocida (el **titulante**) que reacciona exactamente con el analito.

| Término | Significado |
|---|---|
| **Punto de equivalencia** | se agregó la cantidad estequiométrica de titulante |
| **Punto final** | lo que se observa: el viraje del indicador |
| Error de titulación | diferencia entre ambos |
| **Patrón primario** | sustancia pura y estable con la que se prepara o valora una solución |
| Estandarización | determinar la concentración exacta del titulante |

Patrones primarios habituales: ftalato ácido de potasio (para bases), carbonato de sodio (para ácidos), oxalato de sodio (para permanganato), NaCl (para nitrato de plata).

En el punto de equivalencia, para una reacción $a\,\mathrm{A} + t\,\mathrm{T} \to$ productos:

$$\frac{n_A}{a} = \frac{n_T}{t}$$

| Tipo | Detalle |
|---|---|
| Directa | el titulante reacciona con el analito |
| **Por retroceso** | se agrega un exceso medido de reactivo y se titula lo que sobra |
| Indirecta | se titula un producto de la reacción del analito |

### Ácido-base

> [!NOTE]
> **Ejemplo.** 25,00 mL de vinagre diluido gastan 18,40 mL de NaOH 0,1000 M.
> $$n_{\text{NaOH}} = 0{,}01840 \times 0{,}1000 = 1{,}840 \times 10^{-3}\ \text{mol} = n_{\text{ácido acético}}$$
> $$[\mathrm{CH_3COOH}] = \frac{1{,}840 \times 10^{-3}}{0{,}02500} = 0{,}0736\ \text{M}$$

La **curva de titulación** representa el pH en función del volumen agregado.

| Titulación | pH en la equivalencia | Indicador adecuado |
|---|---|---|
| Ácido fuerte con base fuerte | 7 | casi cualquiera |
| Ácido débil con base fuerte | mayor que 7 | fenolftaleína |
| Base débil con ácido fuerte | menor que 7 | rojo de metilo, naranja de metilo |

En la titulación de un ácido débil, a mitad de camino hacia la equivalencia se cumple $\mathrm{pH} = \mathrm{p}K_a$.

Un **indicador** es un ácido débil cuyas formas ácida y básica tienen distinto color. Vira en el intervalo $\mathrm{p}K_{\text{ind}} \pm 1$, que debe caer dentro del salto de la curva.

| Indicador | Intervalo de viraje | Color ácido → básico |
|---|---|---|
| Naranja de metilo | 3,1 – 4,4 | rojo → amarillo |
| Rojo de metilo | 4,4 – 6,2 | rojo → amarillo |
| Azul de bromotimol | 6,0 – 7,6 | amarillo → azul |
| Fenolftaleína | 8,2 – 10,0 | incoloro → rosa |

### Redox

| Titulante | Uso | Punto final |
|---|---|---|
| Permanganato, $\mathrm{KMnO_4}$ | hierro(II), oxalato, peróxido | el propio color violeta |
| Dicromato, $\mathrm{K_2Cr_2O_7}$ | hierro(II); demanda química de oxígeno | indicador redox |
| Yodo / tiosulfato | cloro, cobre, vitamina C | almidón (azul) |

> [!NOTE]
> **Ejemplo.** Una muestra con hierro(II) gasta 22,50 mL de $\mathrm{KMnO_4}$ 0,02000 M.
> $$\mathrm{MnO_4^- + 5\,Fe^{2+} + 8\,H^+ \to Mn^{2+} + 5\,Fe^{3+} + 4\,H_2O}$$
> $$n_{\mathrm{Fe}} = 5 \times 0{,}02250 \times 0{,}02000 = 2{,}250 \times 10^{-3}\ \text{mol} \quad\Rightarrow\quad m_{\mathrm{Fe}} = 2{,}250 \times 10^{-3} \times 55{,}85 = 0{,}1257\ \text{g}$$

### Precipitación

Se usan sobre todo con nitrato de plata (**argentometría**) para haluros.

| Método | Indicador | Señal |
|---|---|---|
| Mohr | cromato | precipitado rojo de $\mathrm{Ag_2CrO_4}$ |
| Volhard | hierro(III), por retroceso con tiocianato | complejo rojo |
| Fajans | indicador de adsorción (fluoresceína) | cambio de color sobre el precipitado |

### Complejométricas

El **EDTA** forma complejos muy estables, en relación **1 : 1**, con casi todos los cationes metálicos.

Se usa para determinar la **dureza del agua** (calcio y magnesio), con negro de eriocromo T como indicador, a pH 10.

> [!NOTE]
> **Ejemplo.** 50,00 mL de agua gastan 12,30 mL de EDTA 0,01000 M.
> $$n = 0{,}01230 \times 0{,}01000 = 1{,}230 \times 10^{-4}\ \text{mol}$$
> Expresada como $\mathrm{CaCO_3}$ ($M = 100{,}09$):
> $$\frac{1{,}230 \times 10^{-4} \times 100{,}09 \times 1000\ \text{mg/g}}{0{,}05000\ \text{L}} = 246\ \text{mg/L}$$
> Es un agua dura.

## Métodos instrumentales

| Familia | Propiedad medida | Técnicas |
|---|---|---|
| **Espectroscópicos** | absorción o emisión de radiación | UV-visible, absorción atómica, infrarrojo, fluorescencia |
| **Electroquímicos** | potencial, corriente, carga | potenciometría (pH), voltamperometría, conductimetría |
| **De separación** | tiempo de retención | cromatografía de gases y de líquidos, electroforesis |
| Otros | masa de los iones, calor | espectrometría de masas, análisis térmico |

### Potenciometría

El **electrodo de vidrio** mide el pH: su potencial varía linealmente con él, unos 59 mV por unidad a 25 °C (ecuación de Nernst). Hay que calibrarlo con soluciones reguladoras.

Los **electrodos selectivos** responden a un ion en particular (fluoruro, nitrato, sodio).

### Cromatografía

Separa los componentes de una mezcla según su distinta afinidad por una **fase estacionaria** y una **fase móvil**. Los que interactúan más con la fase estacionaria avanzan más lento.

| Técnica | Fase móvil | Para |
|---|---|---|
| De gases | un gas inerte | compuestos volátiles y estables |
| De líquidos de alta resolución (HPLC) | un líquido a presión | compuestos no volátiles o termolábiles |
| En capa fina | un solvente | controles rápidos y baratos |
| De intercambio iónico | una solución | iones |

Cada compuesto se identifica por su **tiempo de retención** y se cuantifica por el **área** del pico. Acoplada a un espectrómetro de masas, identifica compuestos en mezclas complejas.

## Calibración

Los métodos instrumentales son **relativos**: la señal se convierte en concentración mediante patrones.

### Curva de calibración

Se miden patrones de concentración conocida y se ajusta una recta por mínimos cuadrados:

$$S = m\,c + b$$

La concentración de la muestra se obtiene interpolando: $c = (S - b)/m$.

> [!NOTE]
> **Ejemplo.** Patrones de hierro:
>
> | Concentración (mg/L) | 0 | 1,0 | 2,0 | 4,0 | 6,0 |
> |---|---|---|---|---|---|
> | Absorbancia | 0,002 | 0,105 | 0,208 | 0,410 | 0,618 |
>
> El ajuste da $A = 0{,}1025\,c + 0{,}0022$. Una muestra con $A = 0{,}315$ contiene
> $$c = \frac{0{,}315 - 0{,}0022}{0{,}1025} = 3{,}05\ \text{mg/L}$$

> [!WARNING]
> La muestra debe caer **dentro** del intervalo calibrado. Si la señal supera al patrón más alto, se diluye y se vuelve a medir: nunca se extrapola.

### Otros métodos

| Método | Cuándo |
|---|---|
| **Adición de patrón** | la matriz altera la señal: se agregan cantidades conocidas de analito a la propia muestra |
| **Patrón interno** | la señal fluctúa: se agrega una sustancia de referencia y se mide el cociente de señales |

Un **blanco** contiene todo menos el analito; su señal se resta.

## Buenas prácticas

- Material volumétrico calibrado: pipetas y matraces aforados para volúmenes exactos; las probetas solo para aproximados.
- Hacer al menos tres repeticiones.
- Incluir blancos y muestras de control.
- Registrar todo en el cuaderno, con trazabilidad.
- Informar con las **cifras significativas** que justifica la incertidumbre.

> [!TIP]
> En todo cálculo volumétrico el camino es el mismo: volumen y concentración del titulante → moles de titulante → (estequiometría) → moles de analito → masa o concentración.
