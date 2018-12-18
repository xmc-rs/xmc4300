#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCON {
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
#[doc = "Possible values of the field `WSPFLASH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSPFLASHR {
    #[doc = "PFLASH access in one clock cycle"]
    VALUE1,
    #[doc = "PFLASH access in one clock cycle"]
    VALUE2,
    #[doc = "PFLASH access in two clock cycles"]
    VALUE3,
    #[doc = "PFLASH access in three clock cycles"]
    VALUE4,
    #[doc = "PFLASH access in fifteen clock cycles."]
    VALUE5,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WSPFLASHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WSPFLASHR::VALUE1 => 0,
            WSPFLASHR::VALUE2 => 1,
            WSPFLASHR::VALUE3 => 2,
            WSPFLASHR::VALUE4 => 3,
            WSPFLASHR::VALUE5 => 15,
            WSPFLASHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WSPFLASHR {
        match value {
            0 => WSPFLASHR::VALUE1,
            1 => WSPFLASHR::VALUE2,
            2 => WSPFLASHR::VALUE3,
            3 => WSPFLASHR::VALUE4,
            15 => WSPFLASHR::VALUE5,
            i => WSPFLASHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WSPFLASHR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WSPFLASHR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == WSPFLASHR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == WSPFLASHR::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline]
    pub fn is_value5(&self) -> bool {
        *self == WSPFLASHR::VALUE5
    }
}
#[doc = "Possible values of the field `WSECPF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSECPFR {
    #[doc = "No additional wait state for error correction"]
    VALUE1,
    #[doc = "One additional wait state for error correction during read access to Program Flash. If enabled, this wait state is only used for the first transfer of a burst transfer."]
    VALUE2,
}
impl WSECPFR {
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
            WSECPFR::VALUE1 => false,
            WSECPFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WSECPFR {
        match value {
            false => WSECPFR::VALUE1,
            true => WSECPFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == WSECPFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == WSECPFR::VALUE2
    }
}
#[doc = "Possible values of the field `IDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLER {
    #[doc = "Normal/standard Flash read operation"]
    VALUE1,
    #[doc = "Dynamic idle of Program Flash enabled for power saving; static prefetching disabled"]
    VALUE2,
}
impl IDLER {
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
            IDLER::VALUE1 => false,
            IDLER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDLER {
        match value {
            false => IDLER::VALUE1,
            true => IDLER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == IDLER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == IDLER::VALUE2
    }
}
#[doc = "Possible values of the field `ESLDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESLDISR {
    #[doc = "External sleep request signal input is enabled"]
    VALUE1,
    #[doc = "Externally requested Flash sleep is disabled"]
    VALUE2,
}
impl ESLDISR {
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
            ESLDISR::VALUE1 => false,
            ESLDISR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESLDISR {
        match value {
            false => ESLDISR::VALUE1,
            true => ESLDISR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == ESLDISR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == ESLDISR::VALUE2
    }
}
#[doc = "Possible values of the field `SLEEP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPR {
    #[doc = "Normal state or wake-up"]
    VALUE1,
    #[doc = "Flash sleep mode is requested"]
    VALUE2,
}
impl SLEEPR {
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
            SLEEPR::VALUE1 => false,
            SLEEPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLEEPR {
        match value {
            false => SLEEPR::VALUE1,
            true => SLEEPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SLEEPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SLEEPR::VALUE2
    }
}
#[doc = "Possible values of the field `RPA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPAR {
    #[doc = "The Flash-internal read protection is not activated. Bits DCF, DDF are not taken into account. Bits DCF, DDFx can be cleared"]
    VALUE1,
    #[doc = "The Flash-internal read protection is activated. Bits DCF, DDF are enabled and evaluated."]
    VALUE2,
}
impl RPAR {
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
            RPAR::VALUE1 => false,
            RPAR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RPAR {
        match value {
            false => RPAR::VALUE1,
            true => RPAR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RPAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RPAR::VALUE2
    }
}
#[doc = "Possible values of the field `DCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCFR {
    #[doc = "Code fetching from the Flash memory area is allowed."]
    VALUE1,
    #[doc = "Code fetching from the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    VALUE2,
}
impl DCFR {
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
            DCFR::VALUE1 => false,
            DCFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DCFR {
        match value {
            false => DCFR::VALUE1,
            true => DCFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DCFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DCFR::VALUE2
    }
}
#[doc = "Possible values of the field `DDF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDFR {
    #[doc = "Data read access to the Flash memory area is allowed."]
    VALUE1,
    #[doc = "Data read access to the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    VALUE2,
}
impl DDFR {
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
            DDFR::VALUE1 => false,
            DDFR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DDFR {
        match value {
            false => DDFR::VALUE1,
            true => DDFR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DDFR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DDFR::VALUE2
    }
}
#[doc = "Possible values of the field `VOPERM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VOPERMR {
    #[doc = "Interrupt not enabled"]
    VALUE1,
    #[doc = "Flash interrupt because of Verify Error or Operation Error in Flash array (FSI) is enabled"]
    VALUE2,
}
impl VOPERMR {
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
            VOPERMR::VALUE1 => false,
            VOPERMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VOPERMR {
        match value {
            false => VOPERMR::VALUE1,
            true => VOPERMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == VOPERMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == VOPERMR::VALUE2
    }
}
#[doc = "Possible values of the field `SQERM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SQERMR {
    #[doc = "Interrupt not enabled"]
    VALUE1,
    #[doc = "Flash interrupt because of Sequence Error is enabled"]
    VALUE2,
}
impl SQERMR {
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
            SQERMR::VALUE1 => false,
            SQERMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SQERMR {
        match value {
            false => SQERMR::VALUE1,
            true => SQERMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SQERMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SQERMR::VALUE2
    }
}
#[doc = "Possible values of the field `PROERM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROERMR {
    #[doc = "Interrupt not enabled"]
    VALUE1,
    #[doc = "Flash interrupt because of Protection Error is enabled"]
    VALUE2,
}
impl PROERMR {
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
            PROERMR::VALUE1 => false,
            PROERMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PROERMR {
        match value {
            false => PROERMR::VALUE1,
            true => PROERMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PROERMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PROERMR::VALUE2
    }
}
#[doc = "Possible values of the field `PFSBERM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFSBERMR {
    #[doc = "No Single-Bit Error interrupt enabled"]
    VALUE1,
    #[doc = "Single-Bit Error interrupt enabled for PFLASH"]
    VALUE2,
}
impl PFSBERMR {
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
            PFSBERMR::VALUE1 => false,
            PFSBERMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PFSBERMR {
        match value {
            false => PFSBERMR::VALUE1,
            true => PFSBERMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PFSBERMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PFSBERMR::VALUE2
    }
}
#[doc = "Possible values of the field `PFDBERM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFDBERMR {
    #[doc = "Double-Bit Error interrupt for PFLASH not enabled"]
    VALUE1,
    #[doc = "Double-Bit Error interrupt for PFLASH enabled. Especially intended for margin check"]
    VALUE2,
}
impl PFDBERMR {
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
            PFDBERMR::VALUE1 => false,
            PFDBERMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PFDBERMR {
        match value {
            false => PFDBERMR::VALUE1,
            true => PFDBERMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PFDBERMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PFDBERMR::VALUE2
    }
}
#[doc = "Possible values of the field `EOBM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOBMR {
    #[doc = "Interrupt not enabled"]
    VALUE1,
    #[doc = "EOB interrupt is enabled"]
    VALUE2,
}
impl EOBMR {
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
            EOBMR::VALUE1 => false,
            EOBMR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOBMR {
        match value {
            false => EOBMR::VALUE1,
            true => EOBMR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == EOBMR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == EOBMR::VALUE2
    }
}
#[doc = "Values that can be written to the field `WSPFLASH`"]
pub enum WSPFLASHW {
    #[doc = "PFLASH access in one clock cycle"]
    VALUE1,
    #[doc = "PFLASH access in one clock cycle"]
    VALUE2,
    #[doc = "PFLASH access in two clock cycles"]
    VALUE3,
    #[doc = "PFLASH access in three clock cycles"]
    VALUE4,
    #[doc = "PFLASH access in fifteen clock cycles."]
    VALUE5,
}
impl WSPFLASHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WSPFLASHW::VALUE1 => 0,
            WSPFLASHW::VALUE2 => 1,
            WSPFLASHW::VALUE3 => 2,
            WSPFLASHW::VALUE4 => 3,
            WSPFLASHW::VALUE5 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WSPFLASHW<'a> {
    w: &'a mut W,
}
impl<'a> _WSPFLASHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WSPFLASHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PFLASH access in one clock cycle"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WSPFLASHW::VALUE1)
    }
    #[doc = "PFLASH access in one clock cycle"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WSPFLASHW::VALUE2)
    }
    #[doc = "PFLASH access in two clock cycles"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(WSPFLASHW::VALUE3)
    }
    #[doc = "PFLASH access in three clock cycles"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(WSPFLASHW::VALUE4)
    }
    #[doc = "PFLASH access in fifteen clock cycles."]
    #[inline]
    pub fn value5(self) -> &'a mut W {
        self.variant(WSPFLASHW::VALUE5)
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
#[doc = "Values that can be written to the field `WSECPF`"]
pub enum WSECPFW {
    #[doc = "No additional wait state for error correction"]
    VALUE1,
    #[doc = "One additional wait state for error correction during read access to Program Flash. If enabled, this wait state is only used for the first transfer of a burst transfer."]
    VALUE2,
}
impl WSECPFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WSECPFW::VALUE1 => false,
            WSECPFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WSECPFW<'a> {
    w: &'a mut W,
}
impl<'a> _WSECPFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WSECPFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No additional wait state for error correction"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(WSECPFW::VALUE1)
    }
    #[doc = "One additional wait state for error correction during read access to Program Flash. If enabled, this wait state is only used for the first transfer of a burst transfer."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(WSECPFW::VALUE2)
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
#[doc = "Values that can be written to the field `IDLE`"]
pub enum IDLEW {
    #[doc = "Normal/standard Flash read operation"]
    VALUE1,
    #[doc = "Dynamic idle of Program Flash enabled for power saving; static prefetching disabled"]
    VALUE2,
}
impl IDLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDLEW::VALUE1 => false,
            IDLEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _IDLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal/standard Flash read operation"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(IDLEW::VALUE1)
    }
    #[doc = "Dynamic idle of Program Flash enabled for power saving; static prefetching disabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(IDLEW::VALUE2)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ESLDIS`"]
pub enum ESLDISW {
    #[doc = "External sleep request signal input is enabled"]
    VALUE1,
    #[doc = "Externally requested Flash sleep is disabled"]
    VALUE2,
}
impl ESLDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ESLDISW::VALUE1 => false,
            ESLDISW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ESLDISW<'a> {
    w: &'a mut W,
}
impl<'a> _ESLDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ESLDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External sleep request signal input is enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(ESLDISW::VALUE1)
    }
    #[doc = "Externally requested Flash sleep is disabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(ESLDISW::VALUE2)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLEEP`"]
pub enum SLEEPW {
    #[doc = "Normal state or wake-up"]
    VALUE1,
    #[doc = "Flash sleep mode is requested"]
    VALUE2,
}
impl SLEEPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLEEPW::VALUE1 => false,
            SLEEPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLEEPW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLEEPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal state or wake-up"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SLEEPW::VALUE1)
    }
    #[doc = "Flash sleep mode is requested"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SLEEPW::VALUE2)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCF`"]
pub enum DCFW {
    #[doc = "Code fetching from the Flash memory area is allowed."]
    VALUE1,
    #[doc = "Code fetching from the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    VALUE2,
}
impl DCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DCFW::VALUE1 => false,
            DCFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCFW<'a> {
    w: &'a mut W,
}
impl<'a> _DCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Code fetching from the Flash memory area is allowed."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DCFW::VALUE1)
    }
    #[doc = "Code fetching from the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DCFW::VALUE2)
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
#[doc = "Values that can be written to the field `DDF`"]
pub enum DDFW {
    #[doc = "Data read access to the Flash memory area is allowed."]
    VALUE1,
    #[doc = "Data read access to the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    VALUE2,
}
impl DDFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DDFW::VALUE1 => false,
            DDFW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DDFW<'a> {
    w: &'a mut W,
}
impl<'a> _DDFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DDFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data read access to the Flash memory area is allowed."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DDFW::VALUE1)
    }
    #[doc = "Data read access to the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DDFW::VALUE2)
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
#[doc = "Values that can be written to the field `VOPERM`"]
pub enum VOPERMW {
    #[doc = "Interrupt not enabled"]
    VALUE1,
    #[doc = "Flash interrupt because of Verify Error or Operation Error in Flash array (FSI) is enabled"]
    VALUE2,
}
impl VOPERMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VOPERMW::VALUE1 => false,
            VOPERMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VOPERMW<'a> {
    w: &'a mut W,
}
impl<'a> _VOPERMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VOPERMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(VOPERMW::VALUE1)
    }
    #[doc = "Flash interrupt because of Verify Error or Operation Error in Flash array (FSI) is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(VOPERMW::VALUE2)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SQERM`"]
pub enum SQERMW {
    #[doc = "Interrupt not enabled"]
    VALUE1,
    #[doc = "Flash interrupt because of Sequence Error is enabled"]
    VALUE2,
}
impl SQERMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SQERMW::VALUE1 => false,
            SQERMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SQERMW<'a> {
    w: &'a mut W,
}
impl<'a> _SQERMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SQERMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SQERMW::VALUE1)
    }
    #[doc = "Flash interrupt because of Sequence Error is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SQERMW::VALUE2)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PROERM`"]
pub enum PROERMW {
    #[doc = "Interrupt not enabled"]
    VALUE1,
    #[doc = "Flash interrupt because of Protection Error is enabled"]
    VALUE2,
}
impl PROERMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PROERMW::VALUE1 => false,
            PROERMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PROERMW<'a> {
    w: &'a mut W,
}
impl<'a> _PROERMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PROERMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PROERMW::VALUE1)
    }
    #[doc = "Flash interrupt because of Protection Error is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PROERMW::VALUE2)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PFSBERM`"]
pub enum PFSBERMW {
    #[doc = "No Single-Bit Error interrupt enabled"]
    VALUE1,
    #[doc = "Single-Bit Error interrupt enabled for PFLASH"]
    VALUE2,
}
impl PFSBERMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PFSBERMW::VALUE1 => false,
            PFSBERMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PFSBERMW<'a> {
    w: &'a mut W,
}
impl<'a> _PFSBERMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PFSBERMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Single-Bit Error interrupt enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PFSBERMW::VALUE1)
    }
    #[doc = "Single-Bit Error interrupt enabled for PFLASH"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PFSBERMW::VALUE2)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PFDBERM`"]
pub enum PFDBERMW {
    #[doc = "Double-Bit Error interrupt for PFLASH not enabled"]
    VALUE1,
    #[doc = "Double-Bit Error interrupt for PFLASH enabled. Especially intended for margin check"]
    VALUE2,
}
impl PFDBERMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PFDBERMW::VALUE1 => false,
            PFDBERMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PFDBERMW<'a> {
    w: &'a mut W,
}
impl<'a> _PFDBERMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PFDBERMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Double-Bit Error interrupt for PFLASH not enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PFDBERMW::VALUE1)
    }
    #[doc = "Double-Bit Error interrupt for PFLASH enabled. Especially intended for margin check"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PFDBERMW::VALUE2)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EOBM`"]
pub enum EOBMW {
    #[doc = "Interrupt not enabled"]
    VALUE1,
    #[doc = "EOB interrupt is enabled"]
    VALUE2,
}
impl EOBMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOBMW::VALUE1 => false,
            EOBMW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOBMW<'a> {
    w: &'a mut W,
}
impl<'a> _EOBMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOBMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(EOBMW::VALUE1)
    }
    #[doc = "EOB interrupt is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(EOBMW::VALUE2)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:3 - Wait States for read access to PFLASH"]
    #[inline]
    pub fn wspflash(&self) -> WSPFLASHR {
        WSPFLASHR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Wait State for Error Correction of PFLASH"]
    #[inline]
    pub fn wsecpf(&self) -> WSECPFR {
        WSECPFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Dynamic Flash Idle"]
    #[inline]
    pub fn idle(&self) -> IDLER {
        IDLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - External Sleep Request Disable"]
    #[inline]
    pub fn esldis(&self) -> ESLDISR {
        ESLDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Flash SLEEP"]
    #[inline]
    pub fn sleep(&self) -> SLEEPR {
        SLEEPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Read Protection Activated"]
    #[inline]
    pub fn rpa(&self) -> RPAR {
        RPAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Disable Code Fetch from Flash Memory"]
    #[inline]
    pub fn dcf(&self) -> DCFR {
        DCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Disable Any Data Fetch from Flash"]
    #[inline]
    pub fn ddf(&self) -> DDFR {
        DDFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Verify and Operation Error Interrupt Mask"]
    #[inline]
    pub fn voperm(&self) -> VOPERMR {
        VOPERMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Command Sequence Error Interrupt Mask"]
    #[inline]
    pub fn sqerm(&self) -> SQERMR {
        SQERMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Protection Error Interrupt Mask"]
    #[inline]
    pub fn proerm(&self) -> PROERMR {
        PROERMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - PFLASH Single-Bit Error Interrupt Mask"]
    #[inline]
    pub fn pfsberm(&self) -> PFSBERMR {
        PFSBERMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - PFLASH Double-Bit Error Interrupt Mask"]
    #[inline]
    pub fn pfdberm(&self) -> PFDBERMR {
        PFDBERMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - End of Busy Interrupt Mask"]
    #[inline]
    pub fn eobm(&self) -> EOBMR {
        EOBMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 6 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Wait States for read access to PFLASH"]
    #[inline]
    pub fn wspflash(&mut self) -> _WSPFLASHW {
        _WSPFLASHW { w: self }
    }
    #[doc = "Bit 4 - Wait State for Error Correction of PFLASH"]
    #[inline]
    pub fn wsecpf(&mut self) -> _WSECPFW {
        _WSECPFW { w: self }
    }
    #[doc = "Bit 13 - Dynamic Flash Idle"]
    #[inline]
    pub fn idle(&mut self) -> _IDLEW {
        _IDLEW { w: self }
    }
    #[doc = "Bit 14 - External Sleep Request Disable"]
    #[inline]
    pub fn esldis(&mut self) -> _ESLDISW {
        _ESLDISW { w: self }
    }
    #[doc = "Bit 15 - Flash SLEEP"]
    #[inline]
    pub fn sleep(&mut self) -> _SLEEPW {
        _SLEEPW { w: self }
    }
    #[doc = "Bit 17 - Disable Code Fetch from Flash Memory"]
    #[inline]
    pub fn dcf(&mut self) -> _DCFW {
        _DCFW { w: self }
    }
    #[doc = "Bit 18 - Disable Any Data Fetch from Flash"]
    #[inline]
    pub fn ddf(&mut self) -> _DDFW {
        _DDFW { w: self }
    }
    #[doc = "Bit 24 - Verify and Operation Error Interrupt Mask"]
    #[inline]
    pub fn voperm(&mut self) -> _VOPERMW {
        _VOPERMW { w: self }
    }
    #[doc = "Bit 25 - Command Sequence Error Interrupt Mask"]
    #[inline]
    pub fn sqerm(&mut self) -> _SQERMW {
        _SQERMW { w: self }
    }
    #[doc = "Bit 26 - Protection Error Interrupt Mask"]
    #[inline]
    pub fn proerm(&mut self) -> _PROERMW {
        _PROERMW { w: self }
    }
    #[doc = "Bit 27 - PFLASH Single-Bit Error Interrupt Mask"]
    #[inline]
    pub fn pfsberm(&mut self) -> _PFSBERMW {
        _PFSBERMW { w: self }
    }
    #[doc = "Bit 29 - PFLASH Double-Bit Error Interrupt Mask"]
    #[inline]
    pub fn pfdberm(&mut self) -> _PFDBERMW {
        _PFDBERMW { w: self }
    }
    #[doc = "Bit 31 - End of Busy Interrupt Mask"]
    #[inline]
    pub fn eobm(&mut self) -> _EOBMW {
        _EOBMW { w: self }
    }
}
