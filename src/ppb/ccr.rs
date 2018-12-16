#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCR {
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
#[doc = "Possible values of the field `NONBASETHRDENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NONBASETHRDENAR {
    #[doc = "processor can enter Thread mode only when no exception is active."]
    VALUE1,
    #[doc = "processor can enter Thread mode from any level under the control of an EXC_RETURN value, see Exception returnException return occurs when the processor is in Handler mode and executes one of the following instructions to load the EXC_RETURN value into the PC:an LDM or POP instruction that loads the PCan LDR instruction with PC as the destinationa BX instruction using any register.EXC_RETURN is the value loaded into the LR on exception entry. The exception mechanism relies on this value to detect when the processor has completed an exception handler. The lowest five bits of this value provide information on the return stack and processor mode. shows the EXC_RETURN values with a description of the exception return behavior. All EXC_RETURN values have bits\\[31:5\\] set to one. When this value is loaded into the PC it indicates to the processor that the exception is complete, and the processor initiates the appropriate exception return sequence.Exception return behaviorEXC_RETURN\\[31:0\\]Description 0xFFFFFFF1 Return to Handler mode, exception return uses non-floating-point state from the MSP and execution uses MSP after return. 0xFFFFFFF9 Return to Thread mode, exception return uses non-floating-point state from MSP and execution uses MSP after return. 0xFFFFFFFD Return to Thread mode, exception return uses non-floating-point state from the PSP and execution uses PSP after return. 0xFFFFFFE1 Return to Handler mode, exception return uses floating-point-state from MSP and execution uses MSP after return. 0xFFFFFFE9 Return to Thread mode, exception return uses floating-point state from MSP and execution uses MSP after return. 0xFFFFFFED Return to Thread mode, exception return uses floating-point state from PSP and execution uses PSP after return. ."]
    VALUE2,
}
impl NONBASETHRDENAR {
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
            NONBASETHRDENAR::VALUE1 => false,
            NONBASETHRDENAR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NONBASETHRDENAR {
        match value {
            false => NONBASETHRDENAR::VALUE1,
            true => NONBASETHRDENAR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == NONBASETHRDENAR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == NONBASETHRDENAR::VALUE2
    }
}
#[doc = "Possible values of the field `USERSETMPEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USERSETMPENDR {
    #[doc = "disable"]
    VALUE1,
    #[doc = "enable"]
    VALUE2,
}
impl USERSETMPENDR {
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
            USERSETMPENDR::VALUE1 => false,
            USERSETMPENDR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USERSETMPENDR {
        match value {
            false => USERSETMPENDR::VALUE1,
            true => USERSETMPENDR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == USERSETMPENDR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == USERSETMPENDR::VALUE2
    }
}
#[doc = "Possible values of the field `UNALIGN_TRP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALIGN_TRPR {
    #[doc = "do not trap unaligned halfword and word accesses"]
    VALUE1,
    #[doc = "trap unaligned halfword and word accesses."]
    VALUE2,
}
impl UNALIGN_TRPR {
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
            UNALIGN_TRPR::VALUE1 => false,
            UNALIGN_TRPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UNALIGN_TRPR {
        match value {
            false => UNALIGN_TRPR::VALUE1,
            true => UNALIGN_TRPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == UNALIGN_TRPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == UNALIGN_TRPR::VALUE2
    }
}
#[doc = "Possible values of the field `DIV_0_TRP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIV_0_TRPR {
    #[doc = "do not trap divide by 0"]
    VALUE1,
    #[doc = "trap divide by 0."]
    VALUE2,
}
impl DIV_0_TRPR {
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
            DIV_0_TRPR::VALUE1 => false,
            DIV_0_TRPR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIV_0_TRPR {
        match value {
            false => DIV_0_TRPR::VALUE1,
            true => DIV_0_TRPR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == DIV_0_TRPR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == DIV_0_TRPR::VALUE2
    }
}
#[doc = "Possible values of the field `BFHFNMIGN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFHFNMIGNR {
    #[doc = "data bus faults caused by load and store instructions cause a lock-up"]
    VALUE1,
    #[doc = "handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions."]
    VALUE2,
}
impl BFHFNMIGNR {
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
            BFHFNMIGNR::VALUE1 => false,
            BFHFNMIGNR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BFHFNMIGNR {
        match value {
            false => BFHFNMIGNR::VALUE1,
            true => BFHFNMIGNR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == BFHFNMIGNR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == BFHFNMIGNR::VALUE2
    }
}
#[doc = "Possible values of the field `STKALIGN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STKALIGNR {
    #[doc = "4-byte aligned"]
    VALUE1,
    #[doc = "8-byte aligned."]
    VALUE2,
}
impl STKALIGNR {
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
            STKALIGNR::VALUE1 => false,
            STKALIGNR::VALUE2 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STKALIGNR {
        match value {
            false => STKALIGNR::VALUE1,
            true => STKALIGNR::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline]
    pub fn is_value1(&self) -> bool {
        *self == STKALIGNR::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline]
    pub fn is_value2(&self) -> bool {
        *self == STKALIGNR::VALUE2
    }
}
#[doc = "Values that can be written to the field `NONBASETHRDENA`"]
pub enum NONBASETHRDENAW {
    #[doc = "processor can enter Thread mode only when no exception is active."]
    VALUE1,
    #[doc = "processor can enter Thread mode from any level under the control of an EXC_RETURN value, see Exception returnException return occurs when the processor is in Handler mode and executes one of the following instructions to load the EXC_RETURN value into the PC:an LDM or POP instruction that loads the PCan LDR instruction with PC as the destinationa BX instruction using any register.EXC_RETURN is the value loaded into the LR on exception entry. The exception mechanism relies on this value to detect when the processor has completed an exception handler. The lowest five bits of this value provide information on the return stack and processor mode. shows the EXC_RETURN values with a description of the exception return behavior. All EXC_RETURN values have bits\\[31:5\\] set to one. When this value is loaded into the PC it indicates to the processor that the exception is complete, and the processor initiates the appropriate exception return sequence.Exception return behaviorEXC_RETURN\\[31:0\\]Description 0xFFFFFFF1 Return to Handler mode, exception return uses non-floating-point state from the MSP and execution uses MSP after return. 0xFFFFFFF9 Return to Thread mode, exception return uses non-floating-point state from MSP and execution uses MSP after return. 0xFFFFFFFD Return to Thread mode, exception return uses non-floating-point state from the PSP and execution uses PSP after return. 0xFFFFFFE1 Return to Handler mode, exception return uses floating-point-state from MSP and execution uses MSP after return. 0xFFFFFFE9 Return to Thread mode, exception return uses floating-point state from MSP and execution uses MSP after return. 0xFFFFFFED Return to Thread mode, exception return uses floating-point state from PSP and execution uses PSP after return. ."]
    VALUE2,
}
impl NONBASETHRDENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NONBASETHRDENAW::VALUE1 => false,
            NONBASETHRDENAW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NONBASETHRDENAW<'a> {
    w: &'a mut W,
}
impl<'a> _NONBASETHRDENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NONBASETHRDENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "processor can enter Thread mode only when no exception is active."]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(NONBASETHRDENAW::VALUE1)
    }
    #[doc = "processor can enter Thread mode from any level under the control of an EXC_RETURN value, see Exception returnException return occurs when the processor is in Handler mode and executes one of the following instructions to load the EXC_RETURN value into the PC:an LDM or POP instruction that loads the PCan LDR instruction with PC as the destinationa BX instruction using any register.EXC_RETURN is the value loaded into the LR on exception entry. The exception mechanism relies on this value to detect when the processor has completed an exception handler. The lowest five bits of this value provide information on the return stack and processor mode. shows the EXC_RETURN values with a description of the exception return behavior. All EXC_RETURN values have bits\\[31:5\\] set to one. When this value is loaded into the PC it indicates to the processor that the exception is complete, and the processor initiates the appropriate exception return sequence.Exception return behaviorEXC_RETURN\\[31:0\\]Description 0xFFFFFFF1 Return to Handler mode, exception return uses non-floating-point state from the MSP and execution uses MSP after return. 0xFFFFFFF9 Return to Thread mode, exception return uses non-floating-point state from MSP and execution uses MSP after return. 0xFFFFFFFD Return to Thread mode, exception return uses non-floating-point state from the PSP and execution uses PSP after return. 0xFFFFFFE1 Return to Handler mode, exception return uses floating-point-state from MSP and execution uses MSP after return. 0xFFFFFFE9 Return to Thread mode, exception return uses floating-point state from MSP and execution uses MSP after return. 0xFFFFFFED Return to Thread mode, exception return uses floating-point state from PSP and execution uses PSP after return. ."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(NONBASETHRDENAW::VALUE2)
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
#[doc = "Values that can be written to the field `USERSETMPEND`"]
pub enum USERSETMPENDW {
    #[doc = "disable"]
    VALUE1,
    #[doc = "enable"]
    VALUE2,
}
impl USERSETMPENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USERSETMPENDW::VALUE1 => false,
            USERSETMPENDW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USERSETMPENDW<'a> {
    w: &'a mut W,
}
impl<'a> _USERSETMPENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USERSETMPENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "disable"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(USERSETMPENDW::VALUE1)
    }
    #[doc = "enable"]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(USERSETMPENDW::VALUE2)
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
#[doc = "Values that can be written to the field `UNALIGN_TRP`"]
pub enum UNALIGN_TRPW {
    #[doc = "do not trap unaligned halfword and word accesses"]
    VALUE1,
    #[doc = "trap unaligned halfword and word accesses."]
    VALUE2,
}
impl UNALIGN_TRPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UNALIGN_TRPW::VALUE1 => false,
            UNALIGN_TRPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UNALIGN_TRPW<'a> {
    w: &'a mut W,
}
impl<'a> _UNALIGN_TRPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UNALIGN_TRPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not trap unaligned halfword and word accesses"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(UNALIGN_TRPW::VALUE1)
    }
    #[doc = "trap unaligned halfword and word accesses."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(UNALIGN_TRPW::VALUE2)
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
#[doc = "Values that can be written to the field `DIV_0_TRP`"]
pub enum DIV_0_TRPW {
    #[doc = "do not trap divide by 0"]
    VALUE1,
    #[doc = "trap divide by 0."]
    VALUE2,
}
impl DIV_0_TRPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIV_0_TRPW::VALUE1 => false,
            DIV_0_TRPW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIV_0_TRPW<'a> {
    w: &'a mut W,
}
impl<'a> _DIV_0_TRPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIV_0_TRPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "do not trap divide by 0"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(DIV_0_TRPW::VALUE1)
    }
    #[doc = "trap divide by 0."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(DIV_0_TRPW::VALUE2)
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
#[doc = "Values that can be written to the field `BFHFNMIGN`"]
pub enum BFHFNMIGNW {
    #[doc = "data bus faults caused by load and store instructions cause a lock-up"]
    VALUE1,
    #[doc = "handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions."]
    VALUE2,
}
impl BFHFNMIGNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BFHFNMIGNW::VALUE1 => false,
            BFHFNMIGNW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BFHFNMIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _BFHFNMIGNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BFHFNMIGNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "data bus faults caused by load and store instructions cause a lock-up"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(BFHFNMIGNW::VALUE1)
    }
    #[doc = "handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(BFHFNMIGNW::VALUE2)
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
#[doc = "Values that can be written to the field `STKALIGN`"]
pub enum STKALIGNW {
    #[doc = "4-byte aligned"]
    VALUE1,
    #[doc = "8-byte aligned."]
    VALUE2,
}
impl STKALIGNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STKALIGNW::VALUE1 => false,
            STKALIGNW::VALUE2 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STKALIGNW<'a> {
    w: &'a mut W,
}
impl<'a> _STKALIGNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STKALIGNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "4-byte aligned"]
    #[inline]
    pub fn value1(self) -> &'a mut W {
        self.variant(STKALIGNW::VALUE1)
    }
    #[doc = "8-byte aligned."]
    #[inline]
    pub fn value2(self) -> &'a mut W {
        self.variant(STKALIGNW::VALUE2)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Non Base Thread Mode Enable"]
    #[inline]
    pub fn nonbasethrdena(&self) -> NONBASETHRDENAR {
        NONBASETHRDENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - User Set Pending Enable"]
    #[inline]
    pub fn usersetmpend(&self) -> USERSETMPENDR {
        USERSETMPENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Unaligned Access Trap Enable"]
    #[inline]
    pub fn unalign_trp(&self) -> UNALIGN_TRPR {
        UNALIGN_TRPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Divide by Zero Trap Enable"]
    #[inline]
    pub fn div_0_trp(&self) -> DIV_0_TRPR {
        DIV_0_TRPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Bus Fault Hard Fault and NMI Ignore"]
    #[inline]
    pub fn bfhfnmign(&self) -> BFHFNMIGNR {
        BFHFNMIGNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Stack Alignment"]
    #[inline]
    pub fn stkalign(&self) -> STKALIGNR {
        STKALIGNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 512 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Non Base Thread Mode Enable"]
    #[inline]
    pub fn nonbasethrdena(&mut self) -> _NONBASETHRDENAW {
        _NONBASETHRDENAW { w: self }
    }
    #[doc = "Bit 1 - User Set Pending Enable"]
    #[inline]
    pub fn usersetmpend(&mut self) -> _USERSETMPENDW {
        _USERSETMPENDW { w: self }
    }
    #[doc = "Bit 3 - Unaligned Access Trap Enable"]
    #[inline]
    pub fn unalign_trp(&mut self) -> _UNALIGN_TRPW {
        _UNALIGN_TRPW { w: self }
    }
    #[doc = "Bit 4 - Divide by Zero Trap Enable"]
    #[inline]
    pub fn div_0_trp(&mut self) -> _DIV_0_TRPW {
        _DIV_0_TRPW { w: self }
    }
    #[doc = "Bit 8 - Bus Fault Hard Fault and NMI Ignore"]
    #[inline]
    pub fn bfhfnmign(&mut self) -> _BFHFNMIGNW {
        _BFHFNMIGNW { w: self }
    }
    #[doc = "Bit 9 - Stack Alignment"]
    #[inline]
    pub fn stkalign(&mut self) -> _STKALIGNW {
        _STKALIGNW { w: self }
    }
}
