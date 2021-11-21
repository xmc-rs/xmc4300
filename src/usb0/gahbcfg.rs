#[doc = "Register `GAHBCFG` reader"]
pub struct R(crate::R<GAHBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAHBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAHBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAHBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAHBCFG` writer"]
pub struct W(crate::W<GAHBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAHBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GAHBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAHBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Global Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GLBLINTRMSK_A {
    #[doc = "0: Mask the interrupt assertion to the application."]
    VALUE1 = 0,
    #[doc = "1: Unmask the interrupt assertion to the application."]
    VALUE2 = 1,
}
impl From<GLBLINTRMSK_A> for bool {
    #[inline(always)]
    fn from(variant: GLBLINTRMSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GlblIntrMsk` reader - Global Interrupt Mask"]
pub struct GLBLINTRMSK_R(crate::FieldReader<bool, GLBLINTRMSK_A>);
impl GLBLINTRMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        GLBLINTRMSK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GLBLINTRMSK_A {
        match self.bits {
            false => GLBLINTRMSK_A::VALUE1,
            true => GLBLINTRMSK_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == GLBLINTRMSK_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == GLBLINTRMSK_A::VALUE2
    }
}
impl core::ops::Deref for GLBLINTRMSK_R {
    type Target = crate::FieldReader<bool, GLBLINTRMSK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GlblIntrMsk` writer - Global Interrupt Mask"]
pub struct GLBLINTRMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> GLBLINTRMSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GLBLINTRMSK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mask the interrupt assertion to the application."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(GLBLINTRMSK_A::VALUE1)
    }
    #[doc = "Unmask the interrupt assertion to the application."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(GLBLINTRMSK_A::VALUE2)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Burst Length/Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HBSTLEN_A {
    #[doc = "0: Single"]
    VALUE1 = 0,
    #[doc = "1: INCR"]
    VALUE2 = 1,
    #[doc = "3: INCR4"]
    VALUE3 = 3,
    #[doc = "5: INCR8"]
    VALUE4 = 5,
    #[doc = "7: INCR16"]
    VALUE5 = 7,
}
impl From<HBSTLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: HBSTLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HBstLen` reader - Burst Length/Type"]
pub struct HBSTLEN_R(crate::FieldReader<u8, HBSTLEN_A>);
impl HBSTLEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        HBSTLEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HBSTLEN_A> {
        match self.bits {
            0 => Some(HBSTLEN_A::VALUE1),
            1 => Some(HBSTLEN_A::VALUE2),
            3 => Some(HBSTLEN_A::VALUE3),
            5 => Some(HBSTLEN_A::VALUE4),
            7 => Some(HBSTLEN_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == HBSTLEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == HBSTLEN_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        **self == HBSTLEN_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        **self == HBSTLEN_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        **self == HBSTLEN_A::VALUE5
    }
}
impl core::ops::Deref for HBSTLEN_R {
    type Target = crate::FieldReader<u8, HBSTLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HBstLen` writer - Burst Length/Type"]
pub struct HBSTLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HBSTLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HBSTLEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Single"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(HBSTLEN_A::VALUE1)
    }
    #[doc = "INCR"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(HBSTLEN_A::VALUE2)
    }
    #[doc = "INCR4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(HBSTLEN_A::VALUE3)
    }
    #[doc = "INCR8"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(HBSTLEN_A::VALUE4)
    }
    #[doc = "INCR16"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(HBSTLEN_A::VALUE5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | ((value as u32 & 0x0f) << 1);
        self.w
    }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: Core operates in Slave mode"]
    VALUE1 = 0,
    #[doc = "1: Core operates in a DMA mode"]
    VALUE2 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEn` reader - DMA Enable"]
pub struct DMAEN_R(crate::FieldReader<bool, DMAEN_A>);
impl DMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::VALUE1,
            true => DMAEN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DMAEN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DMAEN_A::VALUE2
    }
}
impl core::ops::Deref for DMAEN_R {
    type Target = crate::FieldReader<bool, DMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAEn` writer - DMA Enable"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Core operates in Slave mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DMAEN_A::VALUE1)
    }
    #[doc = "Core operates in a DMA mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(DMAEN_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Non-Periodic TxFIFO Empty Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NPTXFEMPLVL_A {
    #[doc = "0: DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is half empty"]
    VALUE1 = 0,
    #[doc = "1: DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is completely empty"]
    VALUE2 = 1,
}
impl From<NPTXFEMPLVL_A> for bool {
    #[inline(always)]
    fn from(variant: NPTXFEMPLVL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NPTxFEmpLvl` reader - Non-Periodic TxFIFO Empty Level"]
pub struct NPTXFEMPLVL_R(crate::FieldReader<bool, NPTXFEMPLVL_A>);
impl NPTXFEMPLVL_R {
    pub(crate) fn new(bits: bool) -> Self {
        NPTXFEMPLVL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NPTXFEMPLVL_A {
        match self.bits {
            false => NPTXFEMPLVL_A::VALUE1,
            true => NPTXFEMPLVL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == NPTXFEMPLVL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == NPTXFEMPLVL_A::VALUE2
    }
}
impl core::ops::Deref for NPTXFEMPLVL_R {
    type Target = crate::FieldReader<bool, NPTXFEMPLVL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NPTxFEmpLvl` writer - Non-Periodic TxFIFO Empty Level"]
pub struct NPTXFEMPLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> NPTXFEMPLVL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NPTXFEMPLVL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is half empty"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(NPTXFEMPLVL_A::VALUE1)
    }
    #[doc = "DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is completely empty"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(NPTXFEMPLVL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Periodic TxFIFO Empty Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTXFEMPLVL_A {
    #[doc = "0: GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is half empty"]
    VALUE1 = 0,
    #[doc = "1: GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is completely empty"]
    VALUE2 = 1,
}
impl From<PTXFEMPLVL_A> for bool {
    #[inline(always)]
    fn from(variant: PTXFEMPLVL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTxFEmpLvl` reader - Periodic TxFIFO Empty Level"]
pub struct PTXFEMPLVL_R(crate::FieldReader<bool, PTXFEMPLVL_A>);
impl PTXFEMPLVL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PTXFEMPLVL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTXFEMPLVL_A {
        match self.bits {
            false => PTXFEMPLVL_A::VALUE1,
            true => PTXFEMPLVL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == PTXFEMPLVL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == PTXFEMPLVL_A::VALUE2
    }
}
impl core::ops::Deref for PTXFEMPLVL_R {
    type Target = crate::FieldReader<bool, PTXFEMPLVL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTxFEmpLvl` writer - Periodic TxFIFO Empty Level"]
pub struct PTXFEMPLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFEMPLVL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTXFEMPLVL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is half empty"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PTXFEMPLVL_A::VALUE1)
    }
    #[doc = "GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is completely empty"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PTXFEMPLVL_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "AHB Single Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHBSINGLE_A {
    #[doc = "0: The remaining data in a transfer is sent using INCR burst size. This is the default mode."]
    VALUE1 = 0,
    #[doc = "1: The remaining data in a transfer is sent using single burst size."]
    VALUE2 = 1,
}
impl From<AHBSINGLE_A> for bool {
    #[inline(always)]
    fn from(variant: AHBSINGLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHBSingle` reader - AHB Single Support"]
pub struct AHBSINGLE_R(crate::FieldReader<bool, AHBSINGLE_A>);
impl AHBSINGLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHBSINGLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHBSINGLE_A {
        match self.bits {
            false => AHBSINGLE_A::VALUE1,
            true => AHBSINGLE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == AHBSINGLE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == AHBSINGLE_A::VALUE2
    }
}
impl core::ops::Deref for AHBSINGLE_R {
    type Target = crate::FieldReader<bool, AHBSINGLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHBSingle` writer - AHB Single Support"]
pub struct AHBSINGLE_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBSINGLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHBSINGLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The remaining data in a transfer is sent using INCR burst size. This is the default mode."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(AHBSINGLE_A::VALUE1)
    }
    #[doc = "The remaining data in a transfer is sent using single burst size."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(AHBSINGLE_A::VALUE2)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Global Interrupt Mask"]
    #[inline(always)]
    pub fn glbl_intr_msk(&self) -> GLBLINTRMSK_R {
        GLBLINTRMSK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - Burst Length/Type"]
    #[inline(always)]
    pub fn hbst_len(&self) -> HBSTLEN_R {
        HBSTLEN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO Empty Level"]
    #[inline(always)]
    pub fn nptx_femp_lvl(&self) -> NPTXFEMPLVL_R {
        NPTXFEMPLVL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Periodic TxFIFO Empty Level"]
    #[inline(always)]
    pub fn ptx_femp_lvl(&self) -> PTXFEMPLVL_R {
        PTXFEMPLVL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 23 - AHB Single Support"]
    #[inline(always)]
    pub fn ahbsingle(&self) -> AHBSINGLE_R {
        AHBSINGLE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Interrupt Mask"]
    #[inline(always)]
    pub fn glbl_intr_msk(&mut self) -> GLBLINTRMSK_W {
        GLBLINTRMSK_W { w: self }
    }
    #[doc = "Bits 1:4 - Burst Length/Type"]
    #[inline(always)]
    pub fn hbst_len(&mut self) -> HBSTLEN_W {
        HBSTLEN_W { w: self }
    }
    #[doc = "Bit 5 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO Empty Level"]
    #[inline(always)]
    pub fn nptx_femp_lvl(&mut self) -> NPTXFEMPLVL_W {
        NPTXFEMPLVL_W { w: self }
    }
    #[doc = "Bit 8 - Periodic TxFIFO Empty Level"]
    #[inline(always)]
    pub fn ptx_femp_lvl(&mut self) -> PTXFEMPLVL_W {
        PTXFEMPLVL_W { w: self }
    }
    #[doc = "Bit 23 - AHB Single Support"]
    #[inline(always)]
    pub fn ahbsingle(&mut self) -> AHBSINGLE_W {
        AHBSINGLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gahbcfg](index.html) module"]
pub struct GAHBCFG_SPEC;
impl crate::RegisterSpec for GAHBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gahbcfg::R](R) reader structure"]
impl crate::Readable for GAHBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gahbcfg::W](W) writer structure"]
impl crate::Writable for GAHBCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GAHBCFG to value 0"]
impl crate::Resettable for GAHBCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
