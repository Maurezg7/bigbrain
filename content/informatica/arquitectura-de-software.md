+++
title = "Arquitectura de software"
summary = "Capas, arquitectura hexagonal, monolito y microservicios, comunicación síncrona y asíncrona, caché, escalabilidad y observabilidad."
branch = "Buenas prácticas"
order = 5
level = "Avanzado"
tags = ["arquitectura", "microservicios", "hexagonal", "escalabilidad", "Docker"]
+++

## Qué es la arquitectura

Son las decisiones **difíciles de cambiar** más adelante: cómo se divide el sistema, cómo se comunican las partes, dónde viven los datos. No hay una arquitectura mejor en general; hay compromisos entre simplicidad, costo, rendimiento y capacidad de cambio.

## Arquitectura en capas

| Capa | Responsabilidad |
|---|---|
| Presentación | HTTP, interfaz: recibir y responder |
| Aplicación | casos de uso: coordina los pasos |
| Dominio | reglas del negocio |
| Infraestructura | base de datos, correo, servicios externos |

Cada capa conoce solo a la de abajo. Es simple y alcanza para la mayoría de las aplicaciones.

## Arquitectura hexagonal

También llamada **puertos y adaptadores**. Pone el dominio en el centro, sin depender de nada externo.

![Arquitectura hexagonal: el dominio en el centro, con puertos y adaptadores hacia la web, la base de datos y otros servicios](/media/informatica/arquitectura-hexagonal.svg)

- Un **puerto** es una interfaz que el dominio define: «necesito guardar temas».
- Un **adaptador** la implementa con una tecnología concreta: Postgres, un archivo, memoria.

```rust
// puerto: lo define el dominio
pub trait RepositorioDeTemas {
    async fn buscar(&self, slug: &str) -> Result<Option<Tema>, Error>;
}

// adaptador: detalle de infraestructura
pub struct RepositorioPostgres { pool: PgPool }

impl RepositorioDeTemas for RepositorioPostgres {
    async fn buscar(&self, slug: &str) -> Result<Option<Tema>, Error> {
        // consulta SQL
    }
}
```

Ventajas: el dominio se prueba sin base ni red, y cambiar de tecnología toca solo un adaptador. Costo: más código e indirección; en una aplicación chica puede sobrar.

## Monolito y microservicios

| | Monolito | Microservicios |
|---|---|---|
| Despliegue | una sola unidad | muchos servicios independientes |
| Comunicación interna | llamadas a funciones | red |
| Datos | una base | una base por servicio |
| Transacciones | simples | distribuidas y difíciles |
| Operación | sencilla | compleja: orquestación, trazas, redes |
| Escalar equipos | cuesta con muchos equipos | cada equipo, su servicio |
| Depurar | fácil | difícil |

> [!TIP]
> Empezá con un **monolito modular**: una sola aplicación con módulos bien separados y límites claros. Si algún día hace falta, un módulo se extrae como servicio. Los microservicios resuelven problemas de organizaciones grandes, y traen los problemas de un sistema distribuido.

## Comunicación entre servicios

| Estilo | Ejemplo | Característica |
|---|---|---|
| Síncrona | HTTP, gRPC | simple; quien llama espera, y si el otro cae, falla |
| Asíncrona | colas y eventos (RabbitMQ, Kafka) | desacopla y absorbe picos; más difícil de seguir |

En comunicación asíncrona los mensajes pueden llegar repetidos o desordenados, así que los consumidores deben ser **idempotentes**.

## Resiliencia

En un sistema distribuido las fallas son normales.

| Patrón | Qué hace |
|---|---|
| Límite de tiempo | no esperar para siempre |
| Reintento con espera creciente | reintentar cada vez más espaciado y con algo de azar |
| Cortacircuitos | dejar de llamar por un rato a un servicio que está fallando |
| Mamparo | aislar recursos para que una falla no se propague |
| Degradación | responder con menos funciones en lugar de caerse |

## Caché

Guardar resultados costosos para reutilizarlos.

| Nivel | Ejemplo |
|---|---|
| Navegador | `Cache-Control` |
| CDN | archivos estáticos cerca del usuario |
| Aplicación | Redis, memoria |
| Base de datos | índices, vistas materializadas |

Lo difícil es la **invalidación**: saber cuándo el dato guardado dejó de ser válido. Lo más simple es un tiempo de vida corto.

## Escalabilidad

| Tipo | Cómo | Límite |
|---|---|---|
| Vertical | una máquina más potente | precio y tope físico |
| Horizontal | más máquinas detrás de un balanceador | exige servicios sin estado |

Para escalar horizontalmente la aplicación no debe guardar estado en memoria ni en el disco local: las sesiones y los archivos van a servicios compartidos. En la base de datos se suman réplicas de lectura y, en casos extremos, particionado.

## Teorema CAP

Ante una **partición** de red, un sistema distribuido debe elegir entre **consistencia** (todos ven el mismo dato) y **disponibilidad** (siempre responde). No puede garantizar las dos a la vez. Un banco prefiere consistencia; una red social, disponibilidad con consistencia eventual.

## Las doce reglas de una aplicación para la nube

Algunas de las más importantes de la metodología *twelve-factor*:

| Factor | Regla |
|---|---|
| Configuración | en variables de entorno |
| Dependencias | declaradas y aisladas |
| Procesos | sin estado |
| Registros | a la salida estándar, como flujo |
| Paridad | desarrollo y producción lo más parecidos posible |
| Desechabilidad | arranque rápido y apagado ordenado |

## Contenedores

Un **contenedor** empaqueta la aplicación con todo lo que necesita, y corre igual en cualquier máquina.

```dockerfile
FROM rust:1 AS compilacion
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=compilacion /app/target/release/bigbrain-api /usr/local/bin/
USER nobody
CMD ["bigbrain-api"]
```

La construcción en dos etapas deja una imagen final chica, sin el compilador. **Kubernetes** orquesta contenedores en muchas máquinas: los reinicia, los escala y reparte el tráfico.

## Observabilidad

| Pilar | Responde |
|---|---|
| Registros | ¿qué pasó? |
| Métricas | ¿cuánto y con qué frecuencia? |
| Trazas | ¿por dónde pasó este pedido y dónde se demoró? |

**OpenTelemetry** es el estándar para generarlos. Las métricas básicas de un servicio: tasa de pedidos, tasa de errores y duración.

## Documentar decisiones

Un **registro de decisión de arquitectura** (ADR) es una nota corta: contexto, decisión, alternativas y consecuencias. Dentro de un año explica por qué el sistema es como es.

## Buenas prácticas

- La solución más simple que cumpla los requisitos reales.
- Límites claros entre módulos, con dependencias en una sola dirección.
- Medir antes de optimizar.
- Automatizar el despliegue y poder volver atrás.
- Diseñar pensando en que las partes van a fallar.
