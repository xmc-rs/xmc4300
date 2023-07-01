#[doc = "Register `DIEPCTL_INTBULK` reader"]
pub struct R(crate::R<DIEPCTL_INTBULK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPCTL_INTBULK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPCTL_INTBULK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPCTL_INTBULK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPCTL_INTBULK` writer"]
pub struct W(crate::W<DIEPCTL_INTBULK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPCTL_INTBULK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DIEPCTL_INTBULK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPCTL_INTBULK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPS` reader - Maximum Packet Size"]
pub type MPS_R = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - Maximum Packet Size"]
pub type MPS_W<'a, const O: u8> = crate::FieldWriter<'a, DIEPCTL_INTBULK_SPEC, 11, O, u16>;
#[doc = "Field `USBActEP` reader - USB Active Endpoint"]
pub type USBACT_EP_R = crate::BitReader;
#[doc = "Field `USBActEP` writer - USB Active Endpoint"]
pub type USBACT_EP_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL_INTBULK_SPEC, O>;
#[doc = "Field `DPID` reader - Endpoint Data PID"]
pub type DPID_R = crate::BitReader<DPID_A>;
#[doc = "Endpoint Data PID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPID_A {
    #[doc = "0: DATA0"]
    VALUE1 = 0,
    #[doc = "1: DATA1"]
    VALUE2 = 1,
}
impl From<DPID_A> for bool {
    #[inline(always)]
    fn from(variant: DPID_A) -> Self {
        variant as u8 != 0
    }
}
impl DPID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPID_A {
        match self.bits {
            false => DPID_A::VALUE1,
            true => DPID_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DPID_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DPID_A::VALUE2
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
    pub fn variant(&self) -> NAKSTS_A {
        match self.bits {
            false => NAKSTS_A::VALUE1,
            true => NAKSTS_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NAKSTS_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
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
    pub fn variant(&self) -> EPTYPE_A {
        match self.bits {
            0 => EPTYPE_A::VALUE1,
            1 => EPTYPE_A::VALUE2,
            2 => EPTYPE_A::VALUE3,
            3 => EPTYPE_A::VALUE4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == EPTYPE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == EPTYPE_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == EPTYPE_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == EPTYPE_A::VALUE4
    }
}
#[doc = "Field `EPType` writer - Endpoint Type"]
pub type EPTYPE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, DIEPCTL_INTBULK_SPEC, 2, O, EPTYPE_A>;
impl<'a, const O: u8> EPTYPE_W<'a, O> {
    #[doc = "Control"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(EPTYPE_A::VALUE1)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(EPTYPE_A::VALUE2)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(EPTYPE_A::VALUE3)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(EPTYPE_A::VALUE4)
    }
}
#[doc = "Field `Snp` reader - Snoop Mode"]
pub type SNP_R = crate::BitReader;
#[doc = "Field `Snp` writer - Snoop Mode"]
pub type SNP_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL_INTBULK_SPEC, O>;
#[doc = "Field `Stall` reader - STALL Handshake"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `Stall` writer - STALL Handshake"]
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL_INTBULK_SPEC, O>;
#[doc = "Field `TxFNum` reader - TxFIFO Number"]
pub type TX_FNUM_R = crate::FieldReader;
#[doc = "Field `TxFNum` writer - TxFIFO Number"]
pub type TX_FNUM_W<'a, const O: u8> = crate::FieldWriter<'a, DIEPCTL_INTBULK_SPEC, 4, O>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CNAK_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL_INTBULK_SPEC, O>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SNAK_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL_INTBULK_SPEC, O>;
#[doc = "Field `SetD0PID` writer - Set DATA0 PID"]
pub type SET_D0PID_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL_INTBULK_SPEC, O>;
#[doc = "Field `SetD1PID` writer - 29 Set DATA1 PID"]
pub type SET_D1PID_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL_INTBULK_SPEC, O>;
#[doc = "Field `EPDis` reader - Endpoint Disable"]
pub type EPDIS_R = crate::BitReader;
#[doc = "Field `EPDis` writer - Endpoint Disable"]
pub type EPDIS_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL_INTBULK_SPEC, O>;
#[doc = "Field `EPEna` reader - Endpoint Enable"]
pub type EPENA_R = crate::BitReader;
#[doc = "Field `EPEna` writer - Endpoint Enable"]
pub type EPENA_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL_INTBULK_SPEC, O>;
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
    #[doc = "Bit 16 - Endpoint Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 16) & 1) != 0)
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
    pub fn mps(&mut self) -> MPS_W<0> {
        MPS_W::new(self)
    }
    #[doc = "Bit 15 - USB Active Endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn usbact_ep(&mut self) -> USBACT_EP_W<15> {
        USBACT_EP_W::new(self)
    }
    #[doc = "Bits 18:19 - Endpoint Type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EPTYPE_W<18> {
        EPTYPE_W::new(self)
    }
    #[doc = "Bit 20 - Snoop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn snp(&mut self) -> SNP_W<20> {
        SNP_W::new(self)
    }
    #[doc = "Bit 21 - STALL Handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<21> {
        STALL_W::new(self)
    }
    #[doc = "Bits 22:25 - TxFIFO Number"]
    #[inline(always)]
    #[must_use]
    pub fn tx_fnum(&mut self) -> TX_FNUM_W<22> {
        TX_FNUM_W::new(self)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<26> {
        CNAK_W::new(self)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<27> {
        SNAK_W::new(self)
    }
    #[doc = "Bit 28 - Set DATA0 PID"]
    #[inline(always)]
    #[must_use]
    pub fn set_d0pid(&mut self) -> SET_D0PID_W<28> {
        SET_D0PID_W::new(self)
    }
    #[doc = "Bit 29 - 29 Set DATA1 PID"]
    #[inline(always)]
    #[must_use]
    pub fn set_d1pid(&mut self) -> SET_D1PID_W<29> {
        SET_D1PID_W::new(self)
    }
    #[doc = "Bit 30 - Endpoint Disable"]
    #[inline(always)]
    #[must_use]
    pub fn epdis(&mut self) -> EPDIS_W<30> {
        EPDIS_W::new(self)
    }
    #[doc = "Bit 31 - Endpoint Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epena(&mut self) -> EPENA_W<31> {
        EPENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Endpoint Control Register \\[INTBULK\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepctl_intbulk](index.html) module"]
pub struct DIEPCTL_INTBULK_SPEC;
impl crate::RegisterSpec for DIEPCTL_INTBULK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepctl_intbulk::R](R) reader structure"]
impl crate::Readable for DIEPCTL_INTBULK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepctl_intbulk::W](W) writer structure"]
impl crate::Writable for DIEPCTL_INTBULK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPCTL_INTBULK to value 0"]
impl crate::Resettable for DIEPCTL_INTBULK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
