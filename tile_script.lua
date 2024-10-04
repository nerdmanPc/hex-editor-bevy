bitmask = 
  DownRightTerrain * 1 +
  UpRightTerrain * 3 +
  LeftTerrain * 9;
table = [
  {tileIndex: 0, rotation: 0},
  {tileIndex: 1, rotation: 0},
  {tileIndex: 2, rotation: 0},
  {tileIndex: 1, rotation: 120},
  {tileIndex: 3, rotation: 0},
  {tileIndex: 4, rotation: 0},
  {tileIndex: 2, rotation: 120},
  {tileIndex: 5, rotation: 0},
  {tileIndex: 6, rotation: 0},
  {tileIndex: 1, rotation: 240},
  {tileIndex: 3, rotation: 240},
  {tileIndex: 5, rotation: 240},
  {tileIndex: 3, rotation: 120},
  {tileIndex: 7, rotation: 0},
  {tileIndex: 8, rotation: 0},
  {tileIndex: 4, rotation: 120},
  {tileIndex: 8, rotation: 120},
  {tileIndex: 9, rotation: 0},
  {tileIndex: 2, rotation: 240},
  {tileIndex: 4, rotation: 240},
  {tileIndex: 6, rotation: 240},
  {tileIndex: 5, rotation: 120},
  {tileIndex: 8, rotation: 240},
  {tileIndex: 9, rotation: 240},
  {tileIndex: 6, rotation: 120},
  {tileIndex: 9, rotation: 120},
  {tileIndex: 10, rotation: 0},
];
return table[bitmask];