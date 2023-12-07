# cycle detection taken from https://topaz.github.io/paste/#XQAAAQDeEAAAAAAAAAA0m0pnuFI8c/e2SDRqvyl0DGid1Rx5W42tStn+BRzbLAr/JndLruvOMW7160tkMywF1IdThvU6P2T7qJ1uCU6EcUBbdODJP0V3JAf+9my7rrAwRqyZL2WX8wlY6YrnganSh1otE1W3VsuRq4lgoWvwrxaCb3OgMEWPMkZeYY0H5+0eHmRT3hW3wEJeGzFU/Hd08ukSqmUeXjPOCzf7nWr6l7nW5T5S2dr+NU2x9uOQ8N2lMIWH/S5mAUrdIlXwNiWTfjH4mwd40FLb+1JBgp1DLaC/BT3smbXQEETYg9ASyuid0TDKCH2D4zlQx9+Mk1szjRSTm+0/W1UNbp8y1U266CXodvQ77K6MgYgChP8CGHijwxpBM+5754y537CBOI0EVFAaRme16PTZhAGM6PVBIA5+zoxlkqzLzz4kPgSDqsywAghB9K2uhCfFi6c3gahAPr+iE34KfjlsGmYUFApyUb1W9lznSY/DGT2U/fc+KQj5Ne3h0iIeVsjTYB+g8AUya1gfswGfmA8nlEhLXJA4tCGzcxaIpU5yNIPtoQkKe9coY8kNaLMZB9wHmqWfzoo3UbHGZEkt3DCGqR+9DXHEPybzAy1MnBesT0wqAQEpp94nTTyno0ajBNRdYHTSDa+nGpHRHwvT8oYxJwzk/kOp45eBR2dAutPzQdI9VM5pLaMEm54KUIHCJnVbFHrIlQjZK1Pupoga4j/GLDKgw25BGmrHY/zNRNoJV5g06w7KsIQG+O1jQ/XTOmZrfGeN7T7OHXOOlyUBFBu5gAJcwWWjGX7kOXJ+WXT/egge8MuU0GQS0UBgfDQhKUWkD5Wpn/wod+Io1cU1+K87QYfMrI2ApAsBgKZDUjoJOBcVlVF5PYwQAj70BHvImCvx1PVsdupsPor7dnU5NBmjJxJLgQJhltdocdNoar56vg0rFp/aObCvi/GVDpBgRZwemJwS3Je1Pw3Gq8InsIDNVtWISF+CgtXzg738nF0NNyC3aiCrVRYvr0Hil+v2ysQUWGbmZxTduJgGgrC4HXvF4dmvxg5nRl9n9bv30q+cN8r1Jifwtab2IG4ogUe0o0+H5WTMMn4C6Im3toCh4NqqjHC+vBrrbEWQ9wau543z40lVI+ItSxT8ogvCa7rpTKqVlF33N/ioMLV2dXtvXcjWyUjRQJzWTGctVjaQ8Us6WoJVICl5CbpViNjxAP6aTQSxxmp9eXYDhPOxZ0z68v9G1lX01IM6Ptxv29/h4KZvnG6b+3wZvgdQPWlTF7BfWaUYTylrY+PYvNL4XcQj2sgFM/KhwP6Efm9uEzwV9BNKW88Y/v04NdsY3wNCFvSE+o46A9CzAE1hOU3Orr2M5zsT0H5yfhlu/Y885RBYuPuXUZjcUhPCbVCpwQEvxjIfh/8/xVAFRlx3yYqqiRig80WR+9BShigKLcIGCEBaABL31C6KhumsY0FNsRINV6GFanZpM9tQPPuMGV3ZPjj1ywaI6cxoiEkQpvv/hTdMm5m2Gh+a+Rt144+AYTppSXbOuBdnuDn6aKmLUTNXXiwIOSlDUiGxzQF7PZ7UNsj0vzfexBSIe46O4we2iK+/gkzZDcRoLQYlvn9E0oQmevPxxsJMp0cesCOf8grbkjDb2SqasYPmi0xS0fQdIH+0BY2zyVpsTzVxGYpbKxG2hUNHMwq9dhtSeUNi/muyBxYHNwaE12IysoWfLoEgtj3nISDIuGig7zWw+FT2vRnZUgQaG5YgPdY89wKdn+zpzR+Mo3xpcl4v+YNmRGGVaiR1S8cczDhoXCQ+088IeFvzT5jmyj9PUDfdGSFGaRCIwXbWcn3LgGXvHy6iWw+FUGblpOOmrX7n7I8ynaPAOkmsaJ5442Ol1ONlYcL9VR1TnR32ILuz6/p+S9anovCTTMEwtZce0XjjavFBjiBON+cJif89z/gA

year = 2022
num = 17
isB = True
test = False

filename = f'{year}/{num}/{"data" if not test else "test"}'
infile = f'{filename}.in'
outfile = f'{filename}_{"a" if not isB else "b"}.out'


pieces = [[(2, 0), (3, 0), (4, 0), (5, 0)],
          [(3, 0), (2, 1), (3, 1), (4, 1), (3, 2)],
          [(2, 0), (3, 0), (4, 0), (4, 1), (4, 2)],
          [(2, 0), (2, 1), (2, 2), (2, 3)],
          [(2, 0), (2, 1), (3, 0), (3, 1)]]


def generate_top_view(landed):
    heights = [-17 for _ in range(7)]
    for x,y in landed:
        heights[x] = max(heights[x], y)
    max_val = max(heights)
    return tuple(height - max_val for height in heights)


NUM = 1_000_000_000_000


# 2 units from left, 3 units above highest rock
def solver(file):
    landed = set()
    moves = list(file)[0]
    max_height = -1
    print(len(moves))

    rocks = [[x, y+max_height+4] for x, y in pieces[0]]
    count = 0
    seen = dict()
    additional = 0
    while count < NUM:
        for idx, move in enumerate(moves):
            if move == '<':
                if all(x!=0 for x, _ in rocks) and all((x-1,y) not in landed for x, y in rocks):
                    for i in range(len(rocks)):
                        rocks[i][0] -= 1
            elif move == '>':
                if all(x!=6 for x, _ in rocks) and all((x+1,y) not in landed for x, y in rocks):
                    for i in range(len(rocks)):
                        rocks[i][0] += 1
            
            if rocks[0][1] == 0 or any((x,y-1) in landed for x, y in rocks):
                for x, y in rocks:
                    landed.add((x,y))
                max_height = max(max_height, rocks[-1][1])

                count += 1
                rocks = [[x, y+max_height+4] for x, y in pieces[count%len(pieces)]]

                if count >= NUM:
                    break

                top_view = generate_top_view(landed)
                state = (top_view, count%len(pieces), idx)
                if state in seen:
                    print(state)
                    print(count, max_height)
                    _count, _max_height = seen[state]
                    repeat = (NUM - count) // (count - _count)
                    count += (count - _count) * repeat
                    print(NUM - count)
                    additional += repeat * (max_height - _max_height)
                    seen = dict()
                seen[state] = (count, max_height)


            else:
                for i in range(len(rocks)):
                    rocks[i][1] -= 1

    # max_height = max(landed)

    return max_height + 1 + additional
        


def main():
    with open(infile) as file:
        s = solver(file)

    if s is None:
        return

    print(s)
    with open(outfile, 'w') as file:
        file.write(str(s))


if __name__ == '__main__':
    main()
