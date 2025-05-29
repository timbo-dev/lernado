function greet(name: string | null) {
    // Funciona sem erro, mesmo se name for null.
    console.log("Hello, " + name.toUpperCase());
}

greet("Timbo");
greet(null);
