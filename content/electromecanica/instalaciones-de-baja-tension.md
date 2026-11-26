+++
title = "Instalaciones eléctricas de baja tensión"
summary = "Cómo se proyecta una instalación: demanda, selección de conductores por corriente y caída de tensión, tableros y circuitos."
branch = "Instalaciones y sistemas eléctricos"
order = 1
level = "Intermedio"
tags = ["conductores", "caída de tensión", "tableros", "demanda", "AEA"]
+++

## Qué es proyectar una instalación

Una instalación debe llevar la energía desde el punto de suministro hasta cada carga de forma **segura, confiable y económica**. En Argentina la referencia es la reglamentación de la Asociación Electrotécnica Argentina (AEA 90364), basada en la norma internacional IEC 60364.

El proyecto sigue un orden:

1. relevar las cargas y calcular la demanda;
2. definir el esquema de distribución y los tableros;
3. dimensionar conductores;
4. elegir protecciones y verificar su coordinación;
5. proyectar la puesta a tierra.

## Demanda de potencia

No todas las cargas funcionan a la vez ni a plena potencia. La potencia de cálculo se obtiene afectando la instalada por dos factores:

$$P_{dem} = \sum P_{inst}\cdot k_u\cdot k_s$$

- **factor de utilización** $k_u$: fracción de la potencia nominal que realmente usa cada equipo;
- **factor de simultaneidad** $k_s$: fracción de equipos que funcionan al mismo tiempo.

## Selección de conductores

La sección de un cable debe cumplir **todas** estas condiciones; manda la más exigente.

| Criterio | Qué se verifica |
|---|---|
| Corriente admisible | que el cable no se caliente por encima del límite de su aislación (70 °C en PVC, 90 °C en XLPE) |
| Caída de tensión | que la tensión llegue a la carga dentro de la tolerancia |
| Cortocircuito | que soporte la corriente de falla hasta que actúe la protección |
| Sección mínima | la que exige la reglamentación para cada tipo de circuito |

La corriente admisible de tabla se corrige por temperatura ambiente, agrupamiento con otros circuitos y forma de instalación (bandeja, caño, enterrado).

### Caída de tensión

Para un circuito de longitud $L$, con resistencia $r$ y reactancia $x$ por unidad de longitud:

$$\Delta V_{mono} = 2\,L\,I\,(r\cos\varphi + x\sin\varphi),\qquad \Delta V_{tri} = \sqrt3\,L\,I\,(r\cos\varphi + x\sin\varphi)$$

Los límites habituales son 3 % para iluminación y 5 % para fuerza motriz en régimen, admitiéndose hasta 15 % durante el arranque de motores.

> [!NOTE]
> Motor trifásico de 380 V que toma 30 A con $\cos\varphi = 0{,}85$, a 80 m del tablero, con cable de cobre de $6\ \text{mm}^2$ ($r \approx 3{,}7\ \Omega$/km; se desprecia $x$).
>
> $\Delta V = \sqrt3\cdot 0{,}08\cdot 30\cdot 3{,}7\cdot 0{,}85 \approx 13{,}1$ V, un 3,4 %: cumple.
>
> Con $4\ \text{mm}^2$ ($r \approx 5{,}5\ \Omega$/km) serían 19,4 V, un 5,1 %: no cumple aunque la corriente admisible alcance.

## Tableros y circuitos

La instalación se organiza en forma de árbol: tablero general, tableros seccionales y circuitos terminales. Conviene:

- separar los circuitos de iluminación, de tomacorrientes y de fuerza motriz;
- repartir las cargas monofásicas entre las tres fases;
- dejar reserva de espacio y de potencia para ampliaciones;
- identificar cada circuito y mantener actualizado el esquema unifilar.

## Canalizaciones

| Tipo | Uso |
|---|---|
| Cañería embutida o a la vista | edificios, circuitos terminales |
| Bandeja portacables | industria, muchos cables |
| Cable enterrado | alimentaciones exteriores |
| Blindobarra | grandes corrientes, distribución flexible |

Cada envolvente tiene un **grado de protección IP** de dos cifras: la primera indica protección contra sólidos y la segunda contra agua. IP65 es estanco al polvo y resiste chorros de agua.

## Colores de los conductores

| Conductor | Color (Argentina) |
|---|---|
| Fases L1, L2, L3 | marrón, negro, rojo |
| Neutro | celeste |
| Protección (tierra) | verde y amarillo |

> [!WARNING]
> El conductor verde y amarillo no se usa para otra función bajo ninguna circunstancia. Y nunca se trabaja sobre un circuito sin cortar, bloquear, verificar ausencia de tensión y señalizar.
