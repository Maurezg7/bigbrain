+++
title = "Sistemas distribuidos"
summary = "Fallas parciales, relojes lógicos, replicación, consenso con Raft, colas de mensajes, idempotencia, microservicios y patrones de resiliencia."
branch = "Sistemas distribuidos y nube"
order = 1
level = "Avanzado"
tags = ["sistemas distribuidos", "consenso", "Raft", "replicación", "microservicios", "idempotencia"]
+++

## Qué es un sistema distribuido

Un conjunto de computadoras que se comunican por una red y cooperan para aparecer ante el usuario como un único sistema.

| Motivo | Explicación |
|---|---|
| Escalabilidad | atender más carga de la que soporta una sola máquina |
| Disponibilidad | seguir funcionando aunque fallen máquinas |
| Latencia | acercar los datos a usuarios de distintas regiones |

> [!WARNING]
> Distribuir un sistema lo vuelve mucho más complejo. Si una sola máquina alcanza, es la mejor opción.

## Por qué es difícil

### Fallas parciales

En una sola máquina, el programa funciona o no. En un sistema distribuido, **una parte puede fallar mientras el resto sigue**, y no siempre se sabe cuál.

Si se envía una petición y no llega respuesta, hay tres posibilidades indistinguibles:

| Posibilidad | ¿Se ejecutó la operación? |
|---|---|
| la petición se perdió | no |
| el servidor falló antes de procesarla | no |
| se procesó, pero la respuesta se perdió o se demoró | **sí** |

### Las falacias de la computación distribuida

Suposiciones falsas que se cometen al diseñar:

1. La red es confiable.
2. La latencia es cero.
3. El ancho de banda es infinito.
4. La red es segura.
5. La topología no cambia.
6. Hay un solo administrador.
7. El costo de transporte es cero.
8. La red es homogénea.

### No hay reloj global

Los relojes de las máquinas difieren y se desvían. No se puede confiar en las marcas de tiempo para saber qué evento ocurrió primero.

## Relojes lógicos

El **reloj de Lamport** ordena eventos sin relojes físicos. Cada proceso lleva un contador:

1. Antes de cada evento local, incrementa el contador.
2. Al enviar un mensaje, incluye el contador.
3. Al recibir uno, toma el máximo entre el propio y el recibido, y suma 1.

Si el evento $a$ causó $b$, entonces $C(a) < C(b)$. La recíproca no vale: los **relojes vectoriales** (un contador por proceso) sí permiten detectar si dos eventos son **concurrentes**.

## Semánticas de entrega

| Garantía | Significado | Cómo |
|---|---|---|
| A lo sumo una vez | puede perderse; nunca se duplica | enviar sin reintentar |
| Al menos una vez | no se pierde; puede duplicarse | reintentar hasta recibir confirmación |
| Exactamente una vez | | al menos una vez **+ idempotencia** |

### Idempotencia

Una operación es **idempotente** si ejecutarla varias veces tiene el mismo efecto que ejecutarla una.

| Idempotente | No idempotente |
|---|---|
| «fijar el saldo en 100» | «sumar 100 al saldo» |
| borrar el pedido 7 | crear un pedido |

Para volver idempotente una operación, el cliente envía una **clave de idempotencia** única; el servidor recuerda las ya procesadas y devuelve el resultado guardado.

```http
POST /pagos
Idempotency-Key: 7f3c2a9e-51b4-4c0a-9d1e-2b8a6f4e1c33
```

> [!IMPORTANT]
> Como una petición sin respuesta **puede haberse ejecutado**, reintentar solo es seguro si la operación es idempotente.

## Replicación

Mantener copias de los datos en varios nodos.

| Modo | Funcionamiento | Riesgo |
|---|---|---|
| Sincrónica | la escritura se confirma cuando llegó a las réplicas | más lenta; se detiene si una réplica no responde |
| Asincrónica | se confirma enseguida y se propaga después | si el líder cae, se pierden las últimas escrituras |

El **retraso de replicación** produce anomalías: un usuario escribe, lee de una réplica atrasada y no ve su propio cambio.

## Consenso

Lograr que varios nodos **se pongan de acuerdo** en un valor aunque algunos fallen. Es la base de la elección de líder, los bloqueos distribuidos y la replicación consistente.

Un algoritmo de consenso tolera $f$ fallas con $2f + 1$ nodos: necesita una **mayoría** (cuórum).

| Nodos | Mayoría | Fallas toleradas |
|---|---|---|
| 3 | 2 | 1 |
| 5 | 3 | 2 |
| 7 | 4 | 3 |

Dos mayorías siempre comparten al menos un nodo: eso impide que dos grupos aislados tomen decisiones contradictorias (**cerebro dividido**).

### Raft

Diseñado para ser comprensible. Cada nodo es **seguidor**, **candidato** o **líder**.

**Elección de líder**

1. El líder envía latidos periódicos.
2. Si un seguidor no recibe latidos durante un tiempo (aleatorio), se vuelve candidato, incrementa el **mandato** (*term*) y pide votos.
3. Quien obtiene la mayoría es el nuevo líder.

**Replicación del registro**

1. El cliente envía una orden al líder.
2. El líder la agrega a su registro y la envía a los seguidores.
3. Cuando una **mayoría** la guardó, la orden queda **confirmada** y se aplica.

Los tiempos de espera aleatorios evitan que varios candidatos empaten una y otra vez. Usan Raft etcd (y con él Kubernetes), Consul y CockroachDB. **Paxos** es el algoritmo clásico equivalente.

> [!NOTE]
> El **teorema FLP** demuestra que, en una red totalmente asíncrona, ningún algoritmo determinista garantiza alcanzar el consenso si un solo nodo puede fallar. Los algoritmos reales lo sortean con tiempos de espera.

## Transacciones distribuidas

### Confirmación en dos fases

| Fase | El coordinador |
|---|---|
| 1. Preparación | pregunta a cada participante si puede confirmar |
| 2. Confirmación | si todos aceptaron, ordena confirmar; si no, abortar |

Si el coordinador cae entre ambas fases, los participantes quedan bloqueados.

### Sagas

Una **saga** divide la operación en transacciones locales, cada una con una **acción compensatoria** que la deshace.

| Paso | Compensación |
|---|---|
| reservar el stock | liberar el stock |
| cobrar el pago | devolver el pago |
| crear el envío | cancelar el envío |

Si un paso falla, se ejecutan las compensaciones de los anteriores. No hay aislamiento: otros ven estados intermedios.

## Comunicación

| Estilo | Ejemplos | Característica |
|---|---|---|
| Sincrónica | HTTP/REST, gRPC | simple; acopla la disponibilidad de ambos servicios |
| Asincrónica | colas y registros de eventos | desacopla; absorbe picos de carga |

| Tipo | Ejemplos | Modelo |
|---|---|---|
| Cola de mensajes | RabbitMQ, SQS | cada mensaje lo procesa un consumidor y se elimina |
| Registro de eventos | Kafka | los eventos se conservan en orden; cada consumidor lleva su posición y puede releer |

En **publicación y suscripción**, quien publica no conoce a los suscriptores.

## Microservicios

Dividir una aplicación en servicios pequeños, cada uno con su propia base de datos, desplegados por separado.

| | Monolito | Microservicios |
|---|---|---|
| Despliegue | una unidad | independiente por servicio |
| Llamadas | funciones en memoria | red (lenta, puede fallar) |
| Datos | una base, transacciones simples | una base por servicio, consistencia eventual |
| Operación | simple | compleja: observabilidad, redes, versiones |
| Equipos | acoplados | autónomos |

> [!WARNING]
> Los microservicios resuelven un problema **de organización** (muchos equipos sobre el mismo código), no de rendimiento. Para un equipo pequeño, un **monolito modular** bien estructurado suele ser mejor.

## Patrones de resiliencia

| Patrón | Qué hace |
|---|---|
| **Tiempo de espera** | no esperar para siempre una respuesta |
| **Reintento con retroceso exponencial** | esperar 1 s, 2 s, 4 s... con una variación aleatoria (*jitter*) |
| **Cortacircuitos** | tras varios fallos, dejar de llamar al servicio por un tiempo |
| Mamparos (*bulkhead*) | aislar recursos para que un fallo no arrastre al resto |
| Limitación de tasa | rechazar el exceso de peticiones |
| Degradación | responder con funcionalidad reducida |
| Contrapresión | avisar al emisor que debe ir más despacio |

```python
import random, time

def con_reintentos(operacion, intentos=5, base=0.5):
    for n in range(intentos):
        try:
            return operacion()
        except ErrorTransitorio:
            if n == intentos - 1:
                raise
            time.sleep(base * 2**n + random.uniform(0, base))
```

> [!NOTE]
> Sin la variación aleatoria, todos los clientes reintentan en el mismo instante y vuelven a tumbar al servicio que se estaba recuperando.

## Balanceo de carga

Reparte las peticiones entre varias instancias.

| Algoritmo | Criterio |
|---|---|
| Rotación (*round robin*) | por turnos |
| Menos conexiones | la instancia menos ocupada |
| Hash | la misma clave va siempre a la misma instancia |

Las **comprobaciones de salud** retiran las instancias que no responden. Para escalar horizontalmente, los servicios deben ser **sin estado**: el estado vive en la base de datos o en una caché compartida.

## Observabilidad

| Pilar | Responde |
|---|---|
| **Registros** (*logs*) | ¿qué pasó? |
| **Métricas** | ¿cuánto y con qué frecuencia? (latencia, errores, tráfico, saturación) |
| **Trazas** distribuidas | ¿por dónde pasó esta petición y dónde tardó? |

Un **identificador de correlación** que acompaña a la petición por todos los servicios permite reconstruir su recorrido.

La latencia se mide con **percentiles** (p50, p95, p99), no con el promedio: el promedio oculta a los usuarios que esperan mucho.

### Disponibilidad

| Disponibilidad | Inactividad por año |
|---|---|
| 99 % | 3,65 días |
| 99,9 % | 8,8 horas |
| 99,99 % | 53 minutos |
| 99,999 % | 5,3 minutos |

Si una petición necesita que funcionen $n$ servicios en serie, las disponibilidades se multiplican: con 5 servicios de 99,9 %, resulta $0{,}999^5 \approx 99{,}5\ \%$.

> [!TIP]
> Diseñar suponiendo que **todo falla**: cada llamada de red necesita un tiempo de espera, una política de reintentos y una respuesta a la pregunta «¿qué pasa si esto se ejecuta dos veces?».
