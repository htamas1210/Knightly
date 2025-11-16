# Követelmény-specifikáció

## 1. Áttekintés

A jelen dokumentum célja, hogy bemutassa a CastlingCreations megrendelésére készülő Knightly nevű alkalmazás alapvető céljait, funkcionális és nem funkcionális követelményeinek áttekintését, valamint a fejlesztés kontextusát.

A Knightly egy modern, digitális sakkalkalmazás, amely kezdetben helyi hálózaton (LAN) keresztül teszi lehetővé két játékos számára, hogy valós időben mérkőzzenek meg egymással. A rendszer egy grafikus felhasználói felületen keresztül biztosítja a játék indítását, a szerver futtatását és a másik félhez történő csatlakozást.

A projekt hosszú távú célja, hogy a Knightly egy online platformmá fejlődjön, amely hasonló módon működik, mint a népszerű sakkportálok (pl. chess.com): a felhasználók fiókot hozhatnak létre, bejelentkezhetnek, és egy központi szerveren keresztül kereshetnek, illetve indíthatnak mérkőzéseket.

A fejlesztés első szakasza azonban a LAN-alapú verzió megvalósítására koncentrál, amely a sakkjátszma logikai alapjainak, a játékállapot kezelésének és a hálózati kommunikáció modelljének megvalósítását célozza. A későbbi online verzió ezekre az alapokra építkezve bővíthető tovább.

## 2. Vágyálom rendszer
A vágyálom rendszer célja egy teljes körű, modern online sakkplatform létrehozása, amely nemcsak a klasszikus sakkjáték digitális megvalósítását kínálja, hanem egy közösségi, versenyképes és kényelmes felhasználói élményt is biztosít. A hosszú távú cél, hogy a rendszer működése és felépítése a nagyobb nemzetközi sakkoldalakhoz (például a chess.com-hoz vagy a lichess.org-hoz) hasonló legyen, de saját, könnyen kezelhető és letisztult felülettel.

A felhasználók a rendszerben saját profilt hozhatnak létre, amellyel be tudnak jelentkezni, és részt vehetnek online mérkőzéseken más játékosok ellen. A rendszer automatikusan párosítaná őket ellenfelekkel, de lehetőséget adna arra is, hogy barátokat hívjanak meg privát meccsekre. A lejátszott mérkőzések mentésre kerülnének, így a játékosok bármikor visszanézhetnék vagy elemezhetnék azokat.

A játék mellett a felhasználók statisztikákat is láthatnának magukról, például nyerési arányt, aktuális értékszámot (rating), leggyakoribb megnyitásokat, illetve fejlődési tendenciát az idő során. A rendszer ezen felül egy egyszerű chat funkciót is tartalmazna, hogy a játékosok kommunikálhassanak egymással a játszmák közben vagy akár azokon kívül is.

A vágyálom rendszer alapját egy központi szerver képezné, amely kezeli a felhasználói fiókokat, a bejelentkezéseket, a matchmaking folyamatot, valamint a játékok futását és szinkronizálását. A szerver a kliensalkalmazásokkal valós idejű adatkapcsolatot tartana fenn, így a játék során minden lépés azonnal megjelenne a másik játékosnál is.

A platform célja a megbízható és folyamatos működés, akár nagyobb terhelés mellett is. A rendszer fejlesztése során kiemelt szempont lenne a biztonság (adatvédelem, csalás elleni védelem), a stabil hálózati kommunikáció, valamint a bővíthetőség – például ranglisták, versenyek vagy mobilalkalmazás későbbi integrálásának lehetősége.

Összességében a vágyálom rendszer egy minden szempontból teljes értékű, közösségorientált sakkalkalmazás lenne, amely a mostani, helyi hálózaton működő változatból fejlődne tovább egy interneten keresztül bárhonnan elérhető platformmá.

## 3. Igényelt funkciók

### Alapok
- Két játékos közti sakkjátszma lebonyolítása.
- Teljes szabályrendszer megvalósítása (érvényes lépések, sakk/sakkmatt/patt felismerése).
- Új játék indítása.
- Játékosok nevének megadása a játszma elején.
- Felhasználóbarát grafikus felület (UI) – látható tábla, figurák, órák, státuszjelzések.
- Játékoslépések vizuális kiemelése (pl. kijelölt mező, lehetséges lépések megjelenítése).
- A játék állapotának kijelzése (folyamatban, sakk, matt, döntetlen).
  
### LAN és hálózati funkciók
- „Szerver indítása” funkció – a játékos hostként indíthat egy helyi szervert.
- „Csatlakozás” funkció – másik játékos IP-cím alapján tud csatlakozni.
- Helyi hálózaton keresztüli valós idejű kommunikáció.
- LAN játék automatikus felfedezése (broadcast keresés).
- Játék mentése és visszatöltése hálózati módban.

### Online vágyálom funkciók
- Felhasználói regisztráció és bejelentkezés.
- Jelszóval védett fiókok, email- vagy OAuth-alapú hitelesítés (Google, GitHub stb.).
- Profiloldal megtekintése (név, avatar, statisztikák, értékszám).
- Automatikus matchmaking rendszer.
- Kézi játékindítás – meghívó küldése barátnak.
- Játszmák mentése és visszajátszása.
- Játszmaelemzés – lépések listázása, hibák kiemelése.
- Webes felület vagy mobilalkalmazás támogatása.
- Játék előzményeinek és statisztikáinak megtekintése (győzelmek, vereségek, döntetlenek).
- Automatikus szervermentés és adatbázis szinkronizáció.

### Felhasználói felület
- Letisztult, reszponzív, platformfüggetlen felület (asztali és webes verzió).
- Sötét/világos téma lehetősége.
- Egyéni figurakészlet vagy tábla kinézet választása.
- Animált figuramozgások.
- Egérrel és billentyűzettel is vezérelhető játék.
- Hangjelzések a lépésekhez és az idő lejártához.
- Lépéselőzmények (move list) megjelenítése oldalt.
- Tábla forgatásának lehetősége (pl. a fehér vagy fekete nézőpontból).
- Állapotjelzők (sakk, matt, döntetlen, várakozás az ellenfélre).
- Teljes képernyős mód.

### Technikai / fejlesztői funkciók
- Kliens–szerver architektúra.
- REST API vagy WebSocket alapú kommunikáció.
- Adatbázis a felhasználói adatok és meccsek tárolására (pl. SQLite, PostgreSQL, MongoDB).
- Logolási és hibakezelési rendszer.
- Automatikus mentés és adatvisszaállítás.
- Verziókezelés (Git).
- Tesztelhető, moduláris kódszerkezet (külön modulok: logika, UI, hálózat, adat).
- Cross-platform működés (Windows, Linux, esetleg web).

### További funkciók
- Egyszemélyes mód (ember vs. gép, AI-bot).
- Több nehézségi szintű AI.
- Oktató mód (javasolt lépések, hibák magyarázata).
- Hivatalos FEN/PGN formátum export/import.
- Beépített sakkfeladványok, kihívások.
- Érintéses vezérlés mobilon.
- Többnyelvű felület (pl. magyar, angol).

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
