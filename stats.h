/*****************************************************************************
 * Project: Dungeon Buddy Firmware                                           *
 * File: stats.h                                                              *
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

#define HP_MAX 67
#define TEMP_MAX 999

struct statblock {
  uint16_t hp;
  uint16_t temp;
  uint32_t xp;
};

void stat_increase(struct statblock *sb, enum stat_t stat, uint8_t points);
void stat_decrease(struct statblock *sb, enum stat_t stat, uint8_t points);
