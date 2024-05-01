#[doc = "Register `USBCLKCR` reader"]
pub type R = crate::R<UsbclkcrSpec>;
#[doc = "Register `USBCLKCR` writer"]
pub type W = crate::W<UsbclkcrSpec>;
#[doc = "Field `USBDIV` reader - USB Clock Divider Value"]
pub type UsbdivR = crate::FieldReader;
#[doc = "Field `USBDIV` writer - USB Clock Divider Value"]
pub type UsbdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "USB Clock Selection Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbsel {
    #[doc = "0: USB PLL Clock"]
    Const0 = 0,
    #[doc = "1: PLL Clock"]
    Const1 = 1,
}
impl From<Usbsel> for bool {
    #[inline(always)]
    fn from(variant: Usbsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBSEL` reader - USB Clock Selection Value"]
pub type UsbselR = crate::BitReader<Usbsel>;
impl UsbselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbsel {
        match self.bits {
            false => Usbsel::Const0,
            true => Usbsel::Const1,
        }
    }
    #[doc = "USB PLL Clock"]
    #[inline(always)]
    pub fn is_const_0(&self) -> bool {
        *self == Usbsel::Const0
    }
    #[doc = "PLL Clock"]
    #[inline(always)]
    pub fn is_const_1(&self) -> bool {
        *self == Usbsel::Const1
    }
}
#[doc = "Field `USBSEL` writer - USB Clock Selection Value"]
pub type UsbselW<'a, REG> = crate::BitWriter<'a, REG, Usbsel>;
impl<'a, REG> UsbselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB PLL Clock"]
    #[inline(always)]
    pub fn const_0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbsel::Const0)
    }
    #[doc = "PLL Clock"]
    #[inline(always)]
    pub fn const_1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbsel::Const1)
    }
}
impl R {
    #[doc = "Bits 0:2 - USB Clock Divider Value"]
    #[inline(always)]
    pub fn usbdiv(&self) -> UsbdivR {
        UsbdivR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 16 - USB Clock Selection Value"]
    #[inline(always)]
    pub fn usbsel(&self) -> UsbselR {
        UsbselR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - USB Clock Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn usbdiv(&mut self) -> UsbdivW<UsbclkcrSpec> {
        UsbdivW::new(self, 0)
    }
    #[doc = "Bit 16 - USB Clock Selection Value"]
    #[inline(always)]
    #[must_use]
    pub fn usbsel(&mut self) -> UsbselW<UsbclkcrSpec> {
        UsbselW::new(self, 16)
    }
}
#[doc = "USB Clock Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbclkcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbclkcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbclkcrSpec;
impl crate::RegisterSpec for UsbclkcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbclkcr::R`](R) reader structure"]
impl crate::Readable for UsbclkcrSpec {}
#[doc = "`write(|w| ..)` method takes [`usbclkcr::W`](W) writer structure"]
impl crate::Writable for UsbclkcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBCLKCR to value 0"]
impl crate::Resettable for UsbclkcrSpec {
    const RESET_VALUE: u32 = 0;
}
