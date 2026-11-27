+++
title = "Redes neuronales y aprendizaje profundo"
summary = "Neurona artificial, funciones de activación, perceptrón multicapa, retropropagación, optimizadores, redes convolucionales, recurrentes, transformadores y modelos de lenguaje."
branch = "Inteligencia artificial"
order = 2
level = "Avanzado"
tags = ["redes neuronales", "retropropagación", "aprendizaje profundo", "convolucionales", "transformadores", "LLM"]
+++

## La neurona artificial

Una **neurona** calcula una suma ponderada de sus entradas y le aplica una función no lineal:

$$a = f\left( \sum_{i} w_i x_i + b \right) = f(\mathbf{w} \cdot \mathbf{x} + b)$$

| Símbolo | Nombre |
|---|---|
| $x_i$ | entradas |
| $w_i$ | pesos |
| $b$ | sesgo |
| $f$ | función de activación |
| $a$ | salida (activación) |

Una sola neurona con activación sigmoide es una regresión logística: solo separa clases con una frontera **lineal**. No puede aprender, por ejemplo, la función XOR.

## Funciones de activación

| Función | Fórmula | Rango | Uso |
|---|---|---|---|
| Sigmoide | $\dfrac{1}{1 + e^{-z}}$ | $(0, 1)$ | salida de clasificación binaria |
| Tangente hiperbólica | $\tanh z$ | $(-1, 1)$ | redes recurrentes |
| **ReLU** | $\max(0, z)$ | $[0, \infty)$ | capas ocultas; la más usada |
| GELU, SiLU | variantes suaves de ReLU | | transformadores |
| Softmax | $\dfrac{e^{z_i}}{\sum_j e^{z_j}}$ | probabilidades que suman 1 | salida de clasificación en varias clases |

> [!IMPORTANT]
> Sin funciones de activación **no lineales**, apilar capas no sirve: la composición de funciones lineales es otra función lineal.

## Perceptrón multicapa

Las neuronas se organizan en **capas**: una de entrada, una o más **ocultas** y una de salida. Cada capa calcula

$$\mathbf{a}^{(l)} = f\left( W^{(l)} \mathbf{a}^{(l-1)} + \mathbf{b}^{(l)} \right)$$

donde $W^{(l)}$ es la matriz de pesos de la capa $l$. Calcular la salida a partir de la entrada es la **propagación hacia adelante**.

**Teorema de aproximación universal**: una red con una capa oculta suficientemente ancha puede aproximar cualquier función continua. En la práctica, las redes **profundas** (muchas capas) aprenden representaciones jerárquicas con menos parámetros: bordes, luego formas, luego objetos.

> [!NOTE]
> **Cantidad de parámetros.** Una red con 784 entradas, una capa oculta de 128 neuronas y 10 salidas tiene
> $$(784 \times 128 + 128) + (128 \times 10 + 10) = 101\,770 \text{ parámetros}$$

## Entrenamiento

Se busca minimizar una **función de pérdida** $L$ con descenso por gradiente. Hace falta la derivada de $L$ respecto de **cada** peso.

### Retropropagación

Aplica la **regla de la cadena** desde la salida hacia la entrada, reutilizando los cálculos de cada capa.

Para un peso de la última capa:

$$\frac{\partial L}{\partial w} = \frac{\partial L}{\partial a} \cdot \frac{\partial a}{\partial z} \cdot \frac{\partial z}{\partial w}$$

Definiendo el error de cada capa, $\boldsymbol{\delta}^{(l)} = \partial L / \partial \mathbf{z}^{(l)}$:

$$\boldsymbol{\delta}^{(l)} = \left( W^{(l+1)\,T} \boldsymbol{\delta}^{(l+1)} \right) \odot f'(\mathbf{z}^{(l)}) \qquad \frac{\partial L}{\partial W^{(l)}} = \boldsymbol{\delta}^{(l)} \, \mathbf{a}^{(l-1)\,T}$$

El costo de calcular todos los gradientes es del mismo orden que el de una propagación hacia adelante.

Las bibliotecas lo hacen solas mediante **diferenciación automática**: registran las operaciones y derivan el grafo.

### Ciclo de entrenamiento

```python
import torch
from torch import nn

modelo = nn.Sequential(
    nn.Flatten(),
    nn.Linear(784, 128),
    nn.ReLU(),
    nn.Linear(128, 10),
)
perdida = nn.CrossEntropyLoss()
optimizador = torch.optim.Adam(modelo.parameters(), lr=1e-3)

for epoca in range(10):
    for imagenes, etiquetas in cargador:          # minilotes
        prediccion = modelo(imagenes)             # hacia adelante
        costo = perdida(prediccion, etiquetas)

        optimizador.zero_grad()
        costo.backward()                          # retropropagación
        optimizador.step()                        # actualiza los pesos
```

Una **época** es una pasada completa por los datos de entrenamiento.

### Optimizadores

| Optimizador | Idea |
|---|---|
| SGD | descenso por gradiente con minilotes |
| SGD con momento | acumula una «velocidad» que suaviza las oscilaciones |
| **Adam**, AdamW | adapta la tasa de aprendizaje de cada parámetro; el punto de partida habitual |

### Dificultades

| Problema | Descripción | Remedio |
|---|---|---|
| Gradiente que se desvanece | en redes profundas, el gradiente se hace diminuto en las primeras capas | ReLU, conexiones residuales, normalización |
| Gradiente que explota | el gradiente crece sin control | recorte del gradiente |
| Sobreajuste | memoriza los datos | *dropout*, regularización, aumento de datos, detención temprana |
| Mala inicialización | las señales se saturan o se anulan | inicialización de He o de Xavier |

| Técnica | Qué hace |
|---|---|
| *Dropout* | durante el entrenamiento, apaga neuronas al azar |
| Normalización por lotes o por capa | estabiliza las activaciones |
| Conexiones residuales | suman la entrada de un bloque a su salida: $\mathbf{y} = F(\mathbf{x}) + \mathbf{x}$ |

## Redes convolucionales

Para **imágenes**. En lugar de conectar cada píxel con cada neurona, una capa **convolucional** desliza pequeños **filtros** (de $3 \times 3$, por ejemplo) sobre la imagen.

| Ventaja | Motivo |
|---|---|
| Pocos parámetros | el mismo filtro se usa en toda la imagen |
| Invariancia a la traslación | detecta un patrón dondequiera que aparezca |
| Jerarquía | las primeras capas detectan bordes; las últimas, objetos |

Tamaño de la salida, con entrada de lado $n$, filtro de lado $k$, relleno $p$ y paso $s$:

$$n_{\text{salida}} = \left\lfloor \frac{n + 2p - k}{s} \right\rfloor + 1$$

> [!NOTE]
> Imagen de $32 \times 32$, filtro de $5 \times 5$, sin relleno, paso 1: salida de $28 \times 28$.

Las capas de **agrupamiento** (*pooling*) reducen la resolución tomando el máximo o el promedio de cada región.

Arquitecturas clásicas: LeNet, AlexNet, VGG, **ResNet** (que introdujo las conexiones residuales).

## Redes recurrentes

Para **secuencias**. Mantienen un **estado oculto** que se actualiza en cada paso:

$$\mathbf{h}_t = f(W_h \mathbf{h}_{t-1} + W_x \mathbf{x}_t + \mathbf{b})$$

Les cuesta recordar dependencias lejanas, por el desvanecimiento del gradiente. Las celdas **LSTM** y **GRU** lo mitigan con compuertas que controlan qué recordar y qué olvidar. Para texto, hoy han sido reemplazadas casi por completo por los transformadores.

## Transformadores

Introducidos en 2017 («*Attention Is All You Need*»). Procesan toda la secuencia a la vez mediante **atención**.

### Autoatención

Cada elemento de la secuencia consulta a todos los demás y decide cuánto le importa cada uno. Con matrices de consultas $Q$, claves $K$ y valores $V$:

$$\text{Atención}(Q, K, V) = \text{softmax}\left( \frac{Q K^T}{\sqrt{d_k}} \right) V$$

En «El gato no cruzó la calle porque **estaba** cansado», la atención permite que «estaba» se relacione con «gato».

| Componente | Función |
|---|---|
| Representaciones (*embeddings*) | convierten cada *token* en un vector |
| Codificación posicional | aporta el orden, que la atención por sí sola ignora |
| Atención de varias cabezas | varias atenciones en paralelo, cada una atenta a relaciones distintas |
| Red densa | procesa cada posición |
| Conexiones residuales y normalización | permiten apilar muchas capas |

Ventajas sobre las recurrentes: se **paralelizan** en GPU y relacionan directamente elementos alejados. Su costo crece con el cuadrado de la longitud de la secuencia.

## Modelos de lenguaje

Un **modelo de lenguaje grande** (LLM) es un transformador entrenado para **predecir el siguiente *token*** sobre cantidades enormes de texto.

| Etapa | Descripción |
|---|---|
| Segmentación | el texto se divide en *tokens* (fragmentos de palabras) |
| Preentrenamiento | predecir el siguiente *token*; autosupervisado, sin etiquetas humanas |
| Ajuste fino | entrenamiento adicional para seguir instrucciones o para una tarea |
| Inferencia | generar un *token* por vez, y realimentarlo como entrada |

| Concepto | Significado |
|---|---|
| Ventana de contexto | cuántos *tokens* puede considerar a la vez |
| Temperatura | cuánta aleatoriedad hay al elegir el siguiente *token* |
| Instrucción (*prompt*) | el texto de entrada |
| Generación aumentada por recuperación (RAG) | buscar documentos relevantes e incluirlos en la instrucción |

> [!WARNING]
> Un modelo de lenguaje genera texto **plausible**, no necesariamente verdadero: puede afirmar con seguridad cosas falsas («alucinaciones»). Hay que verificar los datos, los cálculos y las citas.

## Otras arquitecturas

| Arquitectura | Uso |
|---|---|
| Autocodificador | comprimir, detectar anomalías |
| Redes generativas antagónicas (GAN) | generar imágenes |
| Modelos de difusión | generar imágenes, audio y video a partir de ruido |
| Redes sobre grafos | moléculas, redes sociales |

## Transferencia de aprendizaje

Entrenar desde cero exige muchos datos y cómputo. Lo habitual es partir de un modelo **preentrenado** y ajustarlo:

| Estrategia | Cuándo |
|---|---|
| Usar el modelo como extractor de características y entrenar solo la última capa | pocos datos |
| Ajuste fino de todas las capas, con tasa de aprendizaje baja | más datos |
| Adaptadores (LoRA) | ajustar modelos enormes entrenando pocos parámetros |

## En la práctica

| Aspecto | Recomendación |
|---|---|
| Bibliotecas | PyTorch, JAX, TensorFlow/Keras |
| Hardware | GPU: el entrenamiento son productos de matrices |
| Datos tabulares | los árboles con potenciación del gradiente suelen ganar a las redes |
| Imágenes, texto, audio | redes profundas |

> [!TIP]
> Antes de entrenar con todos los datos, comprobar que la red puede **sobreajustar un lote pequeño** (llevar la pérdida casi a cero con 10 ejemplos). Si no lo logra, hay un error en el modelo o en los datos.
