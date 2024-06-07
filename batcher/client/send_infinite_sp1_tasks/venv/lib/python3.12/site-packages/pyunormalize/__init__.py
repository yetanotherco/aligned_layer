"""A pure Python implementation of the Unicode normalization algorithm
independent from the Python core Unicode database. This package supports
version 15.1 of the Unicode standard (released in September 2023).
It has been thoroughly tested against the Unicode test file found
at https://www.unicode.org/Public/15.1.0/ucd/NormalizationTest.txt

To get the version of the Unicode character database currently used:

    >>> from pyunormalize import UCD_VERSION
    >>> UCD_VERSION
    '15.1.0'

For the formal specification of the Unicode normalization algorithm,
see Section 3.11, Normalization Forms, in the Unicode core specification.
"""

import sys
if sys.version_info < (3, 6):
    raise SystemExit(f"\n{__package__.title()} requires Python 3.6 or later.")
del sys

__all__ = [
    "NFC",
    "NFD",
    "NFKC",
    "NFKD",
    "normalize",
    "UCD_VERSION",
    "UNICODE_VERSION",
    "__version__",
]

# Unicode standard used to process the data
# Release date: September 2023
UNICODE_VERSION = UCD_VERSION = "15.1.0"


from pyunormalize import _version
__version__ = _version.__version__
del _version

from pyunormalize.unicode import UNICODE_VERSION as _UNICODE
if _UNICODE != UNICODE_VERSION:
    raise SystemExit(f"\nWrong Unicode version number in {unicode.__name__}")

from pyunormalize.normalization import *
