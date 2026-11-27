+++
title = "TypeScript en el servidor: Node.js, Express, Fastify y NestJS"
summary = "El bucle de eventos, runtimes, rutas y middleware, la arquitectura modular de NestJS y los ORM Prisma y Drizzle."
branch = "Frameworks de TypeScript"
order = 6
level = "Intermedio"
tags = ["TypeScript", "Node.js", "Express", "NestJS", "Prisma"]
+++

## Runtimes

| Runtime | Característica |
|---|---|
| **Node.js** | el estándar; el ecosistema más grande (npm) |
| **Bun** | muy rápido; incluye gestor de paquetes, empaquetador y pruebas; ejecuta TypeScript directamente |
| **Deno** | seguro por defecto, con permisos explícitos y TypeScript nativo |

## El bucle de eventos

JavaScript ejecuta el código en **un solo hilo**. La entrada y salida (red, disco) se delega al sistema, y cuando termina, su continuación se encola. El **bucle de eventos** toma tareas de la cola una por una.

```ts
console.log('1');
setTimeout(() => console.log('3'), 0);
Promise.resolve().then(() => console.log('2'));
// imprime 1, 2, 3
```

Por eso un servidor Node atiende miles de conexiones con un hilo, siempre que el código **no bloquee**.

> [!WARNING]
> Un cálculo largo y síncrono detiene a **todos** los usuarios. El trabajo pesado va a un *worker thread*, a una cola de tareas o a otro servicio.

## Express

Es el framework minimalista clásico: rutas y **middleware**.

```ts
import express, { type NextFunction, type Request, type Response } from 'express';

const app = express();
app.use(express.json());

app.get('/api/temas/:id', async (req, res) => {
  const tema = await repositorio.buscar(Number(req.params.id));
  if (!tema) return res.status(404).json({ error: 'no encontrado' });
  res.json(tema);
});

app.use((err: Error, _req: Request, res: Response, _next: NextFunction) => {
  console.error(err);
  res.status(500).json({ error: 'error interno' });
});

app.listen(3000);
```

Un middleware es una función `(req, res, next)` que se ejecuta en orden: puede modificar el pedido, responder o llamar a `next()`.

## Fastify y Hono

| Framework | Rasgo |
|---|---|
| **Fastify** | más rápido que Express; validación por esquema y buen soporte de TypeScript |
| **Hono** | muy liviano; corre en Node, Bun, Deno y en el borde de la red |

```ts
import Fastify from 'fastify';

const app = Fastify({ logger: true });

app.post('/api/temas', {
  schema: {
    body: {
      type: 'object',
      required: ['titulo'],
      properties: { titulo: { type: 'string', maxLength: 120 } },
    },
  },
}, async (req, reply) => {
  reply.code(201);
  return crear(req.body);
});
```

## NestJS

Es un framework **con estructura**, inspirado en Angular y parecido a Spring: módulos, controladores, servicios e inyección de dependencias con decoradores.

```ts
@Injectable()
export class TemasService {
  constructor(private readonly prisma: PrismaService) {}

  async ver(id: number) {
    const tema = await this.prisma.tema.findUnique({ where: { id } });
    if (!tema) throw new NotFoundException('Tema no encontrado');
    return tema;
  }
}

@Controller('temas')
export class TemasController {
  constructor(private readonly temas: TemasService) {}

  @Get(':id')
  ver(@Param('id', ParseIntPipe) id: number) {
    return this.temas.ver(id);
  }

  @Post()
  @UseGuards(AuthGuard)
  crear(@Body() nuevo: CrearTemaDto) {
    return this.temas.crear(nuevo);
  }
}

@Module({ controllers: [TemasController], providers: [TemasService, PrismaService] })
export class TemasModule {}
```

| Pieza | Función |
|---|---|
| Módulo | agrupa una funcionalidad |
| Controlador | recibe los pedidos |
| Proveedor (servicio) | lógica de negocio |
| *Pipe* | transforma y valida la entrada |
| *Guard* | decide si el pedido puede pasar |
| Interceptor | actúa antes y después del handler |
| Filtro de excepciones | convierte errores en respuestas |

Validación con DTO:

```ts
export class CrearTemaDto {
  @IsString() @MaxLength(120)
  titulo!: string;

  @IsIn(['Básico', 'Intermedio', 'Avanzado'])
  nivel!: string;
}

app.useGlobalPipes(new ValidationPipe({ whitelist: true }));
```

`whitelist` descarta las propiedades que el DTO no declara.

## Bases de datos

| Herramienta | Enfoque |
|---|---|
| **Prisma** | esquema propio; genera un cliente totalmente tipado |
| **Drizzle** | esquema en TypeScript y consultas cercanas a SQL |
| **TypeORM** | ORM clásico con decoradores |

```prisma
model Tema {
  id     Int    @id @default(autoincrement())
  titulo String
  area   Area   @relation(fields: [areaId], references: [id])
  areaId Int
}
```

```ts
const temas = await prisma.tema.findMany({
  where: { area: { slug: 'fisica' } },
  include: { area: true },
  orderBy: { titulo: 'asc' },
  take: 20,
});
```

## Cuál elegir

| Necesidad | Opción |
|---|---|
| API chica o aprendizaje | Express u Hono |
| Rendimiento y validación por esquema | Fastify |
| Aplicación grande con varios equipos | NestJS |
| Web con interfaz incluida | Next.js, Nuxt o SvelteKit |

## Buenas prácticas

- Validar toda entrada con un esquema (Zod, JSON Schema, `class-validator`).
- Un manejador central de errores; nunca devolver la traza al cliente.
- `async` y `await` con errores capturados: una promesa rechazada sin manejar puede tumbar el proceso.
- Configuración por variables de entorno, validada al arrancar.
- Encabezados de seguridad (`helmet`), límite de pedidos y CORS restringido.
- Registro estructurado con `pino`.
- Fijar las versiones con el archivo de bloqueo y revisar las dependencias (`npm audit`).
- Apagado ordenado: dejar de aceptar pedidos y cerrar las conexiones al recibir `SIGTERM`.
