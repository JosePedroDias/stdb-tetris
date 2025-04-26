import { Board } from './Board';
import { BoardData, Cell, DbConnection, ErrorContext, EventContext } from './module_bindings';
import { Identity } from '@clockworklabs/spacetimedb-sdk';

////

const b = new Board();

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
});
conn.db.cell.onUpdate((_ctx: EventContext, _c: Cell, c: Cell) => {
    //console.log('Cell updated to:', c);
    b.setCell(c.x, c.y, c.value);
});
/*conn.db.cell.onDelete((_ctx: EventContext, c: Cell) => {
    //console.log('Cell deleted:', c);
});*/

// @ts-ignore
window.conn = conn;
// @ts-ignore
window.b = b;
//conn.reducers.whoAmI();
