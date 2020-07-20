#[doc = "Reader of register PCR_IISMode"]
pub type R = crate::R<u32, super::PCR_IISMODE>;
#[doc = "Writer for register PCR_IISMode"]
pub type W = crate::W<u32, super::PCR_IISMODE>;
#[doc = "Register PCR_IISMode `reset()`'s with value 0"]
impl crate::ResetValue for super::PCR_IISMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "WA Generation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAGEN_A {
    #[doc = "0: The IIS can be used as slave. The generation of the word address signal is disabled. The output signal WA is 0. The MCLKO signal generation depends on PCR.MCLK."]
    VALUE1 = 0,
    #[doc = "1: The IIS can be used as master. The generation of the word address signal is enabled. The signal starts with a 0 after being enabled. The generation of MCLK is enabled, independent of PCR.MCLK. After clearing WAGEN, the USIC module stops the generation of the WA signal within the next 4 WA periods."]
    VALUE2 = 1,
}
impl From<WAGEN_A> for bool {
    #[inline(always)]
    fn from(variant: WAGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAGEN`"]
pub type WAGEN_R = crate::R<bool, WAGEN_A>;
impl WAGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAGEN_A {
        match self.bits {
            false => WAGEN_A::VALUE1,
            true => WAGEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WAGEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WAGEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `WAGEN`"]
pub struct WAGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The IIS can be used as slave. The generation of the word address signal is disabled. The output signal WA is 0. The MCLKO signal generation depends on PCR.MCLK."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAGEN_A::VALUE1)
    }
    #[doc = "The IIS can be used as master. The generation of the word address signal is enabled. The signal starts with a 0 after being enabled. The generation of MCLK is enabled, independent of PCR.MCLK. After clearing WAGEN, the USIC module stops the generation of the WA signal within the next 4 WA periods."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAGEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Data Transfers Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTEN_A {
    #[doc = "0: The changes of the WA input signal are ignored and no transfers take place."]
    VALUE1 = 0,
    #[doc = "1: Transfers are enabled."]
    VALUE2 = 1,
}
impl From<DTEN_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DTEN`"]
pub type DTEN_R = crate::R<bool, DTEN_A>;
impl DTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEN_A {
        match self.bits {
            false => DTEN_A::VALUE1,
            true => DTEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DTEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DTEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `DTEN`"]
pub struct DTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The changes of the WA input signal are ignored and no transfers take place."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DTEN_A::VALUE1)
    }
    #[doc = "Transfers are enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DTEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Select Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELINV_A {
    #[doc = "0: The SELOx outputs have the same polarity as the WA signal."]
    VALUE1 = 0,
    #[doc = "1: The SELOx outputs have the inverted polarity to the WA signal."]
    VALUE2 = 1,
}
impl From<SELINV_A> for bool {
    #[inline(always)]
    fn from(variant: SELINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SELINV`"]
pub type SELINV_R = crate::R<bool, SELINV_A>;
impl SELINV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELINV_A {
        match self.bits {
            false => SELINV_A::VALUE1,
            true => SELINV_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SELINV_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SELINV_A::VALUE2
    }
}
#[doc = "Write proxy for field `SELINV`"]
pub struct SELINV_W<'a> {
    w: &'a mut W,
}
impl<'a> SELINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The SELOx outputs have the same polarity as the WA signal."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(SELINV_A::VALUE1)
    }
    #[doc = "The SELOx outputs have the inverted polarity to the WA signal."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(SELINV_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "WA Falling Edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAFEIEN_A {
    #[doc = "0: A protocol interrupt is not activated if a falling edge of WA is generated."]
    VALUE1 = 0,
    #[doc = "1: A protocol interrupt is activated if a falling edge of WA is generated."]
    VALUE2 = 1,
}
impl From<WAFEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: WAFEIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAFEIEN`"]
pub type WAFEIEN_R = crate::R<bool, WAFEIEN_A>;
impl WAFEIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAFEIEN_A {
        match self.bits {
            false => WAFEIEN_A::VALUE1,
            true => WAFEIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WAFEIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WAFEIEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `WAFEIEN`"]
pub struct WAFEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAFEIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAFEIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A protocol interrupt is not activated if a falling edge of WA is generated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAFEIEN_A::VALUE1)
    }
    #[doc = "A protocol interrupt is activated if a falling edge of WA is generated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAFEIEN_A::VALUE2)
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
#[doc = "WA Rising Edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAREIEN_A {
    #[doc = "0: A protocol interrupt is not activated if a rising edge of WA is generated."]
    VALUE1 = 0,
    #[doc = "1: A protocol interrupt is activated if a rising edge of WA is generated."]
    VALUE2 = 1,
}
impl From<WAREIEN_A> for bool {
    #[inline(always)]
    fn from(variant: WAREIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAREIEN`"]
pub type WAREIEN_R = crate::R<bool, WAREIEN_A>;
impl WAREIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAREIEN_A {
        match self.bits {
            false => WAREIEN_A::VALUE1,
            true => WAREIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WAREIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WAREIEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `WAREIEN`"]
pub struct WAREIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAREIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAREIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A protocol interrupt is not activated if a rising edge of WA is generated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(WAREIEN_A::VALUE1)
    }
    #[doc = "A protocol interrupt is activated if a rising edge of WA is generated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(WAREIEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "END Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDIEN_A {
    #[doc = "0: A protocol interrupt is not activated."]
    VALUE1 = 0,
    #[doc = "1: A protocol interrupt is activated."]
    VALUE2 = 1,
}
impl From<ENDIEN_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDIEN`"]
pub type ENDIEN_R = crate::R<bool, ENDIEN_A>;
impl ENDIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDIEN_A {
        match self.bits {
            false => ENDIEN_A::VALUE1,
            true => ENDIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENDIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENDIEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `ENDIEN`"]
pub struct ENDIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A protocol interrupt is not activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENDIEN_A::VALUE1)
    }
    #[doc = "A protocol interrupt is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ENDIEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "DX2T Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DX2TIEN_A {
    #[doc = "0: A protocol interrupt is not generated if DX2T is active."]
    VALUE1 = 0,
    #[doc = "1: A protocol interrupt is generated if DX2T is active."]
    VALUE2 = 1,
}
impl From<DX2TIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DX2TIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DX2TIEN`"]
pub type DX2TIEN_R = crate::R<bool, DX2TIEN_A>;
impl DX2TIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DX2TIEN_A {
        match self.bits {
            false => DX2TIEN_A::VALUE1,
            true => DX2TIEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DX2TIEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DX2TIEN_A::VALUE2
    }
}
#[doc = "Write proxy for field `DX2TIEN`"]
pub struct DX2TIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DX2TIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DX2TIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "A protocol interrupt is not generated if DX2T is active."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DX2TIEN_A::VALUE1)
    }
    #[doc = "A protocol interrupt is generated if DX2T is active."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DX2TIEN_A::VALUE2)
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
#[doc = "Reader of field `TDEL`"]
pub type TDEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TDEL`"]
pub struct TDEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TDEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Master Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLK_A {
    #[doc = "0: The MCLK generation is disabled and MCLK is 0."]
    VALUE1 = 0,
    #[doc = "1: The MCLK generation is enabled."]
    VALUE2 = 1,
}
impl From<MCLK_A> for bool {
    #[inline(always)]
    fn from(variant: MCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCLK`"]
pub type MCLK_R = crate::R<bool, MCLK_A>;
impl MCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLK_A {
        match self.bits {
            false => MCLK_A::VALUE1,
            true => MCLK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MCLK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCLK_A::VALUE2
    }
}
#[doc = "Write proxy for field `MCLK`"]
pub struct MCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The MCLK generation is disabled and MCLK is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(MCLK_A::VALUE1)
    }
    #[doc = "The MCLK generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(MCLK_A::VALUE2)
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
    #[doc = "Bit 0 - WA Generation Enable"]
    #[inline(always)]
    pub fn wagen(&self) -> WAGEN_R {
        WAGEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data Transfers Enable"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Select Inversion"]
    #[inline(always)]
    pub fn selinv(&self) -> SELINV_R {
        SELINV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WA Falling Edge Interrupt Enable"]
    #[inline(always)]
    pub fn wafeien(&self) -> WAFEIEN_R {
        WAFEIEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - WA Rising Edge Interrupt Enable"]
    #[inline(always)]
    pub fn wareien(&self) -> WAREIEN_R {
        WAREIEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - END Interrupt Enable"]
    #[inline(always)]
    pub fn endien(&self) -> ENDIEN_R {
        ENDIEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DX2T Interrupt Enable"]
    #[inline(always)]
    pub fn dx2tien(&self) -> DX2TIEN_R {
        DX2TIEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - Transfer Delay"]
    #[inline(always)]
    pub fn tdel(&self) -> TDEL_R {
        TDEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    pub fn mclk(&self) -> MCLK_R {
        MCLK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WA Generation Enable"]
    #[inline(always)]
    pub fn wagen(&mut self) -> WAGEN_W {
        WAGEN_W { w: self }
    }
    #[doc = "Bit 1 - Data Transfers Enable"]
    #[inline(always)]
    pub fn dten(&mut self) -> DTEN_W {
        DTEN_W { w: self }
    }
    #[doc = "Bit 2 - Select Inversion"]
    #[inline(always)]
    pub fn selinv(&mut self) -> SELINV_W {
        SELINV_W { w: self }
    }
    #[doc = "Bit 4 - WA Falling Edge Interrupt Enable"]
    #[inline(always)]
    pub fn wafeien(&mut self) -> WAFEIEN_W {
        WAFEIEN_W { w: self }
    }
    #[doc = "Bit 5 - WA Rising Edge Interrupt Enable"]
    #[inline(always)]
    pub fn wareien(&mut self) -> WAREIEN_W {
        WAREIEN_W { w: self }
    }
    #[doc = "Bit 6 - END Interrupt Enable"]
    #[inline(always)]
    pub fn endien(&mut self) -> ENDIEN_W {
        ENDIEN_W { w: self }
    }
    #[doc = "Bit 15 - DX2T Interrupt Enable"]
    #[inline(always)]
    pub fn dx2tien(&mut self) -> DX2TIEN_W {
        DX2TIEN_W { w: self }
    }
    #[doc = "Bits 16:21 - Transfer Delay"]
    #[inline(always)]
    pub fn tdel(&mut self) -> TDEL_W {
        TDEL_W { w: self }
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    pub fn mclk(&mut self) -> MCLK_W {
        MCLK_W { w: self }
    }
}
