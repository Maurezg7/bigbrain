import hljs from 'highlight.js/lib/core';
import bash from 'highlight.js/lib/languages/bash';
import c from 'highlight.js/lib/languages/c';
import dockerfile from 'highlight.js/lib/languages/dockerfile';
import http from 'highlight.js/lib/languages/http';
import ini from 'highlight.js/lib/languages/ini';
import java from 'highlight.js/lib/languages/java';
import javascript from 'highlight.js/lib/languages/javascript';
import json from 'highlight.js/lib/languages/json';
import kotlin from 'highlight.js/lib/languages/kotlin';
import python from 'highlight.js/lib/languages/python';
import rust from 'highlight.js/lib/languages/rust';
import sql from 'highlight.js/lib/languages/sql';
import typescript from 'highlight.js/lib/languages/typescript';
import xml from 'highlight.js/lib/languages/xml';
import yaml from 'highlight.js/lib/languages/yaml';

const languages = { bash, c, dockerfile, http, ini, java, javascript, json, kotlin, python, rust, sql, typescript, xml, yaml };
for (const [name, language] of Object.entries(languages)) hljs.registerLanguage(name, language);
hljs.registerAliases(['toml'], { languageName: 'ini' });
hljs.registerAliases(['tsx'], { languageName: 'typescript' });
hljs.registerAliases(['svelte', 'vue', 'html'], { languageName: 'xml' });

const LABELS: Record<string, string> = {
	ts: 'TypeScript',
	tsx: 'TSX',
	js: 'JavaScript',
	javascript: 'JavaScript',
	typescript: 'TypeScript',
	c: 'C',
	rust: 'Rust',
	java: 'Java',
	kotlin: 'Kotlin',
	python: 'Python',
	sql: 'SQL',
	bash: 'Terminal',
	toml: 'TOML',
	yaml: 'YAML',
	json: 'JSON',
	http: 'HTTP',
	html: 'HTML',
	xml: 'XML',
	svelte: 'Svelte',
	vue: 'Vue',
	dockerfile: 'Dockerfile',
	prisma: 'Prisma',
	gcode: 'Código G',
	iecst: 'Texto estructurado',
	text: 'Texto'
};

/** Resalta cada bloque de código y le agrega la etiqueta del lenguaje y el botón de copiar. */
export function enhanceCode(root: HTMLElement) {
	for (const code of root.querySelectorAll<HTMLElement>('pre > code')) {
		const pre = code.parentElement as HTMLElement;
		const lang = [...code.classList].find((c) => c.startsWith('language-'))?.slice(9) ?? '';
		if (hljs.getLanguage(lang)) hljs.highlightElement(code);

		const bar = document.createElement('div');
		bar.className = 'code-bar';
		const label = document.createElement('span');
		label.textContent = LABELS[lang] ?? lang ?? '';
		const button = document.createElement('button');
		button.type = 'button';
		button.textContent = 'Copiar';
		button.addEventListener('click', async () => {
			try {
				await navigator.clipboard.writeText(code.textContent ?? '');
				button.textContent = 'Copiado';
			} catch {
				button.textContent = 'No se pudo copiar';
			}
			setTimeout(() => (button.textContent = 'Copiar'), 1600);
		});
		bar.append(label, button);

		const box = document.createElement('div');
		box.className = 'code-box';
		pre.replaceWith(box);
		box.append(bar, pre);
	}
}
