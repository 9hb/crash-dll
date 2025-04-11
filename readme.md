# Crash-DLL

Injektovatelná dynamická knihovna pro testování různých typů pádů a ukončení procesů v Windows aplikacích.

## Popis

Tato knihovna je navržena pro injektování do běžících Windows procesů, kde vytvoří konzolové okno s možnostmi testování různých typů chybových stavů. Může být využita pro testování odolnosti aplikací, debugování nebo vzdělávací účely.

## Funkce

- **Vytvoření konzolového okna** v cílové aplikaci po injektáži
- **Lokalizovaný výstup** přizpůsobený uživatelskému jazykovému nastavení
- **Přizpůsobitelná velikost konzole** pro optimální zobrazení
- **Interaktivní menu** nabízející následující možnosti:
  1. Vyvolat pád aplikace přístupem mimo hranice pole
  2. Násilně ukončit aplikaci pomocí `abort()` funkce
  3. Standardně ukončit aplikaci pomocí `exit()`

## Technické detaily

- Implementováno v Rustu s využitím Windows API
- Kompilováno jako dynamická knihovna (DLL)
- Automatické spuštění po injektáži díky DllMain handleru
- Podpora kódových stránek pro správné zobrazení znaků

## Použití

1. Zkompilujte knihovnu pomocí příkazu `cargo build --release`
2. Injektujte výsledný soubor `mylib.dll` do cílového procesu pomocí vašeho oblíbeného DLL injektoru
3. V cílové aplikaci se objeví konzolové okno s nabídkou možností
4. Vyberte požadovanou akci zadáním čísla 1-3

## Kompilace

```bash
cargo build --release
```

Výsledná knihovna bude umístěna v `target/release/mylib.dll`.

## Požadavky

- Rust 2021 Edition nebo novější
- Windows OS pro cílovou aplikaci
- DLL injektor pro vložení knihovny do cílového procesu

## Bezpečnostní upozornění

Tato knihovna je určena pouze pro testovací a vzdělávací účely. Použití této knihovny k injektování do aplikací třetích stran může být v rozporu s podmínkami jejich používání.
