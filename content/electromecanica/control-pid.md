+++
title = "Control automático y PID"
summary = "Lazo cerrado, realimentación y el controlador PID: qué hace cada término y cómo ajustarlo."
branch = "Instrumentación y control automático"
order = 2
level = "Avanzado"
tags = ["realimentación", "lazo cerrado", "PID", "sintonía"]
+++

## Lazo abierto y lazo cerrado

Un sistema de control busca que una variable (temperatura, velocidad, nivel) siga un valor deseado.

- En **lazo abierto**, el controlador actúa sin medir el resultado. Un microondas calienta el tiempo programado, esté la comida caliente o no.
- En **lazo cerrado**, se mide la salida y se la compara con la referencia. La diferencia, el **error**, decide la acción. Es la **realimentación** que usa un termostato.

![Diagrama de bloques de un lazo de control realimentado](/media/electromecanica/lazo-control.svg)

*Lazo de control realimentado: el sensor mide la salida y el controlador actúa según el error.*

| Señal | Nombre | Ejemplo: control de temperatura de un horno |
|---|---|---|
| $r(t)$ | referencia (setpoint) | 180 °C |
| $y(t)$ | variable controlada | temperatura medida |
| $e(t) = r - y$ | error | cuánto falta |
| $u(t)$ | acción de control | potencia de la resistencia |

La realimentación permite corregir **perturbaciones** (abrir la puerta del horno) y compensar que el modelo del proceso nunca es exacto.

## El controlador PID

Es el algoritmo de control más usado en la industria. Calcula la acción sumando tres términos:

$$u(t) = K_p\,e(t) + K_i\int_0^t e(\tau)\,d\tau + K_d\,\frac{de(t)}{dt}$$

| Término | Reacciona a | Efecto | Riesgo si es excesivo |
|---|---|---|---|
| **P**roporcional | el error presente | respuesta más rápida | oscilaciones; deja error permanente |
| **I**ntegral | el error acumulado | elimina el error en régimen permanente | sobrepico, saturación (windup) |
| **D**erivativo | la tendencia del error | amortigua, anticipa | amplifica el ruido de medición |

> [!TIP]
> Una forma de recordarlo: P mira el **presente**, I el **pasado** y D el **futuro** del error.

## Qué se le pide a la respuesta

Ante un cambio de referencia tipo escalón se evalúa:

- **Tiempo de subida**: cuánto tarda en llegar cerca del valor final.
- **Sobrepico**: cuánto se pasa, en porcentaje.
- **Tiempo de establecimiento**: cuánto tarda en quedar dentro de una banda (por ejemplo ±2 %).
- **Error en régimen permanente**: diferencia final entre salida y referencia.

Y, antes que todo, **estabilidad**: que la salida no oscile cada vez más.

## Sintonía

Ajustar $K_p$, $K_i$ y $K_d$ se llama sintonizar. Un método manual habitual:

1. Con $K_i = K_d = 0$, aumentá $K_p$ hasta que la respuesta sea rápida pero empiece a oscilar; luego reducila a la mitad aproximadamente.
2. Aumentá $K_i$ hasta eliminar el error permanente en un tiempo razonable.
3. Si hay demasiado sobrepico, agregá un poco de $K_d$.

El método de **Ziegler-Nichols** lo sistematiza: se busca la ganancia crítica $K_u$ con la que el lazo oscila de forma sostenida, se mide el período $T_u$ de esa oscilación y se usa

$$K_p = 0{,}6\,K_u, \qquad T_i = \frac{T_u}{2}, \qquad T_d = \frac{T_u}{8}$$

con $K_i = K_p/T_i$ y $K_d = K_p\,T_d$.

## Implementación digital

En un PLC o un microcontrolador el PID se calcula cada período de muestreo $T$:

```rust
struct Pid { kp: f64, ki: f64, kd: f64, integral: f64, error_previo: f64 }

impl Pid {
    fn actualizar(&mut self, referencia: f64, medicion: f64, t: f64) -> f64 {
        let error = referencia - medicion;
        self.integral += error * t;
        let derivada = (error - self.error_previo) / t;
        self.error_previo = error;
        self.kp * error + self.ki * self.integral + self.kd * derivada
    }
}
```

> [!WARNING]
> Si el actuador se satura (una válvula ya está 100 % abierta), la integral sigue creciendo y luego tarda en descargarse: es el **windup**. Las implementaciones reales limitan la integral.
