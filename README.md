# QUIO
Minimal esoteric programming language.

## Installation
```
git clone https://github.com/joeymalvinni/quio.git
```

## Building

```
cargo build --release
```

## Examples

Print `Hello`:
```
QUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUPUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUPUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUPUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUPUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUPKORIIIIIIIIIIOKOOUUUO
```

## Generating Scripts

To generate Quio scripts to print output, see `/utils/generator.js`

## Language Reference

Quio uses commands to manipulate cells. When Quio starts, the first memory cell is initialized to zero. 

| Command | Description                                                       |
| :----:  | :----                                                             |
| Q       | Main entry point                                                  |
| U       | Increment the first memory cell                                   |
| I       | Decrement the first memory cell                                   |
| R       | Reverse the order of the memory cells                             |
| K       | Swap the data of the first and second memory cells                |
| P       | Push the first cell to the end and replace it with an empty cell  |
| O       | Print the unicode value of cell zero's value                      |
