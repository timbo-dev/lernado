# 📘 Lernado — TypeScript Workspace Documentation

Este documento detalha a estrutura, configuração e propósito do workspace **TypeScript** dentro do monorepo **Lernado**. Aqui você encontrará informações sobre como navegar, compilar e estender os experimentos de configurações de `tsconfig` e outros projetos TypeScript.

---

## 📁 Estrutura de Diretórios

```plaintext
typescript/
├── package.json
├── pnpm-lock.yaml
├── pnpm-workspace.yaml
├── README.md               # Documentação deste workspace
└── tsconfig                # Diretório principal com experimentos de configurações do tsconfig
    └── nullable-properties # Um dos experimentos
```

### Descrição Geral

* `package.json`: Gerencia dependências e scripts globais deste workspace.
* `pnpm-lock.yaml`: Lockfile gerado pelo PNPM para garantir versões consistentes.
* `pnpm-workspace.yaml`: Configuração de workspaces, incluindo apenas subpastas de `tsconfig/*`.
* `README.md`: Esta documentação de alto nível, com instruções de uso e objetivos.

O workspace TypeScript é pensado para hospedar experimentos de configurações do compilador, práticas de codificação e estudos de APIs do TypeScript.

---

## 🔧 Configuração de Workspaces

```yaml
# pnpm-workspace.yaml
packages:
  - 'tsconfig/*'
```

Isso faz com que cada subdiretório em `tsconfig/` seja tratado como um pacote isolado, permitindo:

* Dependências específicas por experimento.
* Scripts locais que não poluem o escopo global.
* Versionamento independente, quando necessário.

---

## 📚 Próximos Passos e Extensões

* Criar flashcards no Anki baseados nestes exemplos.
* Documentar resultados no Obsidian e gerar diagramas.
