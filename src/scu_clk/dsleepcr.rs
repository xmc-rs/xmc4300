#[doc = "Register `DSLEEPCR` reader"]
pub type R = crate::R<DSLEEPCR_SPEC>;
#[doc = "Register `DSLEEPCR` writer"]
pub type W = crate::W<DSLEEPCR_SPEC>;
#[doc = "System Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSSEL_A {
    #[doc = "0: fOFI clock"]
    CONST_0 = 0,
    #[doc = "1: fPLL clock"]
    CONST_1 = 1,
}
impl From<SYSSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SYSSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSSEL` reader - System Clock Selection Value"]
pub type SYSSEL_R = crate::BitReader<SYSSEL_A>;
impl SYSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYSSEL_A {
        match self.bits {
            false => SYSSEL_A::CONST_0,
            true => SYSSEL_A::CONST_1,
        }
    }
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == SYSSEL_A::CONST_0
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == SYSSEL_A::CONST_1
    }
}
#[doc = "Field `SYSSEL` writer - System Clock Selection Value"]
pub type SYSSEL_W<'a, REG> = crate::BitWriter<'a, REG, SYSSEL_A>;
impl<'a, REG> SYSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fOFI clock"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(SYSSEL_A::CONST_0)
    }
    #[doc = "fPLL clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(SYSSEL_A::CONST_1)
    }
}
#[doc = "Flash Power Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPDN_A {
    #[doc = "1: Flash power down module"]
    CONST_1 = 1,
    #[doc = "0: No effect"]
    CONST_0 = 0,
}
impl From<FPDN_A> for bool {
    #[inline(always)]
    fn from(variant: FPDN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPDN` reader - Flash Power Down"]
pub type FPDN_R = crate::BitReader<FPDN_A>;
impl FPDN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPDN_A {
        match self.bits {
            true => FPDN_A::CONST_1,
            false => FPDN_A::CONST_0,
        }
    }
    #[doc = "Flash power down module"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == FPDN_A::CONST_1
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == FPDN_A::CONST_0
    }
}
#[doc = "Field `FPDN` writer - Flash Power Down"]
pub type FPDN_W<'a, REG> = crate::BitWriter<'a, REG, FPDN_A>;
impl<'a, REG> FPDN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash power down module"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(FPDN_A::CONST_1)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(FPDN_A::CONST_0)
    }
}
#[doc = "PLL Power Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLPDN_A {
    #[doc = "1: Switch off main PLL"]
    CONST_1 = 1,
    #[doc = "0: No effect"]
    CONST_0 = 0,
}
impl From<PLLPDN_A> for bool {
    #[inline(always)]
    fn from(variant: PLLPDN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLPDN` reader - PLL Power Down"]
pub type PLLPDN_R = crate::BitReader<PLLPDN_A>;
impl PLLPDN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLPDN_A {
        match self.bits {
            true => PLLPDN_A::CONST_1,
            false => PLLPDN_A::CONST_0,
        }
    }
    #[doc = "Switch off main PLL"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == PLLPDN_A::CONST_1
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == PLLPDN_A::CONST_0
    }
}
#[doc = "Field `PLLPDN` writer - PLL Power Down"]
pub type PLLPDN_W<'a, REG> = crate::BitWriter<'a, REG, PLLPDN_A>;
impl<'a, REG> PLLPDN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Switch off main PLL"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDN_A::CONST_1)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDN_A::CONST_0)
    }
}
#[doc = "VCO Power Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCOPDN_A {
    #[doc = "1: Switch off VCO of main PLL"]
    CONST_1 = 1,
    #[doc = "0: No effect"]
    CONST_0 = 0,
}
impl From<VCOPDN_A> for bool {
    #[inline(always)]
    fn from(variant: VCOPDN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCOPDN` reader - VCO Power Down"]
pub type VCOPDN_R = crate::BitReader<VCOPDN_A>;
impl VCOPDN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VCOPDN_A {
        match self.bits {
            true => VCOPDN_A::CONST_1,
            false => VCOPDN_A::CONST_0,
        }
    }
    #[doc = "Switch off VCO of main PLL"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == VCOPDN_A::CONST_1
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == VCOPDN_A::CONST_0
    }
}
#[doc = "Field `VCOPDN` writer - VCO Power Down"]
pub type VCOPDN_W<'a, REG> = crate::BitWriter<'a, REG, VCOPDN_A>;
impl<'a, REG> VCOPDN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Switch off VCO of main PLL"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(VCOPDN_A::CONST_1)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(VCOPDN_A::CONST_0)
    }
}
#[doc = "USB Clock Control in Deep Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBCR_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<USBCR_A> for bool {
    #[inline(always)]
    fn from(variant: USBCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBCR` reader - USB Clock Control in Deep Sleep Mode"]
pub type USBCR_R = crate::BitReader<USBCR_A>;
impl USBCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> USBCR_A {
        match self.bits {
            false => USBCR_A::CONST_0,
            true => USBCR_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == USBCR_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == USBCR_A::CONST_1
    }
}
#[doc = "Field `USBCR` writer - USB Clock Control in Deep Sleep Mode"]
pub type USBCR_W<'a, REG> = crate::BitWriter<'a, REG, USBCR_A>;
impl<'a, REG> USBCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(USBCR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(USBCR_A::CONST_1)
    }
}
#[doc = "MMC Clock Control in Deep Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MMCCR_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<MMCCR_A> for bool {
    #[inline(always)]
    fn from(variant: MMCCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMCCR` reader - MMC Clock Control in Deep Sleep Mode"]
pub type MMCCR_R = crate::BitReader<MMCCR_A>;
impl MMCCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MMCCR_A {
        match self.bits {
            false => MMCCR_A::CONST_0,
            true => MMCCR_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == MMCCR_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == MMCCR_A::CONST_1
    }
}
#[doc = "Field `MMCCR` writer - MMC Clock Control in Deep Sleep Mode"]
pub type MMCCR_W<'a, REG> = crate::BitWriter<'a, REG, MMCCR_A>;
impl<'a, REG> MMCCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(MMCCR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(MMCCR_A::CONST_1)
    }
}
#[doc = "Ethernet Clock Control in Deep Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETH0CR_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<ETH0CR_A> for bool {
    #[inline(always)]
    fn from(variant: ETH0CR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETH0CR` reader - Ethernet Clock Control in Deep Sleep Mode"]
pub type ETH0CR_R = crate::BitReader<ETH0CR_A>;
impl ETH0CR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ETH0CR_A {
        match self.bits {
            false => ETH0CR_A::CONST_0,
            true => ETH0CR_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == ETH0CR_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == ETH0CR_A::CONST_1
    }
}
#[doc = "Field `ETH0CR` writer - Ethernet Clock Control in Deep Sleep Mode"]
pub type ETH0CR_W<'a, REG> = crate::BitWriter<'a, REG, ETH0CR_A>;
impl<'a, REG> ETH0CR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0CR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(ETH0CR_A::CONST_1)
    }
}
#[doc = "CCU Clock Control in Deep Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUCR_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<CCUCR_A> for bool {
    #[inline(always)]
    fn from(variant: CCUCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCUCR` reader - CCU Clock Control in Deep Sleep Mode"]
pub type CCUCR_R = crate::BitReader<CCUCR_A>;
impl CCUCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CCUCR_A {
        match self.bits {
            false => CCUCR_A::CONST_0,
            true => CCUCR_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == CCUCR_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == CCUCR_A::CONST_1
    }
}
#[doc = "Field `CCUCR` writer - CCU Clock Control in Deep Sleep Mode"]
pub type CCUCR_W<'a, REG> = crate::BitWriter<'a, REG, CCUCR_A>;
impl<'a, REG> CCUCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(CCUCR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(CCUCR_A::CONST_1)
    }
}
#[doc = "WDT Clock Control in Deep Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTCR_A {
    #[doc = "0: Disabled"]
    CONST_0 = 0,
    #[doc = "1: Enabled"]
    CONST_1 = 1,
}
impl From<WDTCR_A> for bool {
    #[inline(always)]
    fn from(variant: WDTCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCR` reader - WDT Clock Control in Deep Sleep Mode"]
pub type WDTCR_R = crate::BitReader<WDTCR_A>;
impl WDTCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WDTCR_A {
        match self.bits {
            false => WDTCR_A::CONST_0,
            true => WDTCR_A::CONST_1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == WDTCR_A::CONST_0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == WDTCR_A::CONST_1
    }
}
#[doc = "Field `WDTCR` writer - WDT Clock Control in Deep Sleep Mode"]
pub type WDTCR_W<'a, REG> = crate::BitWriter<'a, REG, WDTCR_A>;
impl<'a, REG> WDTCR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(WDTCR_A::CONST_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(WDTCR_A::CONST_1)
    }
}
impl R {
    #[doc = "Bit 0 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&self) -> SYSSEL_R {
        SYSSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - Flash Power Down"]
    #[inline(always)]
    pub fn fpdn(&self) -> FPDN_R {
        FPDN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL Power Down"]
    #[inline(always)]
    pub fn pllpdn(&self) -> PLLPDN_R {
        PLLPDN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VCO Power Down"]
    #[inline(always)]
    pub fn vcopdn(&self) -> VCOPDN_R {
        VCOPDN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - USB Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn usbcr(&self) -> USBCR_R {
        USBCR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMC Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn mmccr(&self) -> MMCCR_R {
        MMCCR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Ethernet Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn eth0cr(&self) -> ETH0CR_R {
        ETH0CR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - CCU Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn ccucr(&self) -> CCUCR_R {
        CCUCR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - WDT Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn wdtcr(&self) -> WDTCR_R {
        WDTCR_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System Clock Selection Value"]
    #[inline(always)]
    pub fn syssel(&mut self) -> SYSSEL_W<DSLEEPCR_SPEC> {
        SYSSEL_W::new(self, 0)
    }
    #[doc = "Bit 11 - Flash Power Down"]
    #[inline(always)]
    pub fn fpdn(&mut self) -> FPDN_W<DSLEEPCR_SPEC> {
        FPDN_W::new(self, 11)
    }
    #[doc = "Bit 12 - PLL Power Down"]
    #[inline(always)]
    pub fn pllpdn(&mut self) -> PLLPDN_W<DSLEEPCR_SPEC> {
        PLLPDN_W::new(self, 12)
    }
    #[doc = "Bit 13 - VCO Power Down"]
    #[inline(always)]
    pub fn vcopdn(&mut self) -> VCOPDN_W<DSLEEPCR_SPEC> {
        VCOPDN_W::new(self, 13)
    }
    #[doc = "Bit 16 - USB Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn usbcr(&mut self) -> USBCR_W<DSLEEPCR_SPEC> {
        USBCR_W::new(self, 16)
    }
    #[doc = "Bit 17 - MMC Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn mmccr(&mut self) -> MMCCR_W<DSLEEPCR_SPEC> {
        MMCCR_W::new(self, 17)
    }
    #[doc = "Bit 18 - Ethernet Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn eth0cr(&mut self) -> ETH0CR_W<DSLEEPCR_SPEC> {
        ETH0CR_W::new(self, 18)
    }
    #[doc = "Bit 20 - CCU Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn ccucr(&mut self) -> CCUCR_W<DSLEEPCR_SPEC> {
        CCUCR_W::new(self, 20)
    }
    #[doc = "Bit 21 - WDT Clock Control in Deep Sleep Mode"]
    #[inline(always)]
    pub fn wdtcr(&mut self) -> WDTCR_W<DSLEEPCR_SPEC> {
        WDTCR_W::new(self, 21)
    }
}
#[doc = "Deep Sleep Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsleepcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsleepcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSLEEPCR_SPEC;
impl crate::RegisterSpec for DSLEEPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsleepcr::R`](R) reader structure"]
impl crate::Readable for DSLEEPCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsleepcr::W`](W) writer structure"]
impl crate::Writable for DSLEEPCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSLEEPCR to value 0"]
impl crate::Resettable for DSLEEPCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
