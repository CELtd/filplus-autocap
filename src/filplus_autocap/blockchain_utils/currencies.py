from decimal import Decimal, getcontext, ROUND_DOWN
from typing import Union

# Set high precision and constants
getcontext().prec = 50
FEMTOFIL = Decimal("1e-15")
FEMTO_DATACAP = Decimal("1e-15")

# Function to ensure precision
def quantize_to_unit(amount: Decimal, unit: Decimal) -> Decimal:
    return amount.quantize(unit, rounding=ROUND_DOWN)

class FIL:
    def __init__(self, fil: Union[float, str, Decimal, "FIL"]):
        if isinstance(fil, FIL):
            self.amount = fil.amount
        else:
            self.amount = quantize_to_unit(Decimal(fil), FEMTOFIL)

    def __add__(self, other):
        return FIL(quantize_to_unit(self.amount + FIL(other).amount, FEMTOFIL))

    def __radd__(self, other):
        if other == 0:
            return self
        return self.__add__(FIL(other))

    def __sub__(self, other):
        return FIL(quantize_to_unit(self.amount - FIL(other).amount, FEMTOFIL))

    def __rsub__(self, other):
        return FIL(quantize_to_unit(Decimal(other) - self.amount, FEMTOFIL))

    def __neg__(self):
        return FIL(-self.amount)

    def __abs__(self):
        return FIL(abs(self.amount))

    def __mul__(self, other):
        if isinstance(other, FIL):
            return FIL(quantize_to_unit(self.amount * other.amount, FEMTOFIL))
        return FIL(quantize_to_unit(self.amount * Decimal(other), FEMTOFIL))

    def __rmul__(self, other):
        return self.__mul__(other)

    def __truediv__(self, other):
        if isinstance(other, FIL):
            return quantize_to_unit(self.amount / other.amount, FEMTOFIL)
        return quantize_to_unit(self.amount / Decimal(other), FEMTOFIL)

    def __eq__(self, other):
        return self.amount == FIL(other).amount

    def __lt__(self, other):
        return self.amount < FIL(other).amount

    def __le__(self, other):
        return self.amount <= FIL(other).amount

    def __gt__(self, other):
        return self.amount > FIL(other).amount

    def __ge__(self, other):
        return self.amount >= FIL(other).amount

    def to_decimal(self):
        return self.amount

    def to_float(self):
        return float(self.amount)

    def __repr__(self):
        return f"{self.amount:.18f} FIL"

    def __hash__(self):
        return hash(self.amount)

    def __format__(self, format_spec):
        return format(self.amount, format_spec)


class DAT:
    def __init__(self, datacap: Union[float, str, Decimal, "DAT"]):
        if isinstance(datacap, DAT):
            self.amount = datacap.amount
        else:
            self.amount = quantize_to_unit(Decimal(datacap), FEMTO_DATACAP)

    def __add__(self, other):
        return DAT(quantize_to_unit(self.amount + DAT(other).amount, FEMTO_DATACAP))

    def __radd__(self, other):
        if other == 0:
            return self
        return self.__add__(DAT(other))

    def __sub__(self, other):
        return DAT(quantize_to_unit(self.amount - DAT(other).amount, FEMTO_DATACAP))

    def __rsub__(self, other):
        return DAT(quantize_to_unit(Decimal(other) - self.amount, FEMTO_DATACAP))

    def __neg__(self):
        return DAT(-self.amount)

    def __abs__(self):
        return DAT(abs(self.amount))

    def __mul__(self, other):
        if isinstance(other, DAT):
            return DAT(quantize_to_unit(self.amount * other.amount, FEMTO_DATACAP))
        return DAT(quantize_to_unit(self.amount * Decimal(other), FEMTO_DATACAP))

    def __rmul__(self, other):
        return self.__mul__(other)

    def __truediv__(self, other):
        if isinstance(other, DAT):
            return quantize_to_unit(self.amount / other.amount, FEMTO_DATACAP)
        return quantize_to_unit(self.amount / Decimal(other), FEMTO_DATACAP)

    def __eq__(self, other):
        return self.amount == DAT(other).amount

    def __lt__(self, other):
        return self.amount < DAT(other).amount

    def __le__(self, other):
        return self.amount <= DAT(other).amount

    def __gt__(self, other):
        return self.amount > DAT(other).amount

    def __ge__(self, other):
        return self.amount >= DAT(other).amount

    def to_decimal(self):
        return self.amount

    def to_float(self):
        return float(self.amount)

    def __repr__(self):
        return f"{self.amount:.18f} DAT"

    def __hash__(self):
        return hash(self.amount)

    def __format__(self, format_spec):
        return format(self.amount, format_spec)
