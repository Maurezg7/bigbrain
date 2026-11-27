+++
title = "Redes y el modelo TCP/IP"
summary = "Cómo viaja un dato por Internet: capas, direcciones IP, TCP, UDP, DNS y HTTP."
branch = "Redes"
order = 1
level = "Intermedio"
tags = ["TCP/IP", "IP", "DNS", "HTTP", "protocolos"]
+++

## Protocolos y capas

Un **protocolo** es un conjunto de reglas que dos equipos acuerdan para comunicarse. Como el problema completo es muy complejo, se divide en **capas**: cada una resuelve una parte y usa los servicios de la que tiene debajo.

![Las cuatro capas del modelo TCP/IP y el encapsulamiento de los datos](/media/informatica/tcp-ip.svg)

*Al enviar, cada capa agrega su encabezado a lo que recibe de la capa superior (encapsulamiento). Al recibir, se quitan en orden inverso.*

| Capa | Función | Protocolos | Unidad |
|---|---|---|---|
| Aplicación | servicios para los programas | HTTP, DNS, SMTP, SSH | mensaje |
| Transporte | comunicación entre procesos | TCP, UDP | segmento |
| Internet | llevar paquetes entre redes | IP, ICMP | paquete |
| Acceso a la red | transmitir por un enlace físico | Ethernet, Wi-Fi | trama |

## Direcciones IP

Cada equipo conectado tiene una **dirección IP**. En **IPv4** son 32 bits, escritos como cuatro números de 0 a 255: `192.168.1.10`. Una parte identifica la **red** y otra el **equipo** dentro de ella; la **máscara** indica dónde está el límite.

> [!NOTE]
> La dirección `192.168.1.10/24` tiene 24 bits de red y 8 de equipo.
>
> - Máscara: `255.255.255.0`
> - Dirección de red: `192.168.1.0`
> - Difusión (*broadcast*): `192.168.1.255`
> - Equipos disponibles: $2^8 - 2 = 254$

Las $2^{32}$ direcciones IPv4 (unos 4300 millones) ya no alcanzan, y por eso existe **IPv6**, con direcciones de 128 bits. Mientras tanto, **NAT** permite que toda una red doméstica salga a Internet con una única dirección pública.

Los **routers** conectan redes entre sí y reenvían cada paquete hacia su destino, salto a salto.

## TCP y UDP

La capa de transporte entrega los datos al programa correcto, identificado por un número de **puerto** (80 para HTTP, 443 para HTTPS, 22 para SSH).

| | TCP | UDP |
|---|---|---|
| Conexión | sí (saludo de tres vías) | no |
| Entrega garantizada | sí: retransmite lo perdido | no |
| Orden | garantizado | no garantizado |
| Velocidad | menor, más sobrecarga | mayor |
| Usos | web, correo, archivos | streaming, juegos, llamadas, DNS |

TCP establece la conexión con el **saludo de tres vías**: el cliente envía `SYN`, el servidor responde `SYN-ACK` y el cliente confirma con `ACK`.

## DNS

Las personas recuerdan nombres; las redes usan números. El **DNS** (sistema de nombres de dominio) traduce `bigbrain.example` a una dirección IP, consultando una jerarquía de servidores y guardando las respuestas en caché.

## HTTP

Es el protocolo de la web. El cliente envía una **petición** y el servidor contesta con una **respuesta**:

```http
GET /api/areas HTTP/1.1
Host: bigbrain.example
Accept: application/json
```

```http
HTTP/1.1 200 OK
Content-Type: application/json

[{"slug": "matematicas", "title": "Matemáticas"}]
```

| Método | Uso | | Código | Significado |
|---|---|---|---|---|
| `GET` | leer un recurso | | `200` | correcto |
| `POST` | crear | | `301` | redirección permanente |
| `PUT` | reemplazar | | `404` | no encontrado |
| `DELETE` | borrar | | `500` | error del servidor |

**HTTPS** es HTTP sobre **TLS**, que cifra la comunicación y verifica la identidad del servidor con un certificado.

## Qué pasa al abrir una página

1. El navegador pregunta al DNS la dirección IP del dominio.
2. Abre una conexión TCP al puerto 443 de esa IP y negocia TLS.
3. Envía la petición HTTP `GET`.
4. Los paquetes atraviesan varios routers hasta el servidor.
5. El servidor responde con el HTML, y el navegador pide luego el CSS, el JavaScript y las imágenes.
