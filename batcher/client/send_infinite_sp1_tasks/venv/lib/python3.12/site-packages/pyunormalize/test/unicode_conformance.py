"""Unicode conformance testing.

Information about conformance testing for Unicode normalization forms:
- https://www.unicode.org/Public/15.1.0/ucd/NormalizationTest.txt
- https://www.unicode.org/reports/tr15/tr15-54.html
"""

import pathlib
import time

from pyunormalize import (
    NFC,
    NFD,
    NFKC,
    NFKD,
    UNICODE_VERSION,
)

# Unicode conformance test file
UNICODE_FILE = "NormalizationTest.txt"


def parse(lines):
    assert UNICODE_VERSION in lines[0], "Wrong Unicode version number."

    data = []    # list of lists
    dec = set()  # needed for character by character test

    for num, line in enumerate(lines, 1):
        if line and not line.startswith(("#", "@")):
            *c, _ = line.split(";", 5)
            record = [
                "".join(chr(int(x, 16)) for x in seq.split())
                for seq in c
            ]
            # record: [source, nfc, nfd, nfkc, nfkd]
            data.append([num, *record])

            if not " " in c[0]:
                dec.add(int(c[0], 16))

    chars = [chr(x) for x in range(0x110000) if x not in dec]

    return data, chars


def main():
    path = pathlib.Path.cwd() / "data" / UNICODE_FILE
    with path.open(encoding="utf-8") as f:
        lines = f.read().splitlines()

    data, chars = parse(lines)

    counter = 0
    start_time = time.perf_counter()

    #
    # NFC
    # c2 ==  toNFC(c1) ==  toNFC(c2) ==  toNFC(c3)
    # c4 ==  toNFC(c4) ==  toNFC(c5)
    #

    print(f"\nNormalization Form C\n{'-' * 70}")

    s = f = 0
    for record in data:
        num, source, nfc, nfd, nfkc, nfkd = record

        lst1 = []
        lst1.append(NFC(source))
        lst1.append(NFC(nfc))
        lst1.append(NFC(nfd))

        lst2 = []
        lst2.append(NFC(nfkc))
        lst2.append(NFC(nfkd))

        if (lst1.count(nfc) == len(lst1)
                and lst2.count(nfkc) == len(lst2)):
            s += 1
        else:
            f += 1
            print(f"Failed on line {num}")

    r = s + f
    if f:
        print(f"FAIL ({r:,} items, {f:,} failures)\n")
    else:
        print(f"OK ({r:,} items)\n")
        counter += 1

    #
    # NFD
    # c3 ==  toNFD(c1) ==  toNFD(c2) ==  toNFD(c3)
    # c5 ==  toNFD(c4) ==  toNFD(c5)
    #

    print(f"Normalization Form D\n{'-' * 70}")

    s = f = 0
    for record in data:
        num, source, nfc, nfd, nfkc, nfkd = record

        lst1 = []
        lst1.append(NFD(source))
        lst1.append(NFD(nfc))
        lst1.append(NFD(nfd))

        lst2 = []
        lst2.append(NFD(nfkc))
        lst2.append(NFD(nfkd))

        if (lst1.count(nfd) == len(lst1)
                and lst2.count(nfkd) == len(lst2)):
            s += 1
        else:
            f += 1
            print(f"Failed on line {num}")

    r = s + f
    if f:
        print(f"FAIL ({r:,} items, {f:,} failures)\n")
    else:
        print(f"OK ({r:,} items)\n")
        counter += 1

    #
    # NFKC
    # c4 == toNFKC(c1) == toNFKC(c2) == toNFKC(c3) == toNFKC(c4) == toNFKC(c5)
    #

    print(f"Normalization Form KC\n{'-' * 70}")

    s = f = 0
    for record in data:
        num, source, nfc, nfd, nfkc, nfkd = record

        lst = []
        lst.append(NFKC(source))
        lst.append(NFKC(nfc))
        lst.append(NFKC(nfd))
        lst.append(NFKC(nfkc))
        lst.append(NFKC(nfkd))

        if lst.count(nfkc) == len(lst):
            s += 1
        else:
            f += 1
            print(f"Failed on line {num}")

    r = s + f
    if f:
        print(f"FAIL ({r:,} items, {f:,} failures)\n")
    else:
        print(f"OK ({r:,} items)\n")
        counter += 1

    #
    # NFKD
    # c5 == toNFKD(c1) == toNFKD(c2) == toNFKD(c3) == toNFKD(c4) == toNFKD(c5)
    #

    print(f"Normalization Form KD\n{'-' * 70}")

    s = f = 0
    for record in data:
        num, source, nfc, nfd, nfkc, nfkd = record

        lst = []
        lst.append(NFKD(source))
        lst.append(NFKD(nfc))
        lst.append(NFKD(nfd))
        lst.append(NFKD(nfkc))
        lst.append(NFKD(nfkd))
 
        if lst.count(nfkd) == len(lst):
            s += 1
        else:
            f += 1
            print(f"Failed on line {num}")

    r = s + f
    if f:
        print(f"FAIL ({r:,} items, {f:,} failures)\n")
    else:
        print(f"OK ({r:,} items)\n")
        counter += 1

    #
    # Character by character test
    # X == toNFC(X) == toNFD(X) == toNFKC(X) == toNFKD(X)
    #

    print(f"Character by character test, all normalization forms\n{'-' * 70}")

    s = f = 0
    for x in chars:
        lst = []
        lst.append(NFC(x))
        lst.append(NFD(x))
        lst.append(NFKC(x))
        lst.append(NFKD(x))

        if lst.count(x) == len(lst):
            s += 1
        else:
            f += 1
            print(f"Failed for U+{ord(x):04X}")

    r = s + f
    if f:
        print(f"FAIL ({r:,} items, {f:,} failures)\n")
    else:
        print(f"OK ({r:,} items)\n")
        counter += 1


    uax = f"UAX #15, version {UNICODE_VERSION}."
    if counter == 5:
        print(f".. Implementation conforms to {uax}")
    else:
        print(f".. Implementation does not conform to {uax}")

    print(f".. {time.perf_counter() - start_time:.3f} seconds")


if __name__ == "__main__":
    main()
