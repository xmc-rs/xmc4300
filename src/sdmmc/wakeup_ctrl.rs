#[doc = "Register `WAKEUP_CTRL` reader"]
pub type R = crate::R<WAKEUP_CTRL_SPEC>;
#[doc = "Register `WAKEUP_CTRL` writer"]
pub type W = crate::W<WAKEUP_CTRL_SPEC>;
#[doc = "Wakeup Event Enable On Card Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKEUP_EVENT_EN_INT_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<WAKEUP_EVENT_EN_INT_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP_EVENT_EN_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_EVENT_EN_INT` reader - Wakeup Event Enable On Card Interrupt"]
pub type WAKEUP_EVENT_EN_INT_R = crate::BitReader<WAKEUP_EVENT_EN_INT_A>;
impl WAKEUP_EVENT_EN_INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAKEUP_EVENT_EN_INT_A {
        match self.bits {
            false => WAKEUP_EVENT_EN_INT_A::VALUE1,
            true => WAKEUP_EVENT_EN_INT_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WAKEUP_EVENT_EN_INT_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WAKEUP_EVENT_EN_INT_A::VALUE2
    }
}
#[doc = "Field `WAKEUP_EVENT_EN_INT` writer - Wakeup Event Enable On Card Interrupt"]
pub type WAKEUP_EVENT_EN_INT_W<'a, REG> = crate::BitWriter<'a, REG, WAKEUP_EVENT_EN_INT_A>;
impl<'a, REG> WAKEUP_EVENT_EN_INT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WAKEUP_EVENT_EN_INT_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WAKEUP_EVENT_EN_INT_A::VALUE2)
    }
}
#[doc = "Wakeup Event Enable On SD Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKEUP_EVENT_EN_INS_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<WAKEUP_EVENT_EN_INS_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP_EVENT_EN_INS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_EVENT_EN_INS` reader - Wakeup Event Enable On SD Card Insertion"]
pub type WAKEUP_EVENT_EN_INS_R = crate::BitReader<WAKEUP_EVENT_EN_INS_A>;
impl WAKEUP_EVENT_EN_INS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAKEUP_EVENT_EN_INS_A {
        match self.bits {
            false => WAKEUP_EVENT_EN_INS_A::VALUE1,
            true => WAKEUP_EVENT_EN_INS_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WAKEUP_EVENT_EN_INS_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WAKEUP_EVENT_EN_INS_A::VALUE2
    }
}
#[doc = "Field `WAKEUP_EVENT_EN_INS` writer - Wakeup Event Enable On SD Card Insertion"]
pub type WAKEUP_EVENT_EN_INS_W<'a, REG> = crate::BitWriter<'a, REG, WAKEUP_EVENT_EN_INS_A>;
impl<'a, REG> WAKEUP_EVENT_EN_INS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WAKEUP_EVENT_EN_INS_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WAKEUP_EVENT_EN_INS_A::VALUE2)
    }
}
#[doc = "Wakeup Event Enable On SD Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKEUP_EVENT_EN_REM_A {
    #[doc = "0: Disable"]
    VALUE1 = 0,
    #[doc = "1: Enable"]
    VALUE2 = 1,
}
impl From<WAKEUP_EVENT_EN_REM_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUP_EVENT_EN_REM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP_EVENT_EN_REM` reader - Wakeup Event Enable On SD Card Removal"]
pub type WAKEUP_EVENT_EN_REM_R = crate::BitReader<WAKEUP_EVENT_EN_REM_A>;
impl WAKEUP_EVENT_EN_REM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WAKEUP_EVENT_EN_REM_A {
        match self.bits {
            false => WAKEUP_EVENT_EN_REM_A::VALUE1,
            true => WAKEUP_EVENT_EN_REM_A::VALUE2,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == WAKEUP_EVENT_EN_REM_A::VALUE1
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == WAKEUP_EVENT_EN_REM_A::VALUE2
    }
}
#[doc = "Field `WAKEUP_EVENT_EN_REM` writer - Wakeup Event Enable On SD Card Removal"]
pub type WAKEUP_EVENT_EN_REM_W<'a, REG> = crate::BitWriter<'a, REG, WAKEUP_EVENT_EN_REM_A>;
impl<'a, REG> WAKEUP_EVENT_EN_REM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(WAKEUP_EVENT_EN_REM_A::VALUE1)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(WAKEUP_EVENT_EN_REM_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup Event Enable On Card Interrupt"]
    #[inline(always)]
    pub fn wakeup_event_en_int(&self) -> WAKEUP_EVENT_EN_INT_R {
        WAKEUP_EVENT_EN_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup Event Enable On SD Card Insertion"]
    #[inline(always)]
    pub fn wakeup_event_en_ins(&self) -> WAKEUP_EVENT_EN_INS_R {
        WAKEUP_EVENT_EN_INS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup Event Enable On SD Card Removal"]
    #[inline(always)]
    pub fn wakeup_event_en_rem(&self) -> WAKEUP_EVENT_EN_REM_R {
        WAKEUP_EVENT_EN_REM_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Event Enable On Card Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_event_en_int(&mut self) -> WAKEUP_EVENT_EN_INT_W<WAKEUP_CTRL_SPEC> {
        WAKEUP_EVENT_EN_INT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Wakeup Event Enable On SD Card Insertion"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_event_en_ins(&mut self) -> WAKEUP_EVENT_EN_INS_W<WAKEUP_CTRL_SPEC> {
        WAKEUP_EVENT_EN_INS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup Event Enable On SD Card Removal"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_event_en_rem(&mut self) -> WAKEUP_EVENT_EN_REM_W<WAKEUP_CTRL_SPEC> {
        WAKEUP_EVENT_EN_REM_W::new(self, 2)
    }
}
#[doc = "Wake-up Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeup_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WAKEUP_CTRL_SPEC;
impl crate::RegisterSpec for WAKEUP_CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wakeup_ctrl::R`](R) reader structure"]
impl crate::Readable for WAKEUP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wakeup_ctrl::W`](W) writer structure"]
impl crate::Writable for WAKEUP_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WAKEUP_CTRL to value 0"]
impl crate::Resettable for WAKEUP_CTRL_SPEC {
    const RESET_VALUE: u8 = 0;
}
