# Knightly — Rendszerterv (végleges architektúra)

Ez a dokumentum a *Knightly* sakkprojekt végleges rendszertervét írja le. A terv a korábban megosztott draw.io diagramon alapul, és fejlesztésre kész, technikai + működési leírást egyaránt tartalmaz.

---

## Tartalom
1. Összefoglaló célok
2. Fő komponensek és szerepek
3. Kommunikációs modell (WebSocket)
4. Üzenetsémák (JSON) — szabványosított formátumok
5. Matchmaking és meccskezelés
6. Játékfolyamat (lifecycle)
7. Engine integráció és validáció
8. UI–Server–Engine adatáramlás
9. Hálózat, üzemeltetés és deployment
10. Biztonság és jogosultságok
11. Naplózás, hibakezelés és monitoring
12. Tesztelés és CI/CD integráció
13. Üzemeltetési kézikönyv (runbook)
14. Fejlesztési roadmap és ajánlott következő lépések
15. Mellékletek — fontos konfigurációk, environment változók

---

## 1. Összefoglaló célok
- Kisméretű, LAN-on vagy interneten keresztül futtatható sakk-szerver tervezése.
- Stabil WebSocket alapú kommunikáció a UI (Rust) és a Server (Rust) között.
- Egyszerű, megbízható matchmaking (sorbaállás) és 1v1 meccskezelés.
- A szerver felel a meccsek létrehozásáért, a játék állapotáért és az engine-nek továbbított validálásért.
- A rendszer skálázható alapokra épül, később központi (cloud) hosztolásra bővíthető.

---

## 2. Fő komponensek és szerepek

### 2.1 UI (Client)
- Nyelv: Rust (egységes platform). Desktop UI (pl. egui/Tauri/SDL stb.).
- Feladatai: felhasználói interakció, önálló render, locale input, kliens-oldali lépésellenőrzés előzetes vizsgálata, WebSocket kapcsolat kezelése.
- Kommunikáció: WebSocket (ws:// vagy wss://) a szerverrel.

### 2.2 Server
- Nyelv: Rust, aszinkron (`tokio`), WebSocket támogatás (`tokio-tungstenite` vagy `axum`+`tokio-tungstenite`).
- Feladatai: kapcsolatok kezelése, matchmaking, meccsek életciklusa, üzenetek továbbítása, engine-hez való kommunikáció a lépések ellenőrzésére és szerepeltetésére.
- Állapot: memória alapú strukturák (`players`, `waiting_queue`, `matches`), opcionális perzisztencia (logok, ranglista) később.

### 2.3 Engine
- Feladat: sakk-szabályok végrehajtása, lépések validálása, legális lépések listázása, opció: AI játékos.
- Integrációs lehetőségek:
  - könyvtárként (Rust crate) közvetlenül a szerveren belül, vagy
  - külön folyamatként (stdin/stdout) vagy helyi RPC (Unix socket), illetve
  - távoli szolgáltatás (gRPC/HTTP) később.

---

## 3. Kommunikációs modell (WebSocket)
- Egy porton fut a WS szerver (pl. `0.0.0.0:8080`). Nem szükséges több port.
- Minden kliens egyedi WebSocket kapcsolatot nyit. A szerver minden kapcsolatot azonosít (UUID vagy generált client id).
- A szerver tartja a `tx` (küldő) csatornát minden csatlakozott játékoshoz, így onnan tud üzenetet küldeni.
- Az üzenetek JSON formátumban érkeznek és mennek (text frames). Binary nem szükséges a kezdetekkor.

**Kapcsolódási lépések:**
1. UI csatlakozik → `Join` üzenet (felhasználónév).
2. Szerver visszaad `Welcome` (player id) vagy `Error`.
3. Keresés vagy hosting esetén a UI küld `FindMatch` vagy `HostLocal` parancsot.
4. Szerver párosít és `MatchFound` üzenetet küld mindkét félnek, tartalmazza az ellenfél metaadatait és ki kezd.

---

## 4. Üzenetsémák (JSON)
Az alábbiak a tervezett, szabványosított JSON üzenettípusok. Nem pszeudokód — egyszerű, pontos séma.

### 4.1 **Client -> Server**
- `Join`
```json
{ "type": "join", "username": "Alice" }
```
- `FindMatch` (sorbaállás)
```json
{ "type": "find_match", "mode": "1v1" }
```
- `HostLocal` (ha a kliens hostolni akarja a lokális meccset)
```json
{ "type": "host_local", "port": 9001 }
```
- `Move` (lépés beküldése)
```json
{ "type": "move", "from": "e2", "to": "e4", "promotion": null }
```
- `RequestLegalMoves`
```json
{ "type": "legal_moves", "fen": "..." }
```
- `Resign` / `OfferDraw` / `Chat` — hasonló egyszerű objektumok.

### 4.2 **Server -> Client**
- `Welcome`
```json
{ "type": "welcome", "player_id": "<uuid>" }
```
- `MatchFound`
```json
{ "type": "match_found", "match_id": "<uuid>", "opponent": {"id":"...","name":"Bob"}, "color": "white" }
```
- `OpponentMove`
```json
{ "type": "opponent_move", "from": "e2", "to": "e4", "promotion": null }
```
- `MoveResult` (valid/invalid, updated FEN, clocks)
```json
{ "type": "move_result", "valid": true, "fen": "...", "turn": "black" }
```
- `LegalMovesResponse`
```json
{ "type": "legal_moves", "moves": ["e2e4","d2d4"] }
```
- `Error` / `Info` / `GameEnd`

---

## 5. Matchmaking és meccskezelés
- **Várólista (FIFO):** a `waiting_queue` (`VecDeque<Uuid>`) tartja a *FindMatch*-előket.
- **Automatikus párosítás:** ha legalább két játékos van a várólistában, a szerver párba állítja őket és létrehoz egy `Match` struct-ot.
- **Match struct tartalma:** `match_id`, `white_id`, `black_id`, `fen` (kezdőállás), `move_history`, `clocks`.
- **Szereposztás:** véletlenszerű (érme dobás szerű) döntés vagy rang alapján.
- **Állapotkezelés:** a szerver az egyetlen egységes állapotgazda; minden új lépés a szerveren kerül ellenőrzésre és csak ha valid, akkor broadcastolásra.

**Különleges esetek:**
- Ha egyik fél kilép a meccs közben → szerver értesíti az ellenfelet, meccs státusza `aborted` vagy `win_by_disconnect`.
- Reconnect: ha a kliens újracsatlakozik, a szerver match-id alapján visszaállíthatja a játékot (ha ezt implementáljuk).

---

## 6. Játékfolyamat (lifecycle)
1. **Csatlakozás:** client küld `join`, szerver visszaad `welcome`.
2. **Keresés vagy hostolás:** client küld `find_match` vagy `host_local`.
3. **Párosítás:** szerver párosít, küld `match_found` mindkét félnek.
4. **Kezdés:** szerver küld kezdő FEN-t és azt, ki a fehér.
5. **Lépés küldése:** player A küld `move` üzenetet.
6. **Validálás:** szerver lekéri a félellenőrzést az engine-től (vagy saját szabályellenőrzés), ha valid → update `fen`, append `move_history`, küld `move_result` és `opponent_move` a másik félnek.
7. **Végállapot:** ha matt/döntetlen/timeout → szerver `game_end` és törli match-t, kerül a statisztikákba.
8. **Utak a menübe:** a játékosokat visszairányítjuk a főmenübe, match eltávolítva.

---

## 7. Engine integráció és validáció
- **Ajánlott integráció:** a chess engine legyen Rust crate, amelyet a szerver hív pontos függvényekkel — így egyszerű és gyors. Ha nem lehetséges, használjunk helyi folyamatot (`stdin/stdout`), vagy IPC (Unix domain socket).
- **Funkciók az engine-ben:** validálás (`is_move_legal`), lépések generálása (`generate_moves`), FEN kezelése, játék vége ellenőrzése.
- **SLA/Timeout:** minden engine kéréshez legyen timeout (pl. 2s). Ha az engine nem válaszol → szerver fallback: elutasítja a lépést vagy elfogadja lokálisan, attól függően.

---

## 8. UI–Server–Engine adatáramlás
- **Kliens → Szerver:** JSON üzenetek (lásd 4. rész).
- **Szerver → Engine:** függvényhívás vagy IPC hívás a board állás kapcsán.
- **Engine → Szerver:** válasz: legális/illegális, lehetséges lépések, új FEN.
- **Szerver → Kliens(ek):** a véglegesített, validált lépést, valamint állapot- és hibainformációkat.

---

## 9. Hálózat, üzemeltetés és deployment
- **Port:** alap WebSocket port (például `9001` vagy `8080`).
- **Bind cím:** `0.0.0.0` (LAN- és DDNS-elérést is lehetővé téve), dev környezetben `127.0.0.1` is használható.
- **Domain/DDNS:** a szerver elérhetővá tehető DDNS-en keresztül (pl. `mychess.ddns.net:9001`).
- **TLS:** ha publikus elérés szükséges, használj `wss://` TLS-t — pl. reverse proxy (nginx) terminálja a TLS-t és belsőleg csatlakozik `ws://`-on.
- **Self-hosted runner & CI:** a fejlesztői runner lehet persistens; minden workflow végén takarítás javasolt. Artefaktok feltöltése (logok) az Action-ökben.

---

## 10. Biztonság és jogosultságok
- **Input validáció:** a szerver soha ne bízzon a kliensben — minden lépést validálni kell az engine-nel.
- **Rate limiting:** egyszerű védelem (per IP/játékos) botok ellen.
- **TLS:** javasolt publikus hosztingnál.
- **Auth:** kezdetben opcionális, később token/username+password vagy OAuth implementálása.

---

## 11. Naplózás, hibakezelés és monitoring
- **Naplózási szint:** `info` alap, `debug` fejlesztéshez.
- **Naplófájlok:** per-meccs és per-játékos események — eseményeket timeframe-ekkel rögzíteni.
- **Error handling:** minden külső hívás (engine, disk, network) timeout-tal és visszapattanó logikával kezelve legyen.
- **Monitoring:** egyszerű health endpoint (HTTP) és process monitoring (systemd, Prometheus exporter később).

---

## 12. Tesztelés és CI/CD integráció
- **Unit tesztek:** rule engine, move validation, matchmaker logic.
- **Integration tesztek:** szerver–kliens kommunikáció; mock engine használata.
- **CI:** GitHub Actions — teszt workflow-ok per-projekt (Engine/Server/UI). Rendszertervben előírt log exportálás a Google Sheets-be lehetőség.

---

## 13. Üzemeltetési kézikönyv (runbook)
- **Indítás:** `cargo run --release` vagy systemd service. Ajánlott: service fájl, ami a runner felhasználó alatt fut.
- **Stop:** graceful shutdown jelzés a futó meccsek befejezésére (vagy mentés után stop).
- **Frissítés:** stop → pull → `cargo build --release` → restart.
- **Hiba esetén:** ellenőrizd a naplókat, engine elérhetőségét, hálózati port forwardokat.

---

## 14. Fejlesztési roadmap és ajánlott következő lépések
1. Alap WebSocket server implementáció és egyszerű client teszt (lokálisan).  
2. Shared message schema (JSON) és basic `Join/FindMatch/Move` támogatás.  
3. Matchmaker + in-memory match tárolás.  
4. Engine integráció (egy egyszerű lib vagy local process).  
5. Reconnect és állapot-szinkronizáció (sync_state).  
6. TLS és reverse proxy beállítása (ha publikus).  
7. Perzisztencia (match history, eredmények).  
8. Skálázás (ha szükséges: több match host, load balancing, stateless match managers).

---

## 15. Mellékletek — fontos konfigurációk
- **Environment változók**
  - `SERVER_BIND=0.0.0.0:9001`
  - `LOG_LEVEL=info`
  - `ENGINE_PATH=/opt/knightly/engine` (ha külön process)
- **Ajánlott crate-ek**: `tokio`, `tokio-tungstenite`, `serde`, `serde_json`, `uuid`, `tracing`.

---

## Záró megjegyzés
Ez a dokumentum a diagram és a megbeszélések alapján készült. A terv elég részletes ahhoz, hogy a szerver fejlesztését megkezdjétek: tartalmazza az üzenetsémákat, a matchmaking logikát, az engine integrációs lehetőségeket és az üzemeltetési követelményeket.

Ha szeretnéd, legközelebb tudok készíteni belőle egy rövidebb fejlesztői checklistát (sprint backlog), vagy generálhatok egy külön implementációs tervet (fájlok/funkciók szerinti bontás).
