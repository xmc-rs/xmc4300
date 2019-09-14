#[doc = "Reader of register HCFG"]
pub type R = crate::R<u32, super::HCFG>;
#[doc = "Writer for register HCFG"]
pub type W = crate::W<u32, super::HCFG>;
#[doc = "Register HCFG `reset()`'s with value 0x0200"]
impl crate::ResetValue for super::HCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
    }
}
#[doc = "FS PHY Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSLSPCLKSEL_A {
    #[doc = "1: PHY clock is running at 48 MHz"]
    VALUE1,
}
impl From<FSLSPCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FSLSPCLKSEL_A) -> Self {
        match variant {
            FSLSPCLKSEL_A::VALUE1 => 1,
        }
    }
}
#[doc = "Reader of field `FSLSPclkSel`"]
pub type FSLSPCLKSEL_R = crate::R<u8, FSLSPCLKSEL_A>;
impl FSLSPCLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FSLSPCLKSEL_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(FSLSPCLKSEL_A::VALUE1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FSLSPCLKSEL_A::VALUE1
    }
}
#[doc = "Write proxy for field `FSLSPclkSel`"]
pub struct FSLSPCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSLSPCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSLSPCLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PHY clock is running at 48 MHz"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FSLSPCLKSEL_A::VALUE1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "FS-Only Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSLSSUPP_A {
    #[doc = "0: FS-only, connected device can supports also only FS."]
    VALUE1,
    #[doc = "1: FS-only, even if the connected device can support HS"]
    VALUE2,
}
impl From<FSLSSUPP_A> for bool {
    #[inline(always)]
    fn from(variant: FSLSSUPP_A) -> Self {
        match variant {
            FSLSSUPP_A::VALUE1 => false,
            FSLSSUPP_A::VALUE2 => true,
        }
    }
}
#[doc = "Reader of field `FSLSSupp`"]
pub type FSLSSUPP_R = crate::R<bool, FSLSSUPP_A>;
impl FSLSSUPP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSLSSUPP_A {
        match self.bits {
            false => FSLSSUPP_A::VALUE1,
            true => FSLSSUPP_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FSLSSUPP_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FSLSSUPP_A::VALUE2
    }
}
#[doc = "Write proxy for field `FSLSSupp`"]
pub struct FSLSSUPP_W<'a> {
    w: &'a mut W,
}
impl<'a> FSLSSUPP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSLSSUPP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FS-only, connected device can supports also only FS."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FSLSSUPP_A::VALUE1)
    }
    #[doc = "FS-only, even if the connected device can support HS"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FSLSSUPP_A::VALUE2)
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
#[doc = "Reader of field `DescDMA`"]
pub type DESCDMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DescDMA`"]
pub struct DESCDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DESCDMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Frame List Entries\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRLISTEN_A {
    #[doc = "0: 8 Entries"]
    VALUE1,
    #[doc = "1: 16 Entries"]
    VALUE2,
    #[doc = "2: 32 Entries"]
    VALUE3,
    #[doc = "3: 64 Entries"]
    VALUE4,
}
impl From<FRLISTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: FRLISTEN_A) -> Self {
        match variant {
            FRLISTEN_A::VALUE1 => 0,
            FRLISTEN_A::VALUE2 => 1,
            FRLISTEN_A::VALUE3 => 2,
            FRLISTEN_A::VALUE4 => 3,
        }
    }
}
#[doc = "Reader of field `FrListEn`"]
pub type FRLISTEN_R = crate::R<u8, FRLISTEN_A>;
impl FRLISTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRLISTEN_A {
        match self.bits {
            0 => FRLISTEN_A::VALUE1,
            1 => FRLISTEN_A::VALUE2,
            2 => FRLISTEN_A::VALUE3,
            3 => FRLISTEN_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == FRLISTEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == FRLISTEN_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == FRLISTEN_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == FRLISTEN_A::VALUE4
    }
}
#[doc = "Write proxy for field `FrListEn`"]
pub struct FRLISTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRLISTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRLISTEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "8 Entries"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(FRLISTEN_A::VALUE1)
    }
    #[doc = "16 Entries"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(FRLISTEN_A::VALUE2)
    }
    #[doc = "32 Entries"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(FRLISTEN_A::VALUE3)
    }
    #[doc = "64 Entries"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(FRLISTEN_A::VALUE4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `PerSchedEna`"]
pub type PERSCHEDENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PerSchedEna`"]
pub struct PERSCHEDENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PERSCHEDENA_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - FS PHY Clock Select"]
    #[inline(always)]
    pub fn fslspclk_sel(&self) -> FSLSPCLKSEL_R {
        FSLSPCLKSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - FS-Only Support"]
    #[inline(always)]
    pub fn fslssupp(&self) -> FSLSSUPP_R {
        FSLSSUPP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable Scatter/gather DMA in Host mode"]
    #[inline(always)]
    pub fn desc_dma(&self) -> DESCDMA_R {
        DESCDMA_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Frame List Entries"]
    #[inline(always)]
    pub fn fr_list_en(&self) -> FRLISTEN_R {
        FRLISTEN_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Enable Periodic Scheduling"]
    #[inline(always)]
    pub fn per_sched_ena(&self) -> PERSCHEDENA_R {
        PERSCHEDENA_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FS PHY Clock Select"]
    #[inline(always)]
    pub fn fslspclk_sel(&mut self) -> FSLSPCLKSEL_W {
        FSLSPCLKSEL_W { w: self }
    }
    #[doc = "Bit 2 - FS-Only Support"]
    #[inline(always)]
    pub fn fslssupp(&mut self) -> FSLSSUPP_W {
        FSLSSUPP_W { w: self }
    }
    #[doc = "Bit 23 - Enable Scatter/gather DMA in Host mode"]
    #[inline(always)]
    pub fn desc_dma(&mut self) -> DESCDMA_W {
        DESCDMA_W { w: self }
    }
    #[doc = "Bits 24:25 - Frame List Entries"]
    #[inline(always)]
    pub fn fr_list_en(&mut self) -> FRLISTEN_W {
        FRLISTEN_W { w: self }
    }
    #[doc = "Bit 26 - Enable Periodic Scheduling"]
    #[inline(always)]
    pub fn per_sched_ena(&mut self) -> PERSCHEDENA_W {
        PERSCHEDENA_W { w: self }
    }
}
