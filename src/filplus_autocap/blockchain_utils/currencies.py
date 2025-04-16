from decimal import Decimal, getcontext, ROUND_DOWN
from typing import Union

# Set high precision and constants for FEMTOFIL and FEMTO_DATACAP (the smallest unit of FIL and DAT)
getcontext().prec = 50
FEMTOFIL = Decimal("1e-15")  # Smallest unit of FIL
FEMTO_DATACAP = Decimal("1e-15")  # Smallest unit of DATACAP

# Function to ensure that amounts are properly rounded to the unit's precision
def quantize_to_unit(amount: Decimal, unit: Decimal) -> Decimal:
    """
    Quantizes the given amount to the nearest unit, rounding down if necessary.
    
    Args:
        amount (Decimal): The amount to be quantized.
        unit (Decimal): The unit to quantize to (e.g., FEMTOFIL or FEMTO_DATACAP).

    Returns:
        Decimal: The quantized amount.
    """
    return amount.quantize(unit, rounding=ROUND_DOWN)


class FIL:
    """
    A class representing the FIL currency with operations to handle precision and basic arithmetic.
    This class ensures FIL amounts are always in the smallest unit (FEMTOFIL) for precision.

    Attributes:
        amount (Decimal): The FIL amount in femto units (1e-15).
    """
    
    def __init__(self, fil: Union[float, str, Decimal, "FIL"]):
        """
        Initializes a FIL object from different input types (float, str, Decimal, or another FIL object).

        Args:
            fil (Union[float, str, Decimal, "FIL"]): The amount to initialize the FIL object.
        """
        if isinstance(fil, FIL):
            self.amount = fil.amount
        else:
            # Convert input to a Decimal and ensure precision with FEMTOFIL
            self.amount = quantize_to_unit(Decimal(fil), FEMTOFIL)

    def __add__(self, other):
        """Add two FIL amounts together."""
        return FIL(quantize_to_unit(self.amount + FIL(other).amount, FEMTOFIL))

    def __radd__(self, other):
        """Handle reverse addition."""
        if other == 0:
            return self
        return self.__add__(FIL(other))

    def __sub__(self, other):
        """Subtract one FIL amount from another."""
        return FIL(quantize_to_unit(self.amount - FIL(other).amount, FEMTOFIL))

    def __rsub__(self, other):
        """Handle reverse subtraction."""
        return FIL(quantize_to_unit(Decimal(other) - self.amount, FEMTOFIL))

    def __neg__(self):
        """Negate the FIL amount."""
        return FIL(-self.amount)

    def __abs__(self):
        """Return the absolute value of the FIL amount."""
        return FIL(abs(self.amount))

    def __mul__(self, other):
        """Multiply a FIL amount by another FIL or scalar."""
        if isinstance(other, FIL):
            return FIL(quantize_to_unit(self.amount * other.amount, FEMTOFIL))
        return FIL(quantize_to_unit(self.amount * Decimal(other), FEMTOFIL))

    def __rmul__(self, other):
        """Handle reverse multiplication."""
        return self.__mul__(other)

    def __truediv__(self, other):
        """Divide a FIL amount by another FIL or scalar."""
        if isinstance(other, FIL):
            return quantize_to_unit(self.amount / other.amount, FEMTOFIL)
        return quantize_to_unit(self.amount / Decimal(other), FEMTOFIL)

    def __eq__(self, other):
        """Check if two FIL amounts are equal."""
        return self.amount == FIL(other).amount

    def __lt__(self, other):
        """Check if one FIL amount is less than another."""
        return self.amount < FIL(other).amount

    def __le__(self, other):
        """Check if one FIL amount is less than or equal to another."""
        return self.amount <= FIL(other).amount

    def __gt__(self, other):
        """Check if one FIL amount is greater than another."""
        return self.amount > FIL(other).amount

    def __ge__(self, other):
        """Check if one FIL amount is greater than or equal to another."""
        return self.amount >= FIL(other).amount

    def to_decimal(self):
        """Return the FIL amount as a Decimal."""
        return self.amount

    def to_float(self):
        """Return the FIL amount as a float."""
        return float(self.amount)

    def __repr__(self):
        """Return a string representation of the FIL amount, formatted to 18 decimal places."""
        return f"{self.amount:.18f} FIL"

    def __hash__(self):
        """Generate a hash value for the FIL object."""
        return hash(self.amount)

    def __format__(self, format_spec):
        """Format the FIL amount according to the specified format."""
        return format(self.amount, format_spec)


class DAT:
    """
    A class representing the DAT currency with operations to handle precision and basic arithmetic.
    This class ensures DAT amounts are always in the smallest unit (FEMTO_DATACAP) for precision.

    Attributes:
        amount (Decimal): The DAT amount in femto units (1e-15).
    """
    
    def __init__(self, datacap: Union[float, str, Decimal, "DAT"]):
        """
        Initializes a DAT object from different input types (float, str, Decimal, or another DAT object).

        Args:
            datacap (Union[float, str, Decimal, "DAT"]): The amount to initialize the DAT object.
        """
        if isinstance(datacap, DAT):
            self.amount = datacap.amount
        else:
            # Convert input to a Decimal and ensure precision with FEMTO_DATACAP
            self.amount = quantize_to_unit(Decimal(datacap), FEMTO_DATACAP)

    def __add__(self, other):
        """Add two DAT amounts together."""
        return DAT(quantize_to_unit(self.amount + DAT(other).amount, FEMTO_DATACAP))

    def __radd__(self, other):
        """Handle reverse addition."""
        if other == 0:
            return self
        return self.__add__(DAT(other))

    def __sub__(self, other):
        """Subtract one DAT amount from another."""
        return DAT(quantize_to_unit(self.amount - DAT(other).amount, FEMTO_DATACAP))

    def __rsub__(self, other):
        """Handle reverse subtraction."""
        return DAT(quantize_to_unit(Decimal(other) - self.amount, FEMTO_DATACAP))

    def __neg__(self):
        """Negate the DAT amount."""
        return DAT(-self.amount)

    def __abs__(self):
        """Return the absolute value of the DAT amount."""
        return DAT(abs(self.amount))

    def __mul__(self, other):
        """Multiply a DAT amount by another DAT or scalar."""
        if isinstance(other, DAT):
            return DAT(quantize_to_unit(self.amount * other.amount, FEMTO_DATACAP))
        return DAT(quantize_to_unit(self.amount * Decimal(other), FEMTO_DATACAP))

    def __rmul__(self, other):
        """Handle reverse multiplication."""
        return self.__mul__(other)

    def __truediv__(self, other):
        """Divide a DAT amount by another DAT or scalar."""
        if isinstance(other, DAT):
            return quantize_to_unit(self.amount / other.amount, FEMTO_DATACAP)
        return quantize_to_unit(self.amount / Decimal(other), FEMTO_DATACAP)

    def __eq__(self, other):
        """Check if two DAT amounts are equal."""
        return self.amount == DAT(other).amount

    def __lt__(self, other):
        """Check if one DAT amount is less than another."""
        return self.amount < DAT(other).amount

    def __le__(self, other):
        """Check if one DAT amount is less than or equal to another."""
        return self.amount <= DAT(other).amount

    def __gt__(self, other):
        """Check if one DAT amount is greater than another."""
        return self.amount > DAT(other).amount

    def __ge__(self, other):
        """Check if one DAT amount is greater than or equal to another."""
        return self.amount >= DAT(other).amount

    def to_decimal(self):
        """Return the DAT amount as a Decimal."""
        return self.amount

    def to_float(self):
        """Return the DAT amount as a float."""
        return float(self.amount)

    def __repr__(self):
        """Return a string representation of the DAT amount, formatted to 18 decimal places."""
        return f"{self.amount:.18f} DAT"

    def __hash__(self):
        """Generate a hash value for the DAT object."""
        return hash(self.amount)

    def __format__(self, format_spec):
        """Format the DAT amount according to the specified format."""
        return format(self.amount, format_spec)
