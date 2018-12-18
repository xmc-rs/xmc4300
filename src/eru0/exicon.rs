#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EXICON {
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
#[doc = "Possible values of the field `PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER {
    #[doc = "The trigger pulse generation is disabled"]
    VALUE1,
    #[doc = "The trigger pulse generation is enabled"]
    VALUE2,
}
impl PER {
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
            PER::VALUE1 => false,
            PER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PER {
        match value {
            false => PER::VALUE1,
            true => PER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == PER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == PER::VALUE2
    }
}
#[doc = "Possible values of the field `LD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDR {
    #[doc = "The status flag FL is not cleared by hardware and is used as \"sticky\" bit. Once set, it is not influenced by any edge until it becomes cleared by software."]
    VALUE1,
    #[doc = "The status flag FL rebuilds a level detection of the desired event. It becomes automatically set with a rising edge if RE = 1 or with a falling edge if FE = 1. It becomes automatically cleared with a rising edge if RE = 0 or with a falling edge if FE = 0."]
    VALUE2,
}
impl LDR {
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
            LDR::VALUE1 => false,
            LDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LDR {
        match value {
            false => LDR::VALUE1,
            true => LDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == LDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == LDR::VALUE2
    }
}
#[doc = "Possible values of the field `RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RER {
    #[doc = "A rising edge is not considered as edge event"]
    VALUE1,
    #[doc = "A rising edge is considered as edge event"]
    VALUE2,
}
impl RER {
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
            RER::VALUE1 => false,
            RER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RER {
        match value {
            false => RER::VALUE1,
            true => RER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == RER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == RER::VALUE2
    }
}
#[doc = "Possible values of the field `FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FER {
    #[doc = "A falling edge is not considered as edge event"]
    VALUE1,
    #[doc = "A falling edge is considered as edge event"]
    VALUE2,
}
impl FER {
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
            FER::VALUE1 => false,
            FER::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FER {
        match value {
            false => FER::VALUE1,
            true => FER::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FER::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FER::VALUE2
    }
}
#[doc = "Possible values of the field `OCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCSR {
    #[doc = "Trigger pulses are sent to OGU0"]
    VALUE1,
    #[doc = "Trigger pulses are sent to OGU1"]
    VALUE2,
    #[doc = "Trigger pulses are sent to OGU2"]
    VALUE3,
    #[doc = "Trigger pulses are sent to OGU3"]
    VALUE4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OCSR::VALUE1 => 0,
            OCSR::VALUE2 => 1,
            OCSR::VALUE3 => 2,
            OCSR::VALUE4 => 3,
            OCSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OCSR {
        match value {
            0 => OCSR::VALUE1,
            1 => OCSR::VALUE2,
            2 => OCSR::VALUE3,
            3 => OCSR::VALUE4,
            i => OCSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == OCSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == OCSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == OCSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == OCSR::VALUE4
    }
}
#[doc = "Possible values of the field `FL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLR {
    #[doc = "The enabled edge event has not been detected"]
    VALUE1,
    #[doc = "The enabled edge event has been detected"]
    VALUE2,
}
impl FLR {
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
            FLR::VALUE1 => false,
            FLR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLR {
        match value {
            false => FLR::VALUE1,
            true => FLR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == FLR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == FLR::VALUE2
    }
}
#[doc = "Possible values of the field `SS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSR {
    #[doc = "Input A without additional combination"]
    VALUE1,
    #[doc = "Input B without additional combination"]
    VALUE2,
    #[doc = "Input A OR input B"]
    VALUE3,
    #[doc = "Input A AND input B"]
    VALUE4,
}
impl SSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SSR::VALUE1 => 0,
            SSR::VALUE2 => 1,
            SSR::VALUE3 => 2,
            SSR::VALUE4 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SSR {
        match value {
            0 => SSR::VALUE1,
            1 => SSR::VALUE2,
            2 => SSR::VALUE3,
            3 => SSR::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == SSR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == SSR::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline]
    pub fn is_value3(&self) -> bool {
        *self == SSR::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline]
    pub fn is_value4(&self) -> bool {
        *self == SSR::VALUE4
    }
}
#[doc = "Possible values of the field `NA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NAR {
    #[doc = "Input A is used directly"]
    VALUE1,
    #[doc = "Input A is inverted"]
    VALUE2,
}
impl NAR {
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
            NAR::VALUE1 => false,
            NAR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NAR {
        match value {
            false => NAR::VALUE1,
            true => NAR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == NAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == NAR::VALUE2
    }
}
#[doc = "Possible values of the field `NB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NBR {
    #[doc = "Input B is used directly"]
    VALUE1,
    #[doc = "Input B is inverted"]
    VALUE2,
}
impl NBR {
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
            NBR::VALUE1 => false,
            NBR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NBR {
        match value {
            false => NBR::VALUE1,
            true => NBR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == NBR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == NBR::VALUE2
    }
}
#[doc = "Values that can be written to the field `PE`"]
pub enum PEW {
    #[doc = "The trigger pulse generation is disabled"]
    VALUE1,
    #[doc = "The trigger pulse generation is enabled"]
    VALUE2,
}
impl PEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEW::VALUE1 => false,
            PEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The trigger pulse generation is disabled"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(PEW::VALUE1)
    }
    #[doc = "The trigger pulse generation is enabled"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(PEW::VALUE2)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LD`"]
pub enum LDW {
    #[doc = "The status flag FL is not cleared by hardware and is used as \"sticky\" bit. Once set, it is not influenced by any edge until it becomes cleared by software."]
    VALUE1,
    #[doc = "The status flag FL rebuilds a level detection of the desired event. It becomes automatically set with a rising edge if RE = 1 or with a falling edge if FE = 1. It becomes automatically cleared with a rising edge if RE = 0 or with a falling edge if FE = 0."]
    VALUE2,
}
impl LDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LDW::VALUE1 => false,
            LDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LDW<'a> {
    w: &'a mut W,
}
impl<'a> _LDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The status flag FL is not cleared by hardware and is used as \"sticky\" bit. Once set, it is not influenced by any edge until it becomes cleared by software."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(LDW::VALUE1)
    }
    #[doc = "The status flag FL rebuilds a level detection of the desired event. It becomes automatically set with a rising edge if RE = 1 or with a falling edge if FE = 1. It becomes automatically cleared with a rising edge if RE = 0 or with a falling edge if FE = 0."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(LDW::VALUE2)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RE`"]
pub enum REW {
    #[doc = "A rising edge is not considered as edge event"]
    VALUE1,
    #[doc = "A rising edge is considered as edge event"]
    VALUE2,
}
impl REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REW::VALUE1 => false,
            REW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REW<'a> {
    w: &'a mut W,
}
impl<'a> _REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A rising edge is not considered as edge event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(REW::VALUE1)
    }
    #[doc = "A rising edge is considered as edge event"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(REW::VALUE2)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FE`"]
pub enum FEW {
    #[doc = "A falling edge is not considered as edge event"]
    VALUE1,
    #[doc = "A falling edge is considered as edge event"]
    VALUE2,
}
impl FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEW::VALUE1 => false,
            FEW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEW<'a> {
    w: &'a mut W,
}
impl<'a> _FEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge is not considered as edge event"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FEW::VALUE1)
    }
    #[doc = "A falling edge is considered as edge event"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FEW::VALUE2)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OCS`"]
pub enum OCSW {
    #[doc = "Trigger pulses are sent to OGU0"]
    VALUE1,
    #[doc = "Trigger pulses are sent to OGU1"]
    VALUE2,
    #[doc = "Trigger pulses are sent to OGU2"]
    VALUE3,
    #[doc = "Trigger pulses are sent to OGU3"]
    VALUE4,
}
impl OCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OCSW::VALUE1 => 0,
            OCSW::VALUE2 => 1,
            OCSW::VALUE3 => 2,
            OCSW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OCSW<'a> {
    w: &'a mut W,
}
impl<'a> _OCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OCSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Trigger pulses are sent to OGU0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(OCSW::VALUE1)
    }
    #[doc = "Trigger pulses are sent to OGU1"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(OCSW::VALUE2)
    }
    #[doc = "Trigger pulses are sent to OGU2"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(OCSW::VALUE3)
    }
    #[doc = "Trigger pulses are sent to OGU3"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(OCSW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FL`"]
pub enum FLW {
    #[doc = "The enabled edge event has not been detected"]
    VALUE1,
    #[doc = "The enabled edge event has been detected"]
    VALUE2,
}
impl FLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLW::VALUE1 => false,
            FLW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLW<'a> {
    w: &'a mut W,
}
impl<'a> _FLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The enabled edge event has not been detected"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(FLW::VALUE1)
    }
    #[doc = "The enabled edge event has been detected"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(FLW::VALUE2)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SS`"]
pub enum SSW {
    #[doc = "Input A without additional combination"]
    VALUE1,
    #[doc = "Input B without additional combination"]
    VALUE2,
    #[doc = "Input A OR input B"]
    VALUE3,
    #[doc = "Input A AND input B"]
    VALUE4,
}
impl SSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SSW::VALUE1 => 0,
            SSW::VALUE2 => 1,
            SSW::VALUE3 => 2,
            SSW::VALUE4 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSW<'a> {
    w: &'a mut W,
}
impl<'a> _SSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Input A without additional combination"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(SSW::VALUE1)
    }
    #[doc = "Input B without additional combination"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(SSW::VALUE2)
    }
    #[doc = "Input A OR input B"]
    #[inline]
    pub fn value3(self) -> &'a mut W {
        self.variant(SSW::VALUE3)
    }
    #[doc = "Input A AND input B"]
    #[inline]
    pub fn value4(self) -> &'a mut W {
        self.variant(SSW::VALUE4)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NA`"]
pub enum NAW {
    #[doc = "Input A is used directly"]
    VALUE1,
    #[doc = "Input A is inverted"]
    VALUE2,
}
impl NAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NAW::VALUE1 => false,
            NAW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NAW<'a> {
    w: &'a mut W,
}
impl<'a> _NAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input A is used directly"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(NAW::VALUE1)
    }
    #[doc = "Input A is inverted"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(NAW::VALUE2)
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
#[doc = "Values that can be written to the field `NB`"]
pub enum NBW {
    #[doc = "Input B is used directly"]
    VALUE1,
    #[doc = "Input B is inverted"]
    VALUE2,
}
impl NBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NBW::VALUE1 => false,
            NBW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NBW<'a> {
    w: &'a mut W,
}
impl<'a> _NBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Input B is used directly"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(NBW::VALUE1)
    }
    #[doc = "Input B is inverted"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(NBW::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Output Trigger Pulse Enable for ETLx"]
    #[inline]
    pub fn pe(&self) -> PER {
        PER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Rebuild Level Detection for Status Flag for ETLx"]
    #[inline]
    pub fn ld(&self) -> LDR {
        LDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Rising Edge Detection Enable ETLx"]
    #[inline]
    pub fn re(&self) -> RER {
        RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Falling Edge Detection Enable ETLx"]
    #[inline]
    pub fn fe(&self) -> FER {
        FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - Output Channel Select for ETLx Output Trigger Pulse"]
    #[inline]
    pub fn ocs(&self) -> OCSR {
        OCSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Status Flag for ETLx"]
    #[inline]
    pub fn fl(&self) -> FLR {
        FLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Input Source Select for ERSx"]
    #[inline]
    pub fn ss(&self) -> SSR {
        SSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Input A Negation Select for ERSx"]
    #[inline]
    pub fn na(&self) -> NAR {
        NAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Input B Negation Select for ERSx"]
    #[inline]
    pub fn nb(&self) -> NBR {
        NBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bit 0 - Output Trigger Pulse Enable for ETLx"]
    #[inline]
    pub fn pe(&mut self) -> _PEW {
        _PEW { w: self }
    }
    #[doc = "Bit 1 - Rebuild Level Detection for Status Flag for ETLx"]
    #[inline]
    pub fn ld(&mut self) -> _LDW {
        _LDW { w: self }
    }
    #[doc = "Bit 2 - Rising Edge Detection Enable ETLx"]
    #[inline]
    pub fn re(&mut self) -> _REW {
        _REW { w: self }
    }
    #[doc = "Bit 3 - Falling Edge Detection Enable ETLx"]
    #[inline]
    pub fn fe(&mut self) -> _FEW {
        _FEW { w: self }
    }
    #[doc = "Bits 4:6 - Output Channel Select for ETLx Output Trigger Pulse"]
    #[inline]
    pub fn ocs(&mut self) -> _OCSW {
        _OCSW { w: self }
    }
    #[doc = "Bit 7 - Status Flag for ETLx"]
    #[inline]
    pub fn fl(&mut self) -> _FLW {
        _FLW { w: self }
    }
    #[doc = "Bits 8:9 - Input Source Select for ERSx"]
    #[inline]
    pub fn ss(&mut self) -> _SSW {
        _SSW { w: self }
    }
    #[doc = "Bit 10 - Input A Negation Select for ERSx"]
    #[inline]
    pub fn na(&mut self) -> _NAW {
        _NAW { w: self }
    }
    #[doc = "Bit 11 - Input B Negation Select for ERSx"]
    #[inline]
    pub fn nb(&mut self) -> _NBW {
        _NBW { w: self }
    }
}
