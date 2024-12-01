#[doc = "Register `PRCLR1` writer"]
pub type W = crate::W<PRCLR1_SPEC>;
#[doc = "LEDTS Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEDTSCU0RS_A {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: De-assert reset"]
    CONST_1 = 1,
}
impl From<LEDTSCU0RS_A> for bool {
    #[inline(always)]
    fn from(variant: LEDTSCU0RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEDTSCU0RS` writer - LEDTS Reset Clear"]
pub type LEDTSCU0RS_W<'a, REG> = crate::BitWriter<'a, REG, LEDTSCU0RS_A>;
impl<'a, REG> LEDTSCU0RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(LEDTSCU0RS_A::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(LEDTSCU0RS_A::CONST_1)
    }
}
#[doc = "MultiCAN Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCAN0RS_A {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: De-assert reset"]
    CONST_1 = 1,
}
impl From<MCAN0RS_A> for bool {
    #[inline(always)]
    fn from(variant: MCAN0RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCAN0RS` writer - MultiCAN Reset Clear"]
pub type MCAN0RS_W<'a, REG> = crate::BitWriter<'a, REG, MCAN0RS_A>;
impl<'a, REG> MCAN0RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(MCAN0RS_A::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(MCAN0RS_A::CONST_1)
    }
}
#[doc = "DAC Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACRS_A {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: De-assert reset"]
    CONST_1 = 1,
}
impl From<DACRS_A> for bool {
    #[inline(always)]
    fn from(variant: DACRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACRS` writer - DAC Reset Clear"]
pub type DACRS_W<'a, REG> = crate::BitWriter<'a, REG, DACRS_A>;
impl<'a, REG> DACRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(DACRS_A::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(DACRS_A::CONST_1)
    }
}
#[doc = "MMC Interface Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMCIRS_A {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: De-assert reset"]
    CONST_1 = 1,
}
impl From<MMCIRS_A> for bool {
    #[inline(always)]
    fn from(variant: MMCIRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCIRS` writer - MMC Interface Reset Clear"]
pub type MMCIRS_W<'a, REG> = crate::BitWriter<'a, REG, MMCIRS_A>;
impl<'a, REG> MMCIRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(MMCIRS_A::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(MMCIRS_A::CONST_1)
    }
}
#[doc = "USIC1 Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USIC1RS_A {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: De-assert reset"]
    CONST_1 = 1,
}
impl From<USIC1RS_A> for bool {
    #[inline(always)]
    fn from(variant: USIC1RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USIC1RS` writer - USIC1 Reset Clear"]
pub type USIC1RS_W<'a, REG> = crate::BitWriter<'a, REG, USIC1RS_A>;
impl<'a, REG> USIC1RS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(USIC1RS_A::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(USIC1RS_A::CONST_1)
    }
}
#[doc = "PORTS Reset Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PPORTSRS_A {
    #[doc = "0: No effect"]
    CONST_0 = 0,
    #[doc = "1: De-assert reset"]
    CONST_1 = 1,
}
impl From<PPORTSRS_A> for bool {
    #[inline(always)]
    fn from(variant: PPORTSRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPORTSRS` writer - PORTS Reset Clear"]
pub type PPORTSRS_W<'a, REG> = crate::BitWriter<'a, REG, PPORTSRS_A>;
impl<'a, REG> PPORTSRS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PPORTSRS_A::CONST_0)
    }
    #[doc = "De-assert reset"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PPORTSRS_A::CONST_1)
    }
}
impl W {
    #[doc = "Bit 3 - LEDTS Reset Clear"]
    #[inline(always)]
    pub fn ledtscu0rs(&mut self) -> LEDTSCU0RS_W<PRCLR1_SPEC> {
        LEDTSCU0RS_W::new(self, 3)
    }
    #[doc = "Bit 4 - MultiCAN Reset Clear"]
    #[inline(always)]
    pub fn mcan0rs(&mut self) -> MCAN0RS_W<PRCLR1_SPEC> {
        MCAN0RS_W::new(self, 4)
    }
    #[doc = "Bit 5 - DAC Reset Clear"]
    #[inline(always)]
    pub fn dacrs(&mut self) -> DACRS_W<PRCLR1_SPEC> {
        DACRS_W::new(self, 5)
    }
    #[doc = "Bit 6 - MMC Interface Reset Clear"]
    #[inline(always)]
    pub fn mmcirs(&mut self) -> MMCIRS_W<PRCLR1_SPEC> {
        MMCIRS_W::new(self, 6)
    }
    #[doc = "Bit 7 - USIC1 Reset Clear"]
    #[inline(always)]
    pub fn usic1rs(&mut self) -> USIC1RS_W<PRCLR1_SPEC> {
        USIC1RS_W::new(self, 7)
    }
    #[doc = "Bit 9 - PORTS Reset Clear"]
    #[inline(always)]
    pub fn pportsrs(&mut self) -> PPORTSRS_W<PRCLR1_SPEC> {
        PPORTSRS_W::new(self, 9)
    }
}
#[doc = "RCU Peripheral 1 Reset Clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prclr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRCLR1_SPEC;
impl crate::RegisterSpec for PRCLR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prclr1::W`](W) writer structure"]
impl crate::Writable for PRCLR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRCLR1 to value 0"]
impl crate::Resettable for PRCLR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
