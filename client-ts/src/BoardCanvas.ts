import { Board, WIDTH, HEIGHT } from "./Board";

import { I, J, L, O, S, T, Z } from "./bricks";

const SCL = 12;

const COLORS = [
    "red",
    "green",
    "blue",
    "yellow",
    "purple",
    "orange",
    "cyan",
];

const BRICKS = [
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
]

export class BoardCanvas {
    canvas: HTMLCanvasElement;
    ctx: CanvasRenderingContext2D;
    board: Board;
    isOurs: boolean = false;

    constructor(board: Board) {
        this.canvas = document.createElement('canvas');
        document.body.appendChild(this.canvas);
        this.ctx = this.canvas.getContext("2d")!;
        this.board = board;
        this.resizeCanvas();
        //window.addEventListener("resize", () => this.resizeCanvas());
    }

    resizeCanvas() {
        //const rect = this.canvas.getBoundingClientRect();
        this.canvas.width = (WIDTH + 5) * SCL;
        this.canvas.height = HEIGHT * SCL;
    }

    drawPiece(dx: number, dy: number, piece: number, variant: number) {
        const ctx = this.ctx;
        const brickCells = BRICKS[piece][variant];
        ctx.fillStyle = COLORS[piece];
        for (let [x, y] of brickCells) {
            ctx.fillRect((x+dx) * SCL, (y+dy) * SCL, SCL, SCL);
        }
    }

    draw() {
        const ctx = this.ctx;
        ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        if (this.isOurs) {
            ctx.fillStyle = "#000000";
        } else {
            ctx.fillStyle = "#333333";
        }
        ctx.fillRect(0, 0, WIDTH * SCL, HEIGHT * SCL);

        // cells from ghost piece
        ctx.globalAlpha = 0.5;
        this.drawPiece(this.board.ghostX, this.board.ghostY, this.board.selectedPiece, this.board.selectedPieceVariant);
        ctx.globalAlpha = 1;

        // cells in the grid
        for (let y = 0; y < HEIGHT; y++) {
            for (let x = 0; x < WIDTH; x++) {
                const cell = this.board.getCell(x, y);
                if (cell) {
                    ctx.fillStyle = COLORS[cell - 1];
                    ctx.fillRect(x * SCL, y * SCL, SCL, SCL);
                }
            }
        }

        // cells from next piece
        this.drawPiece(WIDTH + 2, 4, this.board.nextPiece, this.board.nextPieceVariant);

        ctx.font = "14px Arial";
        ctx.fillStyle = "#ffffff";
        ctx.textAlign = "right";
        {
            const x = (WIDTH + 4)* SCL;
            let y = 9 * SCL;
            ctx.fillText(`score:`, x, y);               y += 20;
            ctx.fillText(`${this.board.score}`, x, y);  y += 40;
            ctx.fillText(`lines:`, x, y);               y += 20;
            ctx.fillText(`${this.board.lines}`, x, y);
        }
    }
}
