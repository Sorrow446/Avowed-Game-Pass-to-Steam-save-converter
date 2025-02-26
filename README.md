# Avowed-Game-Pass-to-Steam-save-converter
Converts your Game Pass saves so you can load them in the Steam copy of the game.    
![](https://i.imgur.com/9AOrCFu.png)    
[Pre-compiled binaries](https://github.com/Sorrow446/Avowed-Game-Pass-to-Steam-save-converter/releases)

## Usage
Game pass save dir: `%localappdata%\packages\Microsoft.Avowed_8wekyb3d8bbwe\SystemAppData\wgs`    
Steam save dir: `%userprofile%\Saved Games\Avowed`

### Drag and drop batch file
If you can't be bothered with CLI, simply drag and drop your Game Pass save folder onto the included batch file. The gpts binary's still required.

### CLI   
`gpts.exe -i G:\rust\avowed\1A7123D9F06C4863A72BED4A89F18487`

Wrap the input and output paths in double quotes if they contain any spaces.

```
Usage: gpts.exe [OPTIONS] --in-path <IN_PATH>

Options:
  -i, --in-path <IN_PATH>    Input path to Game Pass save folder to convert.
  -o, --out-path <OUT_PATH>  Output path to write converted Steam save to excluding filename. Leave empty for binary/script dir.
  -h, --help                 Print help
```

## Disclaimer   
- This tool has no partnership, sponsorship or endorsement with Obsidian Entertainment, Microsoft or Game Pass.
