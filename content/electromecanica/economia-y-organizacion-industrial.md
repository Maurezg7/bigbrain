+++
title = "Economía y organización industrial"
summary = "Costos fijos y variables, punto de equilibrio, valor del dinero en el tiempo, VAN y TIR, amortización, productividad y planificación de proyectos."
branch = "Gestión, ambiente y seguridad"
order = 3
level = "Intermedio"
tags = ["costos", "punto de equilibrio", "VAN", "TIR", "productividad"]
+++

## Por qué un ingeniero estudia economía

Una solución técnica solo sirve si además es viable económicamente. Elegir entre dos equipos, justificar una inversión o fijar un precio son decisiones de ingeniería.

## Costos

| Clasificación | Tipos | Ejemplos |
|---|---|---|
| Según el volumen | **fijos**: no cambian con la producción | alquiler, sueldos de estructura, seguros |
| | **variables**: proporcionales a la producción | materia prima, energía de proceso |
| Según su asignación | **directos**: se identifican con el producto | materiales, mano de obra directa |
| | **indirectos**: se reparten entre productos | supervisión, mantenimiento |

$$C_{total} = C_F + c_v\,Q$$

El costo **unitario** baja al aumentar la producción, porque los fijos se reparten entre más unidades: es la economía de escala.

## Punto de equilibrio

![Gráfico de punto de equilibrio con las rectas de ingresos y de costos totales](/media/electromecanica/punto-equilibrio.svg)

*A la izquierda del cruce hay pérdida; a la derecha, ganancia.*

Es la cantidad a la que los ingresos igualan a los costos. Con precio $p$:

$$Q_e = \frac{C_F}{p - c_v}$$

La diferencia $p - c_v$ es la **contribución marginal**: lo que cada unidad aporta para cubrir los costos fijos.

> [!NOTE]
> Costos fijos de 1 200 000 por mes, costo variable de 400 por unidad y precio de 700:
>
> $Q_e = \dfrac{1\,200\,000}{700 - 400} = 4000$ unidades por mes. Vendiendo 5000, la ganancia es $1000\cdot 300 = 300\,000$.

## El valor del dinero en el tiempo

Un peso hoy vale más que un peso dentro de un año, porque puede invertirse. Con tasa de interés $i$ por período:

$$F = P\,(1 + i)^n,\qquad P = \frac{F}{(1 + i)^n}$$

$P$ es el valor presente y $F$ el valor futuro tras $n$ períodos. Una serie de $n$ pagos iguales $A$ equivale hoy a

$$P = A\,\frac{(1 + i)^n - 1}{i\,(1 + i)^n}$$

Con inflación, se trabaja con valores constantes y tasa real, o con valores corrientes y tasa nominal; nunca mezclando.

## Evaluación de inversiones

Un proyecto se describe por su **flujo de fondos**: la inversión inicial $I_0$ y los flujos netos $F_t$ de cada año.

| Indicador | Definición | Criterio |
|---|---|---|
| Valor actual neto (VAN) | $-I_0 + \displaystyle\sum_{t=1}^{n}\frac{F_t}{(1 + i)^t}$ | conviene si es positivo |
| Tasa interna de retorno (TIR) | la tasa que hace el VAN igual a cero | conviene si supera la tasa exigida |
| Período de repago | tiempo en que se recupera la inversión | cuanto menor, mejor; ignora lo que viene después |

> [!NOTE]
> Un variador de velocidad cuesta 10 000 y ahorra 3500 por año durante 5 años. Con tasa del 12 %:
>
> $P = 3500\cdot\dfrac{1{,}12^5 - 1}{0{,}12\cdot 1{,}12^5} = 3500\cdot 3{,}605\approx 12\,617$.
>
> $\text{VAN} = 12\,617 - 10\,000 = 2617 > 0$: conviene. El repago simple es $10\,000/3500\approx 2{,}9$ años y la TIR ronda el 22 %.

Para comparar alternativas de distinta vida útil se usa el **costo anual equivalente**. Conviene completar el análisis con uno de **sensibilidad**: ver cómo cambia el resultado si varían el precio, el volumen o la tasa.

## Amortización

Un equipo pierde valor por desgaste y obsolescencia. La **amortización** reparte su costo a lo largo de la vida útil. El método lineal es el más usado:

$$a = \frac{V_0 - V_r}{n}$$

con $V_0$ el valor inicial, $V_r$ el residual y $n$ los años de vida útil. No es una salida de dinero, pero es un costo y reduce el impuesto a las ganancias.

## Organización de la empresa

| Estructura | Característica |
|---|---|
| Funcional | áreas por especialidad: producción, mantenimiento, compras, ventas |
| Por producto o divisional | cada línea tiene sus propias áreas |
| Matricial | combina funciones y proyectos |

| Tipo de producción | Características |
|---|---|
| Por proyecto | producto único: una central, un puente |
| Por lotes o taller | variedad alta, volumen bajo |
| En línea | variedad baja, volumen alto |
| Continua | proceso ininterrumpido: refinería, cementera |

## Productividad

$$\text{Productividad} = \frac{\text{producción obtenida}}{\text{recursos utilizados}}$$

El **estudio del trabajo** la mejora analizando métodos y midiendo tiempos. El tiempo estándar de una operación es el tiempo observado, corregido por el ritmo del operario, más suplementos por fatiga y necesidades.

La **manufactura esbelta** (*lean*) busca eliminar todo lo que no agrega valor: sobreproducción, esperas, transportes, exceso de proceso, inventarios, movimientos y defectos.

## Inventarios

Mantener existencias cuesta dinero; quedarse sin ellas, también. El **lote económico** equilibra el costo de pedir con el de almacenar:

$$Q^* = \sqrt{\frac{2\,D\,S}{H}}$$

con $D$ la demanda anual, $S$ el costo de cada pedido y $H$ el costo de mantener una unidad durante un año.

> [!NOTE]
> Demanda de 12 000 unidades por año, 500 por pedido y 30 por unidad y año: $Q^* = \sqrt{\dfrac{2\cdot 12\,000\cdot 500}{30}}\approx 632$ unidades, unos 19 pedidos al año.

La clasificación **ABC** concentra el control en los pocos artículos que representan la mayor parte del valor. El sistema **justo a tiempo** produce solo lo pedido, con inventarios mínimos.

## Planificación de proyectos

| Herramienta | Uso |
|---|---|
| Diagrama de Gantt | barras que muestran cada tarea en el calendario |
| Camino crítico (CPM) | red de tareas con sus dependencias |
| PERT | igual, con duraciones probabilísticas |

El **camino crítico** es la secuencia de tareas más larga: determina la duración total del proyecto y sus tareas no tienen **holgura**. Atrasar una de ellas atrasa todo.

En PERT, la duración esperada de una tarea se estima con los tiempos optimista $a$, más probable $m$ y pesimista $b$:

$$t_e = \frac{a + 4m + b}{6}$$
