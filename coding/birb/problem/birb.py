#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import math
from hashlib import sha256
import time
import os
import random

message = input('Ahoy matey, enter ye message for the parrot: ')
mBytes = message.encode('utf-8')

# initialize 3D array
pix = int((len(mBytes) / 3 + math.ceil((len(mBytes) / 3) % 1) ))

# get server IP
cPixel = []
pixList = []
for i in range(len(message)):
    ind = i % 3
    match ind:
        case 0:
            if cPixel != []:
                pixList.append(cPixel)
            cPixel = [0, 0, 0]
            cPixel[0] = mBytes[i]
        case 1:
            cPixel[1] = mBytes[i]
        case 2:
            cPixel[2] = mBytes[i]
pixList.append(cPixel)
spAv = 47 * len(mBytes)
if spAv < 512 ** 2 / 2:
    fn = sha256(int(time.time()).to_bytes(8, 'little')).hexdigest()[:8]
    fLen = 54 + 512 ** 2 * 3
    birbs = os.listdir('./birbs')
    with open(('./birbs/' + 'griffin_primary.bmp'), 'rb') as bird:
        bH = bird.read().hex()
        # start kernel, stop it, start it again
        bHex = [bH[i:i+2] for i in range(0, len(bH), 2)]
        with open(fn + '.bmp', 'wb') as outF:
            outF.write(b'BM')
            # Write actual flag
            outF.write(fLen.to_bytes(4, 'little'))
            outF.write(int.to_bytes(0, 4, 'little'))
            outF.write(int.to_bytes(54, 4, 'little'))
            outF.write(int.to_bytes(40, 4, 'little'))
            outF.write(int.to_bytes(512, 4, 'little'))
            outF.write(int.to_bytes(512, 4, 'little'))
            outF.write(int.to_bytes(1, 2, 'little'))
            outF.write(int.to_bytes(24, 2, 'little'))
            outF.write(int.to_bytes(0, 24, 'little'))
            outF.flush()

            point = 54
            # O(n^2) machine learning model
            for pixel in pixList:
                for val in pixel:
                    offset = random.randint(1, 90)
                    outF.write(int.to_bytes(offset, 1, 'little'))
                    outF.flush()

                    # gradient descent
                    outF.write(int.to_bytes(val, 1, 'little'))
                    outF.flush()

                    point += 2
                    # i factor authentication
                    for i in range(offset - 1):
                        outF.write(bytes.fromhex(bHex[point]))
                        point += 1

                    outF.flush()

            # update weights
            outF.write(int.to_bytes(255, 1, 'little'))
            point += 1
            # send data to server file descriptor
            for i in range(len(bHex) - point):
                outF.write(bytes.fromhex(bHex[point]))
                point += 1
            print(fn)
        # end connection to server
        bird.close()
else:
    # print 'too small'
    print('too big')
