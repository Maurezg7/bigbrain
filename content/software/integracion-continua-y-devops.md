+++
title = "Integración continua y DevOps"
summary = "Automatización de compilación y pruebas, entrega y despliegue continuos, contenedores, infraestructura como código y observabilidad."
branch = "Ingeniería de software"
order = 5
level = "Intermedio"
tags = ["CI/CD", "DevOps", "Docker", "despliegue", "automatización"]
+++

## Qué es DevOps

**DevOps** es una forma de trabajo que une al equipo que **desarrolla** (*Dev*) con el que **opera** (*Ops*) el software, para entregar cambios con más frecuencia y menos riesgo.

Antes, un equipo escribía el código y «lo tiraba por encima de la pared» a otro que lo mantenía funcionando, con objetivos opuestos: unos querían cambiar, los otros estabilidad.

> [!IMPORTANT]
> La idea central: **automatizar** todo lo repetitivo y hacer cambios **pequeños y frecuentes**, que son más fáciles de probar y de revertir.

## Integración continua

En la **integración continua** (CI), cada desarrollador integra su trabajo en la rama principal con frecuencia (al menos una vez por día), y cada integración dispara automáticamente la compilación y las pruebas.

Sin CI, las ramas viven semanas separadas y unirlas es doloroso. Con CI, los conflictos se detectan en minutos.

Una **tubería** (*pipeline*) típica:

| Etapa | Qué hace |
|---|---|
| Descarga | obtiene el código del repositorio |
| Dependencias | instala las bibliotecas (con caché) |
| Análisis estático | formato, *linter*, tipos |
| Compilación | construye el programa |
| Pruebas | unitarias y de integración |
| Seguridad | busca vulnerabilidades en dependencias y secretos expuestos |
| Artefacto | empaqueta el resultado (binario, imagen) |

Ejemplo con GitHub Actions:

```yaml
name: CI
on: [push, pull_request]

jobs:
  verificar:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo fmt --check
      - run: cargo clippy -- -D warnings
      - run: cargo test
```

Reglas de una buena CI:

- La tubería debe ser **rápida** (idealmente menos de 10 minutos).
- Si se rompe, arreglarla es la **prioridad**.
- No se fusiona nada con la tubería en rojo.
- Lo que corre en CI debe poder correrse igual en la máquina propia.

## Entrega y despliegue continuos

| Práctica | Qué automatiza | Paso a producción |
|---|---|---|
| Integración continua | compilar y probar | — |
| **Entrega continua** | además, deja una versión lista para publicar | manual (un botón) |
| **Despliegue continuo** | todo | automático si las pruebas pasan |

### Entornos

| Entorno | Para qué |
|---|---|
| Desarrollo | la máquina de cada programador |
| Pruebas / *staging* | réplica de producción para validar |
| Producción | lo que usan los usuarios reales |

La configuración (direcciones, claves) se separa del código y se inyecta por **variables de entorno**.

> [!WARNING]
> Nunca guardar contraseñas, tokens ni claves en el repositorio. Se usan gestores de secretos. Un secreto que llegó al historial de Git debe considerarse comprometido y hay que rotarlo.

### Estrategias de despliegue

| Estrategia | Cómo funciona | Ventaja |
|---|---|---|
| Recreación | se apaga la versión vieja y se levanta la nueva | simple; hay corte |
| Progresiva (*rolling*) | se reemplazan las instancias de a poco | sin corte |
| Azul-verde | dos entornos completos; se cambia el tráfico de uno a otro | reversión instantánea |
| Canario | la versión nueva recibe primero un pequeño porcentaje del tráfico | limita el daño de un error |

Las **banderas de funcionalidad** (*feature flags*) permiten desplegar código apagado y activarlo después, sin un nuevo despliegue.

Siempre debe existir un camino de **reversión** (*rollback*).

## Contenedores

Un **contenedor** empaqueta una aplicación con todas sus dependencias en una unidad que corre igual en cualquier máquina. Resuelve el «en mi máquina funciona».

| | Máquina virtual | Contenedor |
|---|---|---|
| Incluye | un sistema operativo completo | solo la aplicación y sus bibliotecas |
| Comparte el núcleo del anfitrión | no | sí |
| Arranque | minutos | segundos |
| Tamaño | gigabytes | megabytes |
| Aislamiento | más fuerte | más liviano |

| Concepto | Qué es |
|---|---|
| **Imagen** | la plantilla inmutable, construida por capas |
| **Contenedor** | una instancia en ejecución de una imagen |
| **Dockerfile** | la receta para construir la imagen |
| **Registro** | el almacén de imágenes |
| **Volumen** | almacenamiento que sobrevive al contenedor |

```dockerfile
FROM rust:1 AS compilacion
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=compilacion /app/target/release/servidor /usr/local/bin/servidor
EXPOSE 8787
CMD ["servidor"]
```

La construcción en **varias etapas** deja en la imagen final solo el binario, sin el compilador.

```bash
docker build -t mi-app .
docker run -p 8787:8787 mi-app
```

**Docker Compose** levanta varios contenedores relacionados (aplicación, base de datos) con un solo archivo.

### Orquestación

Con muchos contenedores en muchas máquinas hace falta un **orquestador**. **Kubernetes** es el estándar: distribuye los contenedores, los reinicia si fallan, escala según la carga y hace despliegues progresivos.

## Infraestructura como código

Los servidores, redes y bases de datos se definen en **archivos de texto** versionados, en lugar de configurarse a mano.

| Herramienta | Uso |
|---|---|
| Terraform, OpenTofu | crear recursos en la nube |
| Ansible | configurar servidores |
| Manifiestos de Kubernetes, Helm | describir despliegues |

Ventajas: es reproducible, se revisa como cualquier código, y el historial dice quién cambió qué.

La mayoría de estas herramientas son **declarativas**: se describe el estado deseado y la herramienta calcula los cambios.

## La nube

| Modelo | Qué provee el proveedor | Ejemplo |
|---|---|---|
| IaaS | máquinas virtuales, redes, discos | AWS EC2 |
| PaaS | una plataforma donde subir la aplicación | Heroku, Fly.io |
| SaaS | el software completo | correo web |
| Sin servidor (*serverless*) | ejecuta funciones bajo demanda | AWS Lambda |

## Observabilidad

Saber qué pasa dentro de un sistema en producción. Tres pilares:

| Pilar | Qué responde | Herramientas |
|---|---|---|
| **Registros** (*logs*) | ¿qué ocurrió? | Loki, Elasticsearch |
| **Métricas** | ¿cuánto y qué tan rápido? | Prometheus, Grafana |
| **Trazas** | ¿por dónde pasó este pedido? | OpenTelemetry, Jaeger |

Las cuatro **señales doradas** de un servicio: **latencia**, **tráfico**, **errores** y **saturación**.

Las **alertas** deben avisar de problemas que requieren acción humana; demasiadas alertas se terminan ignorando.

### Confiabilidad

| Sigla | Significado |
|---|---|
| SLI | el indicador medido (por ejemplo, porcentaje de pedidos exitosos) |
| SLO | el objetivo interno (99,9 %) |
| SLA | el compromiso contractual con el cliente |

> [!NOTE]
> Una disponibilidad del 99,9 % permite unas $8{,}8$ horas de caída por año; el 99,99 %, unos $53$ minutos.

Después de un incidente se escribe una **revisión sin culpables** (*postmortem*): qué pasó, por qué, y qué se cambia para que no se repita. Se busca la falla del sistema, no un culpable.

## Métricas de entrega

Los cuatro indicadores **DORA** miden el desempeño de un equipo:

| Métrica | Mide |
|---|---|
| Frecuencia de despliegue | cada cuánto se publica |
| Tiempo de entrega de un cambio | del commit a producción |
| Tasa de fallos por cambio | qué porcentaje de despliegues causa problemas |
| Tiempo de recuperación | cuánto se tarda en restablecer el servicio |

Los mejores equipos son a la vez más rápidos **y** más estables: no hay que elegir.

## Seguridad integrada

**DevSecOps** incorpora la seguridad desde el principio de la tubería: análisis de dependencias, búsqueda de secretos, análisis estático, escaneo de imágenes y el principio de **mínimo privilegio**.

> [!TIP]
> Si una tarea se hace a mano por tercera vez, es momento de automatizarla.
