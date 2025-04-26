# Tetris STDB

## setup / cheat sheet

```
spacetime start --listen-addr='0.0.0.0:3000'
clear && spacetime publish --project-path server-rs tetris-game --delete-data -y
spacetime generate --lang typescript --out-dir client-ts/src/module_bindings --project-path server-rs

spacetime logs tetris-game
//spacetime sql tetris-game "SELECT * FROM player"
spacetime call tetris-game who_am_i

cd client-ts

clear && npm run build && npm run preview
npm run dev
```

## TODO
