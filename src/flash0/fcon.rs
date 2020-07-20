#[doc = "Reader of register FCON"]
pub type R = crate::R<u32, super::FCON>;
#[doc = "Writer for register FCON"]
pub type W = crate::W<u32, super::FCON>;
#[doc = "Register FCON `reset()`'s with value 0x06"]
impl crate::ResetValue for super::FCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x06
    }
}
#[doc = "Wait States for read access to PFLASH\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WSPFLASH_A {
    #[doc = "0: PFLASH access in one clock cycle"]
    VALUE1 = 0,
    #[doc = "1: PFLASH access in one clock cycle"]
    VALUE2 = 1,
    #[doc = "2: PFLASH access in two clock cycles"]
    VALUE3 = 2,
    #[doc = "3: PFLASH access in three clock cycles"]
    VALUE4 = 3,
    #[doc = "15: PFLASH access in fifteen clock cycles."]
    VALUE5 = 15,
}
impl From<WSPFLASH_A> for u8 {
    #[inline(always)]
    fn from(variant: WSPFLASH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WSPFLASH`"]
pub type WSPFLASH_R = crate::R<u8, WSPFLASH_A>;
impl WSPFLASH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WSPFLASH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WSPFLASH_A::VALUE1),
            1 => Val(WSPFLASH_A::VALUE2),
            2 => Val(WSPFLASH_A::VALUE3),
            3 => Val(WSPFLASH_A::VALUE4),
            15 => Val(WSPFLASH_A::VALUE5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WSPFLASH_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WSPFLASH_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == WSPFLASH_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == WSPFLASH_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == WSPFLASH_A::VALUE5
    }
}
#[doc = "Write proxy for field `WSPFLASH`"]
pub struct WSPFLASH_W<'a> {
    w: &'a mut W,
}
impl<'a> WSPFLASH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WSPFLASH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PFLASH access in one clock cycle"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WSPFLASH_A::VALUE1)
    }
    #[doc = "PFLASH access in one clock cycle"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WSPFLASH_A::VALUE2)
    }
    #[doc = "PFLASH access in two clock cycles"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(WSPFLASH_A::VALUE3)
    }
    #[doc = "PFLASH access in three clock cycles"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(WSPFLASH_A::VALUE4)
    }
    #[doc = "PFLASH access in fifteen clock cycles."]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(WSPFLASH_A::VALUE5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Wait State for Error Correction of PFLASH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WSECPF_A {
    #[doc = "0: No additional wait state for error correction"]
    VALUE1 = 0,
    #[doc = "1: One additional wait state for error correction during read access to Program Flash. If enabled, this wait state is only used for the first transfer of a burst transfer."]
    VALUE2 = 1,
}
impl From<WSECPF_A> for bool {
    #[inline(always)]
    fn from(variant: WSECPF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WSECPF`"]
pub type WSECPF_R = crate::R<bool, WSECPF_A>;
impl WSECPF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WSECPF_A {
        match self.bits {
            false => WSECPF_A::VALUE1,
            true => WSECPF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WSECPF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WSECPF_A::VALUE2
    }
}
#[doc = "Write proxy for field `WSECPF`"]
pub struct WSECPF_W<'a> {
    w: &'a mut W,
}
impl<'a> WSECPF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WSECPF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No additional wait state for error correction"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WSECPF_A::VALUE1)
    }
    #[doc = "One additional wait state for error correction during read access to Program Flash. If enabled, this wait state is only used for the first transfer of a burst transfer."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WSECPF_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Dynamic Flash Idle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLE_A {
    #[doc = "0: Normal/standard Flash read operation"]
    VALUE1 = 0,
    #[doc = "1: Dynamic idle of Program Flash enabled for power saving; static prefetching disabled"]
    VALUE2 = 1,
}
impl From<IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IDLE`"]
pub type IDLE_R = crate::R<bool, IDLE_A>;
impl IDLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLE_A {
        match self.bits {
            false => IDLE_A::VALUE1,
            true => IDLE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == IDLE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == IDLE_A::VALUE2
    }
}
#[doc = "Write proxy for field `IDLE`"]
pub struct IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal/standard Flash read operation"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(IDLE_A::VALUE1)
    }
    #[doc = "Dynamic idle of Program Flash enabled for power saving; static prefetching disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(IDLE_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "External Sleep Request Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESLDIS_A {
    #[doc = "0: External sleep request signal input is enabled"]
    VALUE1 = 0,
    #[doc = "1: Externally requested Flash sleep is disabled"]
    VALUE2 = 1,
}
impl From<ESLDIS_A> for bool {
    #[inline(always)]
    fn from(variant: ESLDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ESLDIS`"]
pub type ESLDIS_R = crate::R<bool, ESLDIS_A>;
impl ESLDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESLDIS_A {
        match self.bits {
            false => ESLDIS_A::VALUE1,
            true => ESLDIS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ESLDIS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ESLDIS_A::VALUE2
    }
}
#[doc = "Write proxy for field `ESLDIS`"]
pub struct ESLDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ESLDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ESLDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External sleep request signal input is enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ESLDIS_A::VALUE1)
    }
    #[doc = "Externally requested Flash sleep is disabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ESLDIS_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Flash SLEEP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEP_A {
    #[doc = "0: Normal state or wake-up"]
    VALUE1 = 0,
    #[doc = "1: Flash sleep mode is requested"]
    VALUE2 = 1,
}
impl From<SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLEEP`"]
pub type SLEEP_R = crate::R<bool, SLEEP_A>;
impl SLEEP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEP_A {
        match self.bits {
            false => SLEEP_A::VALUE1,
            true => SLEEP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SLEEP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SLEEP_A::VALUE2
    }
}
#[doc = "Write proxy for field `SLEEP`"]
pub struct SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal state or wake-up"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SLEEP_A::VALUE1)
    }
    #[doc = "Flash sleep mode is requested"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SLEEP_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Read Protection Activated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPA_A {
    #[doc = "0: The Flash-internal read protection is not activated. Bits DCF, DDF are not taken into account. Bits DCF, DDFx can be cleared"]
    VALUE1 = 0,
    #[doc = "1: The Flash-internal read protection is activated. Bits DCF, DDF are enabled and evaluated."]
    VALUE2 = 1,
}
impl From<RPA_A> for bool {
    #[inline(always)]
    fn from(variant: RPA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RPA`"]
pub type RPA_R = crate::R<bool, RPA_A>;
impl RPA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPA_A {
        match self.bits {
            false => RPA_A::VALUE1,
            true => RPA_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RPA_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RPA_A::VALUE2
    }
}
#[doc = "Disable Code Fetch from Flash Memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCF_A {
    #[doc = "0: Code fetching from the Flash memory area is allowed."]
    VALUE1 = 0,
    #[doc = "1: Code fetching from the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    VALUE2 = 1,
}
impl From<DCF_A> for bool {
    #[inline(always)]
    fn from(variant: DCF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCF`"]
pub type DCF_R = crate::R<bool, DCF_A>;
impl DCF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCF_A {
        match self.bits {
            false => DCF_A::VALUE1,
            true => DCF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DCF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DCF_A::VALUE2
    }
}
#[doc = "Write proxy for field `DCF`"]
pub struct DCF_W<'a> {
    w: &'a mut W,
}
impl<'a> DCF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Code fetching from the Flash memory area is allowed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DCF_A::VALUE1)
    }
    #[doc = "Code fetching from the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DCF_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Disable Any Data Fetch from Flash\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDF_A {
    #[doc = "0: Data read access to the Flash memory area is allowed."]
    VALUE1 = 0,
    #[doc = "1: Data read access to the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    VALUE2 = 1,
}
impl From<DDF_A> for bool {
    #[inline(always)]
    fn from(variant: DDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DDF`"]
pub type DDF_R = crate::R<bool, DDF_A>;
impl DDF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDF_A {
        match self.bits {
            false => DDF_A::VALUE1,
            true => DDF_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DDF_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DDF_A::VALUE2
    }
}
#[doc = "Write proxy for field `DDF`"]
pub struct DDF_W<'a> {
    w: &'a mut W,
}
impl<'a> DDF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data read access to the Flash memory area is allowed."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DDF_A::VALUE1)
    }
    #[doc = "Data read access to the Flash memory area is not allowed. This bit is not taken into account while RPA='0'."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DDF_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Verify and Operation Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VOPERM_A {
    #[doc = "0: Interrupt not enabled"]
    VALUE1 = 0,
    #[doc = "1: Flash interrupt because of Verify Error or Operation Error in Flash array (FSI) is enabled"]
    VALUE2 = 1,
}
impl From<VOPERM_A> for bool {
    #[inline(always)]
    fn from(variant: VOPERM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VOPERM`"]
pub type VOPERM_R = crate::R<bool, VOPERM_A>;
impl VOPERM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VOPERM_A {
        match self.bits {
            false => VOPERM_A::VALUE1,
            true => VOPERM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == VOPERM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == VOPERM_A::VALUE2
    }
}
#[doc = "Write proxy for field `VOPERM`"]
pub struct VOPERM_W<'a> {
    w: &'a mut W,
}
impl<'a> VOPERM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VOPERM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(VOPERM_A::VALUE1)
    }
    #[doc = "Flash interrupt because of Verify Error or Operation Error in Flash array (FSI) is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(VOPERM_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Command Sequence Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SQERM_A {
    #[doc = "0: Interrupt not enabled"]
    VALUE1 = 0,
    #[doc = "1: Flash interrupt because of Sequence Error is enabled"]
    VALUE2 = 1,
}
impl From<SQERM_A> for bool {
    #[inline(always)]
    fn from(variant: SQERM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SQERM`"]
pub type SQERM_R = crate::R<bool, SQERM_A>;
impl SQERM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SQERM_A {
        match self.bits {
            false => SQERM_A::VALUE1,
            true => SQERM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SQERM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SQERM_A::VALUE2
    }
}
#[doc = "Write proxy for field `SQERM`"]
pub struct SQERM_W<'a> {
    w: &'a mut W,
}
impl<'a> SQERM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SQERM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SQERM_A::VALUE1)
    }
    #[doc = "Flash interrupt because of Sequence Error is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SQERM_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Protection Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROERM_A {
    #[doc = "0: Interrupt not enabled"]
    VALUE1 = 0,
    #[doc = "1: Flash interrupt because of Protection Error is enabled"]
    VALUE2 = 1,
}
impl From<PROERM_A> for bool {
    #[inline(always)]
    fn from(variant: PROERM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PROERM`"]
pub type PROERM_R = crate::R<bool, PROERM_A>;
impl PROERM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROERM_A {
        match self.bits {
            false => PROERM_A::VALUE1,
            true => PROERM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PROERM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PROERM_A::VALUE2
    }
}
#[doc = "Write proxy for field `PROERM`"]
pub struct PROERM_W<'a> {
    w: &'a mut W,
}
impl<'a> PROERM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROERM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PROERM_A::VALUE1)
    }
    #[doc = "Flash interrupt because of Protection Error is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PROERM_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "PFLASH Single-Bit Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFSBERM_A {
    #[doc = "0: No Single-Bit Error interrupt enabled"]
    VALUE1 = 0,
    #[doc = "1: Single-Bit Error interrupt enabled for PFLASH"]
    VALUE2 = 1,
}
impl From<PFSBERM_A> for bool {
    #[inline(always)]
    fn from(variant: PFSBERM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PFSBERM`"]
pub type PFSBERM_R = crate::R<bool, PFSBERM_A>;
impl PFSBERM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFSBERM_A {
        match self.bits {
            false => PFSBERM_A::VALUE1,
            true => PFSBERM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PFSBERM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PFSBERM_A::VALUE2
    }
}
#[doc = "Write proxy for field `PFSBERM`"]
pub struct PFSBERM_W<'a> {
    w: &'a mut W,
}
impl<'a> PFSBERM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PFSBERM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Single-Bit Error interrupt enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PFSBERM_A::VALUE1)
    }
    #[doc = "Single-Bit Error interrupt enabled for PFLASH"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PFSBERM_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "PFLASH Double-Bit Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFDBERM_A {
    #[doc = "0: Double-Bit Error interrupt for PFLASH not enabled"]
    VALUE1 = 0,
    #[doc = "1: Double-Bit Error interrupt for PFLASH enabled. Especially intended for margin check"]
    VALUE2 = 1,
}
impl From<PFDBERM_A> for bool {
    #[inline(always)]
    fn from(variant: PFDBERM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PFDBERM`"]
pub type PFDBERM_R = crate::R<bool, PFDBERM_A>;
impl PFDBERM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFDBERM_A {
        match self.bits {
            false => PFDBERM_A::VALUE1,
            true => PFDBERM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PFDBERM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PFDBERM_A::VALUE2
    }
}
#[doc = "Write proxy for field `PFDBERM`"]
pub struct PFDBERM_W<'a> {
    w: &'a mut W,
}
impl<'a> PFDBERM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PFDBERM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Double-Bit Error interrupt for PFLASH not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PFDBERM_A::VALUE1)
    }
    #[doc = "Double-Bit Error interrupt for PFLASH enabled. Especially intended for margin check"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PFDBERM_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "End of Busy Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOBM_A {
    #[doc = "0: Interrupt not enabled"]
    VALUE1 = 0,
    #[doc = "1: EOB interrupt is enabled"]
    VALUE2 = 1,
}
impl From<EOBM_A> for bool {
    #[inline(always)]
    fn from(variant: EOBM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EOBM`"]
pub type EOBM_R = crate::R<bool, EOBM_A>;
impl EOBM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOBM_A {
        match self.bits {
            false => EOBM_A::VALUE1,
            true => EOBM_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EOBM_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EOBM_A::VALUE2
    }
}
#[doc = "Write proxy for field `EOBM`"]
pub struct EOBM_W<'a> {
    w: &'a mut W,
}
impl<'a> EOBM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EOBM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EOBM_A::VALUE1)
    }
    #[doc = "EOB interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EOBM_A::VALUE2)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Wait States for read access to PFLASH"]
    #[inline(always)]
    pub fn wspflash(&self) -> WSPFLASH_R {
        WSPFLASH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Wait State for Error Correction of PFLASH"]
    #[inline(always)]
    pub fn wsecpf(&self) -> WSECPF_R {
        WSECPF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Dynamic Flash Idle"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - External Sleep Request Disable"]
    #[inline(always)]
    pub fn esldis(&self) -> ESLDIS_R {
        ESLDIS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Flash SLEEP"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Read Protection Activated"]
    #[inline(always)]
    pub fn rpa(&self) -> RPA_R {
        RPA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Disable Code Fetch from Flash Memory"]
    #[inline(always)]
    pub fn dcf(&self) -> DCF_R {
        DCF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Disable Any Data Fetch from Flash"]
    #[inline(always)]
    pub fn ddf(&self) -> DDF_R {
        DDF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Verify and Operation Error Interrupt Mask"]
    #[inline(always)]
    pub fn voperm(&self) -> VOPERM_R {
        VOPERM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Command Sequence Error Interrupt Mask"]
    #[inline(always)]
    pub fn sqerm(&self) -> SQERM_R {
        SQERM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Protection Error Interrupt Mask"]
    #[inline(always)]
    pub fn proerm(&self) -> PROERM_R {
        PROERM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - PFLASH Single-Bit Error Interrupt Mask"]
    #[inline(always)]
    pub fn pfsberm(&self) -> PFSBERM_R {
        PFSBERM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 29 - PFLASH Double-Bit Error Interrupt Mask"]
    #[inline(always)]
    pub fn pfdberm(&self) -> PFDBERM_R {
        PFDBERM_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 31 - End of Busy Interrupt Mask"]
    #[inline(always)]
    pub fn eobm(&self) -> EOBM_R {
        EOBM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Wait States for read access to PFLASH"]
    #[inline(always)]
    pub fn wspflash(&mut self) -> WSPFLASH_W {
        WSPFLASH_W { w: self }
    }
    #[doc = "Bit 4 - Wait State for Error Correction of PFLASH"]
    #[inline(always)]
    pub fn wsecpf(&mut self) -> WSECPF_W {
        WSECPF_W { w: self }
    }
    #[doc = "Bit 13 - Dynamic Flash Idle"]
    #[inline(always)]
    pub fn idle(&mut self) -> IDLE_W {
        IDLE_W { w: self }
    }
    #[doc = "Bit 14 - External Sleep Request Disable"]
    #[inline(always)]
    pub fn esldis(&mut self) -> ESLDIS_W {
        ESLDIS_W { w: self }
    }
    #[doc = "Bit 15 - Flash SLEEP"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W { w: self }
    }
    #[doc = "Bit 17 - Disable Code Fetch from Flash Memory"]
    #[inline(always)]
    pub fn dcf(&mut self) -> DCF_W {
        DCF_W { w: self }
    }
    #[doc = "Bit 18 - Disable Any Data Fetch from Flash"]
    #[inline(always)]
    pub fn ddf(&mut self) -> DDF_W {
        DDF_W { w: self }
    }
    #[doc = "Bit 24 - Verify and Operation Error Interrupt Mask"]
    #[inline(always)]
    pub fn voperm(&mut self) -> VOPERM_W {
        VOPERM_W { w: self }
    }
    #[doc = "Bit 25 - Command Sequence Error Interrupt Mask"]
    #[inline(always)]
    pub fn sqerm(&mut self) -> SQERM_W {
        SQERM_W { w: self }
    }
    #[doc = "Bit 26 - Protection Error Interrupt Mask"]
    #[inline(always)]
    pub fn proerm(&mut self) -> PROERM_W {
        PROERM_W { w: self }
    }
    #[doc = "Bit 27 - PFLASH Single-Bit Error Interrupt Mask"]
    #[inline(always)]
    pub fn pfsberm(&mut self) -> PFSBERM_W {
        PFSBERM_W { w: self }
    }
    #[doc = "Bit 29 - PFLASH Double-Bit Error Interrupt Mask"]
    #[inline(always)]
    pub fn pfdberm(&mut self) -> PFDBERM_W {
        PFDBERM_W { w: self }
    }
    #[doc = "Bit 31 - End of Busy Interrupt Mask"]
    #[inline(always)]
    pub fn eobm(&mut self) -> EOBM_W {
        EOBM_W { w: self }
    }
}
