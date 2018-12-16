#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MOFCR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `MMC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMCR {
    #[doc = "Standard Message Object"]
    VALUE1,
    #[doc = "Receive FIFO Base Object"]
    VALUE2,
    #[doc = "Transmit FIFO Base Object"]
    VALUE3,
    #[doc = "Transmit FIFO Slave Object"]
    VALUE4,
    #[doc = "Gateway Source Object"]
    VALUE5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MMCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MMCR::VALUE1 => 0,
            MMCR::VALUE2 => 1,
            MMCR::VALUE3 => 2,
            MMCR::VALUE4 => 3,
            MMCR::VALUE5 => 4,
            MMCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MMCR {
        match value {
            0 => MMCR::VALUE1,
            1 => MMCR::VALUE2,
            2 => MMCR::VALUE3,
            3 => MMCR::VALUE4,
            4 => MMCR::VALUE5,
            i => MMCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == MMCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == MMCR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == MMCR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == MMCR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == MMCR::VALUE5
    }
}
#[doc = "Possible values of the field `RXTOE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTOER {
    #[doc = "Message does not take part in receive time-out check"]
    VALUE1,
    #[doc = "Message takes part in receive time-out check"]
    VALUE2,
}
impl RXTOER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RXTOER::VALUE1 => false,
            RXTOER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXTOER {
        match value {
            false => RXTOER::VALUE1,
            true => RXTOER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXTOER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RXTOER::VALUE2
    }
}
#[doc = "Possible values of the field `GDFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GDFSR {
    #[doc = "TXRQ is unchanged in the destination object."]
    VALUE1,
    #[doc = "TXRQ is set in the gateway destination object after the internal transfer from the gateway source to the gateway destination object."]
    VALUE2,
}
impl GDFSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            GDFSR::VALUE1 => false,
            GDFSR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GDFSR {
        match value {
            false => GDFSR::VALUE1,
            true => GDFSR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == GDFSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == GDFSR::VALUE2
    }
}
#[doc = "Possible values of the field `IDC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDCR {
    #[doc = "The identifier of the gateway source object is not copied."]
    VALUE1,
    #[doc = "The identifier of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    VALUE2,
}
impl IDCR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            IDCR::VALUE1 => false,
            IDCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDCR {
        match value {
            false => IDCR::VALUE1,
            true => IDCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == IDCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == IDCR::VALUE2
    }
}
#[doc = "Possible values of the field `DLCC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DLCCR {
    #[doc = "Data length code is not copied."]
    VALUE1,
    #[doc = "Data length code of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    VALUE2,
}
impl DLCCR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DLCCR::VALUE1 => false,
            DLCCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DLCCR {
        match value {
            false => DLCCR::VALUE1,
            true => DLCCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DLCCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DLCCR::VALUE2
    }
}
#[doc = "Possible values of the field `DATC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATCR {
    #[doc = "Data fields are not copied."]
    VALUE1,
    #[doc = "Data fields in registers MODATALn and MODATAHn of the gateway source object (after storing the received frame in the source) are copied to the gateway destination."]
    VALUE2,
}
impl DATCR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DATCR::VALUE1 => false,
            DATCR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DATCR {
        match value {
            false => DATCR::VALUE1,
            true => DATCR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DATCR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DATCR::VALUE2
    }
}
#[doc = "Possible values of the field `RXIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIER {
    #[doc = "Message receive interrupt is disabled."]
    VALUE1,
    #[doc = "Message receive interrupt is enabled."]
    VALUE2,
}
impl RXIER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RXIER::VALUE1 => false,
            RXIER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXIER {
        match value {
            false => RXIER::VALUE1,
            true => RXIER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RXIER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RXIER::VALUE2
    }
}
#[doc = "Possible values of the field `TXIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXIER {
    #[doc = "Message transmit interrupt is disabled."]
    VALUE1,
    #[doc = "Message transmit interrupt is enabled."]
    VALUE2,
}
impl TXIER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TXIER::VALUE1 => false,
            TXIER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXIER {
        match value {
            false => TXIER::VALUE1,
            true => TXIER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == TXIER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == TXIER::VALUE2
    }
}
#[doc = "Possible values of the field `OVIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVIER {
    #[doc = "FIFO full interrupt is disabled."]
    VALUE1,
    #[doc = "FIFO full interrupt is enabled."]
    VALUE2,
}
impl OVIER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            OVIER::VALUE1 => false,
            OVIER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVIER {
        match value {
            false => OVIER::VALUE1,
            true => OVIER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == OVIER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == OVIER::VALUE2
    }
}
#[doc = "Possible values of the field `FRREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRRENR {
    #[doc = "TXRQ of message object n is set on reception of a matching Remote Frame."]
    VALUE1,
    #[doc = "TXRQ of the message object referenced by the pointer CUR is set on reception of a matching Remote Frame."]
    VALUE2,
}
impl FRRENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            FRRENR::VALUE1 => false,
            FRRENR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRRENR {
        match value {
            false => FRRENR::VALUE1,
            true => FRRENR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FRRENR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FRRENR::VALUE2
    }
}
#[doc = "Possible values of the field `RMM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMMR {
    #[doc = "Remote monitoring is disabled: Identifier, IDE bit, and DLC of message object n remain unchanged upon the reception of a matching Remote Frame."]
    VALUE1,
    #[doc = "Remote monitoring is enabled: Identifier, IDE bit, and DLC of a matching Remote Frame are copied to transmit object n in order to monitor incoming Remote Frames."]
    VALUE2,
}
impl RMMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RMMR::VALUE1 => false,
            RMMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RMMR {
        match value {
            false => RMMR::VALUE1,
            true => RMMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RMMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RMMR::VALUE2
    }
}
#[doc = r" Value of the field"]
pub struct SDTR {
    bits: bool,
}
impl SDTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct STTR {
    bits: bool,
}
impl STTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DLCR {
    bits: u8,
}
impl DLCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `MMC`"]
pub enum MMCW {
    #[doc = "Standard Message Object"]
    VALUE1,
    #[doc = "Receive FIFO Base Object"]
    VALUE2,
    #[doc = "Transmit FIFO Base Object"]
    VALUE3,
    #[doc = "Transmit FIFO Slave Object"]
    VALUE4,
    #[doc = "Gateway Source Object"]
    VALUE5,
}
impl MMCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MMCW::VALUE1 => 0,
            MMCW::VALUE2 => 1,
            MMCW::VALUE3 => 2,
            MMCW::VALUE4 => 3,
            MMCW::VALUE5 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MMCW<'a> {
    w: &'a mut W,
}
impl<'a> _MMCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MMCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Standard Message Object"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(MMCW::VALUE1)
    }
    #[doc = "Receive FIFO Base Object"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(MMCW::VALUE2)
    }
    #[doc = "Transmit FIFO Base Object"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(MMCW::VALUE3)
    }
    #[doc = "Transmit FIFO Slave Object"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(MMCW::VALUE4)
    }
    #[doc = "Gateway Source Object"]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(MMCW::VALUE5)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXTOE`"]
pub enum RXTOEW {
    #[doc = "Message does not take part in receive time-out check"]
    VALUE1,
    #[doc = "Message takes part in receive time-out check"]
    VALUE2,
}
impl RXTOEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXTOEW::VALUE1 => false,
            RXTOEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXTOEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXTOEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXTOEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Message does not take part in receive time-out check"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXTOEW::VALUE1)
    }
    #[doc = "Message takes part in receive time-out check"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXTOEW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GDFS`"]
pub enum GDFSW {
    #[doc = "TXRQ is unchanged in the destination object."]
    VALUE1,
    #[doc = "TXRQ is set in the gateway destination object after the internal transfer from the gateway source to the gateway destination object."]
    VALUE2,
}
impl GDFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GDFSW::VALUE1 => false,
            GDFSW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GDFSW<'a> {
    w: &'a mut W,
}
impl<'a> _GDFSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GDFSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TXRQ is unchanged in the destination object."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(GDFSW::VALUE1)
    }
    #[doc = "TXRQ is set in the gateway destination object after the internal transfer from the gateway source to the gateway destination object."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(GDFSW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IDC`"]
pub enum IDCW {
    #[doc = "The identifier of the gateway source object is not copied."]
    VALUE1,
    #[doc = "The identifier of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    VALUE2,
}
impl IDCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDCW::VALUE1 => false,
            IDCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDCW<'a> {
    w: &'a mut W,
}
impl<'a> _IDCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The identifier of the gateway source object is not copied."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(IDCW::VALUE1)
    }
    #[doc = "The identifier of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(IDCW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DLCC`"]
pub enum DLCCW {
    #[doc = "Data length code is not copied."]
    VALUE1,
    #[doc = "Data length code of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    VALUE2,
}
impl DLCCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DLCCW::VALUE1 => false,
            DLCCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DLCCW<'a> {
    w: &'a mut W,
}
impl<'a> _DLCCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DLCCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data length code is not copied."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DLCCW::VALUE1)
    }
    #[doc = "Data length code of the gateway source object (after storing the received frame in the source) is copied to the gateway destination object."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DLCCW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DATC`"]
pub enum DATCW {
    #[doc = "Data fields are not copied."]
    VALUE1,
    #[doc = "Data fields in registers MODATALn and MODATAHn of the gateway source object (after storing the received frame in the source) are copied to the gateway destination."]
    VALUE2,
}
impl DATCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DATCW::VALUE1 => false,
            DATCW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATCW<'a> {
    w: &'a mut W,
}
impl<'a> _DATCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data fields are not copied."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DATCW::VALUE1)
    }
    #[doc = "Data fields in registers MODATALn and MODATAHn of the gateway source object (after storing the received frame in the source) are copied to the gateway destination."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DATCW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXIE`"]
pub enum RXIEW {
    #[doc = "Message receive interrupt is disabled."]
    VALUE1,
    #[doc = "Message receive interrupt is enabled."]
    VALUE2,
}
impl RXIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXIEW::VALUE1 => false,
            RXIEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Message receive interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXIEW::VALUE1)
    }
    #[doc = "Message receive interrupt is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RXIEW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXIE`"]
pub enum TXIEW {
    #[doc = "Message transmit interrupt is disabled."]
    VALUE1,
    #[doc = "Message transmit interrupt is enabled."]
    VALUE2,
}
impl TXIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXIEW::VALUE1 => false,
            TXIEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Message transmit interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXIEW::VALUE1)
    }
    #[doc = "Message transmit interrupt is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(TXIEW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OVIE`"]
pub enum OVIEW {
    #[doc = "FIFO full interrupt is disabled."]
    VALUE1,
    #[doc = "FIFO full interrupt is enabled."]
    VALUE2,
}
impl OVIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVIEW::VALUE1 => false,
            OVIEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVIEW<'a> {
    w: &'a mut W,
}
impl<'a> _OVIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FIFO full interrupt is disabled."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(OVIEW::VALUE1)
    }
    #[doc = "FIFO full interrupt is enabled."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(OVIEW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FRREN`"]
pub enum FRRENW {
    #[doc = "TXRQ of message object n is set on reception of a matching Remote Frame."]
    VALUE1,
    #[doc = "TXRQ of the message object referenced by the pointer CUR is set on reception of a matching Remote Frame."]
    VALUE2,
}
impl FRRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRRENW::VALUE1 => false,
            FRRENW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRRENW<'a> {
    w: &'a mut W,
}
impl<'a> _FRRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TXRQ of message object n is set on reception of a matching Remote Frame."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FRRENW::VALUE1)
    }
    #[doc = "TXRQ of the message object referenced by the pointer CUR is set on reception of a matching Remote Frame."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FRRENW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RMM`"]
pub enum RMMW {
    #[doc = "Remote monitoring is disabled: Identifier, IDE bit, and DLC of message object n remain unchanged upon the reception of a matching Remote Frame."]
    VALUE1,
    #[doc = "Remote monitoring is enabled: Identifier, IDE bit, and DLC of a matching Remote Frame are copied to transmit object n in order to monitor incoming Remote Frames."]
    VALUE2,
}
impl RMMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RMMW::VALUE1 => false,
            RMMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RMMW<'a> {
    w: &'a mut W,
}
impl<'a> _RMMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RMMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Remote monitoring is disabled: Identifier, IDE bit, and DLC of message object n remain unchanged upon the reception of a matching Remote Frame."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(RMMW::VALUE1)
    }
    #[doc = "Remote monitoring is enabled: Identifier, IDE bit, and DLC of a matching Remote Frame are copied to transmit object n in order to monitor incoming Remote Frames."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(RMMW::VALUE2)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SDTW<'a> {
    w: &'a mut W,
}
impl<'a> _SDTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STTW<'a> {
    w: &'a mut W,
}
impl<'a> _STTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLCW<'a> {
    w: &'a mut W,
}
impl<'a> _DLCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Message Mode Control"]
    #[inline]
    pub fn mmc(&self) -> MMCR {
        MMCR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Receive Time-Out Enable"]
    #[inline]
    pub fn rxtoe(&self) -> RXTOER {
        RXTOER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Gateway Data Frame Send"]
    #[inline]
    pub fn gdfs(&self) -> GDFSR {
        GDFSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Identifier Copy"]
    #[inline]
    pub fn idc(&self) -> IDCR {
        IDCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Data Length Code Copy"]
    #[inline]
    pub fn dlcc(&self) -> DLCCR {
        DLCCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Data Copy"]
    #[inline]
    pub fn datc(&self) -> DATCR {
        DATCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Receive Interrupt Enable"]
    #[inline]
    pub fn rxie(&self) -> RXIER {
        RXIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Transmit Interrupt Enable"]
    #[inline]
    pub fn txie(&self) -> TXIER {
        TXIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Overflow Interrupt Enable"]
    #[inline]
    pub fn ovie(&self) -> OVIER {
        OVIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Foreign Remote Request Enable"]
    #[inline]
    pub fn frren(&self) -> FRRENR {
        FRRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Transmit Object Remote Monitoring"]
    #[inline]
    pub fn rmm(&self) -> RMMR {
        RMMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Single Data Transfer"]
    #[inline]
    pub fn sdt(&self) -> SDTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SDTR { bits }
    }
    #[doc = "Bit 23 - Single Transmit Trial"]
    #[inline]
    pub fn stt(&self) -> STTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STTR { bits }
    }
    #[doc = "Bits 24:27 - Data Length Code"]
    #[inline]
    pub fn dlc(&self) -> DLCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLCR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Message Mode Control"]
    #[inline]
    pub fn mmc(&mut self) -> _MMCW {
        _MMCW { w: self }
    }
    #[doc = "Bit 4 - Receive Time-Out Enable"]
    #[inline]
    pub fn rxtoe(&mut self) -> _RXTOEW {
        _RXTOEW { w: self }
    }
    #[doc = "Bit 8 - Gateway Data Frame Send"]
    #[inline]
    pub fn gdfs(&mut self) -> _GDFSW {
        _GDFSW { w: self }
    }
    #[doc = "Bit 9 - Identifier Copy"]
    #[inline]
    pub fn idc(&mut self) -> _IDCW {
        _IDCW { w: self }
    }
    #[doc = "Bit 10 - Data Length Code Copy"]
    #[inline]
    pub fn dlcc(&mut self) -> _DLCCW {
        _DLCCW { w: self }
    }
    #[doc = "Bit 11 - Data Copy"]
    #[inline]
    pub fn datc(&mut self) -> _DATCW {
        _DATCW { w: self }
    }
    #[doc = "Bit 16 - Receive Interrupt Enable"]
    #[inline]
    pub fn rxie(&mut self) -> _RXIEW {
        _RXIEW { w: self }
    }
    #[doc = "Bit 17 - Transmit Interrupt Enable"]
    #[inline]
    pub fn txie(&mut self) -> _TXIEW {
        _TXIEW { w: self }
    }
    #[doc = "Bit 18 - Overflow Interrupt Enable"]
    #[inline]
    pub fn ovie(&mut self) -> _OVIEW {
        _OVIEW { w: self }
    }
    #[doc = "Bit 20 - Foreign Remote Request Enable"]
    #[inline]
    pub fn frren(&mut self) -> _FRRENW {
        _FRRENW { w: self }
    }
    #[doc = "Bit 21 - Transmit Object Remote Monitoring"]
    #[inline]
    pub fn rmm(&mut self) -> _RMMW {
        _RMMW { w: self }
    }
    #[doc = "Bit 22 - Single Data Transfer"]
    #[inline]
    pub fn sdt(&mut self) -> _SDTW {
        _SDTW { w: self }
    }
    #[doc = "Bit 23 - Single Transmit Trial"]
    #[inline]
    pub fn stt(&mut self) -> _STTW {
        _STTW { w: self }
    }
    #[doc = "Bits 24:27 - Data Length Code"]
    #[inline]
    pub fn dlc(&mut self) -> _DLCW {
        _DLCW { w: self }
    }
}
