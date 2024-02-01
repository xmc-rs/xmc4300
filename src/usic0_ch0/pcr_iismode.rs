#[doc = "Register `PCR_IISMode` reader"]
pub type R = crate::R<PCR_IISMODE_SPEC>;
#[doc = "Register `PCR_IISMode` writer"]
pub type W = crate::W<PCR_IISMODE_SPEC>;
#[doc = "Field `WAGEN` reader - WA Generation Enable"]
pub type WAGEN_R = crate::BitReader<WAGEN_A>;
#[doc = "WA Generation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl WAGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAGEN_A {
        match self.bits {
            false => WAGEN_A::VALUE1,
            true => WAGEN_A::VALUE2,
        }
    }
    #[doc = "The IIS can be used as slave. The generation of the word address signal is disabled. The output signal WA is 0. The MCLKO signal generation depends on PCR.MCLK."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WAGEN_A::VALUE1
    }
    #[doc = "The IIS can be used as master. The generation of the word address signal is enabled. The signal starts with a 0 after being enabled. The generation of MCLK is enabled, independent of PCR.MCLK. After clearing WAGEN, the USIC module stops the generation of the WA signal within the next 4 WA periods."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WAGEN_A::VALUE2
    }
}
#[doc = "Field `WAGEN` writer - WA Generation Enable"]
pub type WAGEN_W<'a, REG> = crate::BitWriter<'a, REG, WAGEN_A>;
impl<'a, REG> WAGEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The IIS can be used as slave. The generation of the word address signal is disabled. The output signal WA is 0. The MCLKO signal generation depends on PCR.MCLK."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WAGEN_A::VALUE1)
    }
    #[doc = "The IIS can be used as master. The generation of the word address signal is enabled. The signal starts with a 0 after being enabled. The generation of MCLK is enabled, independent of PCR.MCLK. After clearing WAGEN, the USIC module stops the generation of the WA signal within the next 4 WA periods."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WAGEN_A::VALUE2)
    }
}
#[doc = "Field `DTEN` reader - Data Transfers Enable"]
pub type DTEN_R = crate::BitReader<DTEN_A>;
#[doc = "Data Transfers Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTEN_A {
        match self.bits {
            false => DTEN_A::VALUE1,
            true => DTEN_A::VALUE2,
        }
    }
    #[doc = "The changes of the WA input signal are ignored and no transfers take place."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DTEN_A::VALUE1
    }
    #[doc = "Transfers are enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DTEN_A::VALUE2
    }
}
#[doc = "Field `DTEN` writer - Data Transfers Enable"]
pub type DTEN_W<'a, REG> = crate::BitWriter<'a, REG, DTEN_A>;
impl<'a, REG> DTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The changes of the WA input signal are ignored and no transfers take place."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DTEN_A::VALUE1)
    }
    #[doc = "Transfers are enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DTEN_A::VALUE2)
    }
}
#[doc = "Field `SELINV` reader - Select Inversion"]
pub type SELINV_R = crate::BitReader<SELINV_A>;
#[doc = "Select Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SELINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SELINV_A {
        match self.bits {
            false => SELINV_A::VALUE1,
            true => SELINV_A::VALUE2,
        }
    }
    #[doc = "The SELOx outputs have the same polarity as the WA signal."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SELINV_A::VALUE1
    }
    #[doc = "The SELOx outputs have the inverted polarity to the WA signal."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SELINV_A::VALUE2
    }
}
#[doc = "Field `SELINV` writer - Select Inversion"]
pub type SELINV_W<'a, REG> = crate::BitWriter<'a, REG, SELINV_A>;
impl<'a, REG> SELINV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The SELOx outputs have the same polarity as the WA signal."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SELINV_A::VALUE1)
    }
    #[doc = "The SELOx outputs have the inverted polarity to the WA signal."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SELINV_A::VALUE2)
    }
}
#[doc = "Field `WAFEIEN` reader - WA Falling Edge Interrupt Enable"]
pub type WAFEIEN_R = crate::BitReader<WAFEIEN_A>;
#[doc = "WA Falling Edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl WAFEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAFEIEN_A {
        match self.bits {
            false => WAFEIEN_A::VALUE1,
            true => WAFEIEN_A::VALUE2,
        }
    }
    #[doc = "A protocol interrupt is not activated if a falling edge of WA is generated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WAFEIEN_A::VALUE1
    }
    #[doc = "A protocol interrupt is activated if a falling edge of WA is generated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WAFEIEN_A::VALUE2
    }
}
#[doc = "Field `WAFEIEN` writer - WA Falling Edge Interrupt Enable"]
pub type WAFEIEN_W<'a, REG> = crate::BitWriter<'a, REG, WAFEIEN_A>;
impl<'a, REG> WAFEIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A protocol interrupt is not activated if a falling edge of WA is generated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WAFEIEN_A::VALUE1)
    }
    #[doc = "A protocol interrupt is activated if a falling edge of WA is generated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WAFEIEN_A::VALUE2)
    }
}
#[doc = "Field `WAREIEN` reader - WA Rising Edge Interrupt Enable"]
pub type WAREIEN_R = crate::BitReader<WAREIEN_A>;
#[doc = "WA Rising Edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl WAREIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAREIEN_A {
        match self.bits {
            false => WAREIEN_A::VALUE1,
            true => WAREIEN_A::VALUE2,
        }
    }
    #[doc = "A protocol interrupt is not activated if a rising edge of WA is generated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WAREIEN_A::VALUE1
    }
    #[doc = "A protocol interrupt is activated if a rising edge of WA is generated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WAREIEN_A::VALUE2
    }
}
#[doc = "Field `WAREIEN` writer - WA Rising Edge Interrupt Enable"]
pub type WAREIEN_W<'a, REG> = crate::BitWriter<'a, REG, WAREIEN_A>;
impl<'a, REG> WAREIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A protocol interrupt is not activated if a rising edge of WA is generated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WAREIEN_A::VALUE1)
    }
    #[doc = "A protocol interrupt is activated if a rising edge of WA is generated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WAREIEN_A::VALUE2)
    }
}
#[doc = "Field `ENDIEN` reader - END Interrupt Enable"]
pub type ENDIEN_R = crate::BitReader<ENDIEN_A>;
#[doc = "END Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ENDIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENDIEN_A {
        match self.bits {
            false => ENDIEN_A::VALUE1,
            true => ENDIEN_A::VALUE2,
        }
    }
    #[doc = "A protocol interrupt is not activated."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENDIEN_A::VALUE1
    }
    #[doc = "A protocol interrupt is activated."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENDIEN_A::VALUE2
    }
}
#[doc = "Field `ENDIEN` writer - END Interrupt Enable"]
pub type ENDIEN_W<'a, REG> = crate::BitWriter<'a, REG, ENDIEN_A>;
impl<'a, REG> ENDIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A protocol interrupt is not activated."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ENDIEN_A::VALUE1)
    }
    #[doc = "A protocol interrupt is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ENDIEN_A::VALUE2)
    }
}
#[doc = "Field `DX2TIEN` reader - DX2T Interrupt Enable"]
pub type DX2TIEN_R = crate::BitReader<DX2TIEN_A>;
#[doc = "DX2T Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl DX2TIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DX2TIEN_A {
        match self.bits {
            false => DX2TIEN_A::VALUE1,
            true => DX2TIEN_A::VALUE2,
        }
    }
    #[doc = "A protocol interrupt is not generated if DX2T is active."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DX2TIEN_A::VALUE1
    }
    #[doc = "A protocol interrupt is generated if DX2T is active."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DX2TIEN_A::VALUE2
    }
}
#[doc = "Field `DX2TIEN` writer - DX2T Interrupt Enable"]
pub type DX2TIEN_W<'a, REG> = crate::BitWriter<'a, REG, DX2TIEN_A>;
impl<'a, REG> DX2TIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A protocol interrupt is not generated if DX2T is active."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DX2TIEN_A::VALUE1)
    }
    #[doc = "A protocol interrupt is generated if DX2T is active."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DX2TIEN_A::VALUE2)
    }
}
#[doc = "Field `TDEL` reader - Transfer Delay"]
pub type TDEL_R = crate::FieldReader;
#[doc = "Field `TDEL` writer - Transfer Delay"]
pub type TDEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MCLK` reader - Master Clock Enable"]
pub type MCLK_R = crate::BitReader<MCLK_A>;
#[doc = "Master Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl MCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MCLK_A {
        match self.bits {
            false => MCLK_A::VALUE1,
            true => MCLK_A::VALUE2,
        }
    }
    #[doc = "The MCLK generation is disabled and MCLK is 0."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == MCLK_A::VALUE1
    }
    #[doc = "The MCLK generation is enabled."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == MCLK_A::VALUE2
    }
}
#[doc = "Field `MCLK` writer - Master Clock Enable"]
pub type MCLK_W<'a, REG> = crate::BitWriter<'a, REG, MCLK_A>;
impl<'a, REG> MCLK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The MCLK generation is disabled and MCLK is 0."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MCLK_A::VALUE1)
    }
    #[doc = "The MCLK generation is enabled."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MCLK_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - WA Generation Enable"]
    #[inline(always)]
    pub fn wagen(&self) -> WAGEN_R {
        WAGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Transfers Enable"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select Inversion"]
    #[inline(always)]
    pub fn selinv(&self) -> SELINV_R {
        SELINV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - WA Falling Edge Interrupt Enable"]
    #[inline(always)]
    pub fn wafeien(&self) -> WAFEIEN_R {
        WAFEIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WA Rising Edge Interrupt Enable"]
    #[inline(always)]
    pub fn wareien(&self) -> WAREIEN_R {
        WAREIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - END Interrupt Enable"]
    #[inline(always)]
    pub fn endien(&self) -> ENDIEN_R {
        ENDIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 15 - DX2T Interrupt Enable"]
    #[inline(always)]
    pub fn dx2tien(&self) -> DX2TIEN_R {
        DX2TIEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Transfer Delay"]
    #[inline(always)]
    pub fn tdel(&self) -> TDEL_R {
        TDEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    pub fn mclk(&self) -> MCLK_R {
        MCLK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WA Generation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wagen(&mut self) -> WAGEN_W<PCR_IISMODE_SPEC> {
        WAGEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data Transfers Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DTEN_W<PCR_IISMODE_SPEC> {
        DTEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Select Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn selinv(&mut self) -> SELINV_W<PCR_IISMODE_SPEC> {
        SELINV_W::new(self, 2)
    }
    #[doc = "Bit 4 - WA Falling Edge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wafeien(&mut self) -> WAFEIEN_W<PCR_IISMODE_SPEC> {
        WAFEIEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - WA Rising Edge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wareien(&mut self) -> WAREIEN_W<PCR_IISMODE_SPEC> {
        WAREIEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - END Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn endien(&mut self) -> ENDIEN_W<PCR_IISMODE_SPEC> {
        ENDIEN_W::new(self, 6)
    }
    #[doc = "Bit 15 - DX2T Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dx2tien(&mut self) -> DX2TIEN_W<PCR_IISMODE_SPEC> {
        DX2TIEN_W::new(self, 15)
    }
    #[doc = "Bits 16:21 - Transfer Delay"]
    #[inline(always)]
    #[must_use]
    pub fn tdel(&mut self) -> TDEL_W<PCR_IISMODE_SPEC> {
        TDEL_W::new(self, 16)
    }
    #[doc = "Bit 31 - Master Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mclk(&mut self) -> MCLK_W<PCR_IISMODE_SPEC> {
        MCLK_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Protocol Control Register \\[IIS Mode\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr_iismode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr_iismode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCR_IISMODE_SPEC;
impl crate::RegisterSpec for PCR_IISMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcr_iismode::R`](R) reader structure"]
impl crate::Readable for PCR_IISMODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pcr_iismode::W`](W) writer structure"]
impl crate::Writable for PCR_IISMODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCR_IISMode to value 0"]
impl crate::Resettable for PCR_IISMODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
