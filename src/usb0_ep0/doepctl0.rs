#[doc = "Register `DOEPCTL0` reader"]
pub type R = crate::R<Doepctl0Spec>;
#[doc = "Register `DOEPCTL0` writer"]
pub type W = crate::W<Doepctl0Spec>;
#[doc = "Maximum Packet Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mps {
    #[doc = "0: 64 bytes"]
    Value1 = 0,
    #[doc = "1: 32 bytes"]
    Value2 = 1,
    #[doc = "2: 16 bytes"]
    Value3 = 2,
    #[doc = "3: 8 bytes"]
    Value4 = 3,
}
impl From<Mps> for u8 {
    #[inline(always)]
    fn from(variant: Mps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mps {
    type Ux = u8;
}
#[doc = "Field `MPS` reader - Maximum Packet Size"]
pub type MpsR = crate::FieldReader<Mps>;
impl MpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mps {
        match self.bits {
            0 => Mps::Value1,
            1 => Mps::Value2,
            2 => Mps::Value3,
            3 => Mps::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Mps::Value1
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Mps::Value2
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Mps::Value3
    }
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Mps::Value4
    }
}
#[doc = "Field `USBActEP` reader - USB Active Endpoint"]
pub type UsbactEpR = crate::BitReader;
#[doc = "NAK Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Naksts {
    #[doc = "0: The core is transmitting non-NAK handshakes based on the FIFO status."]
    Value1 = 0,
    #[doc = "1: The core is transmitting NAK handshakes on this endpoint."]
    Value2 = 1,
}
impl From<Naksts> for bool {
    #[inline(always)]
    fn from(variant: Naksts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NAKSts` reader - NAK Status"]
pub type NakstsR = crate::BitReader<Naksts>;
impl NakstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Naksts {
        match self.bits {
            false => Naksts::Value1,
            true => Naksts::Value2,
        }
    }
    #[doc = "The core is transmitting non-NAK handshakes based on the FIFO status."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Naksts::Value1
    }
    #[doc = "The core is transmitting NAK handshakes on this endpoint."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Naksts::Value2
    }
}
#[doc = "Field `EPType` reader - Endpoint Type"]
pub type EptypeR = crate::FieldReader;
#[doc = "Field `Snp` reader - Snoop Mode"]
pub type SnpR = crate::BitReader;
#[doc = "Field `Snp` writer - Snoop Mode"]
pub type SnpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Stall` reader - STALL Handshake"]
pub type StallR = crate::BitReader;
#[doc = "Field `Stall` writer - STALL Handshake"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDis` reader - Endpoint Disable"]
pub type EpdisR = crate::BitReader;
#[doc = "Field `EPEna` reader - Endpoint Enable"]
pub type EpenaR = crate::BitReader;
#[doc = "Field `EPEna` writer - Endpoint Enable"]
pub type EpenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&self) -> MpsR {
        MpsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    pub fn usbact_ep(&self) -> UsbactEpR {
        UsbactEpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - NAK Status"]
    #[inline(always)]
    pub fn naksts(&self) -> NakstsR {
        NakstsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EptypeR {
        EptypeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Snoop Mode"]
    #[inline(always)]
    pub fn snp(&self) -> SnpR {
        SnpR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - STALL Handshake"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    pub fn epdis(&self) -> EpdisR {
        EpdisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    pub fn epena(&self) -> EpenaR {
        EpenaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - Snoop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn snp(&mut self) -> SnpW<Doepctl0Spec> {
        SnpW::new(self, 20)
    }
    #[doc = "Bit 21 - STALL Handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> StallW<Doepctl0Spec> {
        StallW::new(self, 21)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CnakW<Doepctl0Spec> {
        CnakW::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SnakW<Doepctl0Spec> {
        SnakW::new(self, 27)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epena(&mut self) -> EpenaW<Doepctl0Spec> {
        EpenaW::new(self, 31)
    }
}
#[doc = "Device Control OUT Endpoint Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doepctl0Spec;
impl crate::RegisterSpec for Doepctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepctl0::R`](R) reader structure"]
impl crate::Readable for Doepctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`doepctl0::W`](W) writer structure"]
impl crate::Writable for Doepctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPCTL0 to value 0x8000"]
impl crate::Resettable for Doepctl0Spec {
    const RESET_VALUE: u32 = 0x8000;
}
