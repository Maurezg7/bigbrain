+++
title = "Transformada de Fourier"
summary = "Definición y transformada inversa, pares básicos, propiedades, convolución, teorema de Parseval, delta de Dirac, muestreo y transformada discreta (FFT)."
branch = "Ecuaciones diferenciales"
order = 6
level = "Avanzado"
tags = ["transformada de Fourier", "convolución", "espectro", "muestreo", "FFT", "Parseval"]
+++

## De la serie a la transformada

La serie de Fourier descompone una función **periódica** en armónicos de frecuencias discretas. Si el período tiende a infinito, las frecuencias se juntan hasta formar un continuo, y la suma se convierte en integral: la **transformada de Fourier**, válida para funciones no periódicas.

$$F(\omega) = \int_{-\infty}^{\infty} f(t)\, e^{-i\omega t}\, dt$$

$$f(t) = \frac{1}{2\pi} \int_{-\infty}^{\infty} F(\omega)\, e^{i\omega t}\, d\omega$$

| Función | Dominio | Describe |
|---|---|---|
| $f(t)$ | tiempo | cómo varía la señal |
| $F(\omega)$ | frecuencia | cuánto hay de cada frecuencia: el **espectro** |

$F(\omega)$ es compleja: su módulo $\lvert F(\omega) \rvert$ es el **espectro de amplitud**, y su argumento, el **de fase**.

> [!WARNING]
> Hay varias convenciones. Con la frecuencia $f$ en hercios ($\omega = 2\pi f$) se escribe $X(f) = \int x(t)\, e^{-i 2\pi f t}\, dt$ y desaparece el factor $\frac{1}{2\pi}$ de la inversa. En física es común repartir $\frac{1}{\sqrt{2\pi}}$ en cada una. Verificar siempre cuál se usa.

Una condición suficiente para que exista es que $f$ sea **absolutamente integrable**: $\int_{-\infty}^{\infty} \lvert f(t) \rvert\, dt < \infty$.

## Primeros ejemplos

> [!NOTE]
> **Pulso rectangular.** $f(t) = 1$ si $\lvert t \rvert < a$, y 0 fuera.
> $$F(\omega) = \int_{-a}^{a} e^{-i\omega t}\, dt = \frac{e^{i\omega a} - e^{-i\omega a}}{i\omega} = \frac{2\sin(\omega a)}{\omega}$$
> Es una función del tipo **sinc**: un lóbulo central de ancho $2\pi/a$ y lóbulos laterales decrecientes.

> [!NOTE]
> **Exponencial decreciente.** $f(t) = e^{-at}$ para $t > 0$ (con $a > 0$), y 0 para $t < 0$.
> $$F(\omega) = \int_0^{\infty} e^{-(a + i\omega)t}\, dt = \frac{1}{a + i\omega}$$
> $$\lvert F(\omega) \rvert = \frac{1}{\sqrt{a^2 + \omega^2}}$$

## Tabla de pares

| $f(t)$ | $F(\omega)$ |
|---|---|
| pulso rectangular de semiancho $a$ | $\dfrac{2\sin(\omega a)}{\omega}$ |
| $e^{-at}u(t)$ | $\dfrac{1}{a + i\omega}$ |
| $e^{-a\lvert t \rvert}$ | $\dfrac{2a}{a^2 + \omega^2}$ |
| $e^{-t^2/(2\sigma^2)}$ | $\sigma\sqrt{2\pi}\; e^{-\sigma^2\omega^2/2}$ |
| $\delta(t)$ | $1$ |
| $1$ | $2\pi\,\delta(\omega)$ |
| $e^{i\omega_0 t}$ | $2\pi\,\delta(\omega - \omega_0)$ |
| $\cos(\omega_0 t)$ | $\pi\,[\delta(\omega - \omega_0) + \delta(\omega + \omega_0)]$ |
| $\sin(\omega_0 t)$ | $-i\pi\,[\delta(\omega - \omega_0) - \delta(\omega + \omega_0)]$ |

$u(t)$ es el escalón unitario. La **gaussiana** se transforma en otra gaussiana.

## Delta de Dirac

Es un impulso idealizado: nulo fuera del origen, infinitamente alto y de área 1. No es una función, sino una **distribución**, definida por su efecto dentro de una integral:

$$\int_{-\infty}^{\infty} f(t)\, \delta(t - t_0)\, dt = f(t_0)$$

Es la **propiedad de muestreo**: extrae el valor de $f$ en $t_0$.

Un impulso contiene **todas las frecuencias por igual** ($F = 1$). Una señal constante contiene solo la frecuencia cero. Una senoide pura es un par de impulsos en $\pm\omega_0$.

## Propiedades

Con $f(t) \leftrightarrow F(\omega)$ y $g(t) \leftrightarrow G(\omega)$:

| Propiedad | En el tiempo | En frecuencia |
|---|---|---|
| Linealidad | $a f + b g$ | $a F + b G$ |
| Desplazamiento temporal | $f(t - t_0)$ | $e^{-i\omega t_0} F(\omega)$ |
| Modulación | $e^{i\omega_0 t} f(t)$ | $F(\omega - \omega_0)$ |
| Cambio de escala | $f(at)$ | $\dfrac{1}{\lvert a \rvert} F\!\left(\dfrac{\omega}{a}\right)$ |
| Derivada | $f'(t)$ | $i\omega\, F(\omega)$ |
| Derivada $n$-ésima | $f^{(n)}(t)$ | $(i\omega)^n F(\omega)$ |
| Multiplicación por $t$ | $t\, f(t)$ | $i\, F'(\omega)$ |
| **Convolución** | $(f * g)(t)$ | $F(\omega)\, G(\omega)$ |
| Producto | $f(t)\, g(t)$ | $\dfrac{1}{2\pi} (F * G)(\omega)$ |
| Dualidad | $F(t)$ | $2\pi\, f(-\omega)$ |

| Si $f(t)$ es | entonces $F(\omega)$ es |
|---|---|
| real | hermítica: $F(-\omega) = \overline{F(\omega)}$ |
| real y par | real y par |
| real e impar | imaginaria e impar |

### Consecuencias

- **Desplazar** en el tiempo no cambia la amplitud del espectro, solo la fase.
- **Derivar** equivale a multiplicar por $i\omega$: las ecuaciones diferenciales se vuelven algebraicas.
- **Modular** (multiplicar por una portadora) traslada el espectro: es el principio de la radio.

### Principio de incertidumbre

Por el cambio de escala, **comprimir** una señal en el tiempo **ensancha** su espectro. Midiendo los anchos con los desvíos estándar:

$$\Delta t \cdot \Delta \omega \ge \frac{1}{2}$$

La igualdad se alcanza solo con la gaussiana. Un pulso muy corto ocupa necesariamente mucho ancho de banda. En mecánica cuántica, la misma relación entre posición y momento es el principio de Heisenberg.

## Convolución

$$(f * g)(t) = \int_{-\infty}^{\infty} f(\tau)\, g(t - \tau)\, d\tau$$

Es un promedio móvil de $f$ ponderado por $g$ reflejada y desplazada.

> [!IMPORTANT]
> **Teorema de convolución.** La transformada de una convolución es el **producto** de las transformadas:
> $$\mathcal{F}\{f * g\} = F(\omega)\, G(\omega)$$

### Sistemas lineales

Un sistema **lineal e invariante en el tiempo** queda caracterizado por su **respuesta al impulso** $h(t)$. Para cualquier entrada $x(t)$:

$$y(t) = (x * h)(t) \qquad\Longleftrightarrow\qquad Y(\omega) = H(\omega)\, X(\omega)$$

$H(\omega)$ es la **respuesta en frecuencia**: indica cuánto amplifica y desfasa el sistema a cada frecuencia. Es la función de transferencia evaluada en $s = i\omega$.

> [!NOTE]
> **Filtro RC pasabajos.** $RC\, y' + y = x$. Transformando:
> $$(i\omega RC + 1)\, Y = X \quad\Rightarrow\quad H(\omega) = \frac{1}{1 + i\omega RC}$$
> $$\lvert H(\omega) \rvert = \frac{1}{\sqrt{1 + (\omega RC)^2}}$$
> A la frecuencia de corte $\omega_c = 1/RC$, la amplitud cae a $1/\sqrt{2}$ (−3 dB).

## Teorema de Parseval

La energía es la misma calculada en el tiempo o en la frecuencia:

$$\int_{-\infty}^{\infty} \lvert f(t) \rvert^2\, dt = \frac{1}{2\pi} \int_{-\infty}^{\infty} \lvert F(\omega) \rvert^2\, d\omega$$

$\lvert F(\omega) \rvert^2$ es la **densidad espectral de energía**: cómo se reparte la energía entre las frecuencias.

> [!NOTE]
> **Ejemplo.** Para $f(t) = e^{-at}u(t)$:
> $$\int_0^{\infty} e^{-2at}\, dt = \frac{1}{2a} \qquad \frac{1}{2\pi} \int_{-\infty}^{\infty} \frac{d\omega}{a^2 + \omega^2} = \frac{1}{2\pi} \cdot \frac{\pi}{a} = \frac{1}{2a}$$

## Ecuaciones en derivadas parciales

La transformada en la variable espacial convierte derivadas en productos.

> [!NOTE]
> **Ecuación del calor** en una barra infinita: $u_t = k\, u_{xx}$, con $u(x, 0) = f(x)$.
>
> Transformando en $x$ (con variable de frecuencia $\xi$):
> $$\hat{u}_t = -k\xi^2\, \hat{u} \quad\Rightarrow\quad \hat{u}(\xi, t) = \hat{f}(\xi)\, e^{-k\xi^2 t}$$
>
> Como un producto de transformadas es una convolución, y la antitransformada de la gaussiana es otra gaussiana:
> $$u(x, t) = \frac{1}{\sqrt{4\pi k t}} \int_{-\infty}^{\infty} f(y)\, e^{-(x - y)^2/(4kt)}\, dy$$
>
> La temperatura es la convolución del dato inicial con una gaussiana que se ensancha con el tiempo. Las frecuencias altas se atenúan más rápido: el calor **suaviza**.

## Relación con otras transformadas

| Transformada | Para | Variable |
|---|---|---|
| Serie de Fourier | funciones periódicas | frecuencias discretas $n\omega_0$ |
| Transformada de Fourier | funciones en $(-\infty, \infty)$ | $\omega$ real |
| Transformada de Laplace | funciones en $[0, \infty)$, incluso crecientes | $s = \sigma + i\omega$ complejo |
| Transformada Z | sucesiones | $z$ complejo |

La de Fourier es la de Laplace evaluada en el eje imaginario, $s = i\omega$, cuando este pertenece a la región de convergencia. Laplace conviene para transitorios y condiciones iniciales; Fourier, para el análisis en frecuencia y el régimen permanente.

## Muestreo

Una señal continua se digitaliza tomando muestras cada $T_s$ segundos: la **frecuencia de muestreo** es $f_s = 1/T_s$.

> [!IMPORTANT]
> **Teorema de Nyquist-Shannon.** Una señal sin componentes por encima de $f_{\max}$ se reconstruye exactamente a partir de sus muestras si
> $$f_s > 2 f_{\max}$$

Muestrear en el tiempo hace **periódico** el espectro, con período $f_s$. Si $f_s$ es insuficiente, las copias se superponen: es el ***aliasing***, por el que una frecuencia alta aparece disfrazada de baja.

> [!NOTE]
> **Ejemplo.** Un tono de 7 kHz muestreado a 10 kHz aparece como uno de $10 - 7 = 3$ kHz, indistinguible de un tono real de 3 kHz.

Por eso, antes de muestrear, se aplica un **filtro antialiasing** que elimina lo que supera $f_s/2$. El audio digital usa 44,1 kHz para cubrir hasta 20 kHz, el límite del oído.

## Transformada discreta de Fourier

Para $N$ muestras $x_0, \dots, x_{N-1}$:

$$X_k = \sum_{n=0}^{N-1} x_n\, e^{-i 2\pi k n / N} \qquad x_n = \frac{1}{N} \sum_{k=0}^{N-1} X_k\, e^{i 2\pi k n / N}$$

| Concepto | Valor |
|---|---|
| Frecuencia del índice $k$ | $f_k = k\, f_s / N$ |
| Resolución en frecuencia | $\Delta f = f_s / N = 1/T$, con $T$ la duración total |
| Frecuencia máxima | $f_s / 2$ (el índice $N/2$) |
| Índices mayores que $N/2$ | frecuencias negativas |

Para mejorar la resolución hay que **observar más tiempo**, no muestrear más rápido.

### FFT

Calcular la definición directamente cuesta $O(N^2)$ operaciones. La **transformada rápida** (FFT) lo hace en $O(N \log N)$, dividiendo el problema en dos mitades de manera recursiva.

| $N$ | Directa ($N^2$) | FFT ($N \log_2 N$) |
|---|---|---|
| 1024 | $10^6$ | $10^4$ |
| $10^6$ | $10^{12}$ | $2 \times 10^7$ |

```python
import numpy as np

fs = 1000                                  # Hz
t = np.arange(0, 1, 1 / fs)                # 1 s → resolución de 1 Hz
x = 3 * np.sin(2 * np.pi * 50 * t) + np.sin(2 * np.pi * 120 * t)

X = np.fft.rfft(x)
f = np.fft.rfftfreq(len(x), 1 / fs)
amplitud = 2 * np.abs(X) / len(x)

for k in np.argsort(amplitud)[-2:][::-1]:
    print(f"{f[k]:.0f} Hz: {amplitud[k]:.2f}")
# 50 Hz: 3.00
# 120 Hz: 1.00
```

### Fuga espectral

La transformada discreta supone que el tramo analizado se repite periódicamente. Si no contiene un número entero de ciclos, hay un salto en los extremos y la energía de una frecuencia se **derrama** sobre las vecinas.

Se reduce multiplicando la señal por una **ventana** que la lleva suavemente a cero en los bordes (Hann, Hamming, Blackman), a costa de algo de resolución.

## Aplicaciones

| Campo | Uso |
|---|---|
| Comunicaciones | modulación, multiplexado, OFDM (wifi, 4G y 5G) |
| Audio | ecualización, compresión (MP3), afinadores |
| Imágenes | compresión JPEG (transformada del coseno), filtrado |
| Vibraciones | diagnóstico de máquinas: cada falla tiene su frecuencia |
| Sistemas eléctricos | análisis de armónicos |
| Medicina | resonancia magnética, tomografía |
| Óptica | la difracción lejana es la transformada de la abertura |
| Cálculo numérico | multiplicación rápida de polinomios y enteros grandes |

Para señales cuyo contenido cambia con el tiempo se usa la **transformada de tiempo corto** (el espectrograma) o las **ondículas** (*wavelets*).

> [!TIP]
> Muchas operaciones difíciles en un dominio son simples en el otro: convolucionar es multiplicar, derivar es multiplicar por $i\omega$, filtrar es recortar el espectro. Ante un problema lineal, preguntarse en qué dominio resulta más fácil.
