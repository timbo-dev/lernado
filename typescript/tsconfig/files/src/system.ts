// A dummy TypeScript code for a system module

export class System {
    private systemName: string;
    private version: string;

    constructor(systemName: string, version: string) {
        this.systemName = systemName;
        this.version = version;
    }

    getSystemInfo(): string {
        return `System: ${this.systemName}, Version: ${this.version}`;
    }

    static getDefaultSystem(): System {
        return new System("DefaultSystem", "1.0.0");
    }
}

// Example usage
const mySystem = new System("LernadoSystem", "2.3.1");
console.log(mySystem.getSystemInfo());

const defaultSystem = System.getDefaultSystem();
console.log(defaultSystem.getSystemInfo());