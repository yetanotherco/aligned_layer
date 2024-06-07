"""Unit tests for pyunormalize."""

import unittest

from pyunormalize.normalization import _decompose, _reorder, _compose
from pyunormalize import (
    NFC,
    NFD,
    NFKC,
    NFKD,
    normalize,
    UNICODE_VERSION as _UNICODE_VERSION,
)

UNICODE_VERSION = "15.1.0"


class Misc(unittest.TestCase):

    def test_UNICODE_VERSION(self):
        self.assertTrue(_UNICODE_VERSION == UNICODE_VERSION)

    def test_normalize(self):
        # Characters whose normalization forms
        # under NFC, NFD, NFKC, and NFKD are all different:
        #   ϓ   U+03D3 GREEK UPSILON WITH ACUTE AND HOOK SYMBOL
        #   ϔ   U+03D4 GREEK UPSILON WITH DIAERESIS AND HOOK SYMBOL
        #   ẛ   U+1E9B LATIN SMALL LETTER LONG S WITH DOT ABOVE
        for s in ["\u03D3", "\u03D4", "\u1E9B"]:
            self.assertTrue(
                normalize("NFC", s) == NFC(s)
            )
            self.assertTrue(
                normalize("NFD", s) == NFD(s)
            )
            self.assertTrue(
                normalize("NFKC", s) == NFKC(s)
            )
            self.assertTrue(
                normalize("NFKD", s) == NFKD(s)
            )

    def test_internals(self):

        self.assertEqual(
            _decompose("\u00C0"),
            [0x0041, 0x0300]
        )

        self.assertEqual(
            _decompose("\u00BE", compatibility=True),
            [0x0033, 0x2044, 0x0034]
        )

        self.assertEqual(
            _decompose("힡"),
            [0x1112, 0x1175, 0x11C0]
        )

        self.assertEqual(
            _reorder([0x017F, 0x0307, 0x0323]),
            [0x017F, 0x0323, 0x0307]
        )

        s = "a\u0328\u0302\u0301"  # a + ogonek + circumflex + acute
        self.assertEqual(
            _decompose(s),
            [0x0061, 0x0328, 0x0302, 0x0301]
        )
        self.assertEqual(
            _reorder([0x0061, 0x0328, 0x0302, 0x0301]),
            [0x0061, 0x0328, 0x0302, 0x0301]
        )
        self.assertEqual(
            _compose([0x0061, 0x0328, 0x0302, 0x0301]),
            [0x0105, 0x0302, 0x0301]
        )

        s = "\u0105\u0302\u0301"  # a-ogonek + circumflex + acute
        self.assertEqual(
            _compose(_decompose(_reorder(s))),
            [0x0105, 0x0302, 0x0301]
        )

        s = "\u0105\u0301\u0302"  # a-ogonek + acute + circumflex
        self.assertEqual(
            _decompose(s),
            [0x0061, 0x0328, 0x0301, 0x0302]
        )
        self.assertEqual(
            _reorder([0x0061, 0x0328, 0x0301, 0x0302]),
            [0x0061, 0x0328, 0x0301, 0x0302]
        )
        self.assertEqual(
            _compose([0x0061, 0x0328, 0x0301, 0x0302]),
            [0x0105, 0x0301, 0x0302]
        )

        self.assertEqual(
            _compose(_decompose(_reorder(s))),
            [0x0105, 0x0301, 0x0302]
        )

        # At https://www.unicode.org/versions/Unicode15.0.0/UnicodeStandard-15.0.pdf,
        # p. 140:  "The replacement of the Starter L in R2 requires continuing
        # to check the succeeding characters until the character at that
        # position is no longer part of any Non-blocked Pair that can be
        # replaced by a Primary Composite. For example, consider the following
        # hypothetical coded character sequence: <U+007A z, U+0335 short stroke
        # overlay, U+0327 cedilla, U+0324 diaeresis below, U+0301 acute>. None
        # of the first three combining marks forms a Primary Composite with
        # the letter z. However, the fourth combining mark in the sequence,
        # acute, does form a Primary Composite with z, and it is not Blocked
        # from the z. Therefore, R2 mandates the replacement of the sequence
        # <U+007A z, ... U+0301 acute> with <U+017A z-acute, ...>, even though
        # there are three other combining marks intervening in the sequence."
        items = [0x007A, 0x0335, 0x0327, 0x0324, 0x0301]
        self.assertEqual(
            _compose(items),
            [0x017A, 0x0335, 0x0327, 0x0324]
        )


if __name__ == "__main__":
    unittest.main()
