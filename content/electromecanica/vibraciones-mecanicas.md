+++
title = "Vibraciones mecánicas"
summary = "Sistema masa-resorte-amortiguador, frecuencia natural, resonancia, aislamiento y diagnóstico de máquinas por vibraciones."
branch = "Mecánica y elementos de máquinas"
order = 2
level = "Avanzado"
tags = ["vibraciones", "frecuencia natural", "resonancia", "amortiguamiento", "balanceo"]
+++

## El modelo básico

Casi cualquier problema de vibraciones se entiende con una masa $m$, un resorte de rigidez $k$ y un amortiguador de constante $c$:

$$m\ddot x + c\dot x + kx = F(t)$$

## Vibración libre

Sin amortiguamiento ni fuerza exterior, la masa oscila a su **frecuencia natural**:

$$\omega_n = \sqrt{\frac{k}{m}},\qquad f_n = \frac{1}{2\pi}\sqrt{\frac{k}{m}}$$

Una forma práctica de estimarla es a partir de la deflexión estática $\delta_{est}$ que produce el peso propio:

$$f_n = \frac{1}{2\pi}\sqrt{\frac{g}{\delta_{est}}}$$

> [!NOTE]
> Una máquina que hunde 2 mm sus apoyos elásticos tiene $f_n = \dfrac{1}{2\pi}\sqrt{\dfrac{9{,}81}{0{,}002}}\approx 11{,}1$ Hz, unas 670 oscilaciones por minuto.

Con amortiguamiento, el factor $\zeta = \dfrac{c}{2\sqrt{km}}$ decide el comportamiento: si $\zeta < 1$ el sistema oscila con amplitud decreciente, y si $\zeta \ge 1$ vuelve al equilibrio sin oscilar.

## Vibración forzada y resonancia

![Curva de amplificación de la vibración en función de la relación de frecuencias con un pico en la resonancia](/media/electromecanica/resonancia-mecanica.svg)

*Cuando la frecuencia de excitación coincide con la natural, la amplitud solo queda limitada por el amortiguamiento.*

Frente a una fuerza armónica $F_0\sin(\omega t)$, la amplitud respecto de la deflexión estática $F_0/k$ es el **factor de amplificación**, con $r = \omega/\omega_n$:

$$M = \frac{1}{\sqrt{(1 - r^2)^2 + (2\zeta r)^2}}$$

- $r \ll 1$: la masa sigue a la fuerza, $M\approx 1$.
- $r = 1$: **resonancia**, $M = \dfrac{1}{2\zeta}$. Con poco amortiguamiento la amplitud es enorme.
- $r \gg 1$: la masa casi no se mueve.

> [!WARNING]
> Ninguna máquina debe funcionar de forma permanente cerca de una frecuencia natural propia o de su estructura. En el arranque y la parada se atraviesan las **velocidades críticas** lo más rápido posible.

## Aislamiento

Para que una máquina no transmita vibraciones al piso, o para proteger un instrumento, se la monta sobre elementos elásticos. La **transmisibilidad** es la fracción de fuerza que pasa:

$$TR = \sqrt{\frac{1 + (2\zeta r)^2}{(1 - r^2)^2 + (2\zeta r)^2}}$$

Solo hay aislamiento si $r > \sqrt2$; cuanto mayor es $r$, mejor. Por eso se eligen apoyos blandos: frecuencia natural baja.

> [!NOTE]
> Motor a 1500 rpm (25 Hz) sobre los apoyos del ejemplo anterior ($f_n = 11{,}1$ Hz): $r = 2{,}25$.
>
> Sin amortiguamiento, $TR = \dfrac{1}{r^2 - 1} = \dfrac{1}{4{,}06}\approx 0{,}25$: solo se transmite la cuarta parte.

## Velocidad crítica de ejes

Un eje con un disco tiene una frecuencia natural de flexión. Cuando gira a esa velocidad, el desbalance residual lo excita en resonancia:

$$n_{cr}\ [\text{rpm}]\approx\frac{30}{\pi}\sqrt{\frac{g}{\delta_{est}}}\approx\frac{946}{\sqrt{\delta_{est}\ [\text{mm}]}}$$

Los ejes **rígidos** trabajan por debajo de la primera velocidad crítica; los **flexibles**, como los de las turbinas, por encima.

## Varios grados de libertad

Un sistema con $n$ masas tiene $n$ frecuencias naturales, cada una con su **modo** de vibración. Las estructuras continuas tienen infinitas; interesan las más bajas. Se calculan por elementos finitos y se miden con ensayos de impacto.

## Diagnóstico por vibraciones

Cada defecto deja una firma en el espectro de frecuencias, lo que permite el mantenimiento predictivo:

| Frecuencia dominante | Causa probable |
|---|---|
| 1 × la velocidad de giro, radial | desbalance |
| 1 × y 2 ×, con componente axial | desalineación |
| Múltiplos y submúltiplos | holguras, aflojamientos |
| Número de dientes × rpm | problemas de engrane |
| Altas, no sincrónicas | defectos en rodamientos |
| 2 × la frecuencia de red | origen eléctrico |

La severidad se evalúa con la velocidad eficaz de vibración en mm/s, según la norma ISO 20816: en máquinas medianas, menos de 2,8 mm/s es aceptable y más de 7,1 mm/s exige intervenir.
