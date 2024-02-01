#[doc = "Register `DOEPCTL_ISOCONT` reader"]
pub type R = crate::R<DOEPCTL_ISOCONT_SPEC>;
#[doc = "Register `DOEPCTL_ISOCONT` writer"]
pub type W = crate::W<DOEPCTL_ISOCONT_SPEC>;
#[doc = "Field `MPS` reader - Maximum Packet Size"]
pub type MPS_R = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - Maximum Packet Size"]
pub type MPS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `USBActEP` reader - USB Active Endpoint"]
pub type USBACT_EP_R = crate::BitReader;
#[doc = "Field `USBActEP` writer - USB Active Endpoint"]
pub type USBACT_EP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EO_FrNum` reader - Even/Odd Frame"]
pub type EO_FR_NUM_R = crate::BitReader<EO_FR_NUM_A>;
#[doc = "Even/Odd Frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EO_FR_NUM_A {
    #[doc = "0: Even frame"]
    VALUE1 = 0,
    #[doc = "1: Odd rame"]
    VALUE2 = 1,
}
impl From<EO_FR_NUM_A> for bool {
    #[inline(always)]
    fn from(variant: EO_FR_NUM_A) -> Self {
        variant as u8 != 0
    }
}
impl EO_FR_NUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EO_FR_NUM_A {
        match self.bits {
            false => EO_FR_NUM_A::VALUE1,
            true => EO_FR_NUM_A::VALUE2,
        }
    }
    #[doc = "Even frame"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EO_FR_NUM_A::VALUE1
    }
    #[doc = "Odd rame"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EO_FR_NUM_A::VALUE2
    }
}
#[doc = "Field `NAKSts` reader - NAK Status"]
pub type NAKSTS_R = crate::BitReader<NAKSTS_A>;
#[doc = "NAK Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NAKSTS_A {
    #[doc = "0: The core is transmitting non-NAK handshakes based on the FIFO status."]
    VALUE1 = 0,
    #[doc = "1: The core is transmitting NAK handshakes on this endpoint."]
    VALUE2 = 1,
}
impl From<NAKSTS_A> for bool {
    #[inline(always)]
    fn from(variant: NAKSTS_A) -> Self {
        variant as u8 != 0
    }
}
impl NAKSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NAKSTS_A {
        match self.bits {
            false => NAKSTS_A::VALUE1,
            true => NAKSTS_A::VALUE2,
        }
    }
    #[doc = "The core is transmitting non-NAK handshakes based on the FIFO status."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NAKSTS_A::VALUE1
    }
    #[doc = "The core is transmitting NAK handshakes on this endpoint."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NAKSTS_A::VALUE2
    }
}
#[doc = "Field `EPType` reader - Endpoint Type"]
pub type EPTYPE_R = crate::FieldReader<EPTYPE_A>;
#[doc = "Endpoint Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EPTYPE_A {
    #[doc = "0: Control"]
    VALUE1 = 0,
    #[doc = "1: Isochronous"]
    VALUE2 = 1,
    #[doc = "2: Bulk"]
    VALUE3 = 2,
    #[doc = "3: Interrupt"]
    VALUE4 = 3,
}
impl From<EPTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPTYPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EPTYPE_A {
    type Ux = u8;
}
impl EPTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EPTYPE_A {
        match self.bits {
            0 => EPTYPE_A::VALUE1,
            1 => EPTYPE_A::VALUE2,
            2 => EPTYPE_A::VALUE3,
            3 => EPTYPE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EPTYPE_A::VALUE1
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EPTYPE_A::VALUE2
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EPTYPE_A::VALUE3
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EPTYPE_A::VALUE4
    }
}
#[doc = "Field `EPType` writer - Endpoint Type"]
pub type EPTYPE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, EPTYPE_A>;
impl<'a, REG> EPTYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::VALUE1)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::VALUE2)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::VALUE3)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(EPTYPE_A::VALUE4)
    }
}
#[doc = "Field `Snp` reader - Snoop Mode"]
pub type SNP_R = crate::BitReader;
#[doc = "Field `Snp` writer - Snoop Mode"]
pub type SNP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Stall` reader - STALL Handshake"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `Stall` writer - STALL Handshake"]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxFNum` reader - TxFIFO Number"]
pub type TX_FNUM_R = crate::FieldReader;
#[doc = "Field `TxFNum` writer - TxFIFO Number"]
pub type TX_FNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SetEvenFr` writer - In non-Scatter/Gather DMA mode: Set Even frame"]
pub type SET_EVEN_FR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SetOddFr` writer - Set Odd frame"]
pub type SET_ODD_FR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDis` reader - Endpoint Disable"]
pub type EPDIS_R = crate::BitReader;
#[doc = "Field `EPDis` writer - Endpoint Disable"]
pub type EPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPEna` reader - Endpoint Enable"]
pub type EPENA_R = crate::BitReader;
#[doc = "Field `EPEna` writer - Endpoint Enable"]
pub type EPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    pub fn usbact_ep(&self) -> USBACT_EP_R {
        USBACT_EP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Even/Odd Frame"]
    #[inline(always)]
    pub fn eo_fr_num(&self) -> EO_FR_NUM_R {
        EO_FR_NUM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NAK Status"]
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Snoop Mode"]
    #[inline(always)]
    pub fn snp(&self) -> SNP_R {
        SNP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - STALL Handshake"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline(always)]
    pub fn tx_fnum(&self) -> TX_FNUM_R {
        TX_FNUM_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Packet Size"]
    #[inline(always)]
    #[must_use]
    pub fn mps(&mut self) -> MPS_W<DOEPCTL_ISOCONT_SPEC> {
        MPS_W::new(self, 0)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn usbact_ep(&mut self) -> USBACT_EP_W<DOEPCTL_ISOCONT_SPEC> {
        USBACT_EP_W::new(self, 15)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<DOEPCTL_ISOCONT_SPEC> {
        EPTYPE_W::new(self, 18)
    }
    #[doc = "Bit 20 - Snoop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn snp(&mut self) -> SNP_W<DOEPCTL_ISOCONT_SPEC> {
        SNP_W::new(self, 20)
    }
    #[doc = "Bit 21 - STALL Handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<DOEPCTL_ISOCONT_SPEC> {
        STALL_W::new(self, 21)
    }
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fnum(&mut self) -> TX_FNUM_W<DOEPCTL_ISOCONT_SPEC> {
        TX_FNUM_W::new(self, 22)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<DOEPCTL_ISOCONT_SPEC> {
        CNAK_W::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<DOEPCTL_ISOCONT_SPEC> {
        SNAK_W::new(self, 27)
    }
    #[doc = "Bit 28 - In non-Scatter/Gather DMA mode: Set Even frame"]
    #[inline(always)]
    #[must_use]
    pub fn set_even_fr(&mut self) -> SET_EVEN_FR_W<DOEPCTL_ISOCONT_SPEC> {
        SET_EVEN_FR_W::new(self, 28)
    }
    #[doc = "Bit 29 - Set Odd frame"]
    #[inline(always)]
    #[must_use]
    pub fn set_odd_fr(&mut self) -> SET_ODD_FR_W<DOEPCTL_ISOCONT_SPEC> {
        SET_ODD_FR_W::new(self, 29)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    #[must_use]
    pub fn epdis(&mut self) -> EPDIS_W<DOEPCTL_ISOCONT_SPEC> {
        EPDIS_W::new(self, 30)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epena(&mut self) -> EPENA_W<DOEPCTL_ISOCONT_SPEC> {
        EPENA_W::new(self, 31)
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
#[doc = "Device Endpoint Control Register \\[ISOCONT\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl_isocont::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl_isocont::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPCTL_ISOCONT_SPEC;
impl crate::RegisterSpec for DOEPCTL_ISOCONT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepctl_isocont::R`](R) reader structure"]
impl crate::Readable for DOEPCTL_ISOCONT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepctl_isocont::W`](W) writer structure"]
impl crate::Writable for DOEPCTL_ISOCONT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPCTL_ISOCONT to value 0"]
impl crate::Resettable for DOEPCTL_ISOCONT_SPEC {
    const RESET_VALUE: u32 = 0;
}
