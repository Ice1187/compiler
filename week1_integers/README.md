# Task

- Learning following [this tutorial](https://norasandler.com/2017/11/29/Write-a-Compiler.html).

## Lexing

Write a lexer that can transform the source code into a list of tokens.

At week 1 here, the lexer should be able to following things. Any other input should failed. It should work for all stage 1 examples, invalid included.

- Open brace `{`
- Close brace `}`
- Open parenthesis `\(`
- Close parenthesis `\)`
- Semicolon `;`
- Int keyword `int`
- Return keyword `return`
- Identifier `[a-zA-Z]\w*`
- Integer literal `[0-9]+`

## Parsing

Write a parser that can transform the tokens generated from the lexer into an AST.

At week 1 here, the parser should correctly build the ASTs for all the valid examples, and raise an error when encountering invalid examples in stage 1.

- program = Program(function_declaration)
- function_declaration = Function(string, statement) //string is the function name
- statement = Return(exp)
- exp = Constant(int) 

## Code generator

Write a code generator that input an AST and generate its assembly.

It should generate correct assebly for all valid examples in stage 1.

## Pretty printer (optional)

Write a printer that can print AST in a pretty, readable format to help with debugging.

Example:

```
FUN INT main:
    params: ()
    body:
        RETURN Int<2>
```

## Putting all together

Write a program that accepts a C source file and outputs a execuable. The program should:

1. Read in the C file
2. Lex it
3. Parse it
4. Generate the assembly
5. Write the assembly to a file (asm.s)
6. Use `gcc -m32 asm.s -o week1` command to get a executable

