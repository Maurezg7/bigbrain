+++
title = "Contenedores y computación en la nube"
summary = "Virtualización, contenedores y Docker, imágenes y Dockerfile, orquestación con Kubernetes, modelos IaaS, PaaS y SaaS, infraestructura como código y costos."
branch = "Sistemas distribuidos y nube"
order = 2
level = "Intermedio"
tags = ["Docker", "contenedores", "Kubernetes", "nube", "IaaS", "infraestructura como código"]
+++

## El problema

«En mi máquina funciona.» Un programa depende de versiones concretas del sistema, de bibliotecas y de configuración. Los **contenedores** empaquetan la aplicación con todo lo que necesita, de modo que se ejecute igual en cualquier lugar.

## Máquinas virtuales y contenedores

| | Máquina virtual | Contenedor |
|---|---|---|
| Qué virtualiza | el hardware | el sistema operativo |
| Incluye | un sistema operativo completo | la aplicación y sus dependencias |
| Núcleo | propio | **comparte** el del anfitrión |
| Tamaño | gigabytes | megabytes |
| Arranque | decenas de segundos | menos de un segundo |
| Aislamiento | más fuerte | más liviano |

Un **hipervisor** (KVM, VMware, Hyper-V) ejecuta las máquinas virtuales.

Un contenedor es un **proceso común** de Linux, aislado con dos mecanismos del núcleo:

| Mecanismo | Función |
|---|---|
| Espacios de nombres (*namespaces*) | limitan lo que el proceso **ve**: procesos, red, archivos, usuarios |
| Grupos de control (*cgroups*) | limitan lo que **consume**: CPU, memoria, entrada y salida |

## Docker

| Concepto | Qué es |
|---|---|
| **Imagen** | plantilla inmutable con el sistema de archivos y la configuración |
| **Contenedor** | una instancia en ejecución de una imagen |
| **Dockerfile** | la receta para construir una imagen |
| **Registro** | almacén de imágenes (Docker Hub, GHCR) |
| **Volumen** | almacenamiento que persiste fuera del contenedor |

### Dockerfile

```dockerfile
FROM python:3.12-slim

WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY . .

USER nobody
EXPOSE 8000
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
```

Cada instrucción crea una **capa**, que se guarda en caché. Por eso se copian primero las dependencias: si solo cambia el código, no se reinstalan.

### Construcción en etapas

Separa el entorno de compilación de la imagen final:

```dockerfile
FROM rust:1.83 AS compilacion
WORKDIR /src
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=compilacion /src/target/release/api /usr/local/bin/api
USER nobody
CMD ["api"]
```

La imagen final contiene solo el binario, no el compilador.

### Comandos

```bash
docker build -t miapp:1.0 .                 # construye la imagen
docker run -d -p 8080:8000 --name web miapp:1.0
docker ps                                   # contenedores en ejecución
docker logs -f web                          # ver la salida
docker exec -it web sh                      # abrir una terminal dentro
docker stop web && docker rm web
docker images                               # imágenes locales
docker push usuario/miapp:1.0               # subir al registro
```

| Opción de `run` | Efecto |
|---|---|
| `-d` | en segundo plano |
| `-p 8080:8000` | puerto del anfitrión : puerto del contenedor |
| `-e CLAVE=valor` | variable de entorno |
| `-v datos:/var/lib/datos` | monta un volumen |
| `--rm` | elimina el contenedor al terminar |

> [!IMPORTANT]
> El sistema de archivos de un contenedor es **efímero**: se pierde al eliminarlo. Los datos que deben persistir van en un **volumen**.

### Docker Compose

Define una aplicación de varios contenedores:

```yaml
services:
  api:
    build: .
    ports:
      - "8080:8000"
    environment:
      DATABASE_URL: postgres://app:secreto@db:5432/app
    depends_on:
      - db

  db:
    image: postgres:17
    environment:
      POSTGRES_USER: app
      POSTGRES_PASSWORD: secreto
      POSTGRES_DB: app
    volumes:
      - datos:/var/lib/postgresql/data

volumes:
  datos:
```

`docker compose up -d` levanta todo. Los servicios se encuentran por su nombre (`db`).

### Buenas prácticas

| Práctica | Motivo |
|---|---|
| Imágenes base pequeñas (`slim`, `alpine`, *distroless*) | menos tamaño y menos vulnerabilidades |
| Fijar las versiones (`python:3.12`, no `latest`) | reproducibilidad |
| Un proceso por contenedor | simplicidad |
| No ejecutar como `root` | seguridad |
| Usar `.dockerignore` | no copiar `.git`, dependencias locales ni secretos |
| No guardar secretos en la imagen | las capas son públicas para quien la descargue |
| Registrar en la salida estándar | la plataforma recolecta los registros |

## Kubernetes

Con muchos contenedores en muchas máquinas hace falta un **orquestador**: decide dónde corre cada uno, los reinicia si fallan, los escala y los conecta.

Kubernetes es **declarativo**: se describe el estado deseado, y el sistema trabaja continuamente para alcanzarlo.

| Objeto | Función |
|---|---|
| **Pod** | la unidad mínima: uno o más contenedores que comparten red |
| **Deployment** | mantiene $n$ réplicas de un pod y gestiona las actualizaciones |
| **Service** | dirección estable y balanceo de carga hacia un grupo de pods |
| **Ingress** | entrada HTTP desde el exterior |
| **ConfigMap**, **Secret** | configuración y datos sensibles |
| **Namespace** | separación lógica dentro del clúster |
| PersistentVolume | almacenamiento persistente |

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api
spec:
  replicas: 3
  selector:
    matchLabels:
      app: api
  template:
    metadata:
      labels:
        app: api
    spec:
      containers:
        - name: api
          image: usuario/miapp:1.0
          ports:
            - containerPort: 8000
          resources:
            requests: { cpu: 100m, memory: 128Mi }
            limits: { memory: 256Mi }
          readinessProbe:
            httpGet: { path: /salud, port: 8000 }
```

```bash
kubectl apply -f deployment.yaml
kubectl get pods
kubectl logs -f deploy/api
kubectl scale deploy/api --replicas=5
kubectl rollout undo deploy/api          # volver a la versión anterior
```

| Capacidad | Descripción |
|---|---|
| Autorreparación | reemplaza los pods que fallan |
| Actualización gradual | cambia de versión de a poco, sin interrupción |
| Escalado automático | agrega réplicas según la carga |
| Descubrimiento de servicios | cada servicio tiene un nombre DNS interno |

> [!WARNING]
> Kubernetes es potente y complejo. Para proyectos pequeños suele sobrar: una plataforma gestionada o un servidor con Docker Compose alcanzan.

## Computación en la nube

Alquilar recursos de cómputo **bajo demanda**, pagando por uso, en lugar de comprar y mantener servidores.

### Modelos de servicio

| Modelo | El proveedor gestiona | El cliente gestiona | Ejemplos |
|---|---|---|---|
| **IaaS** | hardware, red, virtualización | sistema operativo, aplicación, datos | EC2, Compute Engine |
| **PaaS** | además, el sistema y el entorno de ejecución | la aplicación y los datos | Heroku, Render, App Engine |
| **SaaS** | todo | solo el uso | Gmail, Slack |
| Funciones (FaaS, *serverless*) | todo, y escala a cero | funciones sueltas | Lambda, Cloud Functions |

Cuanto más gestiona el proveedor, menos trabajo operativo y menos control.

### Servicios habituales

| Categoría | Qué ofrece | Ejemplos |
|---|---|---|
| Cómputo | máquinas virtuales, contenedores, funciones | EC2, Cloud Run, Lambda |
| Almacenamiento de objetos | archivos de cualquier tamaño, muy duradero | S3, Cloud Storage |
| Bases de datos gestionadas | con copias de seguridad y réplicas | RDS, Cloud SQL, DynamoDB |
| Redes | redes privadas, balanceadores, CDN, DNS | VPC, CloudFront |
| Mensajería | colas y eventos | SQS, Pub/Sub |
| Identidad | usuarios, roles y permisos | IAM |

Los proveedores dividen su infraestructura en **regiones** (zonas geográficas) y, dentro de cada una, en **zonas de disponibilidad** (centros de datos independientes). Repartir un sistema entre zonas lo protege de la caída de un centro de datos.

### Responsabilidad compartida

El proveedor asegura la infraestructura; el cliente, lo que pone encima: configuración, accesos, datos, parches de sus sistemas.

> [!WARNING]
> La mayoría de las filtraciones en la nube se deben a **errores de configuración** del cliente: almacenamientos públicos por descuido, claves subidas a un repositorio, permisos excesivos.

## Infraestructura como código

Describir la infraestructura en **archivos versionados**, en lugar de crearla a mano en una consola web.

```hcl
resource "aws_s3_bucket" "apuntes" {
  bucket = "bigbrain-apuntes"
}

resource "aws_instance" "servidor" {
  ami           = "ami-0abcdef1234567890"
  instance_type = "t3.micro"

  tags = {
    Name = "bigbrain-api"
  }
}
```

| Ventaja | |
|---|---|
| Reproducible | el mismo entorno, las veces que haga falta |
| Revisable | los cambios pasan por revisión, como el código |
| Reversible | el historial permite volver atrás |
| Documentada | el archivo es la descripción exacta de lo que existe |

Herramientas: Terraform y OpenTofu, Pulumi, CloudFormation; Ansible para configurar servidores.

En **GitOps**, el repositorio es la fuente de verdad: un agente compara continuamente el estado real con el declarado y lo corrige.

## Estrategias de despliegue

| Estrategia | Funcionamiento | Riesgo |
|---|---|---|
| Gradual (*rolling*) | reemplaza las instancias de a poco | dos versiones conviven por un tiempo |
| Azul-verde | dos entornos completos; se cambia el tráfico de uno a otro | costo doble; vuelta atrás inmediata |
| Canario | la versión nueva recibe primero una fracción pequeña del tráfico | el más seguro; requiere buenas métricas |
| Banderas de funcionalidad | el código nuevo se despliega apagado y se activa después | separa desplegar de lanzar |

## Costos

La nube convierte la inversión inicial en gasto mensual, pero sin control puede resultar cara.

| Modalidad | Característica |
|---|---|
| Bajo demanda | sin compromiso; el precio más alto |
| Reservada o con compromiso | descuento a cambio de uno a tres años |
| Interrumpible (*spot*) | muy barata; el proveedor puede retirarla en cualquier momento |

| Fuente de sorpresas | |
|---|---|
| Tráfico de salida | sacar datos de la nube se cobra |
| Recursos olvidados | máquinas, discos y direcciones IP sin uso |
| Registros y métricas | crecen sin límite si no se configura su retención |

> [!TIP]
> Configurar **alertas de presupuesto** desde el primer día y etiquetar los recursos por proyecto. Para aprender, las capas gratuitas alcanzan; conviene borrar todo al terminar cada práctica.

## Los doce factores

Una guía clásica para aplicaciones pensadas para la nube. Sus ideas principales:

| Factor | Idea |
|---|---|
| Configuración | en variables de entorno, no en el código |
| Dependencias | declaradas explícitamente |
| Servicios de respaldo | la base de datos y la caché son recursos conectables |
| Procesos | sin estado; el estado va en servicios externos |
| Desechabilidad | arranque rápido y cierre ordenado |
| Paridad | desarrollo, pruebas y producción lo más parecidos posible |
| Registros | como flujo de eventos hacia la salida estándar |
