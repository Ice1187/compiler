  .intel_syntax noprefix
  .file       "./main.c"
  .globl      main
main:
  mov         eax, 10
  push        eax
  mov         eax, 5
  push        eax
  pop         ecx
  pop         eax
  xor         edx, edx
  idiv        ecx
  push        eax
  mov         eax, 2
  push        eax
  pop         ecx
  pop         eax
  add         eax, ecx
  push        eax
  mov         eax, 4
  push        eax
  pop         ecx
  pop         eax
  xor         edx, edx
  idiv        ecx
  ret
