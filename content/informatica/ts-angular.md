+++
title = "Angular"
summary = "Componentes independientes, plantillas, señales, servicios e inyección de dependencias, enrutado, formularios reactivos, HttpClient y RxJS."
branch = "Frameworks de TypeScript"
order = 3
level = "Intermedio"
tags = ["TypeScript", "Angular", "RxJS", "señales", "frontend"]
+++

## Un framework completo

Angular, mantenido por Google, trae **todo incluido**: enrutado, formularios, cliente HTTP, inyección de dependencias, pruebas y una línea de comandos. Impone una estructura, lo que lo hace frecuente en aplicaciones empresariales grandes con muchos equipos.

```bash
ng new bigbrain
ng generate component temas/lista
ng serve
```

## Componentes

Un componente une una clase, una plantilla y estilos.

```ts
import { Component, computed, input, output, signal } from '@angular/core';

@Component({
  selector: 'app-contador',
  template: `
    <button (click)="sumar()">Clics: {{ cuenta() }}</button>
    <p>El doble es {{ doble() }}</p>
  `,
})
export class ContadorComponent {
  inicial = input(0);                 // dato que entra
  cambio = output<number>();          // evento que sale

  cuenta = signal(0);
  doble = computed(() => this.cuenta() * 2);

  sumar() {
    this.cuenta.update((n) => n + 1);
    this.cambio.emit(this.cuenta());
  }
}
```

Los componentes actuales son **independientes** (*standalone*): declaran lo que usan en `imports`, sin necesidad de los antiguos `NgModule`.

## Plantillas

| Sintaxis | Significado |
|---|---|
| `{{ valor }}` | mostrar un valor |
| `[propiedad]="valor"` | pasar un dato hacia el elemento |
| `(evento)="accion()"` | escuchar un evento |
| `[(ngModel)]="campo"` | enlace en ambos sentidos |

Control de flujo:

```html
@if (temas().length > 0) {
  <ul>
    @for (tema of temas(); track tema.id) {
      <li>{{ tema.titulo }}</li>
    }
  </ul>
} @else {
  <p>No hay temas.</p>
}
```

`track` cumple el papel de la `key` de React. En código anterior a Angular 17 se usan `*ngIf` y `*ngFor`.

## Señales

Una **señal** es un valor reactivo: quien la lee queda suscripto y se actualiza cuando cambia.

| Primitiva | Uso |
|---|---|
| `signal(valor)` | estado; se cambia con `set` y `update` |
| `computed(() => ...)` | valor derivado, recalculado solo cuando hace falta |
| `effect(() => ...)` | efecto secundario |

## Servicios e inyección de dependencias

La lógica y el acceso a datos van en **servicios**, que Angular inyecta donde se piden.

```ts
import { HttpClient } from '@angular/common/http';
import { Injectable, inject } from '@angular/core';

@Injectable({ providedIn: 'root' })
export class TemaService {
  private http = inject(HttpClient);

  listar() {
    return this.http.get<Tema[]>('/api/temas');
  }

  ver(slug: string) {
    return this.http.get<Tema>(`/api/temas/${encodeURIComponent(slug)}`);
  }
}
```

`providedIn: 'root'` crea una única instancia para toda la aplicación.

## RxJS

`HttpClient` devuelve **observables**: flujos de valores en el tiempo que se transforman con operadores.

```ts
resultados$ = this.busqueda.valueChanges.pipe(
  debounceTime(300),                 // espera a que el usuario deje de escribir
  distinctUntilChanged(),
  switchMap((texto) => this.temas.buscar(texto)),   // cancela la búsqueda anterior
);
```

| Operador | Hace |
|---|---|
| `map` | transforma cada valor |
| `filter` | deja pasar algunos |
| `switchMap` | cambia a un nuevo flujo y cancela el anterior |
| `debounceTime` | espera una pausa |
| `catchError` | maneja errores |

En la plantilla, `{{ resultados$ | async }}` se suscribe y se desuscribe solo. `toSignal()` convierte un observable en señal.

> [!WARNING]
> Una suscripción manual que no se cancela es una **fuga de memoria**. Usá el `async` pipe, `toSignal` o `takeUntilDestroyed()`.

## Enrutado

```ts
export const routes: Routes = [
  { path: '', component: InicioComponent },
  { path: 'temas/:slug', component: TemaComponent },
  {
    path: 'admin',
    canActivate: [authGuard],
    loadComponent: () => import('./admin/admin.component').then((m) => m.AdminComponent),
  },
  { path: '**', component: NoEncontradoComponent },
];
```

`loadComponent` hace **carga diferida**: ese código se descarga recién al entrar a la ruta. Un **guard** decide si se puede entrar.

## Formularios reactivos

```ts
private fb = inject(FormBuilder);

formulario = this.fb.nonNullable.group({
  titulo: ['', [Validators.required, Validators.maxLength(120)]],
  nivel: ['Básico', Validators.required],
});

guardar() {
  if (this.formulario.invalid) return;
  this.temas.crear(this.formulario.getRawValue()).subscribe();
}
```

El formulario es un objeto tipado que se puede probar sin la página.

## Interceptores

Funciones que actúan sobre todos los pedidos HTTP: agregar el token, registrar, tratar errores.

```ts
export const authInterceptor: HttpInterceptorFn = (req, next) => {
  const token = inject(AuthService).token();
  return next(token ? req.clone({ setHeaders: { Authorization: `Bearer ${token}` } }) : req);
};
```

## Buenas prácticas

- Componentes independientes y señales en código nuevo.
- Detección de cambios `OnPush` para renderizar solo cuando cambian las entradas.
- Componentes de presentación sin lógica; la lógica, en servicios.
- Carga diferida por ruta.
- Seguir la guía de estilo oficial: una cosa por archivo, nombres consistentes.
- No suscribirse dentro de otra suscripción: componer con operadores.
