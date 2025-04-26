import { Identity } from '@clockworklabs/spacetimedb-sdk';
import { BoardData, Cell, DbConnection, ErrorContext, EventContext } from './module_bindings';

import { Board } from './Board';
import { BoardCanvas } from './BoardCanvas';
import { onKey, isRotateLeft, isRotateRight, isMoveLeft, isMoveRight, isMoveDown, isDrop } from './Keyboard';

////

const b = new Board();
const bc = new BoardCanvas(b);

const onConnect = (
    _conn: DbConnection,
    identity: Identity,
    token: string
) => {
    console.log('Our identity:', identity.toHexString());
    localStorage.setItem('auth_token', token);
    console.log(
        'Connected to SpacetimeDB with identity:',
        identity.toHexString()
    );
    
    conn.subscriptionBuilder().subscribe('SELECT * FROM cell');
    conn.subscriptionBuilder().subscribe('SELECT * FROM board_data');

    /*
    conn.reducers.onWhoAmI(() => {
        console.log('Message sent.');
    });
    conn.reducers.whoAmI();
    */
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
    //console.log('New bd:', bd);
    b.lines = bd.lines;
    b.score = bd.score;
    b.nextPiece = bd.nextPiece;
});
conn.db.boardData.onUpdate((_ctx: EventContext, _bd: BoardData, bd: BoardData) => {
    //console.log('Bd updated to:', bd);
    b.lines = bd.lines;
    b.score = bd.score;
    b.nextPiece = bd.nextPiece;
});
/*conn.db.boardData.onDelete((_ctx: EventContext, bd: BoardData) => {
    //console.log('Bd deleted:', bd);
    b.lines = bd.lines;
    b.score = bd.score;
    b.nextPiece = bd.nextPiece;
});*/

conn.db.cell.onInsert((_ctx: EventContext, c: Cell) => {
    //console.log('New cell:', c);
    b.setCell(c.x, c.y, c.value);
    bc.draw(); // TODO
});
conn.db.cell.onUpdate((_ctx: EventContext, _c: Cell, c: Cell) => {
    //console.log('Cell updated to:', c);
    b.setCell(c.x, c.y, c.value);
    bc.draw(); // TODO
});
/*conn.db.cell.onDelete((_ctx: EventContext, c: Cell) => {
    //console.log('Cell deleted:', c);
});*/

onKey((key: string, isDown: boolean) => {
    if (!isDown) return;

    if (isRotateLeft(key)) {
        console.log('rotate left');
    } else if (isRotateRight(key)) {
        console.log('rotate right');
    } else if (isMoveLeft(key)) {
        console.log('move left');
    } else if (isMoveRight(key)) {
        console.log('move right');
    } else if (isMoveDown(key)) {
        console.log('move down');
    } else if (isDrop(key)) {
        console.log('drop');
    }
    //console.log(`key down: ${key}`);
});

// @ts-ignore
window.conn = conn;
// @ts-ignore
window.b = b;
//conn.reducers.whoAmI();
