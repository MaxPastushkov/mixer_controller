import sys

offset = 3

if len(sys.argv) < 3:
    exit(-1)

with open(sys.argv[1], 'r') as inp:
    arr = inp.read().split(' ')
    with open(sys.argv[2], 'w') as out:
        for i in range(len(arr)//2):
            val = ((int(arr[i*2], 16) & 0b1111) << 4) | (int(arr[i*2+1], 16) & 0b1111)
            out.write("0x{:03X}: {:02X}\n".format(abs(i-offset), val))
