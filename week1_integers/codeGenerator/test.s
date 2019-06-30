  .intel_syntax noprefix
  .file       "main.c"
  .globl      main
main:
  mov         eax, 2
  ret
  .globl      func
func:
  mov         eax, 3
  ret

