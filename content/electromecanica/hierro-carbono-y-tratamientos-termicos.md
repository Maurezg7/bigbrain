+++
title = "Diagrama hierro-carbono y tratamientos térmicos"
summary = "Fases del acero, diagrama de equilibrio, clasificación de aceros y fundiciones, temple, revenido, recocido y tratamientos superficiales."
branch = "Materiales"
order = 2
level = "Intermedio"
tags = ["acero", "hierro-carbono", "temple", "revenido", "martensita"]
+++

## El material más usado

El acero es hierro con hasta un 2 % de carbono. Pequeñas variaciones de carbono y de tratamiento cambian sus propiedades por completo, desde un alambre blando hasta una herramienta de corte.

## Fases

| Fase | Descripción | Propiedades |
|---|---|---|
| Ferrita (α) | hierro BCC, disuelve hasta 0,02 % C | blanda y dúctil |
| Austenita (γ) | hierro FCC, disuelve hasta 2,1 % C; estable en caliente | dúctil, no magnética |
| Cementita ($\text{Fe}_3\text{C}$) | carburo con 6,67 % C | muy dura y frágil |
| Perlita | láminas alternadas de ferrita y cementita | resistente, intermedia |
| Martensita | red distorsionada, sobresaturada de carbono | la más dura y frágil |

## El diagrama

![Diagrama hierro-carbono simplificado con las zonas de ferrita, austenita, perlita y cementita](/media/electromecanica/hierro-carbono.svg)

*Zona de los aceros del diagrama hierro-carbono, con el punto eutectoide a 727 °C y 0,77 % C.*

Puntos y líneas clave:

- **Eutectoide**: a 727 °C y 0,77 % C, la austenita se transforma por completo en perlita.
- **A1** (727 °C): por debajo no hay austenita.
- **A3**: por encima, un acero de menos de 0,77 % C es totalmente austenítico.
- **Eutéctico**: a 1148 °C y 4,3 % C, propio de las fundiciones.

| Acero | Carbono | Estructura enfriado lentamente |
|---|---|---|
| Hipoeutectoide | menos de 0,77 % | ferrita y perlita |
| Eutectoide | 0,77 % | perlita |
| Hipereutectoide | 0,77 a 2,1 % | perlita y cementita |

> [!NOTE]
> Por la regla de la palanca, un acero con 0,40 % C enfriado lentamente tiene $\dfrac{0{,}40 - 0{,}02}{0{,}77 - 0{,}02}\approx 51\ \%$ de perlita y 49 % de ferrita.

## Clasificación de los aceros

| Tipo | Carbono | Uso |
|---|---|---|
| Bajo carbono (SAE 1010 a 1020) | hasta 0,25 % | chapas, perfiles, piezas soldadas |
| Medio carbono (SAE 1045) | 0,25 a 0,60 % | ejes, engranajes, bielas |
| Alto carbono (SAE 1070 a 1095) | más de 0,60 % | resortes, herramientas |
| Aleados (SAE 4140, 4340) | con Cr, Ni, Mo | piezas muy exigidas; mejor templabilidad |
| Inoxidables (AISI 304, 316, 420) | más de 11 % Cr | resistencia a la corrosión |

Las **fundiciones** tienen más de 2,1 % C. La gris (grafito en láminas) amortigua vibraciones y se mecaniza bien; la nodular (grafito en esferas) es tenaz, casi como un acero.

## Tratamientos térmicos

Cambian la estructura calentando y enfriando de forma controlada, sin alterar la composición.

| Tratamiento | Proceso | Resultado |
|---|---|---|
| Recocido | austenizar y enfriar muy lento, en el horno | máximo ablandamiento, alivia tensiones |
| Normalizado | austenizar y enfriar al aire | grano fino y uniforme |
| Temple | austenizar y enfriar rápido en agua, aceite o polímero | martensita: máxima dureza |
| Revenido | recalentar el acero templado a 150–650 °C | recupera tenacidad a costa de algo de dureza |

Temple seguido de revenido a alta temperatura se llama **bonificado** y da la mejor combinación de resistencia y tenacidad.

> [!WARNING]
> Un acero templado y sin revenir es tan frágil que puede fisurarse solo. El revenido no es opcional.

La **templabilidad** es la capacidad de endurecer en profundidad y se mide con el ensayo Jominy. Los elementos de aleación la aumentan, lo que permite templar piezas gruesas en aceite, con menos deformaciones.

Las curvas **TTT** indican qué estructura se obtiene según la velocidad de enfriamiento: lenta da perlita, intermedia bainita y rápida martensita.

## Tratamientos superficiales

Muchas piezas necesitan superficie dura, para resistir el desgaste, y núcleo tenaz, para resistir golpes.

| Tratamiento | Método | Piezas típicas |
|---|---|---|
| Cementación | se enriquece la superficie en carbono y se templa | engranajes, levas |
| Nitruración | se difunde nitrógeno a unos 500 °C, sin temple | cigüeñales, matrices |
| Temple por inducción o a la llama | se calienta solo la superficie | ejes, guías, dientes |
