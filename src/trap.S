/*************************************************************************
 * Project           		 : shakti devt board
 * Name of the file              : trap.S 
 * Brief description             : Trap handling is managed from here. 
 * Name of Author                : Abhinav Ramnath & Sathya Narayanan N
 * Email id                      : abhinavramnath13@gmail.com & sathya281@gmail.com
 
 Copyright (C) 2019  IIT Madras. All rights reserved.

 This program is free software: you can redistribute it and/or modify
 it under the terms of the GNU General Public License as published by
 the Free Software Foundation, either version 3 of the License, or
 (at your option) any later version.

 This program is distributed in the hope that it will be useful,
 but WITHOUT ANY WARRANTY; without even the implied warranty of
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 GNU General Public License for more details.

 You should have received a copy of the GNU General Public License
 along with this program.  If not, see <https://www.gnu.org/licenses/>.

****************************************************************************/
/**
@file trap.S
@detail Context switching on trap is taken here. After saving the context, trap handler
        is called. After servicing the trap, context is restored. 
*/

// #include "defines.h"

// =======================================================
.globl REGSIZE
REGSIZE =  8

// Conditionally define LREG and SREG based on the target architecture
.globl LREG
.globl SREG

//#if __riscv_xlen == 64
LREG = ld
SREG = sd
//#else
//LREG = lw
//SREG = sw
//#endif

// Define MSTATUS_MPP and MSTATUS_FS
.globl MSTATUS_MPP
.globl MSTATUS_FS

MSTATUS_MPP = 0x00001800
MSTATUS_FS = 0x00006000

//===============================================================

  .global trap_entry
  .align 2

  trap_entry:

  /*
     If trap comes, do
     1. push all x* register values to stack
     2. handle trap
     3. restore x* register values from stack
   */

  addi sp, sp, -32*REGSIZE
  .p2align 3
  sd x1, 1*REGSIZE(sp)
  sd x2, 2*REGSIZE(sp)
  sd x3, 3*REGSIZE(sp)
  sd x4, 4*REGSIZE(sp)
  sd x5, 5*REGSIZE(sp)
  sd x6, 6*REGSIZE(sp)
  sd x7, 7*REGSIZE(sp)
  sd x8, 8*REGSIZE(sp)
  sd x9, 9*REGSIZE(sp)
  sd x10, 10*REGSIZE(sp)
  sd x11, 11*REGSIZE(sp)
  sd x12, 12*REGSIZE(sp)
  sd x13, 13*REGSIZE(sp)
  sd x14, 14*REGSIZE(sp)
  sd x15, 15*REGSIZE(sp)
  sd x16, 16*REGSIZE(sp)
  sd x17, 17*REGSIZE(sp)
  sd x18, 18*REGSIZE(sp)
  sd x19, 19*REGSIZE(sp)
  sd x20, 20*REGSIZE(sp)
  sd x21, 21*REGSIZE(sp)
  sd x22, 22*REGSIZE(sp)
  sd x23, 23*REGSIZE(sp)
  sd x24, 24*REGSIZE(sp)
  sd x25, 25*REGSIZE(sp)
  sd x26, 26*REGSIZE(sp)
  sd x27, 27*REGSIZE(sp)
  sd x28, 28*REGSIZE(sp)
  sd x29, 29*REGSIZE(sp)
  sd x30, 30*REGSIZE(sp)
  sd x31, 31*REGSIZE(sp)

  csrr a0, mcause
  csrr a1, mepc
  mv a2, sp
  jal handle_trap
  csrw mepc, a0

  ld x1, 1*REGSIZE(sp)
  ld x2, 2*REGSIZE(sp)
  ld x3, 3*REGSIZE(sp)
  ld x4, 4*REGSIZE(sp)
  ld x5, 5*REGSIZE(sp)
  ld x6, 6*REGSIZE(sp)
  ld x7, 7*REGSIZE(sp)
  ld x8, 8*REGSIZE(sp)
  ld x9, 9*REGSIZE(sp)
  ld x10, 10*REGSIZE(sp)
  ld x11, 11*REGSIZE(sp)
  ld x12, 12*REGSIZE(sp)
  ld x13, 13*REGSIZE(sp)
  ld x14, 14*REGSIZE(sp)
  ld x15, 15*REGSIZE(sp)
  ld x16, 16*REGSIZE(sp)
  ld x17, 17*REGSIZE(sp)
  ld x18, 18*REGSIZE(sp)
  ld x19, 19*REGSIZE(sp)
  ld x20, 20*REGSIZE(sp)
  ld x21, 21*REGSIZE(sp)
  ld x22, 22*REGSIZE(sp)
  ld x23, 23*REGSIZE(sp)
  ld x24, 24*REGSIZE(sp)
  ld x25, 25*REGSIZE(sp)
  ld x26, 26*REGSIZE(sp)
  ld x27, 27*REGSIZE(sp)
  ld x28, 28*REGSIZE(sp)
  ld x29, 29*REGSIZE(sp)
  ld x30, 30*REGSIZE(sp)
  ld x31, 31*REGSIZE(sp)

  addi sp, sp, 32*REGSIZE
  mret
