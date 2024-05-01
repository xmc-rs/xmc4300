#[doc = "Register `DOEPCTL_ISOCONT` reader"]
pub type R = crate::R<DoepctlIsocontSpec>;
#[doc = "Register `DOEPCTL_ISOCONT` writer"]
pub type W = crate::W<DoepctlIsocontSpec>;
#[doc = "Field `MPS` reader - Maximum Packet Size"]
pub type MpsR = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - Maximum Packet Size"]
pub type MpsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `USBActEP` reader - USB Active Endpoint"]
pub type UsbactEpR = crate::BitReader;
#[doc = "Field `USBActEP` writer - USB Active Endpoint"]
pub type UsbactEpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Even/Odd Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EoFrNum {
    #[doc = "0: Even frame"]
    Value1 = 0,
    #[doc = "1: Odd rame"]
    Value2 = 1,
}
impl From<EoFrNum> for bool {
    #[inline(always)]
    fn from(variant: EoFrNum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EO_FrNum` reader - Even/Odd Frame"]
pub type EoFrNumR = crate::BitReader<EoFrNum>;
impl EoFrNumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EoFrNum {
        match self.bits {
            false => EoFrNum::Value1,
            true => EoFrNum::Value2,
        }
    }
    #[doc = "Even frame"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EoFrNum::Value1
    }
    #[doc = "Odd rame"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EoFrNum::Value2
    }
}
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
#[doc = "Endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eptype {
    #[doc = "0: Control"]
    Value1 = 0,
    #[doc = "1: Isochronous"]
    Value2 = 1,
    #[doc = "2: Bulk"]
    Value3 = 2,
    #[doc = "3: Interrupt"]
    Value4 = 3,
}
impl From<Eptype> for u8 {
    #[inline(always)]
    fn from(variant: Eptype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eptype {
    type Ux = u8;
}
impl crate::IsEnum for Eptype {}
#[doc = "Field `EPType` reader - Endpoint Type"]
pub type EptypeR = crate::FieldReader<Eptype>;
impl EptypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eptype {
        match self.bits {
            0 => Eptype::Value1,
            1 => Eptype::Value2,
            2 => Eptype::Value3,
            3 => Eptype::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Eptype::Value1
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Eptype::Value2
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Eptype::Value3
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Eptype::Value4
    }
}
#[doc = "Field `EPType` writer - Endpoint Type"]
pub type EptypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Eptype, crate::Safe>;
impl<'a, REG> EptypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Value1)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Value2)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Value3)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Value4)
    }
}
#[doc = "Field `Snp` reader - Snoop Mode"]
pub type SnpR = crate::BitReader;
#[doc = "Field `Snp` writer - Snoop Mode"]
pub type SnpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Stall` reader - STALL Handshake"]
pub type StallR = crate::BitReader;
#[doc = "Field `Stall` writer - STALL Handshake"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxFNum` reader - TxFIFO Number"]
pub type TxFnumR = crate::FieldReader;
#[doc = "Field `TxFNum` writer - TxFIFO Number"]
pub type TxFnumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SetEvenFr` writer - In non-Scatter/Gather DMA mode: Set Even frame"]
pub type SetEvenFrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SetOddFr` writer - Set Odd frame"]
pub type SetOddFrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDis` reader - Endpoint Disable"]
pub type EpdisR = crate::BitReader;
#[doc = "Field `EPDis` writer - Endpoint Disable"]
pub type EpdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPEna` reader - Endpoint Enable"]
pub type EpenaR = crate::BitReader;
#[doc = "Field `EPEna` writer - Endpoint Enable"]
pub type EpenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&self) -> MpsR {
        MpsR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    pub fn usbact_ep(&self) -> UsbactEpR {
        UsbactEpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Even/Odd Frame"]
    #[inline(always)]
    pub fn eo_fr_num(&self) -> EoFrNumR {
        EoFrNumR::new(((self.bits >> 16) & 1) != 0)
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
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline(always)]
    pub fn tx_fnum(&self) -> TxFnumR {
        TxFnumR::new(((self.bits >> 22) & 0x0f) as u8)
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
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    #[must_use]
    pub fn mps(&mut self) -> MpsW<DoepctlIsocontSpec> {
        MpsW::new(self, 0)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn usbact_ep(&mut self) -> UsbactEpW<DoepctlIsocontSpec> {
        UsbactEpW::new(self, 15)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EptypeW<DoepctlIsocontSpec> {
        EptypeW::new(self, 18)
    }
    #[doc = "Bit 20 - Snoop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn snp(&mut self) -> SnpW<DoepctlIsocontSpec> {
        SnpW::new(self, 20)
    }
    #[doc = "Bit 21 - STALL Handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> StallW<DoepctlIsocontSpec> {
        StallW::new(self, 21)
    }
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fnum(&mut self) -> TxFnumW<DoepctlIsocontSpec> {
        TxFnumW::new(self, 22)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CnakW<DoepctlIsocontSpec> {
        CnakW::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SnakW<DoepctlIsocontSpec> {
        SnakW::new(self, 27)
    }
    #[doc = "Bit 28 - In non-Scatter/Gather DMA mode: Set Even frame"]
    #[inline(always)]
    #[must_use]
    pub fn set_even_fr(&mut self) -> SetEvenFrW<DoepctlIsocontSpec> {
        SetEvenFrW::new(self, 28)
    }
    #[doc = "Bit 29 - Set Odd frame"]
    #[inline(always)]
    #[must_use]
    pub fn set_odd_fr(&mut self) -> SetOddFrW<DoepctlIsocontSpec> {
        SetOddFrW::new(self, 29)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    #[must_use]
    pub fn epdis(&mut self) -> EpdisW<DoepctlIsocontSpec> {
        EpdisW::new(self, 30)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epena(&mut self) -> EpenaW<DoepctlIsocontSpec> {
        EpenaW::new(self, 31)
    }
}
#[doc = "Device Endpoint Control Register \\[ISOCONT\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl_isocont::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl_isocont::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoepctlIsocontSpec;
impl crate::RegisterSpec for DoepctlIsocontSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepctl_isocont::R`](R) reader structure"]
impl crate::Readable for DoepctlIsocontSpec {}
#[doc = "`write(|w| ..)` method takes [`doepctl_isocont::W`](W) writer structure"]
impl crate::Writable for DoepctlIsocontSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPCTL_ISOCONT to value 0"]
impl crate::Resettable for DoepctlIsocontSpec {
    const RESET_VALUE: u32 = 0;
}
