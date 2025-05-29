function greet(name: string | null) {
    // Erro: 'name' is possibly 'null'
    console.log("Hello, " + name.toUpperCase());
}

greet("Timbo");
greet(null);
