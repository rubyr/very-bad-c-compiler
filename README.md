# very bad c compiler (VBCC)

my workings through nora sandler's wonderful book,
[Writing A C Compiler](https://nostarch.com/writing-c-compiler)

## usage

don't use this.

for educational purposes only:

```sh
very-bad-c-compiler <file>

# also accepts some arguments:
very-bad-c-compiler --lex <file> # stops after the lexing phase
very-bad-c-compiler --parse <file> # stops after the parsing phase
very-bad-c-compiler --codegen <file> # stops after the codegen phase
very-bad-c-compiler -S <file> # outputs an assembly file (.s) rather than an executable
```

## structure

```mermaid
flowchart TD
    A[???]-->B[???];
    A-->C[???];
    B-->D[???];
    C-->D;
    D-->C;
    D-->A;
    D-->E[?????];
    B-- help -->C;
```
