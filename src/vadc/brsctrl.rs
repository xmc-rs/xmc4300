#[doc = "Register `BRSCTRL` reader"]
pub type R = crate::R<BRSCTRL_SPEC>;
#[doc = "Register `BRSCTRL` writer"]
pub type W = crate::W<BRSCTRL_SPEC>;
#[doc = "Source-specific Result Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRCRESREG_A {
    #[doc = "0: Use GxCHCTRy.RESREG to select a group result register"]
    VALUE1 = 0,
    #[doc = "1: Store result in group result register GxRES1"]
    VALUE2 = 1,
    #[doc = "15: Store result in group result register GxRES15"]
    VALUE3 = 15,
}
impl From<SRCRESREG_A> for u8 {
    #[inline(always)]
    fn from(variant: SRCRESREG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SRCRESREG_A {
    type Ux = u8;
}
impl crate::IsEnum for SRCRESREG_A {}
#[doc = "Field `SRCRESREG` reader - Source-specific Result Register"]
pub type SRCRESREG_R = crate::FieldReader<SRCRESREG_A>;
impl SRCRESREG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SRCRESREG_A> {
        match self.bits {
            0 => Some(SRCRESREG_A::VALUE1),
            1 => Some(SRCRESREG_A::VALUE2),
            15 => Some(SRCRESREG_A::VALUE3),
            _ => None,
        }
    }
    #[doc = "Use GxCHCTRy.RESREG to select a group result register"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == SRCRESREG_A::VALUE1
    }
    #[doc = "Store result in group result register GxRES1"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == SRCRESREG_A::VALUE2
    }
    #[doc = "Store result in group result register GxRES15"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == SRCRESREG_A::VALUE3
    }
}
#[doc = "Field `SRCRESREG` writer - Source-specific Result Register"]
pub type SRCRESREG_W<'a, REG> = crate::FieldWriter<'a, REG, 4, SRCRESREG_A>;
impl<'a, REG> SRCRESREG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use GxCHCTRy.RESREG to select a group result register"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SRCRESREG_A::VALUE1)
    }
    #[doc = "Store result in group result register GxRES1"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SRCRESREG_A::VALUE2)
    }
    #[doc = "Store result in group result register GxRES15"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(SRCRESREG_A::VALUE3)
    }
}
#[doc = "Field `XTSEL` reader - External Trigger Input Selection"]
pub type XTSEL_R = crate::FieldReader;
#[doc = "Field `XTSEL` writer - External Trigger Input Selection"]
pub type XTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `XTLVL` reader - External Trigger Level"]
pub type XTLVL_R = crate::BitReader;
#[doc = "Trigger Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum XTMODE_A {
    #[doc = "0: No external trigger"]
    VALUE1 = 0,
    #[doc = "1: Trigger event upon a falling edge"]
    VALUE2 = 1,
    #[doc = "2: Trigger event upon a rising edge"]
    VALUE3 = 2,
    #[doc = "3: Trigger event upon any edge"]
    VALUE4 = 3,
}
impl From<XTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: XTMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for XTMODE_A {
    type Ux = u8;
}
impl crate::IsEnum for XTMODE_A {}
#[doc = "Field `XTMODE` reader - Trigger Operating Mode"]
pub type XTMODE_R = crate::FieldReader<XTMODE_A>;
impl XTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XTMODE_A {
        match self.bits {
            0 => XTMODE_A::VALUE1,
            1 => XTMODE_A::VALUE2,
            2 => XTMODE_A::VALUE3,
            3 => XTMODE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "No external trigger"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == XTMODE_A::VALUE1
    }
    #[doc = "Trigger event upon a falling edge"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == XTMODE_A::VALUE2
    }
    #[doc = "Trigger event upon a rising edge"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == XTMODE_A::VALUE3
    }
    #[doc = "Trigger event upon any edge"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == XTMODE_A::VALUE4
    }
}
#[doc = "Field `XTMODE` writer - Trigger Operating Mode"]
pub type XTMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, XTMODE_A, crate::Safe>;
impl<'a, REG> XTMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No external trigger"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(XTMODE_A::VALUE1)
    }
    #[doc = "Trigger event upon a falling edge"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(XTMODE_A::VALUE2)
    }
    #[doc = "Trigger event upon a rising edge"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(XTMODE_A::VALUE3)
    }
    #[doc = "Trigger event upon any edge"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(XTMODE_A::VALUE4)
    }
}
#[doc = "Write Control for Trigger Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XTWC_A {
    #[doc = "0: No write access to trigger configuration"]
    VALUE1 = 0,
    #[doc = "1: Bitfields XTMODE and XTSEL can be written"]
    VALUE2 = 1,
}
impl From<XTWC_A> for bool {
    #[inline(always)]
    fn from(variant: XTWC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTWC` writer - Write Control for Trigger Configuration"]
pub type XTWC_W<'a, REG> = crate::BitWriter<'a, REG, XTWC_A>;
impl<'a, REG> XTWC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write access to trigger configuration"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(XTWC_A::VALUE1)
    }
    #[doc = "Bitfields XTMODE and XTSEL can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(XTWC_A::VALUE2)
    }
}
#[doc = "Field `GTSEL` reader - Gate Input Selection"]
pub type GTSEL_R = crate::FieldReader;
#[doc = "Field `GTSEL` writer - Gate Input Selection"]
pub type GTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GTLVL` reader - Gate Input Level"]
pub type GTLVL_R = crate::BitReader;
#[doc = "Write Control for Gate Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GTWC_A {
    #[doc = "0: No write access to gate configuration"]
    VALUE1 = 0,
    #[doc = "1: Bitfield GTSEL can be written"]
    VALUE2 = 1,
}
impl From<GTWC_A> for bool {
    #[inline(always)]
    fn from(variant: GTWC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GTWC` writer - Write Control for Gate Configuration"]
pub type GTWC_W<'a, REG> = crate::BitWriter<'a, REG, GTWC_A>;
impl<'a, REG> GTWC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No write access to gate configuration"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(GTWC_A::VALUE1)
    }
    #[doc = "Bitfield GTSEL can be written"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(GTWC_A::VALUE2)
    }
}
impl R {
    #[doc = "Bits 0:3 - Source-specific Result Register"]
    #[inline(always)]
    pub fn srcresreg(&self) -> SRCRESREG_R {
        SRCRESREG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - External Trigger Input Selection"]
    #[inline(always)]
    pub fn xtsel(&self) -> XTSEL_R {
        XTSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - External Trigger Level"]
    #[inline(always)]
    pub fn xtlvl(&self) -> XTLVL_R {
        XTLVL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Trigger Operating Mode"]
    #[inline(always)]
    pub fn xtmode(&self) -> XTMODE_R {
        XTMODE_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Gate Input Selection"]
    #[inline(always)]
    pub fn gtsel(&self) -> GTSEL_R {
        GTSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Gate Input Level"]
    #[inline(always)]
    pub fn gtlvl(&self) -> GTLVL_R {
        GTLVL_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Source-specific Result Register"]
    #[inline(always)]
    pub fn srcresreg(&mut self) -> SRCRESREG_W<BRSCTRL_SPEC> {
        SRCRESREG_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - External Trigger Input Selection"]
    #[inline(always)]
    pub fn xtsel(&mut self) -> XTSEL_W<BRSCTRL_SPEC> {
        XTSEL_W::new(self, 8)
    }
    #[doc = "Bits 13:14 - Trigger Operating Mode"]
    #[inline(always)]
    pub fn xtmode(&mut self) -> XTMODE_W<BRSCTRL_SPEC> {
        XTMODE_W::new(self, 13)
    }
    #[doc = "Bit 15 - Write Control for Trigger Configuration"]
    #[inline(always)]
    pub fn xtwc(&mut self) -> XTWC_W<BRSCTRL_SPEC> {
        XTWC_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Gate Input Selection"]
    #[inline(always)]
    pub fn gtsel(&mut self) -> GTSEL_W<BRSCTRL_SPEC> {
        GTSEL_W::new(self, 16)
    }
    #[doc = "Bit 23 - Write Control for Gate Configuration"]
    #[inline(always)]
    pub fn gtwc(&mut self) -> GTWC_W<BRSCTRL_SPEC> {
        GTWC_W::new(self, 23)
    }
}
#[doc = "Background Request Source Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`brsctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brsctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRSCTRL_SPEC;
impl crate::RegisterSpec for BRSCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brsctrl::R`](R) reader structure"]
impl crate::Readable for BRSCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brsctrl::W`](W) writer structure"]
impl crate::Writable for BRSCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRSCTRL to value 0"]
impl crate::Resettable for BRSCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
