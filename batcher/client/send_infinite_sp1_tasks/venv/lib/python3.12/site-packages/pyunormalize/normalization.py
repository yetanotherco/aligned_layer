"""Unicode normalization algorithm."""

from pyunormalize.unicode import (
    _DECOMP,       # character decomposition mappings
    _CCC,          # non-zero canonical combining class values
    _COMP_EXCL,    # characters excluded from composition
    _QC_PROP_VAL,  # characters listed for several Quick_Check property values
)

# Full canonical decompositions,
# not including Hangul syllables
_CDECOMP = {}

# Full compatibility decompositions,
# not including Hangul syllables
_KDECOMP = {}

# Precomposed characters (canonical composites),
# not including Hangul syllables
_PRECOMP = {}

# NOTE: Because the Hangul compositions and decompositions are algorithmic,
# the corresponding operations are done in code rather than by storing
# the data in the general-purpose tables.


def _full_decomposition(decomp_dict):
    # A full decomposition of a character sequence results from decomposing
    # each of the characters in the sequence until no characters can be further
    # decomposed.

    for key in decomp_dict:
        tmp = []
        decomposition = [key]
        while True:
            for x in decomposition:
                if x in decomp_dict:
                    tmp.extend(decomp_dict[x])
                else:
                    tmp.append(x)
            if tmp == decomposition:
                decomp_dict[key] = decomposition  # done
                break
            decomposition = tmp
            tmp = []


def _make_dictionaries():
    for key, val in _DECOMP.items():
        if isinstance(val[0], int):
            # assert len(val) in (1, 2)
            _CDECOMP[key] = _KDECOMP[key] = val
            if len(val) == 1 or val[0] in _CCC:  # singleton or non-starter
                continue
            _PRECOMP[tuple(val)] = key
        else:
            _KDECOMP[key] = val[1:]

    # Make full canonical decomposition
    _full_decomposition(_CDECOMP)

    # Make full compatibility decomposition
    _full_decomposition(_KDECOMP)


_make_dictionaries()

del _DECOMP


#
# Public interface
#

# Code points listed for NFD_Quick_Check=No
_NFD_NO = _QC_PROP_VAL["nfd_no"]

def NFD(unistr):
    """Return the canonical equivalent "decomposed" form of the
    original Unicode string *unistr*. That is, transform the Unicode
    string into the Unicode "normalization form D": composite
    characters are replaced by canonically equivalent character
    sequences, in canonical order; compatibility characters are
    unaffected.

    Examples:

    >>> unistr = "élève"
    >>> nfd = NFD(unistr)
    >>> unistr, nfd
    ('élève', 'élève')
    >>> nfd == unistr
    False
    >>> " ".join(f"{ord(x):04X}" for x in unistr)
    '00E9 006C 00E8 0076 0065'
    >>> " ".join(f"{ord(x):04X}" for x in nfd)
    '0065 0301 006C 0065 0300 0076 0065'

    >>> unistr = "한국"
    >>> nfd = NFD(unistr)
    >>> unistr, nfd
    ('한국', '한국')
    >>> " ".join(f"{ord(x):04X}" for x in unistr)
    'D55C AD6D'
    >>> " ".join(f"{ord(x):04X}" for x in nfd)
    '1112 1161 11AB 1100 116E 11A8'

    >>> NFD("ﬃ")
    'ﬃ'

    """
    # Quick check for NFD
    prev_ccc = curr_ccc = 0
    for u in unistr:
        u = ord(u)
        if u in _NFD_NO:  # u in _CDECOMP or 0xAC00 <= u <= 0xD7A3
            break
        if u in _CCC:
            curr_ccc = _CCC[u]
            if curr_ccc < prev_ccc:
                break
        prev_ccc = curr_ccc
        curr_ccc = 0
    else:
        return unistr  # source string is in NFD

    # Normalize to NFD
    # _reorder(_decompose(unistr))
    result = map(chr, _reorder(_decompose(unistr)))
    return "".join(result)


# Code points listed for NFC_Quick_Check=No or NFC_Quick_Check=Maybe
_NFC_NO_OR_MAYBE = _QC_PROP_VAL["nfc_no"] | _QC_PROP_VAL["nfc_maybe"]

def NFC(unistr):
    """Return the canonical equivalent "composed" form of the original
    Unicode string *unistr*. That is, transform the Unicode string into
    the Unicode "normalization form C": character sequences are
    replaced by canonically equivalent composites, where possible;
    compatibility characters are unaffected.

    Examples:

    >>> unistr = "élève"
    >>> nfc = NFC(unistr)
    >>> unistr, nfc
    ('élève', 'élève')
    >>> nfc == unistr
    False
    >>> " ".join(f"{ord(x):04X}" for x in unistr)
    '0065 0301 006C 0065 0300 0076 0065'
    >>> " ".join(f"{ord(x):04X}" for x in nfc)
    '00E9 006C 00E8 0076 0065'

    >>> unistr = "한국"
    >>> nfc = NFC(unistr)
    >>> unistr, nfc
    ('한국', '한국')
    >>> " ".join(f"{ord(x):04X}" for x in unistr)
    '1112 1161 11AB 1100 116E 11A8'
    >>> " ".join(f"{ord(x):04X}" for x in nfc)
    'D55C AD6D'

    >>> NFC("ﬃ")
    'ﬃ'

    """
    # Quick check for NFC
    prev_ccc = curr_ccc = 0
    for u in unistr:
        u = ord(u)
        if u in _NFC_NO_OR_MAYBE:
            break
        if u in _CCC:
            curr_ccc = _CCC[u]
            if curr_ccc < prev_ccc:
                break
        prev_ccc = curr_ccc
        curr_ccc = 0
    else:
        return unistr  # source string is in NFC

    # Normalize to NFC
    # _compose(_reorder(_decompose(unistr)))
    result = map(chr, _compose([*map(ord, NFD(unistr))]))
    return "".join(result)


# Code points listed for NFKD_Quick_Check=No
_NFKD_NO = _QC_PROP_VAL["nfkd_no"]

def NFKD(unistr):
    """Return the compatibility equivalent "decomposed" form of the
    original Unicode string *unistr*. That is, transform the Unicode
    string into the Unicode "normalization form KD": composite
    characters are replaced by canonically equivalent character
    sequences, in canonical order; compatibility characters are
    replaced by their nominal counterparts.

    Example:

    >>> NFKD("⑴")
    '(1)'

    """
    # Quick check for NFKD
    prev_ccc = curr_ccc = 0
    for u in unistr:
        u = ord(u)
        if u in _NFKD_NO:  # u in _KDECOMP or 0xAC00 <= u <= 0xD7A3
            break
        if u in _CCC:
            curr_ccc = _CCC[u]
            if curr_ccc < prev_ccc:
                break
        prev_ccc = curr_ccc
        curr_ccc = 0
    else:
        return unistr  # source string is in NFKD

    # Normalize to NFKD
    # _reorder(_decompose(unistr, compatibility=True))
    result = map(chr, _reorder(_decompose(unistr, compatibility=True)))
    return "".join(result)


# Code points listed for NFKC_Quick_Check=No or NFKC_Quick_Check=Maybe
_NFKC_NO_OR_MAYBE = _QC_PROP_VAL["nfkc_no"] | _QC_PROP_VAL["nfkc_maybe"]

def NFKC(unistr):
    """Return the compatibility equivalent "composed" form of the
    original Unicode string *unistr*. That is, transform the Unicode
    string into the Unicode "normalization form KC": character
    sequences are replaced by canonically equivalent composites, where
    possible; compatibility characters are replaced by their nominal
    counterparts.

    Example:

    >>> NFKC("ﬃ")
    'ffi'

    """
    # Quick check for NFKC
    prev_ccc = curr_ccc = 0
    for u in unistr:
        u = ord(u)
        if u in _NFKC_NO_OR_MAYBE:
            break
        if u in _CCC:
            curr_ccc = _CCC[u]
            if curr_ccc < prev_ccc:
                break
        prev_ccc = curr_ccc
        curr_ccc = 0
    else:
        return unistr  # source string is in NFKC

    # Normalize to NFKC
    # _compose(_reorder(_decompose(unistr, compatibility=True)))
    result = map(chr, _compose([*map(ord, NFKD(unistr))]))
    return "".join(result)


# Dictionary for normalization forms dispatch
_normalization_forms = {
    "NFD"  : NFD,
    "NFC"  : NFC,
    "NFKD" : NFKD,
    "NFKC" : NFKC,
}

def normalize(form, unistr):
    """Transform the Unicode string *unistr* into the Unicode
    normalization form *form*. Valid values for *form* are "NFC",
    "NFD", "NFKC", and "NFKD".

    Examples:

    >>> normalize("NFKD", "⑴ ﬃ ²")
    '(1) ffi 2'

    >>> forms = ["NFC", "NFD", "NFKC", "NFKD"]
    >>> [normalize(f, "\u017F\u0307\u0323") for f in forms]
    ['ẛ̣', 'ẛ̣', 'ṩ', 'ṩ']

    """
    return _normalization_forms[form](unistr)


#
# Internals
#

# Hangul syllables for modern Korean
_SB = 0xAC00
_SL = 0xD7A3

# Hangul leading consonants (syllable onsets)
_LB = 0x1100
_LL = 0x1112

# Hangul vowels (syllable nucleuses)
_VB = 0x1161
_VL = 0x1175

# Hangul trailing consonants (syllable codas)
_TB = 0x11A8
_TL = 0x11C2

_TS = 0x11A7  # _TB - 1

_VCOUNT = 21  # _VL - _VB + 1
_TCOUNT = 28  # _TL - _TS + 1


def _decompose(unistr, *, compatibility=False):
    # Computes the full decomposition of the Unicode string. The type of full
    # decomposition chosen depends on which Unicode normalization form is
    # involved. For NFC or NFD, one does a full canonical decomposition. For
    # NFKC or NFKD, one does a full compatibility decomposition.

    result = []
    decomp = _KDECOMP if compatibility else _CDECOMP

    for u in unistr:
        u = ord(u)
        if u in decomp:
            result.extend(decomp[u])
        elif _SB <= u <= _SL:
            result.extend(_decompose_hangul_syllable(u))
        else:
            result.append(u)

    return result


def _decompose_hangul_syllable(cp):
    # Hangul syllable decomposition algorithm. Arithmetically derives the full
    # canonical decomposition of a precomposed Hangul syllable.

    sindex = cp - _SB
    tindex = sindex % _TCOUNT
    q = (sindex - tindex) // _TCOUNT
    V = _VB + (q  % _VCOUNT)
    L = _LB + (q // _VCOUNT)

    if tindex:
        # LVT syllable
        return (L, V, _TS + tindex)

    # LV syllable
    return (L, V)


def _reorder(elems):
    # Canonical ordering algorithm. Once a string has been fully decomposed,
    # any sequences of combining marks that it contains are put into a
    # well-defined order. Only the subset of combining marks which have
    # non-zero Canonical_Combining_Class property values are subject to
    # potential reordering by the canonical ordering algorithm. Both the
    # composed and decomposed normalization forms impose a canonical ordering
    # on the code point sequence, which is necessary for the normal forms to be
    # unique.

    for n, elem in enumerate(elems):
        if elem not in _CCC:  # character is a starter
            continue
        m = n
        while n < len(elems) and elems[n] in _CCC:
            n += 1
        if n == m + 1:
            continue
        chunk = elems[m:n]
        ccc_values = [_CCC[x] for x in chunk]
        if ccc_values != sorted(ccc_values):
            elems[m:n] = \
                [x for *_, x in sorted(zip(ccc_values, range(m, n), chunk))]

    return elems


def _compose(elems):
    # Canonical composition algorithm. That process transforms the fully
    # decomposed and canonically ordered string into its most fully composed
    # but still canonically equivalent sequence.

    for i, x in enumerate(elems):
        if x is None or x in _CCC:
            continue

        last_cc = False  # _CCC[x] = 0
        blocked = False
        for j, y in enumerate(elems[i + 1 :], i + 1):
            if y in _CCC:
                last_cc = True  # _CCC[y] > 0
            else:
                blocked = True

            if blocked and last_cc:
                continue

            prev = elems[j - 1]

            if prev is None or prev not in _CCC or _CCC[prev] < _CCC[y]:
                pair = (x, y)  # x: last starter preceding y
                if pair in _PRECOMP:
                    precomp = _PRECOMP[pair]
                else:
                    precomp = _compose_hangul_syllable(*pair)

                if precomp is None or precomp in _COMP_EXCL:
                    if blocked:
                        break
                else:
                    elems[i] = x = precomp
                    elems[j] = None
                    if blocked:
                        blocked = False
                    else:
                        last_cc = False

    return [*filter(None, elems)]


def _compose_hangul_syllable(x, y):
    # Hangul syllable composition algorithm. Arithmetically derives the
    # mapping of a canonically decomposed sequence of Hangul jamo characters
    # to an equivalent precomposed Hangul syllable.

    if _LB <= x <= _LL and _VB <= y <= _VL:
        # Compose a leading consonant and a vowel into an LV syllable
        return _SB + (((x - _LB) * _VCOUNT) + y - _VB) * _TCOUNT

    if _SB <= x <= _SL and not (x - _SB) % _TCOUNT and _TB <= y <= _TL:
        # Compose an LV syllable and a trailing consonant into an LVT syllable
        return x + y - _TS

    return None


if __name__ == "__main__":
    import doctest
    doctest.testmod()
