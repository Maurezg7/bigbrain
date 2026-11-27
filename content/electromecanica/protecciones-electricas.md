+++
title = "Protecciones eléctricas"
summary = "Fusibles, interruptores termomagnéticos y diferenciales, guardamotores, selectividad y cálculo de cortocircuito."
branch = "Instalaciones y sistemas eléctricos"
order = 2
level = "Intermedio"
tags = ["fusible", "termomagnética", "diferencial", "cortocircuito", "selectividad"]
+++

## De qué hay que proteger

| Falla | Causa | Consecuencia | Protección |
|---|---|---|---|
| Sobrecarga | más corriente que la nominal durante un tiempo | calentamiento y envejecimiento de la aislación | térmica |
| Cortocircuito | contacto entre fases o con neutro | corrientes de miles de amperes, arco, incendio | magnética o fusible |
| Falla a tierra | pérdida de aislación hacia masa | riesgo de electrocución e incendio | diferencial |
| Sobretensión | rayos, maniobras | perforación de aislaciones | descargadores |

## Fusibles

Un fusible es un hilo calibrado que se funde. Es simple, barato y muy rápido: limita la corriente antes de que alcance su pico. Su desventaja es que hay que reponerlo y que la fusión de uno solo deja a un motor trabajando en dos fases. Los de tipo **gG** protegen cables; los **aM** acompañan a motores y soportan su arranque.

## Interruptor termomagnético

![Curva tiempo-corriente de un interruptor termomagnético con zona térmica y zona magnética](/media/electromecanica/curva-disparo.svg)

*Curva de disparo: zona térmica de tiempo inverso para sobrecargas y zona magnética instantánea para cortocircuitos.*

Reúne dos disparos:

- **térmico**: un bimetal que actúa más rápido cuanto mayor es la sobrecarga;
- **magnético**: una bobina que dispara al instante con un cortocircuito.

| Curva | Disparo magnético | Uso |
|---|---|---|
| B | 3 a 5 veces $I_n$ | cargas resistivas, líneas largas |
| C | 5 a 10 veces $I_n$ | uso general |
| D | 10 a 20 veces $I_n$ | motores y transformadores con grandes picos |

Para proteger un cable de corriente admisible $I_z$ que alimenta una carga de corriente $I_B$:

$$I_B \le I_n \le I_z$$

Además, el **poder de corte** del interruptor debe ser mayor que la corriente de cortocircuito en el punto donde se instala.

## Interruptor diferencial

Compara la corriente que sale por las fases con la que vuelve por el neutro. Si difieren, parte de la corriente se está fugando a tierra, quizá a través de una persona, y abre el circuito.

- **30 mA**: protección de personas.
- **300 mA**: protección contra incendio y uso selectivo aguas arriba.

> [!WARNING]
> El diferencial no protege contra sobrecargas ni cortocircuitos, y tampoco actúa si una persona toca a la vez fase y neutro estando aislada de tierra. Siempre va acompañado de una termomagnética y de una buena puesta a tierra.

## Protección de motores

El **guardamotor** combina un relé térmico regulable a la corriente nominal del motor con un disparo magnético alto, y detecta la falta de una fase. El conjunto habitual es guardamotor más contactor, o fusibles aM, contactor y relé térmico.

## Corriente de cortocircuito

Aguas abajo de un transformador, la corriente de cortocircuito trifásico se estima con su tensión de cortocircuito porcentual $u_{cc}$:

$$I_{cc} = \frac{I_n}{u_{cc}} = \frac{S_n}{\sqrt3\,V_L\,u_{cc}}$$

> [!NOTE]
> Transformador de 630 kVA, 400 V, $u_{cc} = 4\ \%$.
>
> $I_n = \dfrac{630\,000}{\sqrt3\cdot 400} \approx 909$ A e $I_{cc} = \dfrac{909}{0{,}04} \approx 22{,}7$ kA en bornes.
>
> El interruptor general necesita un poder de corte de al menos 25 kA. Aguas abajo, la impedancia de los cables va reduciendo ese valor.

El cable debe soportar el cortocircuito durante el tiempo $t$ que tarda la protección. Con $k = 115$ para cobre con PVC:

$$S \ge \frac{I_{cc}\sqrt{t}}{k}$$

## Selectividad

Ante una falla debe abrir **solo** la protección más cercana, dejando en servicio el resto. Se logra escalonando:

- las **corrientes** de regulación (amperimétrica);
- los **tiempos** de disparo (cronométrica);
- o usando interruptores con comunicación entre sí (lógica).

En media y alta tensión las protecciones son relés (de sobrecorriente, diferenciales, de distancia) que ordenan la apertura de interruptores de potencia.
