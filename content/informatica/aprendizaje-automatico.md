+++
title = "Aprendizaje automático"
summary = "Aprendizaje supervisado y no supervisado, regresión lineal y logística, descenso por gradiente, sobreajuste, validación, métricas, árboles, k-vecinos y agrupamiento."
branch = "Inteligencia artificial"
order = 1
level = "Intermedio"
tags = ["aprendizaje automático", "regresión", "clasificación", "sobreajuste", "descenso por gradiente", "métricas"]
+++

## Aprender de los datos

En la programación tradicional se escriben las reglas. En el **aprendizaje automático**, el programa las **deduce a partir de ejemplos**.

| Tradicional | Aprendizaje automático |
|---|---|
| datos + reglas → respuestas | datos + respuestas → reglas (un **modelo**) |

Conviene cuando las reglas son demasiado complejas para escribirlas a mano: reconocer imágenes, filtrar correo no deseado, traducir.

## Tipos de aprendizaje

| Tipo | Datos | Objetivo | Ejemplos |
|---|---|---|---|
| **Supervisado** | entradas con su respuesta (**etiqueta**) | predecir la respuesta de entradas nuevas | precios, diagnóstico, correo no deseado |
| **No supervisado** | entradas sin etiqueta | hallar estructura | agrupar clientes, reducir dimensiones |
| **Por refuerzo** | recompensas por las acciones | aprender una estrategia | juegos, robótica |

Dentro del supervisado:

| Tarea | Salida | Ejemplo |
|---|---|---|
| **Regresión** | un número | el precio de una casa |
| **Clasificación** | una categoría | ¿es correo no deseado? |

### Vocabulario

| Término | Significado |
|---|---|
| Característica (*feature*) | cada variable de entrada |
| Etiqueta | el valor a predecir |
| Modelo | la función que transforma entradas en predicciones |
| Parámetros | los números que el modelo ajusta al entrenar |
| Hiperparámetros | los valores que fija quien entrena (la tasa de aprendizaje, la profundidad de un árbol) |
| Función de pérdida | mide cuánto se equivoca el modelo |

## Regresión lineal

Predice con una combinación lineal de las características:

$$\hat{y} = w_1 x_1 + w_2 x_2 + \dots + w_n x_n + b = \mathbf{w} \cdot \mathbf{x} + b$$

Los **pesos** $\mathbf{w}$ y el **sesgo** $b$ se eligen para minimizar el **error cuadrático medio** sobre los $m$ ejemplos:

$$J(\mathbf{w}, b) = \frac{1}{m} \sum_{i=1}^{m} \left( \hat{y}^{(i)} - y^{(i)} \right)^2$$

## Descenso por gradiente

Para minimizar $J$, se avanza repetidamente en la dirección en que más disminuye: la opuesta al gradiente.

$$\mathbf{w} \leftarrow \mathbf{w} - \alpha \, \nabla_{\mathbf{w}} J$$

$\alpha$ es la **tasa de aprendizaje**.

| $\alpha$ | Efecto |
|---|---|
| muy pequeña | converge, pero lentamente |
| adecuada | converge rápido |
| muy grande | oscila o diverge |

Para la regresión lineal:

$$\frac{\partial J}{\partial w_j} = \frac{2}{m} \sum_{i} \left( \hat{y}^{(i)} - y^{(i)} \right) x_j^{(i)}$$

```python
import numpy as np

def entrenar(X, y, alfa=0.01, pasos=1000):
    m, n = X.shape
    w, b = np.zeros(n), 0.0
    for _ in range(pasos):
        error = X @ w + b - y
        w -= alfa * (2 / m) * (X.T @ error)
        b -= alfa * (2 / m) * error.sum()
    return w, b
```

| Variante | Usa en cada paso |
|---|---|
| Por lotes | todos los ejemplos |
| Estocástico (SGD) | un ejemplo |
| Por minilotes | un grupo pequeño (32 a 256); es la habitual |

> [!TIP]
> **Escalar las características** (restar la media y dividir por el desvío estándar) acelera mucho la convergencia cuando tienen magnitudes muy distintas.

## Regresión logística

A pesar del nombre, es un método de **clasificación**. Aplica la función **sigmoide** a la combinación lineal, para obtener una probabilidad:

$$\hat{p} = \sigma(\mathbf{w} \cdot \mathbf{x} + b) \qquad \sigma(z) = \frac{1}{1 + e^{-z}}$$

Se predice la clase 1 si $\hat{p} \ge 0{,}5$. Se entrena minimizando la **entropía cruzada**:

$$J = -\frac{1}{m} \sum_{i} \left[ y^{(i)} \ln \hat{p}^{(i)} + (1 - y^{(i)}) \ln (1 - \hat{p}^{(i)}) \right]$$

Para más de dos clases se usa la función **softmax**, que convierte un vector de puntajes en probabilidades que suman 1.

## Sobreajuste y subajuste

| Problema | Síntoma | Causa |
|---|---|---|
| **Subajuste** | error alto en entrenamiento y en prueba | modelo demasiado simple |
| **Sobreajuste** | error bajo en entrenamiento, alto en prueba | el modelo memorizó el ruido |

> [!IMPORTANT]
> El objetivo no es acertar en los datos de entrenamiento, sino **generalizar** a datos nunca vistos.

El error de generalización se descompone en **sesgo** (suposiciones demasiado rígidas) y **varianza** (sensibilidad a los datos particulares). Aumentar la complejidad baja el sesgo y sube la varianza.

### Remedios contra el sobreajuste

| Remedio | Idea |
|---|---|
| Más datos | el más efectivo |
| Modelo más simple | menos parámetros |
| **Regularización** | penalizar los pesos grandes |
| Detención temprana | parar cuando el error de validación deja de bajar |
| Aumento de datos | generar variantes de los ejemplos |

La regularización suma un término a la pérdida:

| Tipo | Penalización | Efecto |
|---|---|---|
| L2 (*ridge*) | $\lambda \sum w_j^2$ | pesos pequeños |
| L1 (*lasso*) | $\lambda \sum \lvert w_j \rvert$ | lleva algunos pesos a cero: selecciona características |

## Evaluación

Los datos se dividen:

| Conjunto | Uso | Proporción típica |
|---|---|---|
| Entrenamiento | ajustar los parámetros | 60–80 % |
| Validación | elegir hiperparámetros y comparar modelos | 10–20 % |
| Prueba | estimar el desempeño final, **una sola vez** | 10–20 % |

En la **validación cruzada** de $k$ pliegues se divide en $k$ partes, se entrena $k$ veces dejando cada vez una parte afuera, y se promedian los resultados.

> [!WARNING]
> **Fuga de datos**: usar, directa o indirectamente, información del conjunto de prueba durante el entrenamiento (por ejemplo, calcular la media para escalar con todos los datos). Produce resultados optimistas que no se repiten en producción.

### Métricas de regresión

| Métrica | Fórmula |
|---|---|
| Error cuadrático medio (MSE) | $\frac{1}{m}\sum(\hat{y} - y)^2$ |
| Raíz del MSE (RMSE) | $\sqrt{\text{MSE}}$; en las unidades de $y$ |
| Error absoluto medio (MAE) | $\frac{1}{m}\sum \lvert \hat{y} - y \rvert$ |
| $R^2$ | proporción de la varianza explicada |

### Métricas de clasificación

La **matriz de confusión**:

| | Predicho positivo | Predicho negativo |
|---|---|---|
| **Real positivo** | verdadero positivo (VP) | falso negativo (FN) |
| **Real negativo** | falso positivo (FP) | verdadero negativo (VN) |

| Métrica | Fórmula | Responde |
|---|---|---|
| Exactitud | $\dfrac{VP + VN}{\text{total}}$ | ¿qué fracción acierta? |
| Precisión | $\dfrac{VP}{VP + FP}$ | de los que marcó positivos, ¿cuántos lo eran? |
| Sensibilidad (*recall*) | $\dfrac{VP}{VP + FN}$ | de los positivos reales, ¿cuántos detectó? |
| F1 | $\dfrac{2 \cdot P \cdot S}{P + S}$ | media armónica de precisión y sensibilidad |

> [!NOTE]
> **Ejemplo.** De 1000 personas, 10 tienen una enfermedad. Un modelo que responde siempre «sano» tiene 99 % de exactitud y **0 % de sensibilidad**: no sirve. Con clases desbalanceadas, la exactitud engaña.

Precisión y sensibilidad compiten: al bajar el umbral de decisión se detectan más positivos, pero con más falsas alarmas. Cuál priorizar depende del costo de cada error.

## Otros modelos supervisados

| Modelo | Idea | Ventajas | Inconvenientes |
|---|---|---|---|
| **k vecinos más cercanos** | predecir según los $k$ ejemplos más parecidos | simple, sin entrenamiento | lento al predecir; sufre con muchas dimensiones |
| **Árbol de decisión** | preguntas sucesivas sobre las características | interpretable | se sobreajusta con facilidad |
| **Bosque aleatorio** | muchos árboles sobre muestras distintas, que votan | robusto, pocos ajustes | menos interpretable |
| **Potenciación del gradiente** | árboles en secuencia; cada uno corrige a los anteriores | suele ser el mejor en datos tabulares | más hiperparámetros |
| Máquinas de vectores de soporte | la frontera con mayor margen entre clases | efectivas en dimensiones altas | escalan mal a muchos datos |
| Bayes ingenuo | teorema de Bayes, suponiendo características independientes | muy rápido; bueno para texto | la suposición rara vez se cumple |

Un árbol elige en cada nodo la pregunta que más reduce la **impureza** (índice de Gini o entropía) de los grupos resultantes.

Combinar muchos modelos (un **ensamble**) suele superar a cualquiera de ellos por separado.

## Aprendizaje no supervisado

### k-medias

Agrupa los datos en $k$ grupos:

1. Elegir $k$ centros iniciales.
2. Asignar cada punto al centro más cercano.
3. Mover cada centro al promedio de sus puntos.
4. Repetir 2 y 3 hasta que no haya cambios.

Hay que elegir $k$ de antemano, y el resultado depende de los centros iniciales.

### Reducción de dimensiones

El **análisis de componentes principales** (PCA) proyecta los datos sobre las direcciones de mayor varianza: los autovectores de la matriz de covarianza. Sirve para visualizar, comprimir y eliminar ruido.

## El flujo de trabajo

1. **Definir** el problema y la métrica.
2. **Reunir** y explorar los datos.
3. **Limpiar**: valores faltantes, atípicos, duplicados.
4. **Preparar**: codificar categorías, escalar, crear características.
5. **Dividir** en entrenamiento, validación y prueba.
6. Entrenar un modelo **simple** como referencia.
7. Probar modelos más complejos y ajustar hiperparámetros.
8. **Evaluar** en el conjunto de prueba.
9. Desplegar y **monitorear**: los datos reales cambian con el tiempo.

```python
from sklearn.datasets import load_breast_cancer
from sklearn.model_selection import train_test_split
from sklearn.pipeline import make_pipeline
from sklearn.preprocessing import StandardScaler
from sklearn.linear_model import LogisticRegression
from sklearn.metrics import classification_report

X, y = load_breast_cancer(return_X_y=True)
X_ent, X_pru, y_ent, y_pru = train_test_split(
    X, y, test_size=0.2, random_state=42, stratify=y
)

modelo = make_pipeline(StandardScaler(), LogisticRegression(max_iter=1000))
modelo.fit(X_ent, y_ent)

print(classification_report(y_pru, modelo.predict(X_pru)))
```

El `pipeline` ajusta el escalado solo con los datos de entrenamiento, lo que evita la fuga de datos.

> [!WARNING]
> Un modelo reproduce los **sesgos** de sus datos. Si los datos históricos reflejan discriminación, el modelo la aprenderá. Y correlación no es causalidad: un modelo predice, no explica.

> [!TIP]
> La calidad de los datos importa más que el algoritmo. Empezar siempre por un modelo simple: da una referencia y muchas veces alcanza.
