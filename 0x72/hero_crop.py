#!/usr/bin/env python3

import os
from PIL import Image

IMG_PATH = 'image/0x72_DungeonTilesetII_v1.4.png'
TILES_PATH = 'titles/hero_list.txt'

def getBox(arr, totalFrame = 1):
    x = int(arr[1])
    y = int(arr[2])
    w = int(arr[3]) * int(totalFrame)
    h = int(arr[4])
    return (x, y, x+w, y+h)

def saveCrop(img, title, box):
    path = os.path.join('./heros/' + title + '.png')
    try:
        croppedImg = img.crop(box)
        croppedImg.save(path)
        print('ok: ' + title)
    except Exception as e:
        print('fail: ' + title + ' -- ' + str(e))

img = Image.open(IMG_PATH)
w, h = img.size

f = open(TILES_PATH, 'r')
for line in f.readlines():
    arr = line.split()
    if len(arr) == 0:
        pass
    if len(arr) == 5:
        saveCrop(img, arr[0], getBox(arr))
    if len(arr) == 6:
        saveCrop(img, arr[0], getBox(arr, arr[5]))

print('')

