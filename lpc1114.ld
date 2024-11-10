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
