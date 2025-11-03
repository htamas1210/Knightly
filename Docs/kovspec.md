## 4. Rendszer követelmények

A rendszer célja egy kétjátékos sakkalkalmazás megvalósítása, amely alapvetően hálózati kapcsolat (LAN vagy internet) segítségével biztosítja a valós idejű játékot. A rendszer kliens–szerver architektúrán alapul, ahol az egyes komponensek jól elkülönülten, meghatározott feladatokat látnak el.

### 4.1 Kötelező funkcionális követelmények

A rendszernek az alábbi alapvető funkciókat mindenképpen biztosítania kell:
- Két játékos közötti sakkjátszma lebonyolítása, a hivatalos sakk szabályai alapján.
- A játékosok felváltva tehetnek lépéseket, a lépések érvényességét a kliens oldali logika ellenőrzi.
- A rendszer valós időben szinkronizálja a két kliens állapotát (mindkét fél ugyanazt a táblát látja).
- A játék vége (sakkmatt, patt, idő lejárta) automatikusan felismerésre kerül.
- A szerver egyidejűleg több játékot is képes kezelni (külön szobákban vagy sessionökben).
- A játékosok elindíthatnak új meccset, illetve befejezett játék után visszatérhetnek a főmenübe.
- A rendszer minden játékban egyedi azonosítót (Game ID) használ a játékállapot nyomon követéséhez.
- A kliens értesítéseket kap az ellenfél lépéseiről és a játék állapotváltozásairól.

### 4.2 Kliens oldali követelmények

A kliens felelős a játékos felhasználói élményéért, a grafikus megjelenítésért és a játéklogika helyi működéséért.

A kliensnek tudnia kell:
- A sakk tábla és a figurák megjelenítése, lépések kezelése (egérkattintás vagy billentyűparancsok).
- Lépések érvényesítése és elküldése a szervernek.
- A szervertől érkező események (ellenfél lépése, állapotváltozás) feldolgozása és megjelenítése.
- Hibák és megszakadt kapcsolat kezelése (újracsatlakozási lehetőség).
- Saját IP vagy szerver cím megadása LAN esetén.
- Alapvető menürendszer (csatlakozás, szerverindítás, új játék, kilépés).
- A hálózati kommunikáció egységes formátumban történjen (pl. JSON alapú üzenetek).

### 4.3 Szerver oldali követelmények

A szerver feladata a kliensek közti kommunikáció kezelése, az állapotok szinkronizálása és a játék logikai integritásának megőrzése.

A szervernek tudnia kell:
- Kapcsolatok fogadása és kezelése több kliens esetén is.
- Új játék (session) létrehozása és azonosító kiosztása.
- Üzenetek továbbítása a kliensek között (pl. lépés, visszajelzés, játék vége).
- A játékállapot naprakészen tartása és küldése mindkét félnek.
- Kapcsolat megszakadása esetén az érintett játék szüneteltetése vagy lezárása.
- Üzenetformátumok ellenőrzése és hibás adatok elutasítása.
- Kliensazonosítás és egyszerű hitelesítés (pl. játékosnév alapján).
- A kommunikáció biztonságos kezelése (üzenetduplikáció, szinkronizációs hibák elkerülése).

### 4.4 A komponensek közti kommunikáció (szerződések)

A rendszer komponensei egy meghatározott üzenetprotokollon keresztül kommunikálnak egymással.
A kommunikáció kétirányú, valós idejű, és az alábbi szerződések szerint zajlik:

A kliens minden lépést csak akkor hajt végre a felhasználói felületen, ha a szervertől visszaigazolást kapott az érvényességéről.
A szerver az üzeneteket sorosítva, FIFO-elv szerint dolgozza fel, és broadcastolja a változásokat az adott játékhoz tartozó összes kliensnek.

### 4.5 Nem funkcionális követelmények

- Megbízhatóság: a rendszernek stabilan kell működnie hálózati késleltetés és csomagvesztés esetén is.
- Teljesítmény: a szerver legalább 10 párhuzamos játékot képes kezelni érezhető lassulás nélkül.
- Biztonság: a kliens csak a szerver által engedélyezett parancsokat hajthatja végre.
- Bővíthetőség: a rendszer felépítése moduláris legyen, hogy később könnyen kiterjeszthető legyen (pl. online matchmaking).
- Platformfüggetlenség: a kliens és a szerver futtatható legyen Windows, Linux és esetleg webes környezetben.
- Karbantarthatóság: kódmodulok (logika, hálózat, UI) elkülönítése, jól dokumentált interfészekkel.

### 4.6 Minimális technikai elvárások

- Programozási nyelv: Rust, C#, Python vagy más, hálózati alkalmazásokra alkalmas nyelv.
- Kommunikációs protokoll: TCP vagy WebSocket alapú kapcsolat.
- Adatcsere formátum: JSON.
- Grafikus felület: desktop GUI (pl. egérvezérlés, drag & drop lépés).
- Követelmény kliensoldalon: legalább 4 GB RAM, modern operációs rendszer.
- Követelmény szerveroldalon: 1 CPU mag, 512 MB RAM, állandó hálózati kapcsolat.

## 5. Követelménylista

### Szerver
| Név | Verzió | Leírás |
| --- | ------ | ------ |
| **WebSocket**                  | 1.0    | A szerver és a kliens között folyamatos kétirányú kommunikációt biztosít. A kapcsolat létrejötte után a szerver valós időben képes fogadni és továbbítani az eseményeket (pl. lépés végrehajtása, állapotfrissítés). Hiba esetén a kapcsolat automatikusan újraépül.              |
| **Kapcsolatok csoportosítása** | 1.0    | A szerver figyeli az elérhető, szabad klienseket, majd két szabad kapcsolatot automatikusan összerendel egy meccsbe. A csoportosítás után a játékosok azonos „room”-ba kerülnek, és a szerver biztosítja az egymás közötti adatkommunikációt.                                     |
| **Kommunikáció az engine-nel** | 1.0    | A szerver a játékosoktól érkező lépéseket és játékinformációkat továbbítja az engine-nek feldolgozásra. Az engine válasza után a szerver visszaküldi az eredményt a klienseknek (pl. érvényes lépés, matt, patt). A kommunikáció aszinkron módon zajlik, válaszidő-ellenőrzéssel. |
| **Kommunikáció a UI-al**       | 1.0    | A szerver WebSocket-en keresztül adatokat továbbít a felhasználói felület és az engine között. A UI által kért műveletek (pl. új meccs létrehozása, állapotlekérés) feldolgozását a szerver közvetíti.|
### Engine
| Név | Verzió | Leírás |
| --- | ------ | ------ |
| **Bitboard** | 1.0 | A játék táblaállapotát bitműveletekkel reprezentálja a hatékonyság érdekében. Minden bábu típus és szín külön bitmask-on kerül tárolásra, lehetővé téve a gyors lekérdezéseket és lépésellenőrzéseket. |
| **Lépésgenerálás LUT** | 1.0 | Előre kiszámított lookup táblák segítségével gyorsítja a lépésgenerálást és szabályellenőrzést. Ez csökkenti a számítási időt, és optimalizálja az engine teljesítményét. |
| **Lépésgenerálás** | 1.0 | A különböző bábutípusok (gyalog, bástya, futó, stb.) lépési logikáját valósítja meg. A függvények ellenőrzik a lépés érvényességét, figyelembe véve az aktuális állást, sakkhelyzetet és speciális szabályokat (pl. sáncolás, en passant). |
| **Util függvények** | 1.0 | Segédfüggvények az engine belső működéséhez, például raycast műveletek, bitműveleti maszkok kezelése, valamint logikai ellenőrzések a lépések és ütések számításához. |

### UI
| Név | Verzió | Leírás |
| --- | ------ | ------ |
| **Belépés** | 1.0 | A felhasználó a kezdőképernyőn keresztül adhatja meg a nevét lokális játékhoz, vagy hitelesítheti magát online játékmód esetén. Hibás adatok esetén a rendszer figyelmeztetést küld. |
| **Főmenü** | 1.0 | Az alkalmazás központi navigációs felülete, ahol a felhasználó meccset kereshet, új játékot indíthat lokálisan, vagy beállításokat módosíthat. A menü megjeleníti az aktuális státuszt (online/offline). |
| **Játék** | 1.0 | A játékfelület megjeleníti a táblát, bábukat, lépéseket, és az aktuális játékállást. Támogatja mind az online, mind a lokális módot. A felület kezeli az interakciókat (lépéskattintás, visszavonás, végeredmény kijelzés). |
| **Kommunikáció a szerverrel** | 1.0 | A kliens a szerveren keresztül kommunikál az engine-nel. A UI felel az üzenetek küldéséért (lépés, új játék, visszajelzés), valamint a szervertől kapott események vizuális megjelenítéséért. |


### GitHub Actions (CI/CD)
| Név | Leírás |
| --- | ------ |
| Folyamatos tesztelés | A projekt minden commit után automatikusan tesztelődik. A pipeline lefuttatja a teszteket, és értesítést küld hibás build esetén. |
| Folyamatos integráció | Az új funkciók beolvadásakor a rendszer automatikusan integrálja a változtatásokat, új buildet hoz létre, és frissíti a fejlesztői környezetet. |
| Tesztadatok | A tesztadatok legyenek elérhetőek egy táblázatban, dátummal ellátva. (Google Sheets) |
