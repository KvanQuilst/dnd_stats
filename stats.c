/*****************************************************************************
 * Project: Dungeon Buddy Firmware                                           *
 * File: stats.c                                                             *
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
#include "stats.h"

inline void stat_hp_increase(struct statblock *sb, uint8_t points)
{
  uint16_t result = sb->hp + (uint16_t)points;
  sb->hp = (result > HP_MAX || result < sb->hp) ? HP_MAX
                                                : result;
}

inline void stat_hp_decrease(struct statblock *sb, uint8_t points)
{
  uint16_t result = sb->hp - (uint16_t)points;
  sb->hp = (result > sb->hp) ? 0
                             : result;
}

inline void stat_temp_increase(struct statblock *sb, uint8_t points)
{
  uint16_t result = sb->temp + (uint16_t)points;
  sb->temp = (result > TEMP_MAX || result < sb->temp) ? TEMP_MAX
                                                      : result;
}

inline void stat_temp_decrease(struct statblock *sb, uint8_t points)
{
  uint16_t result = sb->temp - (uint16_t)points;
  sb->temp = (result > sb->temp) ? 0
                                 : result;
}

inline void stat_xp_increase(struct statblock *sb, uint8_t points)
{
  uint16_t result = sb->xp + (uint32_t)points;
  sb->xp = (result < sb->xp) ? UINT32_MAX
                             : result;
}

inline void stat_xp_decrease(struct statblock *sb, uint8_t points)
{
  uint16_t result = sb->xp - (uint32_t)points;
  sb->xp = (result > sb->xp) ? 0
                             : result;
}
