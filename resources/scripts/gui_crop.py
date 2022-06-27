#!/usr/bin/env python3

import os
from PIL import Image
from pyparsing import col

IMG_PATH = 'images/GUI.png'

img = Image.open(IMG_PATH)
w, h = img.size

split_square = 16

for row in range(19):
    for column in range(12):
        path = os.path.join('./results/GUIs/' + str(row) +
                            '_' + str(column) + '.png')
        x = column * split_square
        y = row * split_square
        box = (x, y, x + split_square, y + split_square)
        croppedImg = img.crop(box)
        croppedImg.save(path)
