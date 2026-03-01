/*****************************************************************************
 * Project: Dungeon Buddy Firmware                                           *
 * File: lpc1114.ld                                                          *
 *                                                                           *
 * Dungeon Buddy Firmware is free software: you can redistribute it and/or   *
 * modify it under terms of the GNU General Public License as published by   *
 * the Free Software Foundation, either version 3 of the License, or (at     *
 * your option) any later version.                                           *
 *                                                                           *
 * Dungeon Buddy is distributed in the hope that it will be useful, but      *
 * WITHOUT ANY WARRANTY; without even the implied warranty of                *
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General  *
 * Public License for more details.                                          *
 *                                                                           *
 * You should have received a copy of the GNU General Public License along   *
 * with Dungeon Buddy Firmware. If not, see <https://www.gnu.org/licenses/>. *
 *                                                                           *
 * Copyright (C) 2024 - Dylan Eskew <dev.dylan@eskew.dev>                    *
 *****************************************************************************/
MEMORY {
  flash (rx) : org = 0x00000000, len = 32k
  sram (rwx) : org = 0x10000000, len = 4k
}

ENTRY(vector);

SECTIONS {

  .text : {
    . = 0x00000000;
    *(.vector);
    . = ALIGN(4);
    *(.text);
    . = ALIGN(4);
  } > flash

  .rodata : {
    *(.rodata);
    . = ALIGN(4);
  } > flash

  .data : {
    . = 0x00000000;
    *(.data);
    . = ALIGN(4);
  } > sram AT > flash

  .bss : {
    *(.bss);
    . = ALIGN(4);
  } > sram

  __DATA_LMA = LOADADDR(.data);
  __DATA_VMA = ADDR(.data);
  __DATA_END = ADDR(.data) + SIZEOF(.data);
  __BSS_VMA  = ADDR(.bss);
  __BSS_END  = ADDR(.bss) + SIZEOF(.bss);
}

PROVIDE(__INITIAL_SP = ORIGIN(sram) + LENGTH(sram));
PROVIDE(__VECTOR_CHECKSUM = 0 - __INITIAL_SP
                              - (__reset + 1)
                              - 1
                              - 1);

PROVIDE(LPC_SYSCON = 0x40048000);
PROVIDE(LPC_IOCONFIG = 0x40044000);

PROVIDE(LPC_GPIO0 = 0x50000000);
PROVIDE(LPC_GPIO1 = 0x50010000);
