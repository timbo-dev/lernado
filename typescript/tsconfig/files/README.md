# Files

**Tipo:** `string[]`
**Introduzido em:** TypeScript 1.5
**Relacionado:** `include`, `exclude`

Especifica uma lista explícita (allowlist) de arquivos a serem incluídos no programa.

❌ Se qualquer arquivo listado não for encontrado, o compilador emitirá um erro.

### ✅ Exemplo:

```json
{
  "compilerOptions": {},
  "files": [
    "core.ts",
    "sys.ts",
    "types.ts",
    "scanner.ts",
    "parser.ts",
    "utilities.ts",
    "binder.ts",
    "checker.ts",
    "tsc.ts"
  ]
}
```

### 📌 Observações

* Útil quando você quer compilar um número pequeno e bem definido de arquivos.
* Não utiliza glob patterns — para isso, use a propriedade `include`.

### 🔄 Comportamento padrão:

Se `files` não for definido ou caso seja `null`, o TypeScript incluirá todos os arquivos `.ts` e `.tsx` no diretório e subdiretórios, exceto os listados em `exclude`.
