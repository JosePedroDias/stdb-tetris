import { Board, WIDTH, HEIGHT } from "./Board";

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
        this.canvas.width = WIDTH * SCL;
        this.canvas.height = HEIGHT * SCL;
    }

    draw() {
        const ctx = this.ctx;
        ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        if (this.isOurs) {
            //ctx.fillStyle = "#440044";
            //ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
        } else {
            //ctx.globalAlpha = 0.75;
            ctx.fillStyle = "#333333";
            ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);
        }
        for (let y = 0; y < HEIGHT; y++) {
            for (let x = 0; x < WIDTH; x++) {
                const cell = this.board.getCell(x, y);
                if (cell) {
                    ctx.fillStyle = COLORS[cell - 1];
                    ctx.fillRect(x * SCL, y * SCL, SCL, SCL);
                }
            }
        }
        ctx.font = "14px Arial";
        ctx.fillStyle = "rgba(255, 255, 255, 0.75)";
        ctx.fillText(`score:${this.board.score} lines:${this.board.lines}`, 2, 14);
    }
}
