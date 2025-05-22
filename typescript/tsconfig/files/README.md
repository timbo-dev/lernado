# Files

* **Tipo:** `string[]`
* **Introduzido em:** TypeScript 1.5
* **Relacionado:** `include`, `exclude`

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

### Testes para `files` no tsconfig

Este package contém testes para validar o comportamento da propriedade `files` no `tsconfig.json`.

### Como testar

Execute os comandos abaixo na raiz do projeto:

- Para testar um `tsconfig` válido:
    ```sh
    npm run build:valid
    ```
    **Comportamento esperado:** A compilação deve ocorrer sem erros, pois todos os arquivos listados em `files` existem.

- Para testar um `tsconfig` inválido (por exemplo, com arquivos inexistentes em `files`):
    ```sh
     npm run build:invalid
     ```
     **Comportamento esperado:** O compilador deve emitir um erro informando que um ou mais arquivos listados em `files` não foram encontrados. O TypeScript carrega os arquivos especificados até tentar compilar um arquivo inexistente; nesse momento, ele exibe o erro, interrompe o processo e retorna o exit code 2.

---

### 📌 Observações

* Útil quando você quer compilar um número pequeno e bem definido de arquivos.
* Não utiliza glob patterns — para isso, use a propriedade `include`.

### 🔄 Comportamento padrão:

Se `files` não for definido ou caso seja `null`, o TypeScript incluirá todos os arquivos `.ts` e `.tsx` no diretório e subdiretórios, exceto os listados em `exclude`.
