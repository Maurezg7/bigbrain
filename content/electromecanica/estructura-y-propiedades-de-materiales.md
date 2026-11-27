+++
title = "Estructura y propiedades de los materiales"
summary = "Enlaces y estructura cristalina, defectos, familias de materiales, propiedades mecánicas, ensayos y corrosión."
branch = "Materiales"
order = 1
level = "Básico"
tags = ["materiales", "estructura cristalina", "dureza", "ensayos", "corrosión"]
+++

## De los átomos a las propiedades

Las propiedades de un material dependen de qué átomos lo forman, cómo se enlazan y cómo se ordenan. La ciencia de materiales conecta cuatro cosas: **estructura, procesamiento, propiedades y desempeño**.

| Familia | Enlace | Propiedades típicas | Ejemplos |
|---|---|---|---|
| Metales | metálico | conductores, dúctiles, tenaces | acero, aluminio, cobre |
| Cerámicos | iónico y covalente | duros, frágiles, aislantes, resisten calor | alúmina, porcelana, vidrio |
| Polímeros | covalente en cadenas | livianos, aislantes, poca rigidez | polietileno, PVC, nailon |
| Compuestos | combinación | propiedades a medida | fibra de vidrio o de carbono con resina |

## Estructura cristalina

Los metales solidifican en redes ordenadas:

| Red | Ejemplos | Rasgo |
|---|---|---|
| Cúbica centrada en el cuerpo (BCC) | hierro α, cromo, tungsteno | resistente, menos dúctil |
| Cúbica centrada en las caras (FCC) | hierro γ, aluminio, cobre, níquel | muy dúctil |
| Hexagonal compacta (HCP) | magnesio, zinc, titanio α | poco deformable |

Un metal real es un agregado de **granos**, cada uno con su orientación.

## Defectos y endurecimiento

La deformación plástica ocurre por el movimiento de **dislocaciones**, defectos lineales de la red. Todo lo que les estorba endurece al metal:

- **solución sólida**: átomos de otro elemento que distorsionan la red;
- **afino de grano**: los bordes de grano frenan las dislocaciones (relación de Hall-Petch);
- **deformación en frío** (acritud): las dislocaciones se multiplican y se traban entre sí;
- **precipitación**: partículas finas dispersas, como en los aluminios de la serie 2000 y 7000;
- **transformación de fase**: como la martensita en el temple del acero.

Ganar resistencia casi siempre cuesta ductilidad.

## Propiedades mecánicas

| Propiedad | Qué mide | Ensayo |
|---|---|---|
| Resistencia | tensión que soporta | tracción |
| Rigidez | oposición a deformarse elásticamente ($E$) | tracción |
| Ductilidad | deformación plástica antes de romper | tracción (alargamiento) |
| Dureza | resistencia a ser penetrado | Brinell, Rockwell, Vickers |
| Tenacidad | energía absorbida antes de romper | impacto Charpy |
| Resistencia a la fatiga | comportamiento con cargas repetidas | flexión rotativa |
| Resistencia al creep | deformación lenta en caliente | fluencia lenta |

En aceros, la dureza Brinell permite estimar la resistencia a la tracción:

$$\sigma_R\ [\text{MPa}]\approx 3{,}45\cdot HB$$

Muchos aceros se vuelven frágiles por debajo de una **temperatura de transición**, que el ensayo Charpy pone en evidencia.

## Ensayos no destructivos

Permiten buscar defectos sin dañar la pieza:

| Ensayo | Detecta |
|---|---|
| Inspección visual y líquidos penetrantes | fisuras abiertas a la superficie |
| Partículas magnéticas | fisuras superficiales en materiales ferromagnéticos |
| Ultrasonido | defectos internos y espesores |
| Radiografía | poros e inclusiones en soldaduras y fundiciones |
| Corrientes inducidas | defectos superficiales en tubos |

## Materiales conductores, aislantes y magnéticos

| Material | Resistividad a 20 °C ($\Omega\cdot\text{mm}^2$/m) | Uso |
|---|---|---|
| Cobre | 0,0172 | conductores, bobinados |
| Aluminio | 0,028 | líneas aéreas, barras |
| Constantán, nicrom | 0,5 a 1,1 | resistencias |

Los aislantes se caracterizan por su **rigidez dieléctrica** y su **clase térmica**: la clase B admite 130 °C, la F 155 °C y la H 180 °C. Cada 10 °C de exceso reduce la vida de una aislación aproximadamente a la mitad.

## Corrosión

Es el deterioro electroquímico de un metal: en presencia de un electrolito se forman zonas anódicas, que se disuelven, y catódicas. Formas habituales: uniforme, por picado, galvánica (entre dos metales distintos en contacto) y bajo tensión.

Se combate con:

- **recubrimientos**: pinturas, galvanizado, cromado;
- **aleaciones resistentes**: aceros inoxidables, con más de 11 % de cromo;
- **protección catódica**: ánodos de sacrificio o corriente impresa;
- **diseño**: evitar rincones que retengan agua y pares de metales distintos.
