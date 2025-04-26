import { DbConnection, ErrorContext } from './module_bindings';
import { Identity } from '@clockworklabs/spacetimedb-sdk';

////

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
    
    //conn.subscriptionBuilder().subscribe('SELECT * FROM card');
    //conn.subscriptionBuilder().subscribe('SELECT * FROM card_face');

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

/*
conn.db.card.onInsert((_ctx: EventContext, card: Card) => {
    console.log('New card:', cardToString(card));
});
conn.db.card.onUpdate((_ctx: EventContext, _card: Card, card: Card) => {
    console.log('Card updated to:', cardToString(card));
});
conn.db.card.onDelete((_ctx: EventContext, card: Card) => {
    console.log('Card deleted:', cardToString(card));
});
*/

// @ts-ignore
window.conn = conn;
// conn.reducers.whoAmI();
