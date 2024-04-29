#[doc = "Register `BRSMR` reader"]
pub type R = crate::R<BRSMR_SPEC>;
#[doc = "Register `BRSMR` writer"]
pub type W = crate::W<BRSMR_SPEC>;
#[doc = "Enable Gate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENGT_A {
    #[doc = "0: No conversion requests are issued"]
    VALUE1 = 0,
    #[doc = "1: Conversion requests are issued if at least one pending bit is set"]
    VALUE2 = 1,
    #[doc = "2: Conversion requests are issued if at least one pending bit is set and REQGTx = 1."]
    VALUE3 = 2,
    #[doc = "3: Conversion requests are issued if at least one pending bit is set and REQGTx = 0."]
    VALUE4 = 3,
}
impl From<ENGT_A> for u8 {
    #[inline(always)]
    fn from(variant: ENGT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ENGT_A {
    type Ux = u8;
}
impl crate::IsEnum for ENGT_A {}
#[doc = "Field `ENGT` reader - Enable Gate"]
pub type ENGT_R = crate::FieldReader<ENGT_A>;
impl ENGT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENGT_A {
        match self.bits {
            0 => ENGT_A::VALUE1,
            1 => ENGT_A::VALUE2,
            2 => ENGT_A::VALUE3,
            3 => ENGT_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "No conversion requests are issued"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENGT_A::VALUE1
    }
    #[doc = "Conversion requests are issued if at least one pending bit is set"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENGT_A::VALUE2
    }
    #[doc = "Conversion requests are issued if at least one pending bit is set and REQGTx = 1."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == ENGT_A::VALUE3
    }
    #[doc = "Conversion requests are issued if at least one pending bit is set and REQGTx = 0."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == ENGT_A::VALUE4
    }
}
#[doc = "Field `ENGT` writer - Enable Gate"]
pub type ENGT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ENGT_A, crate::Safe>;
impl<'a, REG> ENGT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No conversion requests are issued"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ENGT_A::VALUE1)
    }
    #[doc = "Conversion requests are issued if at least one pending bit is set"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ENGT_A::VALUE2)
    }
    #[doc = "Conversion requests are issued if at least one pending bit is set and REQGTx = 1."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(ENGT_A::VALUE3)
    }
    #[doc = "Conversion requests are issued if at least one pending bit is set and REQGTx = 0."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(ENGT_A::VALUE4)
    }
}
#[doc = "Enable External Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENTR_A {
    #[doc = "0: External trigger disabled"]
    VALUE1 = 0,
    #[doc = "1: The selected edge at the selected trigger input signal REQTR generates the load event"]
    VALUE2 = 1,
}
impl From<ENTR_A> for bool {
    #[inline(always)]
    fn from(variant: ENTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENTR` reader - Enable External Trigger"]
pub type ENTR_R = crate::BitReader<ENTR_A>;
impl ENTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENTR_A {
        match self.bits {
            false => ENTR_A::VALUE1,
            true => ENTR_A::VALUE2,
        }
    }
    #[doc = "External trigger disabled"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENTR_A::VALUE1
    }
    #[doc = "The selected edge at the selected trigger input signal REQTR generates the load event"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENTR_A::VALUE2
    }
}
#[doc = "Field `ENTR` writer - Enable External Trigger"]
pub type ENTR_W<'a, REG> = crate::BitWriter<'a, REG, ENTR_A>;
impl<'a, REG> ENTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External trigger disabled"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ENTR_A::VALUE1)
    }
    #[doc = "The selected edge at the selected trigger input signal REQTR generates the load event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ENTR_A::VALUE2)
    }
}
#[doc = "Enable Source Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENSI_A {
    #[doc = "0: No request source interrupt"]
    VALUE1 = 0,
    #[doc = "1: A request source interrupt is generated upon a request source event (last pending conversion is finished)"]
    VALUE2 = 1,
}
impl From<ENSI_A> for bool {
    #[inline(always)]
    fn from(variant: ENSI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENSI` reader - Enable Source Interrupt"]
pub type ENSI_R = crate::BitReader<ENSI_A>;
impl ENSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENSI_A {
        match self.bits {
            false => ENSI_A::VALUE1,
            true => ENSI_A::VALUE2,
        }
    }
    #[doc = "No request source interrupt"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENSI_A::VALUE1
    }
    #[doc = "A request source interrupt is generated upon a request source event (last pending conversion is finished)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == ENSI_A::VALUE2
    }
}
#[doc = "Field `ENSI` writer - Enable Source Interrupt"]
pub type ENSI_W<'a, REG> = crate::BitWriter<'a, REG, ENSI_A>;
impl<'a, REG> ENSI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No request source interrupt"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ENSI_A::VALUE1)
    }
    #[doc = "A request source interrupt is generated upon a request source event (last pending conversion is finished)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ENSI_A::VALUE2)
    }
}
#[doc = "Autoscan Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCAN_A {
    #[doc = "0: No autoscan"]
    VALUE1 = 0,
    #[doc = "1: Autoscan functionality enabled: a request source event automatically generates a load event"]
    VALUE2 = 1,
}
impl From<SCAN_A> for bool {
    #[inline(always)]
    fn from(variant: SCAN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCAN` reader - Autoscan Enable"]
pub type SCAN_R = crate::BitReader<SCAN_A>;
impl SCAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCAN_A {
        match self.bits {
            false => SCAN_A::VALUE1,
            true => SCAN_A::VALUE2,
        }
    }
    #[doc = "No autoscan"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SCAN_A::VALUE1
    }
    #[doc = "Autoscan functionality enabled: a request source event automatically generates a load event"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SCAN_A::VALUE2
    }
}
#[doc = "Field `SCAN` writer - Autoscan Enable"]
pub type SCAN_W<'a, REG> = crate::BitWriter<'a, REG, SCAN_A>;
impl<'a, REG> SCAN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No autoscan"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SCAN_A::VALUE1)
    }
    #[doc = "Autoscan functionality enabled: a request source event automatically generates a load event"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SCAN_A::VALUE2)
    }
}
#[doc = "Autoscan Source Load Event Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDM_A {
    #[doc = "0: Overwrite mode: Copy all bits from the select registers to the pending registers upon a load event"]
    VALUE1 = 0,
    #[doc = "1: Combine mode: Set all pending bits that are set in the select registers upon a load event (logic OR)"]
    VALUE2 = 1,
}
impl From<LDM_A> for bool {
    #[inline(always)]
    fn from(variant: LDM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDM` reader - Autoscan Source Load Event Mode"]
pub type LDM_R = crate::BitReader<LDM_A>;
impl LDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LDM_A {
        match self.bits {
            false => LDM_A::VALUE1,
            true => LDM_A::VALUE2,
        }
    }
    #[doc = "Overwrite mode: Copy all bits from the select registers to the pending registers upon a load event"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == LDM_A::VALUE1
    }
    #[doc = "Combine mode: Set all pending bits that are set in the select registers upon a load event (logic OR)"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == LDM_A::VALUE2
    }
}
#[doc = "Field `LDM` writer - Autoscan Source Load Event Mode"]
pub type LDM_W<'a, REG> = crate::BitWriter<'a, REG, LDM_A>;
impl<'a, REG> LDM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overwrite mode: Copy all bits from the select registers to the pending registers upon a load event"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LDM_A::VALUE1)
    }
    #[doc = "Combine mode: Set all pending bits that are set in the select registers upon a load event (logic OR)"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LDM_A::VALUE2)
    }
}
#[doc = "Request Gate Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REQGT_A {
    #[doc = "0: The gate input is low"]
    VALUE1 = 0,
    #[doc = "1: The gate input is high"]
    VALUE2 = 1,
}
impl From<REQGT_A> for bool {
    #[inline(always)]
    fn from(variant: REQGT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQGT` reader - Request Gate Level"]
pub type REQGT_R = crate::BitReader<REQGT_A>;
impl REQGT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> REQGT_A {
        match self.bits {
            false => REQGT_A::VALUE1,
            true => REQGT_A::VALUE2,
        }
    }
    #[doc = "The gate input is low"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REQGT_A::VALUE1
    }
    #[doc = "The gate input is high"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == REQGT_A::VALUE2
    }
}
#[doc = "Clear Pending Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRPND_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: The bits in registers BRSPNDx are cleared"]
    VALUE2 = 1,
}
impl From<CLRPND_A> for bool {
    #[inline(always)]
    fn from(variant: CLRPND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRPND` writer - Clear Pending Bits"]
pub type CLRPND_W<'a, REG> = crate::BitWriter<'a, REG, CLRPND_A>;
impl<'a, REG> CLRPND_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CLRPND_A::VALUE1)
    }
    #[doc = "The bits in registers BRSPNDx are cleared"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CLRPND_A::VALUE2)
    }
}
#[doc = "Generate Load Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDEV_A {
    #[doc = "0: No action"]
    VALUE1 = 0,
    #[doc = "1: A load event is generated"]
    VALUE2 = 1,
}
impl From<LDEV_A> for bool {
    #[inline(always)]
    fn from(variant: LDEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDEV` writer - Generate Load Event"]
pub type LDEV_W<'a, REG> = crate::BitWriter<'a, REG, LDEV_A>;
impl<'a, REG> LDEV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(LDEV_A::VALUE1)
    }
    #[doc = "A load event is generated"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(LDEV_A::VALUE2)
    }
}
#[doc = "Repeat Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPTDIS_A {
    #[doc = "0: A cancelled conversion is repeated"]
    VALUE1 = 0,
    #[doc = "1: A cancelled conversion is discarded"]
    VALUE2 = 1,
}
impl From<RPTDIS_A> for bool {
    #[inline(always)]
    fn from(variant: RPTDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPTDIS` reader - Repeat Disable"]
pub type RPTDIS_R = crate::BitReader<RPTDIS_A>;
impl RPTDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RPTDIS_A {
        match self.bits {
            false => RPTDIS_A::VALUE1,
            true => RPTDIS_A::VALUE2,
        }
    }
    #[doc = "A cancelled conversion is repeated"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RPTDIS_A::VALUE1
    }
    #[doc = "A cancelled conversion is discarded"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == RPTDIS_A::VALUE2
    }
}
#[doc = "Field `RPTDIS` writer - Repeat Disable"]
pub type RPTDIS_W<'a, REG> = crate::BitWriter<'a, REG, RPTDIS_A>;
impl<'a, REG> RPTDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A cancelled conversion is repeated"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(RPTDIS_A::VALUE1)
    }
    #[doc = "A cancelled conversion is discarded"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(RPTDIS_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Enable Gate"]
    #[inline(always)]
    pub fn engt(&self) -> ENGT_R {
        ENGT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Enable External Trigger"]
    #[inline(always)]
    pub fn entr(&self) -> ENTR_R {
        ENTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Source Interrupt"]
    #[inline(always)]
    pub fn ensi(&self) -> ENSI_R {
        ENSI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Autoscan Enable"]
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Autoscan Source Load Event Mode"]
    #[inline(always)]
    pub fn ldm(&self) -> LDM_R {
        LDM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Request Gate Level"]
    #[inline(always)]
    pub fn reqgt(&self) -> REQGT_R {
        REQGT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Repeat Disable"]
    #[inline(always)]
    pub fn rptdis(&self) -> RPTDIS_R {
        RPTDIS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Enable Gate"]
    #[inline(always)]
    #[must_use]
    pub fn engt(&mut self) -> ENGT_W<BRSMR_SPEC> {
        ENGT_W::new(self, 0)
    }
    #[doc = "Bit 2 - Enable External Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn entr(&mut self) -> ENTR_W<BRSMR_SPEC> {
        ENTR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Source Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ensi(&mut self) -> ENSI_W<BRSMR_SPEC> {
        ENSI_W::new(self, 3)
    }
    #[doc = "Bit 4 - Autoscan Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scan(&mut self) -> SCAN_W<BRSMR_SPEC> {
        SCAN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Autoscan Source Load Event Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ldm(&mut self) -> LDM_W<BRSMR_SPEC> {
        LDM_W::new(self, 5)
    }
    #[doc = "Bit 8 - Clear Pending Bits"]
    #[inline(always)]
    #[must_use]
    pub fn clrpnd(&mut self) -> CLRPND_W<BRSMR_SPEC> {
        CLRPND_W::new(self, 8)
    }
    #[doc = "Bit 9 - Generate Load Event"]
    #[inline(always)]
    #[must_use]
    pub fn ldev(&mut self) -> LDEV_W<BRSMR_SPEC> {
        LDEV_W::new(self, 9)
    }
    #[doc = "Bit 16 - Repeat Disable"]
    #[inline(always)]
    #[must_use]
    pub fn rptdis(&mut self) -> RPTDIS_W<BRSMR_SPEC> {
        RPTDIS_W::new(self, 16)
    }
}
#[doc = "Background Request Source Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brsmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brsmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRSMR_SPEC;
impl crate::RegisterSpec for BRSMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brsmr::R`](R) reader structure"]
impl crate::Readable for BRSMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brsmr::W`](W) writer structure"]
impl crate::Writable for BRSMR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRSMR to value 0"]
impl crate::Resettable for BRSMR_SPEC {
    const RESET_VALUE: u32 = 0;
}
