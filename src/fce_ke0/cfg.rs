#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMI` reader - CRC Mismatch Interrupt"]
pub type CMI_R = crate::BitReader<CMI_A>;
#[doc = "CRC Mismatch Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMI_A {
    #[doc = "0: CRC Mismatch Interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: CRC Mismatch Interrupt is enabled"]
    VALUE2 = 1,
}
impl From<CMI_A> for bool {
    #[inline(always)]
    fn from(variant: CMI_A) -> Self {
        variant as u8 != 0
    }
}
impl CMI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMI_A {
        match self.bits {
            false => CMI_A::VALUE1,
            true => CMI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CMI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CMI_A::VALUE2
    }
}
#[doc = "Field `CMI` writer - CRC Mismatch Interrupt"]
pub type CMI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, CMI_A, O>;
impl<'a, const O: u8> CMI_W<'a, O> {
    #[doc = "CRC Mismatch Interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CMI_A::VALUE1)
    }
    #[doc = "CRC Mismatch Interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CMI_A::VALUE2)
    }
}
#[doc = "Field `CEI` reader - Configuration Error Interrupt"]
pub type CEI_R = crate::BitReader<CEI_A>;
#[doc = "Configuration Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEI_A {
    #[doc = "0: Configuration Error Interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: Configuration Error Interrupt is enabled"]
    VALUE2 = 1,
}
impl From<CEI_A> for bool {
    #[inline(always)]
    fn from(variant: CEI_A) -> Self {
        variant as u8 != 0
    }
}
impl CEI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEI_A {
        match self.bits {
            false => CEI_A::VALUE1,
            true => CEI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CEI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CEI_A::VALUE2
    }
}
#[doc = "Field `CEI` writer - Configuration Error Interrupt"]
pub type CEI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, CEI_A, O>;
impl<'a, const O: u8> CEI_W<'a, O> {
    #[doc = "Configuration Error Interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CEI_A::VALUE1)
    }
    #[doc = "Configuration Error Interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CEI_A::VALUE2)
    }
}
#[doc = "Field `LEI` reader - Length Error Interrupt"]
pub type LEI_R = crate::BitReader<LEI_A>;
#[doc = "Length Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEI_A {
    #[doc = "0: Length Error Interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: Length Error Interrupt is enabled"]
    VALUE2 = 1,
}
impl From<LEI_A> for bool {
    #[inline(always)]
    fn from(variant: LEI_A) -> Self {
        variant as u8 != 0
    }
}
impl LEI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEI_A {
        match self.bits {
            false => LEI_A::VALUE1,
            true => LEI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LEI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LEI_A::VALUE2
    }
}
#[doc = "Field `LEI` writer - Length Error Interrupt"]
pub type LEI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, LEI_A, O>;
impl<'a, const O: u8> LEI_W<'a, O> {
    #[doc = "Length Error Interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(LEI_A::VALUE1)
    }
    #[doc = "Length Error Interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(LEI_A::VALUE2)
    }
}
#[doc = "Field `BEI` reader - Bus Error Interrupt"]
pub type BEI_R = crate::BitReader<BEI_A>;
#[doc = "Bus Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BEI_A {
    #[doc = "0: Bus Error Interrupt is disabled"]
    VALUE1 = 0,
    #[doc = "1: Bus Error Interrupt is enabled"]
    VALUE2 = 1,
}
impl From<BEI_A> for bool {
    #[inline(always)]
    fn from(variant: BEI_A) -> Self {
        variant as u8 != 0
    }
}
impl BEI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEI_A {
        match self.bits {
            false => BEI_A::VALUE1,
            true => BEI_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == BEI_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == BEI_A::VALUE2
    }
}
#[doc = "Field `BEI` writer - Bus Error Interrupt"]
pub type BEI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, BEI_A, O>;
impl<'a, const O: u8> BEI_W<'a, O> {
    #[doc = "Bus Error Interrupt is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(BEI_A::VALUE1)
    }
    #[doc = "Bus Error Interrupt is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(BEI_A::VALUE2)
    }
}
#[doc = "Field `CCE` reader - CRC Check Comparison"]
pub type CCE_R = crate::BitReader<CCE_A>;
#[doc = "CRC Check Comparison\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCE_A {
    #[doc = "0: CRC check comparison at the end of a message is disabled"]
    VALUE1 = 0,
    #[doc = "1: CRC check comparison at the end of a message is enabled"]
    VALUE2 = 1,
}
impl From<CCE_A> for bool {
    #[inline(always)]
    fn from(variant: CCE_A) -> Self {
        variant as u8 != 0
    }
}
impl CCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCE_A {
        match self.bits {
            false => CCE_A::VALUE1,
            true => CCE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == CCE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == CCE_A::VALUE2
    }
}
#[doc = "Field `CCE` writer - CRC Check Comparison"]
pub type CCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, CCE_A, O>;
impl<'a, const O: u8> CCE_W<'a, O> {
    #[doc = "CRC check comparison at the end of a message is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(CCE_A::VALUE1)
    }
    #[doc = "CRC check comparison at the end of a message is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(CCE_A::VALUE2)
    }
}
#[doc = "Field `ALR` reader - Automatic Length Reload"]
pub type ALR_R = crate::BitReader<ALR_A>;
#[doc = "Automatic Length Reload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALR_A {
    #[doc = "0: Disables automatic reload of the LENGTH field."]
    VALUE1 = 0,
    #[doc = "1: Enables automatic reload of the LENGTH field at the end of a message."]
    VALUE2 = 1,
}
impl From<ALR_A> for bool {
    #[inline(always)]
    fn from(variant: ALR_A) -> Self {
        variant as u8 != 0
    }
}
impl ALR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALR_A {
        match self.bits {
            false => ALR_A::VALUE1,
            true => ALR_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ALR_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ALR_A::VALUE2
    }
}
#[doc = "Field `ALR` writer - Automatic Length Reload"]
pub type ALR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, ALR_A, O>;
impl<'a, const O: u8> ALR_W<'a, O> {
    #[doc = "Disables automatic reload of the LENGTH field."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ALR_A::VALUE1)
    }
    #[doc = "Enables automatic reload of the LENGTH field at the end of a message."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(ALR_A::VALUE2)
    }
}
#[doc = "Field `REFIN` reader - IR Byte Wise Reflection"]
pub type REFIN_R = crate::BitReader<REFIN_A>;
#[doc = "IR Byte Wise Reflection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFIN_A {
    #[doc = "0: IR Byte Wise Reflection is disabled"]
    VALUE1 = 0,
    #[doc = "1: IR Byte Wise Reflection is enabled"]
    VALUE2 = 1,
}
impl From<REFIN_A> for bool {
    #[inline(always)]
    fn from(variant: REFIN_A) -> Self {
        variant as u8 != 0
    }
}
impl REFIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFIN_A {
        match self.bits {
            false => REFIN_A::VALUE1,
            true => REFIN_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REFIN_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REFIN_A::VALUE2
    }
}
#[doc = "Field `REFIN` writer - IR Byte Wise Reflection"]
pub type REFIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, REFIN_A, O>;
impl<'a, const O: u8> REFIN_W<'a, O> {
    #[doc = "IR Byte Wise Reflection is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REFIN_A::VALUE1)
    }
    #[doc = "IR Byte Wise Reflection is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REFIN_A::VALUE2)
    }
}
#[doc = "Field `REFOUT` reader - CRC 32-Bit Wise Reflection"]
pub type REFOUT_R = crate::BitReader<REFOUT_A>;
#[doc = "CRC 32-Bit Wise Reflection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFOUT_A {
    #[doc = "0: CRC 32-bit wise is disabled"]
    VALUE1 = 0,
    #[doc = "1: CRC 32-bit wise is enabled"]
    VALUE2 = 1,
}
impl From<REFOUT_A> for bool {
    #[inline(always)]
    fn from(variant: REFOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REFOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFOUT_A {
        match self.bits {
            false => REFOUT_A::VALUE1,
            true => REFOUT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REFOUT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REFOUT_A::VALUE2
    }
}
#[doc = "Field `REFOUT` writer - CRC 32-Bit Wise Reflection"]
pub type REFOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, REFOUT_A, O>;
impl<'a, const O: u8> REFOUT_W<'a, O> {
    #[doc = "CRC 32-bit wise is disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REFOUT_A::VALUE1)
    }
    #[doc = "CRC 32-bit wise is enabled"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(REFOUT_A::VALUE2)
    }
}
#[doc = "Field `XSEL` reader - Selects the value to be xored with the final CRC"]
pub type XSEL_R = crate::BitReader<XSEL_A>;
#[doc = "Selects the value to be xored with the final CRC\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XSEL_A {
    #[doc = "0: 0x00000000"]
    VALUE1 = 0,
    #[doc = "1: 0xFFFFFFFF"]
    VALUE2 = 1,
}
impl From<XSEL_A> for bool {
    #[inline(always)]
    fn from(variant: XSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl XSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XSEL_A {
        match self.bits {
            false => XSEL_A::VALUE1,
            true => XSEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == XSEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == XSEL_A::VALUE2
    }
}
#[doc = "Field `XSEL` writer - Selects the value to be xored with the final CRC"]
pub type XSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, XSEL_A, O>;
impl<'a, const O: u8> XSEL_W<'a, O> {
    #[doc = "0x00000000"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(XSEL_A::VALUE1)
    }
    #[doc = "0xFFFFFFFF"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(XSEL_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - CRC Mismatch Interrupt"]
    #[inline(always)]
    pub fn cmi(&self) -> CMI_R {
        CMI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuration Error Interrupt"]
    #[inline(always)]
    pub fn cei(&self) -> CEI_R {
        CEI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Length Error Interrupt"]
    #[inline(always)]
    pub fn lei(&self) -> LEI_R {
        LEI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bus Error Interrupt"]
    #[inline(always)]
    pub fn bei(&self) -> BEI_R {
        BEI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC Check Comparison"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic Length Reload"]
    #[inline(always)]
    pub fn alr(&self) -> ALR_R {
        ALR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - IR Byte Wise Reflection"]
    #[inline(always)]
    pub fn refin(&self) -> REFIN_R {
        REFIN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CRC 32-Bit Wise Reflection"]
    #[inline(always)]
    pub fn refout(&self) -> REFOUT_R {
        REFOUT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects the value to be xored with the final CRC"]
    #[inline(always)]
    pub fn xsel(&self) -> XSEL_R {
        XSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Mismatch Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cmi(&mut self) -> CMI_W<0> {
        CMI_W::new(self)
    }
    #[doc = "Bit 1 - Configuration Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cei(&mut self) -> CEI_W<1> {
        CEI_W::new(self)
    }
    #[doc = "Bit 2 - Length Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lei(&mut self) -> LEI_W<2> {
        LEI_W::new(self)
    }
    #[doc = "Bit 3 - Bus Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bei(&mut self) -> BEI_W<3> {
        BEI_W::new(self)
    }
    #[doc = "Bit 4 - CRC Check Comparison"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CCE_W<4> {
        CCE_W::new(self)
    }
    #[doc = "Bit 5 - Automatic Length Reload"]
    #[inline(always)]
    #[must_use]
    pub fn alr(&mut self) -> ALR_W<5> {
        ALR_W::new(self)
    }
    #[doc = "Bit 8 - IR Byte Wise Reflection"]
    #[inline(always)]
    #[must_use]
    pub fn refin(&mut self) -> REFIN_W<8> {
        REFIN_W::new(self)
    }
    #[doc = "Bit 9 - CRC 32-Bit Wise Reflection"]
    #[inline(always)]
    #[must_use]
    pub fn refout(&mut self) -> REFOUT_W<9> {
        REFOUT_W::new(self)
    }
    #[doc = "Bit 10 - Selects the value to be xored with the final CRC"]
    #[inline(always)]
    #[must_use]
    pub fn xsel(&mut self) -> XSEL_W<10> {
        XSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0x0700"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0700;
}
