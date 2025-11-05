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

## A github workflow folyamata:

A githubon tárolt kód minden feltöltésnél a kód mindig automatikus tesztelve van, egy workflow által. Ennek a működése:
1. A brancheket a következő képpen kell elnevezni: Projekt/task_neve (a task neve a kanban tablabol jon).
2. 3 projekt van a github repository-ban: Engine, Server és a UI. Ennek megfelelően a következőképpen kell kinézniük a brancheknek: 
    - Engine/task_1
    - Server/task_1
    - UI/task_1
    Erre azért van szükség mert a github workflow ezek alapján az elnevezések alapján fogja eldönteni melyik teszteket futtassa. Ha az Engine projekthez lett feltöltve kód, akkor csak az ahhoz tartozó teszteket futtassa.
3. A workflow folyamat a következő:
    Amikor feltöltődik a három projekt közül valamelyikére egy új commit. A github workflow megnézni mi a branch neve. Ezután letölti a projektet githubról, és lefuttatja azon projekt teszteit. A tesztek eredményét feltölti egy google táblazat fájlba, ahonnan bármikor visszenézhető. Minden teszt eredmény az annak megfelően elnevezett munkalapra kerül be (Engine, Server, UI), egymás alá táblázatos elrendezésben. Az oszlopok a következőek: Dátum, függvény neve, teszt bemenet, teszt kimenet, sikeres-e.
    Amikor a main/master branchre kerül fel kód. A teszt az előbbiek alapján ugyanúgy lefut, csak itt mind a 3 projekt tesztje le fog futni és az eredmény egy main/master nevű munkalapra fog feltöltődni a táblázatban. Amint sikeresen lefut a tesztelés, elindul egy automatikus build, ami le fogja build-elni a projekteket. Ezután létrehoz egy új Release-t githubon, ahol beállítja a verzió számot a rust projektben beállított verzió számra, majd feltölti oda a fájlokat amiket a build hozott létre, és kiteszi ezt a buildet.


A workflow 3 külön fáljba van bontva:
1. A dispatcher (dispatch.yml). Ez indítja el a workflow-t ami teszteli a kódot a megfelelő branchen, elindítja a release workflow-t ami feltölti a master branchről buildelt projekt fájlokat.
2. A teszter (tests.yml). Ez a workflow bele fog lépni a megfelelő projekt mappába és lefutattja a megírt teszteket, és feltölti az eredményt egy google spreadsheet-be.
3. A release (release.yml). Amint sikeresek a tesztek elindul ez a workflow és lefordítja a kódot majd feltölti github-ra egy új release-ként
