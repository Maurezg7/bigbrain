+++
title = "Modelo TCP/IP y capa de transporte"
summary = "Capas y encapsulamiento, Ethernet y conmutación, IP y enrutamiento, UDP, y TCP en detalle: conexión, fiabilidad, control de flujo y de congestión."
branch = "Redes"
order = 2
level = "Intermedio"
tags = ["TCP", "IP", "UDP", "capas", "Ethernet", "enrutamiento"]
+++

## Capas

Una red se organiza en **capas**: cada una resuelve un problema y ofrece un servicio a la de arriba.

| Capa TCP/IP | Función | Unidad | Ejemplos | Capa OSI |
|---|---|---|---|---|
| Aplicación | servicios al usuario | mensaje | HTTP, DNS, SMTP, SSH | 5, 6 y 7 |
| Transporte | comunicación entre procesos | segmento | TCP, UDP, QUIC | 4 |
| Red (internet) | llevar paquetes entre redes | paquete | IP, ICMP | 3 |
| Enlace | transmisión entre nodos vecinos | trama | Ethernet, Wi-Fi | 2 |
| Física | bits por el medio | bit | cobre, fibra, radio | 1 |

El modelo **OSI** de 7 capas es la referencia teórica; internet usa TCP/IP.

### Encapsulamiento

Cada capa agrega su **encabezado** a lo que recibe de la superior:

```text
                                  ┌──────────────┐
aplicación                        │    datos     │
                         ┌────────┼──────────────┤
transporte               │ TCP    │    datos     │      segmento
                ┌────────┼────────┼──────────────┤
red             │ IP     │ TCP    │    datos     │      paquete
       ┌────────┼────────┼────────┼──────────────┼─────┐
enlace │ Ether. │ IP     │ TCP    │    datos     │ FCS │ trama
       └────────┴────────┴────────┴──────────────┴─────┘
```

El receptor quita los encabezados en orden inverso.

## Capa de enlace

Comunica equipos de una **misma red local**. Cada interfaz tiene una dirección **MAC** de 48 bits, por ejemplo `3c:22:fb:a1:09:7e`.

| Equipo | Capa | Qué hace |
|---|---|---|
| Conmutador (*switch*) | 2 | reenvía tramas según la MAC de destino; aprende qué MAC hay en cada puerto |
| Enrutador (*router*) | 3 | reenvía paquetes entre redes según la dirección IP |
| Punto de acceso | 2 | conecta equipos inalámbricos a la red cableada |

**ARP** averigua la MAC que corresponde a una dirección IP de la red local: pregunta a todos «¿quién tiene 192.168.1.1?» y el dueño responde.

Ethernet incluye una suma de verificación (CRC): las tramas dañadas se descartan, no se corrigen.

## Capa de red: IP

IP entrega paquetes de un equipo a otro, a través de muchas redes. Su servicio es de **mejor esfuerzo**:

> [!IMPORTANT]
> IP **no garantiza** que un paquete llegue, ni que llegue una sola vez, ni en orden. La fiabilidad, si hace falta, la agrega la capa de transporte.

Campos principales del encabezado IPv4:

| Campo | Función |
|---|---|
| Dirección de origen y de destino | 32 bits cada una |
| TTL (tiempo de vida) | se resta 1 en cada enrutador; al llegar a 0, el paquete se descarta |
| Protocolo | qué lleva: TCP (6), UDP (17), ICMP (1) |
| Longitud, identificación, fragmentación | |
| Suma de verificación | del encabezado |

### Enrutamiento

Cada enrutador tiene una **tabla de rutas** y decide solo el **siguiente salto**:

| Destino | Siguiente salto | Interfaz |
|---|---|---|
| 192.168.1.0/24 | directo | eth0 |
| 10.0.0.0/8 | 192.168.1.254 | eth0 |
| 0.0.0.0/0 (por defecto) | 200.45.10.1 | eth1 |

Si varias rutas coinciden, gana la del **prefijo más largo** (la más específica).

| Protocolo de enrutamiento | Ámbito | Algoritmo |
|---|---|---|
| OSPF | dentro de una organización | estado de enlace (Dijkstra) |
| RIP | redes pequeñas | vector de distancias (Bellman-Ford) |
| BGP | entre proveedores de internet | vector de caminos y políticas |

**ICMP** transporta mensajes de control. Lo usan `ping` (eco) y `traceroute` (que envía paquetes con TTL creciente para descubrir cada salto).

## Capa de transporte

IP llega hasta un **equipo**; el transporte llega hasta un **proceso**, identificado por un **puerto** (16 bits: 0 a 65 535).

Una conexión queda identificada por cuatro valores: IP y puerto de origen, IP y puerto de destino.

| Puerto | Servicio | Puerto | Servicio |
|---|---|---|---|
| 22 | SSH | 443 | HTTPS |
| 25 | SMTP | 3306 | MySQL |
| 53 | DNS | 5432 | PostgreSQL |
| 80 | HTTP | 6379 | Redis |

Los puertos menores a 1024 están reservados para servicios del sistema.

## UDP

**UDP** agrega a IP solo los puertos y una suma de verificación.

| | |
|---|---|
| Sin conexión | no hay saludo previo |
| Sin garantías | los datagramas pueden perderse, duplicarse o desordenarse |
| Encabezado | 8 bytes |
| Ventaja | mínima demora y sobrecarga |

Se usa donde la rapidez importa más que la entrega completa (voz, video en vivo, juegos), en consultas cortas (DNS) y como base de protocolos que implementan su propia fiabilidad (**QUIC**, usado por HTTP/3).

## TCP

**TCP** ofrece un flujo de bytes **fiable y ordenado** entre dos procesos.

| Servicio | Mecanismo |
|---|---|
| Entrega fiable | números de secuencia, confirmaciones (ACK) y retransmisión |
| Orden | el receptor reordena según los números de secuencia |
| Sin duplicados | se descartan por número de secuencia |
| Control de flujo | ventana de recepción |
| Control de congestión | ventana de congestión |

### Establecer la conexión

El **saludo de tres vías**:

```text
Cliente                              Servidor
   │ ─────── SYN (seq = x) ──────────▶ │
   │ ◀── SYN + ACK (seq = y, ack = x+1) │
   │ ─────── ACK (ack = y+1) ────────▶ │
   │                                    │
   │ ◀────────── datos ──────────────▶ │
```

Ambos lados acuerdan sus números de secuencia iniciales. El cierre usa segmentos **FIN**, uno por cada sentido.

### Fiabilidad

Cada byte tiene un número de secuencia. El receptor confirma con el número del **próximo byte que espera** (confirmación acumulativa).

Si la confirmación no llega antes de que venza un temporizador, el emisor **retransmite**. El temporizador se ajusta según el tiempo de ida y vuelta medido (RTT). Tres confirmaciones duplicadas disparan una **retransmisión rápida**, sin esperar el temporizador.

### Control de flujo

El receptor anuncia cuánto espacio libre tiene en su búfer (**ventana de recepción**). El emisor no envía más que eso: evita desbordar a un receptor lento.

### Control de congestión

Evita saturar **la red**. El emisor mantiene una **ventana de congestión** (`cwnd`) y envía, a lo sumo, el mínimo entre ella y la ventana de recepción.

| Fase | Comportamiento |
|---|---|
| Arranque lento | `cwnd` se **duplica** en cada ida y vuelta, hasta un umbral |
| Evitación de congestión | `cwnd` crece **linealmente** |
| Pérdida detectada | `cwnd` se **reduce** (a la mitad, o a 1 segmento si venció el temporizador) |

Es el esquema de **incremento aditivo y decremento multiplicativo**: tantea la capacidad disponible y retrocede ante las pérdidas. Así muchas conexiones se reparten la red sin colapsarla.

Algoritmos actuales: **CUBIC** (el predeterminado en Linux) y **BBR** (que estima el ancho de banda en lugar de reaccionar a las pérdidas).

## TCP o UDP

| | TCP | UDP |
|---|---|---|
| Conexión | sí | no |
| Fiabilidad y orden | sí | no |
| Control de congestión | sí | no |
| Unidad | flujo de bytes | datagramas |
| Encabezado | 20 bytes o más | 8 bytes |
| Usos | web, correo, archivos, bases de datos | DNS, voz, video en vivo, juegos, QUIC |

> [!WARNING]
> TCP es un **flujo de bytes**, no de mensajes: dos envíos pueden llegar juntos, o uno puede llegar partido. La aplicación debe delimitar sus mensajes (con una longitud previa o un separador).

## Sockets

Los programas usan la red mediante **sockets**.

```python
import socket

# Servidor
servidor = socket.socket(socket.AF_INET, socket.SOCK_STREAM)     # TCP
servidor.bind(("0.0.0.0", 9000))
servidor.listen()

conexion, direccion = servidor.accept()        # espera a un cliente
datos = conexion.recv(1024)
conexion.sendall(datos.upper())
conexion.close()
```

```python
# Cliente
cliente = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
cliente.connect(("127.0.0.1", 9000))
cliente.sendall(b"hola")
print(cliente.recv(1024))                      # b'HOLA'
cliente.close()
```

## Medidas de rendimiento

| Medida | Significado |
|---|---|
| Ancho de banda | capacidad máxima del enlace (bits por segundo) |
| Rendimiento (*throughput*) | tasa realmente lograda |
| Latencia | demora de un extremo al otro |
| RTT | tiempo de ida y vuelta |
| *Jitter* | variación de la latencia |
| Pérdida de paquetes | fracción que no llega |

La latencia tiene un piso físico: la señal viaja por fibra a unos 200 000 km/s, de modo que 10 000 km cuestan al menos 50 ms por sentido.

> [!TIP]
> Para ver el tráfico real, capa por capa, usar **Wireshark** o `tcpdump`. Observar un saludo de tres vías propio enseña más que cualquier diagrama.
