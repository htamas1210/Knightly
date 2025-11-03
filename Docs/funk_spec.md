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
