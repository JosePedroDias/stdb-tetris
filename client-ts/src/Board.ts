export const WIDTH = 10;
export const HEIGHT = 20;

export class Board {
    cells: number[][];

    selectedPiece: number = 0; // 1 - 6
    selectedPieceVariant: number = 0; // 0 - 3

    nextPiece: number = 0; // 1 - 6
    nextPieceVariant: number = 0; // 0 - 3

    ghostX: number = 0;
    ghostY: number = 0;

    score: number = 0;
    lines: number = 0;

    // ghostX
    // ghostY
    
    constructor() {
        this.cells = Array.from({ length: HEIGHT }, () => Array(WIDTH).fill(0));
    }

    setCell(x: number, y: number, value: number): void {
        //if (!this.isValidCell(x, y)) return;
        this.cells[y][x] = value;
    }

    getCell(x: number, y: number): number {
        return this.cells[y][x];
    }
    
    isValidCell(x: number, y: number): boolean {
        return x >= 0 && x < WIDTH && y >= 0 && y < HEIGHT;
    }

    toString(): string {
        return this.cells
            .map((row) => row.map((cell) => (cell ? 'X' : '.')).join(''))
            .join('\n');
    }
}
