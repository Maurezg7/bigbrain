+++
title = "Criptografía y seguridad de redes"
summary = "Cifrado simétrico y asimétrico, funciones hash, firmas digitales, certificados, TLS, almacenamiento de contraseñas, cortafuegos, VPN y ataques comunes."
branch = "Redes"
order = 5
level = "Avanzado"
tags = ["criptografía", "TLS", "hash", "firma digital", "cortafuegos", "seguridad"]
+++

## Objetivos de la seguridad

| Propiedad | Significado | Mecanismo |
|---|---|---|
| **Confidencialidad** | solo los autorizados leen la información | cifrado |
| **Integridad** | la información no se altera sin que se note | hash, MAC, firma |
| **Disponibilidad** | el servicio funciona cuando se lo necesita | redundancia, protección ante DoS |
| Autenticación | saber con quién se habla | contraseñas, certificados |
| No repudio | no poder negar la autoría | firma digital |

Las tres primeras forman la tríada **CIA**.

> [!IMPORTANT]
> **Principio de Kerckhoffs**: un sistema debe ser seguro aunque todo sobre él sea público, salvo la clave. La seguridad no debe depender de mantener en secreto el algoritmo.

## Cifrado simétrico

Emisor y receptor comparten **la misma clave**.

```text
texto claro ──[ cifrar con K ]──▶ texto cifrado ──[ descifrar con K ]──▶ texto claro
```

| Algoritmo | Clave | Estado |
|---|---|---|
| AES | 128, 192 o 256 bits | el estándar actual |
| ChaCha20 | 256 bits | moderno; rápido sin hardware especial |
| DES, 3DES, RC4 | | obsoletos |

Es **muy rápido**, pero plantea un problema: ¿cómo acuerdan la clave dos partes que nunca se vieron?

### Modos de operación

AES cifra bloques de 128 bits. El **modo** define cómo se encadenan:

| Modo | Característica |
|---|---|
| ECB | cifra cada bloque por separado; bloques iguales dan cifrados iguales. **No usar** |
| CBC | encadena cada bloque con el anterior; requiere un vector de inicialización |
| CTR | convierte el cifrado de bloque en uno de flujo |
| **GCM** | cifra y además **autentica** |

> [!WARNING]
> Cifrar no garantiza integridad: un atacante puede alterar el texto cifrado. Usar siempre **cifrado autenticado** (AES-GCM o ChaCha20-Poly1305) y nunca repetir el *nonce* con la misma clave.

## Cifrado asimétrico

Cada parte tiene un **par de claves**:

| Clave | Quién la tiene | Sirve para |
|---|---|---|
| Pública | cualquiera | cifrar para el dueño; verificar sus firmas |
| Privada | solo el dueño | descifrar; firmar |

Lo que se cifra con la pública solo se descifra con la privada.

| Algoritmo | Se basa en | Uso |
|---|---|---|
| RSA | la dificultad de factorizar números grandes | cifrado y firma; claves de 2048 bits o más |
| Diffie-Hellman | el logaritmo discreto | acuerdo de claves |
| Curvas elípticas (ECDH, ECDSA, Ed25519) | el logaritmo discreto en curvas elípticas | acuerdo y firma, con claves mucho más cortas |

Es cientos de veces más lento que el simétrico. Por eso se usa un **esquema híbrido**: la criptografía asimétrica acuerda una clave de sesión, y la simétrica cifra los datos.

### RSA en pequeño

1. Elegir dos primos, $p = 61$ y $q = 53$: $n = pq = 3233$.
2. $\varphi(n) = (p-1)(q-1) = 3120$.
3. Elegir $e = 17$, coprimo con $\varphi(n)$.
4. Calcular $d$ tal que $e \cdot d \equiv 1 \pmod{\varphi(n)}$: $d = 2753$.

La clave pública es $(n, e)$ y la privada, $d$.

$$c = m^e \bmod n \qquad m = c^d \bmod n$$

> [!NOTE]
> Para $m = 65$: $c = 65^{17} \bmod 3233 = 2790$, y $2790^{2753} \bmod 3233 = 65$.

La seguridad depende de que nadie pueda factorizar $n$ para obtener $p$ y $q$.

### Diffie-Hellman

Permite a dos partes acordar un secreto por un canal público. Con un primo $p$ y una base $g$ conocidos por todos:

| Ana | | Bruno |
|---|---|---|
| elige $a$ secreto | | elige $b$ secreto |
| envía $A = g^a \bmod p$ | → | |
| | ← | envía $B = g^b \bmod p$ |
| calcula $B^a \bmod p$ | | calcula $A^b \bmod p$ |

Ambos obtienen $g^{ab} \bmod p$. Un observador ve $A$ y $B$, pero no puede calcular el secreto.

Si las claves son **efímeras** (nuevas en cada sesión) se logra **secreto hacia adelante**: comprometer la clave privada del servidor en el futuro no permite descifrar sesiones pasadas.

## Funciones hash

Una **función hash criptográfica** convierte una entrada de cualquier tamaño en un resumen de longitud fija.

| Propiedad | Significado |
|---|---|
| Determinista | la misma entrada da siempre el mismo resumen |
| Unidireccional | del resumen no se recupera la entrada |
| Resistente a colisiones | es inviable hallar dos entradas con el mismo resumen |
| Efecto avalancha | cambiar un bit cambia el resumen por completo |

| Función | Salida | Estado |
|---|---|---|
| MD5 | 128 bits | rota |
| SHA-1 | 160 bits | rota |
| SHA-256, SHA-512 | 256, 512 bits | seguras |
| SHA-3, BLAKE3 | variable | seguras |

Usos: verificar la integridad de archivos, identificar contenido (Git), firmas digitales, cadenas de bloques.

Un **MAC** (HMAC) combina un hash con una clave secreta: prueba que el mensaje no se alteró **y** que proviene de quien conoce la clave.

## Contraseñas

> [!WARNING]
> Las contraseñas **nunca** se guardan en texto claro ni cifradas, y tampoco con un hash rápido como SHA-256.

Se guardan con una función **lenta a propósito** y con **sal** (un valor aleatorio distinto por usuario):

| Función | |
|---|---|
| Argon2id | la recomendada |
| scrypt, bcrypt | aceptables |

| Elemento | Para qué |
|---|---|
| Sal | dos usuarios con la misma contraseña tienen resúmenes distintos; inutiliza las tablas precalculadas |
| Lentitud | cada intento de adivinar cuesta tiempo y memoria |

La **autenticación de múltiples factores** combina algo que se sabe (contraseña), algo que se tiene (teléfono, llave de seguridad) y algo que se es (huella).

## Firma digital

1. El firmante calcula el hash del documento y lo procesa con su clave **privada**: esa es la firma.
2. Cualquiera la verifica con la clave **pública**.

Garantiza **integridad**, **autenticidad** y **no repudio**.

## Certificados

¿Cómo saber que una clave pública pertenece realmente a `ejemplo.com`? Un **certificado** (X.509) vincula un nombre con una clave pública, y está firmado por una **autoridad de certificación** (CA).

```text
CA raíz (instalada en el sistema)
   └── firma a ──▶ CA intermedia
                      └── firma a ──▶ certificado de ejemplo.com
```

El navegador recorre la cadena hasta una raíz en la que confía, y comprueba el dominio, la vigencia y que no haya sido revocado.

## TLS

**TLS** protege la comunicación entre dos aplicaciones. Es la base de HTTPS.

Saludo de TLS 1.3, simplificado:

| Paso | Qué ocurre |
|---|---|
| 1 | El cliente envía los algoritmos que admite y su parte de Diffie-Hellman |
| 2 | El servidor responde con su parte, su certificado y una firma |
| 3 | Ambos derivan las claves de sesión |
| 4 | El cliente verifica el certificado y la firma |
| 5 | Los datos viajan con cifrado simétrico autenticado |

TLS 1.3 completa el saludo en **una** ida y vuelta. Las versiones anteriores a TLS 1.2 están obsoletas.

## Ataques comunes

| Ataque | Descripción | Defensa |
|---|---|---|
| Escucha (*sniffing*) | leer tráfico ajeno | cifrado (TLS) |
| Intermediario (*man in the middle*) | interponerse y hacerse pasar por cada extremo | certificados, HSTS |
| Suplantación (*spoofing*) | falsificar una dirección IP, MAC o un nombre DNS | autenticación, DNSSEC |
| Denegación de servicio (DoS, DDoS) | saturar al servidor con tráfico | filtrado, CDN, límites de tasa |
| Fuerza bruta | probar contraseñas | límites de intentos, múltiples factores |
| Suplantación de identidad (*phishing*) | engañar al usuario para que entregue sus credenciales | educación, llaves de seguridad |
| Repetición (*replay*) | reenviar un mensaje válido capturado | números de un solo uso, marcas de tiempo |
| Programas maliciosos | virus, *ransomware*, troyanos | actualizaciones, copias de seguridad, mínimo privilegio |

> [!NOTE]
> La mayoría de las intrusiones no rompen la criptografía: aprovechan **contraseñas débiles**, **software sin actualizar**, **configuraciones por defecto** y **errores humanos**.

## Defensas de red

| Herramienta | Función |
|---|---|
| Cortafuegos (*firewall*) | filtra el tráfico según reglas: direcciones, puertos, estado de la conexión |
| VPN | túnel cifrado entre un equipo (o una red) y otra red |
| IDS / IPS | detecta, o detecta y bloquea, patrones de ataque |
| Segmentación | separa la red en zonas (por ejemplo, con VLAN) para limitar el alcance de una intrusión |
| *Proxy* inverso | recibe las peticiones por delante de los servidores |

Política sana de un cortafuegos: **denegar todo por defecto** y abrir solo lo necesario.

### Principios

| Principio | Significado |
|---|---|
| Mínimo privilegio | cada usuario y programa, solo con los permisos que necesita |
| Defensa en profundidad | varias capas: ninguna es infalible |
| Confianza cero | verificar cada acceso, aunque venga de la red interna |
| Seguro por defecto | la configuración inicial debe ser la restrictiva |

> [!WARNING]
> **Nunca implementar criptografía propia.** Usar bibliotecas revisadas (libsodium, las del lenguaje) y protocolos estándar. Los errores sutiles destruyen la seguridad sin dar ningún síntoma.

> [!TIP]
> Una nota sobre el futuro: las computadoras cuánticas grandes romperían RSA y las curvas elípticas. Ya existen algoritmos **poscuánticos** estandarizados (ML-KEM, ML-DSA) y están empezando a desplegarse en TLS.
