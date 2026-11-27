+++
title = "Direccionamiento IP y subredes"
summary = "Direcciones IPv4, máscaras y notación CIDR, cálculo de subredes, direcciones privadas, NAT, DHCP e IPv6."
branch = "Redes"
order = 3
level = "Intermedio"
tags = ["IP", "subredes", "CIDR", "NAT", "DHCP", "IPv6"]
+++

## Direcciones IPv4

Una dirección IPv4 tiene **32 bits**, escritos como cuatro números decimales (octetos) de 0 a 255:

```text
192.168.1.10   =   11000000.10101000.00000001.00001010
```

Hay $2^{32} \approx 4\,300$ millones de direcciones: no alcanzan para todos los dispositivos del mundo, y de ahí nacen NAT e IPv6.

Cada dirección tiene dos partes:

| Parte | Identifica |
|---|---|
| **Red** (prefijo) | la red a la que pertenece el equipo |
| **Equipo** (*host*) | el equipo dentro de esa red |

## Máscara y notación CIDR

La **máscara** indica cuántos bits forman la parte de red: tiene unos en esas posiciones y ceros en las de equipo.

```text
Dirección:  192.168.1.10     11000000.10101000.00000001.00001010
Máscara:    255.255.255.0    11111111.11111111.11111111.00000000
                             └────────── red ─────────┘└ equipo ┘
```

La notación **CIDR** escribe la cantidad de bits de red tras una barra: `192.168.1.10/24`.

| CIDR | Máscara | Direcciones | Equipos utilizables |
|---|---|---|---|
| /8 | 255.0.0.0 | 16 777 216 | 16 777 214 |
| /16 | 255.255.0.0 | 65 536 | 65 534 |
| /24 | 255.255.255.0 | 256 | 254 |
| /25 | 255.255.255.128 | 128 | 126 |
| /26 | 255.255.255.192 | 64 | 62 |
| /27 | 255.255.255.224 | 32 | 30 |
| /28 | 255.255.255.240 | 16 | 14 |
| /30 | 255.255.255.252 | 4 | 2 |
| /32 | 255.255.255.255 | 1 | un solo equipo |

> [!IMPORTANT]
> Con un prefijo de $n$ bits quedan $32 - n$ bits de equipo:
> $$\text{direcciones} = 2^{32-n} \qquad \text{equipos utilizables} = 2^{32-n} - 2$$
> Se restan dos: la **dirección de red** (bits de equipo en 0) y la de **difusión** (bits de equipo en 1).

## Calcular una subred

Dada `172.16.45.200/20`:

**1. Máscara.** 20 bits en uno: `255.255.240.0`.

**2. Dirección de red.** Se hace AND bit a bit entre la dirección y la máscara. Solo cambia el tercer octeto:

```text
45  = 00101101
240 = 11110000
AND = 00100000 = 32        →  red: 172.16.32.0
```

**3. Difusión.** Se ponen en 1 los 12 bits de equipo:

```text
tercer octeto: 0010 1111 = 47     →  difusión: 172.16.47.255
```

**4. Rango utilizable.** De `172.16.32.1` a `172.16.47.254`: $2^{12} - 2 = 4094$ equipos.

### Atajo: el número mágico

En el octeto donde la máscara no es 0 ni 255, restar su valor de 256: da el **tamaño del bloque**. Las redes empiezan en los múltiplos de ese número.

> [!NOTE]
> **Ejemplo.** `192.168.10.77/27`. La máscara es `255.255.255.224`; el bloque es $256 - 224 = 32$. Las redes empiezan en 0, 32, 64, 96... El 77 cae entre 64 y 96:
>
> | | |
> |---|---|
> | Red | 192.168.10.64 |
> | Primer equipo | 192.168.10.65 |
> | Último equipo | 192.168.10.94 |
> | Difusión | 192.168.10.95 |

Dos equipos están en la **misma red** si sus direcciones de red coinciden. Si no, deben comunicarse a través de un enrutador (la **puerta de enlace**).

## Dividir una red

Para partir una red en subredes se **toman prestados** bits de la parte de equipo. Con $k$ bits prestados se obtienen $2^k$ subredes.

> [!NOTE]
> **Ejemplo.** Dividir `192.168.1.0/24` en 4 subredes iguales. Hacen falta 2 bits ($2^2 = 4$): el prefijo pasa a /26, con bloques de 64.
>
> | Subred | Rango utilizable | Difusión |
> |---|---|---|
> | 192.168.1.0/26 | .1 a .62 | .63 |
> | 192.168.1.64/26 | .65 a .126 | .127 |
> | 192.168.1.128/26 | .129 a .190 | .191 |
> | 192.168.1.192/26 | .193 a .254 | .255 |

### Subredes de tamaño variable (VLSM)

Cuando las subredes necesitan distinta cantidad de equipos, se asigna a cada una el prefijo justo, **empezando por la más grande**.

> [!NOTE]
> **Ejemplo.** Con `10.0.0.0/24` hay que alojar redes de 100, 50 y 20 equipos, y un enlace punto a punto.
>
> | Necesidad | Prefijo | Subred | Utilizables |
> |---|---|---|---|
> | 100 equipos | /25 | 10.0.0.0/25 | 126 |
> | 50 equipos | /26 | 10.0.0.128/26 | 62 |
> | 20 equipos | /27 | 10.0.0.192/27 | 30 |
> | enlace (2) | /30 | 10.0.0.224/30 | 2 |

La operación inversa, **agregar** varias redes contiguas en un prefijo más corto, reduce las tablas de rutas: `192.168.0.0/24` y `192.168.1.0/24` se resumen en `192.168.0.0/23`.

## Direcciones especiales

| Rango | Uso |
|---|---|
| 10.0.0.0/8 | privado |
| 172.16.0.0/12 | privado (172.16.0.0 a 172.31.255.255) |
| 192.168.0.0/16 | privado |
| 127.0.0.0/8 | bucle local (`127.0.0.1` es *localhost*) |
| 169.254.0.0/16 | enlace local, autoasignado cuando no hay DHCP |
| 0.0.0.0 | «cualquier dirección» o «esta red» |
| 255.255.255.255 | difusión local |
| 224.0.0.0/4 | multidifusión |

Las direcciones **privadas** no circulan por internet: cualquiera puede usarlas dentro de su red.

## NAT

La **traducción de direcciones** permite que muchos equipos con direcciones privadas compartan **una** dirección pública.

El enrutador reescribe la dirección y el puerto de origen de cada paquete que sale, y recuerda la correspondencia para devolver las respuestas:

| Interno (privado) | Externo (público) |
|---|---|
| 192.168.1.10:51000 | 200.45.10.7:40001 |
| 192.168.1.11:51000 | 200.45.10.7:40002 |

| Ventaja | Desventaja |
|---|---|
| ahorra direcciones públicas | rompe la comunicación de extremo a extremo |
| oculta la estructura interna | las conexiones entrantes requieren redirección de puertos |

La **redirección de puertos** envía lo que llega a un puerto público hacia un equipo interno; es lo que permite alojar un servidor detrás de NAT.

## DHCP

**DHCP** configura automáticamente los equipos que se conectan:

| Paso | Mensaje | Sentido |
|---|---|---|
| 1 | *Discover*: «¿hay algún servidor?» | cliente → difusión |
| 2 | *Offer*: «te ofrezco esta dirección» | servidor → cliente |
| 3 | *Request*: «la acepto» | cliente → difusión |
| 4 | *Ack*: «confirmada» | servidor → cliente |

El servidor entrega la dirección IP, la máscara, la puerta de enlace y los servidores DNS, por un tiempo limitado (**concesión**) que el cliente renueva.

## IPv6

IPv6 usa direcciones de **128 bits**: $2^{128} \approx 3{,}4 \times 10^{38}$.

Se escriben en ocho grupos de cuatro dígitos hexadecimales:

```text
2001:0db8:0000:0000:0000:ff00:0042:8329
```

Reglas para abreviar:

1. Se omiten los ceros a la izquierda de cada grupo.
2. **Una sola vez**, una secuencia de grupos en cero se reemplaza por `::`.

```text
2001:db8::ff00:42:8329
```

| Dirección | Significado |
|---|---|
| `::1` | bucle local |
| `::` | sin especificar |
| `fe80::/10` | enlace local |
| `fc00::/7` | local única (equivale a las privadas) |
| `2000::/3` | globales |
| `ff00::/8` | multidifusión |

Lo habitual es asignar a cada red un prefijo **/64**: la segunda mitad identifica la interfaz.

| | IPv4 | IPv6 |
|---|---|---|
| Longitud | 32 bits | 128 bits |
| Notación | decimal con puntos | hexadecimal con dos puntos |
| Configuración | manual o DHCP | autoconfiguración (SLAAC) o DHCPv6 |
| NAT | habitual | innecesario |
| Difusión | sí | no (usa multidifusión) |
| Resolución de vecinos | ARP | NDP (sobre ICMPv6) |
| Fragmentación | en los enrutadores | solo en el origen |

Las dos versiones conviven: la mayoría de los equipos tiene **doble pila** y usa IPv6 cuando está disponible.

## Diagnóstico

| Comando | Para qué |
|---|---|
| `ip addr` (Linux), `ipconfig` (Windows) | ver las direcciones propias |
| `ip route` | ver la tabla de rutas y la puerta de enlace |
| `ping 8.8.8.8` | probar la conectividad por IP |
| `ping dominio` | probar además la resolución de nombres |
| `traceroute destino` | ver los saltos del camino |
| `ipcalc 192.168.1.77/27` | calcular una subred |

> [!TIP]
> Para diagnosticar, ir de abajo hacia arriba: ¿tengo dirección IP?, ¿llego a la puerta de enlace?, ¿llego a una IP de internet?, ¿resuelvo nombres? El primer paso que falla señala el problema.
