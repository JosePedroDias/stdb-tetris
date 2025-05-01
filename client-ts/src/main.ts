import { Identity } from '@clockworklabs/spacetimedb-sdk';
import { BoardData, Cell, DbConnection, ErrorContext, EventContext } from './module_bindings';

import { Board } from './Board';
import { BoardCanvas } from './BoardCanvas';
import { onKey, isRotateLeft, isRotateRight, isMoveLeft, isMoveRight, isMoveDown, isDrop } from './Keyboard';

////

let ourIdentity: Identity | undefined = undefined;

const boardMap = new Map<number, [Board, BoardCanvas]>();

function getBoard(id: number): [Board, BoardCanvas] {
    if (boardMap.has(id)) {
        return boardMap.get(id)!;
    } else {
        const b = new Board();
        const bc = new BoardCanvas(b);
        boardMap.set(id, [b, bc]);
        return [b, bc];
    }
}

const onConnect = (
    _conn: DbConnection,
    identity: Identity,
    token: string
) => {
    localStorage.setItem('auth_token', token);
    console.log(
        'Connected to SpacetimeDB with identity:',
        identity.toHexString()
    );
    ourIdentity = identity;
    
    conn.subscriptionBuilder().subscribe('SELECT * FROM cell'); // TODO do i need id, or just x and y?
    //conn.subscriptionBuilder().subscribe('SELECT id, owner, lines, score FROM board_data'); // TODO skip pos_x, pos_y
    conn.subscriptionBuilder().subscribe('SELECT * FROM board_data'); // TODO skip pos_x, pos_y
};

const onDisconnect = () => {
    console.log('Disconnected from stdb');
};

const onConnectError = (_ctx: ErrorContext, err: Error) => {
    console.log('Error connecting to stdb:', err);
};

const conn = DbConnection.builder()
    .withUri('ws://localhost:3000')
    .withModuleName('tetris-game')
    .withToken(localStorage.getItem('auth_token') || '')
    .onConnect(onConnect)
    .onDisconnect(onDisconnect)
    .onConnectError(onConnectError)
    .build();

conn.db.boardData.onInsert((_ctx: EventContext, bd: BoardData) => {
    const boardId = bd.id;
    const [b, bc] = getBoard(boardId);

    if (bd.owner.isEqual(ourIdentity as Identity)) {
        bc.isOurs = true;
        bc.canvas.classList.add('ours');
    } else {
        const w = Math.round(bc.canvas.width * 0.5);
        bc.canvas.style.width = `${w}px`;
        bc.canvas.classList.add('opponent');
    }

    //console.log('New bd:', bd);
    b.lines = bd.lines;
    b.score = bd.score;
    b.selectedPiece = bd.selectedPiece;
    b.selectedPieceVariant= bd.selectedPieceVariant;
    b.nextPiece = bd.nextPiece;
    b.nextPieceVariant = bd.nextPieceVariant;
    b.ghostX = bd.posX;
    b.ghostY = bd.ghostY;
    b.dirty = true;
});
conn.db.boardData.onUpdate((_ctx: EventContext, _bd: BoardData, bd: BoardData) => {
    const boardId = bd.id;
    const [b, _] = getBoard(boardId);

    console.log(`Bd updated to: ${JSON.stringify(bd, (a, b) => { return a === 'owner' ? 'X' : b; })}`);
    b.lines = bd.lines;
    b.score = bd.score;
    b.selectedPiece = bd.selectedPiece;
    b.selectedPieceVariant= bd.selectedPieceVariant;
    b.nextPiece = bd.nextPiece;
    b.nextPieceVariant = bd.nextPieceVariant;
    b.ghostX = bd.posX;
    b.ghostY = bd.ghostY;
    b.dirty = true;
});
/*conn.db.boardData.onDelete((_ctx: EventContext, bd: BoardData) => {
    //console.log('Bd deleted:', bd);
    b.lines = bd.lines;
    b.score = bd.score;
    b.nextPiece = bd.nextPiece;
});*/

conn.db.cell.onInsert((_ctx: EventContext, c: Cell) => {
    const [b, _bc] = getBoard( c.boardId);
    //console.log('New cell:', c);
    b.setCell(c.x, c.y, c.value);
    b.dirty = true;
});
conn.db.cell.onUpdate((_ctx: EventContext, _c: Cell, c: Cell) => {
    const [b, _bc] = getBoard( c.boardId);
    //console.log('Cell updated to:', c);
    b.setCell(c.x, c.y, c.value);
    b.dirty = true;
});
/*conn.db.cell.onDelete((_ctx: EventContext, c: Cell) => {
    //console.log('Cell deleted:', c);
});*/

function onTick() {
    requestAnimationFrame(onTick);
    boardMap.forEach(([b, bc]) => {
        if (b.dirty) {
            bc.draw();
            b.dirty = false;
        }
    });
}

onTick();

onKey((key: string, isDown: boolean) => {
    if (!isDown) return;

    if (isRotateLeft(key)) {
        conn.reducers.rotateLeft();
    } else if (isRotateRight(key)) {
        conn.reducers.rotateRight();
    } else if (isMoveLeft(key)) {
        conn.reducers.moveLeft();
    } else if (isMoveRight(key)) {
        conn.reducers.moveRight();
    } else if (isMoveDown(key)) {
        conn.reducers.moveDown();
    } else if (isDrop(key)) {
        conn.reducers.drop();
    }
    //console.log(`key down: ${key}`);
});

// @ts-ignore
//window.conn = conn;
