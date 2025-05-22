// src/utils.ts

/**
 * Utility function to generate a random number between min and max (inclusive).
 * @param min - Minimum value
 * @param max - Maximum value
 * @returns A random number between min and max
 */
export function getRandomNumber(min: number, max: number): number {
    return Math.floor(Math.random() * (max - min + 1)) + min;
}
