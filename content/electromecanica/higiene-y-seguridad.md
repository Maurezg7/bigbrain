+++
title = "Higiene y seguridad industrial"
summary = "Peligro y riesgo, jerarquía de controles, riesgo eléctrico y mecánico, ruido, incendio, bloqueo y etiquetado, y marco legal argentino."
branch = "Gestión, ambiente y seguridad"
order = 1
level = "Básico"
tags = ["seguridad", "riesgo", "EPP", "riesgo eléctrico", "incendio"]
+++

## Peligro y riesgo

- **Peligro**: fuente con potencial de causar daño. Una sierra, un tablero bajo tensión, un solvente.
- **Riesgo**: combinación de la probabilidad de que el daño ocurra y de su gravedad.

$$\text{Riesgo} = \text{Probabilidad}\times\text{Gravedad}$$

El peligro existe o no; el riesgo se puede reducir. La **seguridad** se ocupa de los accidentes; la **higiene**, de los agentes que enferman con el tiempo (ruido, sustancias, calor, vibraciones); la **ergonomía**, de adaptar el trabajo a la persona.

## Evaluación de riesgos

1. Identificar los peligros de cada tarea.
2. Estimar probabilidad y gravedad.
3. Valorar si el riesgo es tolerable.
4. Definir medidas de control.
5. Verificar que funcionan y revisar periódicamente.

| | Gravedad leve | Gravedad media | Gravedad alta |
|---|---|---|---|
| **Probabilidad baja** | trivial | tolerable | moderado |
| **Probabilidad media** | tolerable | moderado | importante |
| **Probabilidad alta** | moderado | importante | intolerable |

## Jerarquía de controles

Las medidas se eligen en este orden, de más a menos eficaces:

| Nivel | Medida | Ejemplo |
|---|---|---|
| 1 | Eliminación | suprimir la tarea peligrosa |
| 2 | Sustitución | cambiar un solvente tóxico por uno inocuo |
| 3 | Controles de ingeniería | resguardos, extracción localizada, enclavamientos |
| 4 | Controles administrativos | procedimientos, permisos de trabajo, capacitación, señalización |
| 5 | Elementos de protección personal | casco, anteojos, guantes, protección auditiva |

> [!WARNING]
> El elemento de protección personal es la **última** barrera, no la primera: no elimina el peligro y depende de que la persona lo use bien.

## Riesgo eléctrico

Lo que daña es la **corriente** que atraviesa el cuerpo, no la tensión por sí misma.

| Corriente alterna de 50 Hz | Efecto |
|---|---|
| 1 mA | umbral de percepción |
| 10 mA | contracción muscular: no se puede soltar |
| 30 mA | dificultad respiratoria |
| Más de 50 a 100 mA | fibrilación ventricular |

Por eso el interruptor diferencial de protección de personas dispara con **30 mA**. La reglamentación argentina considera tensión de seguridad a 24 V en ambientes secos y húmedos.

Las **cinco reglas de oro** para trabajar sin tensión:

1. Cortar de forma visible todas las fuentes de tensión.
2. Bloquear los aparatos de corte y señalizarlos.
3. Verificar la ausencia de tensión.
4. Poner a tierra y en cortocircuito.
5. Delimitar y señalizar la zona de trabajo.

## Bloqueo y etiquetado

Antes de intervenir una máquina se aíslan **todas** sus energías: eléctrica, neumática, hidráulica, mecánica (resortes, piezas elevadas) y térmica. Cada trabajador coloca su propio candado y se lleva la llave, y se comprueba que la máquina no arranca. Solo quien puso el candado puede retirarlo.

## Riesgo mecánico

Los puntos peligrosos de una máquina son los de atrapamiento, corte, aplastamiento y proyección. Se protegen con:

- **resguardos fijos** o móviles con enclavamiento;
- **dispositivos**: barreras fotoeléctricas, mandos bimanuales, alfombras sensibles;
- **parada de emergencia** accesible, de color rojo sobre fondo amarillo.

## Ruido

El nivel sonoro se mide en decibeles ponderados A. La legislación argentina (Resolución 295/03) fija 85 dB(A) como límite para 8 horas diarias, y cada 3 dB de aumento reducen a la mitad el tiempo permitido:

$$t = \frac{8}{2^{(L - 85)/3}}\ \text{horas}$$

| Nivel | Exposición máxima diaria |
|---|---|
| 85 dB(A) | 8 h |
| 88 dB(A) | 4 h |
| 91 dB(A) | 2 h |
| 94 dB(A) | 1 h |
| 100 dB(A) | 15 min |

La pérdida de audición por ruido es irreversible. Se actúa primero sobre la fuente, después sobre la propagación y por último con protectores.

## Otros agentes

| Agente | Control |
|---|---|
| Contaminantes químicos | concentraciones máximas permisibles, ventilación, hojas de seguridad |
| Iluminación | niveles mínimos según la tarea |
| Carga térmica | pausas, hidratación, ventilación |
| Vibraciones | herramientas amortiguadas, límites de exposición |
| Carga física | límites de levantamiento, ayudas mecánicas, rotación |

## Incendio

El fuego necesita combustible, comburente, calor y reacción en cadena (**tetraedro del fuego**). Se extingue quitando alguno.

| Clase | Combustible | Agente extintor |
|---|---|---|
| A | sólidos: madera, papel, telas | agua, polvo ABC |
| B | líquidos inflamables | polvo, espuma, $\text{CO}_2$ |
| C | equipos eléctricos bajo tensión | $\text{CO}_2$, polvo; nunca agua |
| D | metales combustibles | polvos especiales |
| K | aceites de cocina | acetato de potasio |

Se complementa con detección, medios de escape señalizados, plan de evacuación y simulacros.

## Trabajos de alto riesgo

Requieren un **permiso de trabajo** escrito: trabajos en altura (más de 2 m, con arnés y punto de anclaje), en espacios confinados (medición de atmósfera y vigía), en caliente, excavaciones e izajes.

## Marco legal argentino

| Norma | Contenido |
|---|---|
| Ley 19.587 y Decreto 351/79 | higiene y seguridad en el trabajo |
| Ley 24.557 | riesgos del trabajo; crea las aseguradoras (ART) y la Superintendencia (SRT) |
| Decreto 911/96 | industria de la construcción |
| Decreto 617/97 | actividad agraria |
| Resolución SRT 295/03 | ergonomía, ruido, carga térmica, radiaciones, sustancias químicas |

El empleador debe contar con un servicio de higiene y seguridad y otro de medicina del trabajo, capacitar al personal y entregar los elementos de protección. La norma **ISO 45001** establece un sistema de gestión basado en la mejora continua.

## Indicadores

$$\text{Índice de frecuencia} = \frac{\text{accidentes con baja}\times 10^6}{\text{horas trabajadas}}$$

Todo accidente e incidente se **investiga** para encontrar las causas raíz, no culpables. Por cada accidente grave hay muchos leves y muchísimos incidentes sin daño: atender a estos últimos previene los primeros.
