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
