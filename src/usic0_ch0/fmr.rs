#[doc = "Register `FMR` writer"]
pub type W = crate::W<FMR_SPEC>;
#[doc = "Modify Transmit Data Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MTDV_AW {
    #[doc = "0: No action."]
    VALUE1 = 0,
    #[doc = "1: Bit TDV is set, TE is unchanged."]
    VALUE2 = 1,
    #[doc = "2: Bits TDV and TE are cleared."]
    VALUE3 = 2,
}
impl From<MTDV_AW> for u8 {
    #[inline(always)]
    fn from(variant: MTDV_AW) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MTDV_AW {
    type Ux = u8;
}
#[doc = "Field `MTDV` writer - Modify Transmit Data Valid"]
pub type MTDV_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MTDV_AW>;
impl<'a, REG> MTDV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(MTDV_AW::VALUE1)
    }
    #[doc = "Bit TDV is set, TE is unchanged."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(MTDV_AW::VALUE2)
    }
    #[doc = "Bits TDV and TE are cleared."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(MTDV_AW::VALUE3)
    }
}
#[doc = "Activate Bit TVC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATVC_AW {
    #[doc = "0: No action."]
    VALUE1 = 0,
    #[doc = "1: Bit TCSR.TVC is set."]
    VALUE2 = 1,
}
impl From<ATVC_AW> for bool {
    #[inline(always)]
    fn from(variant: ATVC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATVC` writer - Activate Bit TVC"]
pub type ATVC_W<'a, REG> = crate::BitWriter<'a, REG, ATVC_AW>;
impl<'a, REG> ATVC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(ATVC_AW::VALUE1)
    }
    #[doc = "Bit TCSR.TVC is set."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(ATVC_AW::VALUE2)
    }
}
#[doc = "Clear Bits RDV for RBUF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRDV0_AW {
    #[doc = "0: No action."]
    VALUE1 = 0,
    #[doc = "1: Bits RBUF01SR.RDV00 and RBUF01SR.RDV10 are cleared."]
    VALUE2 = 1,
}
impl From<CRDV0_AW> for bool {
    #[inline(always)]
    fn from(variant: CRDV0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRDV0` writer - Clear Bits RDV for RBUF0"]
pub type CRDV0_W<'a, REG> = crate::BitWriter<'a, REG, CRDV0_AW>;
impl<'a, REG> CRDV0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CRDV0_AW::VALUE1)
    }
    #[doc = "Bits RBUF01SR.RDV00 and RBUF01SR.RDV10 are cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CRDV0_AW::VALUE2)
    }
}
#[doc = "Clear Bit RDV for RBUF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRDV1_AW {
    #[doc = "0: No action."]
    VALUE1 = 0,
    #[doc = "1: Bits RBUF01SR.RDV01 and RBUF01SR.RDV11 are cleared."]
    VALUE2 = 1,
}
impl From<CRDV1_AW> for bool {
    #[inline(always)]
    fn from(variant: CRDV1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRDV1` writer - Clear Bit RDV for RBUF1"]
pub type CRDV1_W<'a, REG> = crate::BitWriter<'a, REG, CRDV1_AW>;
impl<'a, REG> CRDV1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(CRDV1_AW::VALUE1)
    }
    #[doc = "Bits RBUF01SR.RDV01 and RBUF01SR.RDV11 are cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(CRDV1_AW::VALUE2)
    }
}
#[doc = "Set Interrupt Output SRx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIO0_AW {
    #[doc = "0: No action."]
    VALUE1 = 0,
    #[doc = "1: The service request output SRx is activated."]
    VALUE2 = 1,
}
impl From<SIO0_AW> for bool {
    #[inline(always)]
    fn from(variant: SIO0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIO0` writer - Set Interrupt Output SRx"]
pub type SIO0_W<'a, REG> = crate::BitWriter<'a, REG, SIO0_AW>;
impl<'a, REG> SIO0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SIO0_AW::VALUE1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SIO0_AW::VALUE2)
    }
}
#[doc = "Set Interrupt Output SRx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIO1_AW {
    #[doc = "0: No action."]
    VALUE1 = 0,
    #[doc = "1: The service request output SRx is activated."]
    VALUE2 = 1,
}
impl From<SIO1_AW> for bool {
    #[inline(always)]
    fn from(variant: SIO1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIO1` writer - Set Interrupt Output SRx"]
pub type SIO1_W<'a, REG> = crate::BitWriter<'a, REG, SIO1_AW>;
impl<'a, REG> SIO1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SIO1_AW::VALUE1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SIO1_AW::VALUE2)
    }
}
#[doc = "Set Interrupt Output SRx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIO2_AW {
    #[doc = "0: No action."]
    VALUE1 = 0,
    #[doc = "1: The service request output SRx is activated."]
    VALUE2 = 1,
}
impl From<SIO2_AW> for bool {
    #[inline(always)]
    fn from(variant: SIO2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIO2` writer - Set Interrupt Output SRx"]
pub type SIO2_W<'a, REG> = crate::BitWriter<'a, REG, SIO2_AW>;
impl<'a, REG> SIO2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SIO2_AW::VALUE1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SIO2_AW::VALUE2)
    }
}
#[doc = "Set Interrupt Output SRx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIO3_AW {
    #[doc = "0: No action."]
    VALUE1 = 0,
    #[doc = "1: The service request output SRx is activated."]
    VALUE2 = 1,
}
impl From<SIO3_AW> for bool {
    #[inline(always)]
    fn from(variant: SIO3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIO3` writer - Set Interrupt Output SRx"]
pub type SIO3_W<'a, REG> = crate::BitWriter<'a, REG, SIO3_AW>;
impl<'a, REG> SIO3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SIO3_AW::VALUE1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SIO3_AW::VALUE2)
    }
}
#[doc = "Set Interrupt Output SRx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIO4_AW {
    #[doc = "0: No action."]
    VALUE1 = 0,
    #[doc = "1: The service request output SRx is activated."]
    VALUE2 = 1,
}
impl From<SIO4_AW> for bool {
    #[inline(always)]
    fn from(variant: SIO4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIO4` writer - Set Interrupt Output SRx"]
pub type SIO4_W<'a, REG> = crate::BitWriter<'a, REG, SIO4_AW>;
impl<'a, REG> SIO4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SIO4_AW::VALUE1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SIO4_AW::VALUE2)
    }
}
#[doc = "Set Interrupt Output SRx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIO5_AW {
    #[doc = "0: No action."]
    VALUE1 = 0,
    #[doc = "1: The service request output SRx is activated."]
    VALUE2 = 1,
}
impl From<SIO5_AW> for bool {
    #[inline(always)]
    fn from(variant: SIO5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIO5` writer - Set Interrupt Output SRx"]
pub type SIO5_W<'a, REG> = crate::BitWriter<'a, REG, SIO5_AW>;
impl<'a, REG> SIO5_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(SIO5_AW::VALUE1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(SIO5_AW::VALUE2)
    }
}
impl W {
    #[doc = "Bits 0:1 - Modify Transmit Data Valid"]
    #[inline(always)]
    #[must_use]
    pub fn mtdv(&mut self) -> MTDV_W<FMR_SPEC> {
        MTDV_W::new(self, 0)
    }
    #[doc = "Bit 4 - Activate Bit TVC"]
    #[inline(always)]
    #[must_use]
    pub fn atvc(&mut self) -> ATVC_W<FMR_SPEC> {
        ATVC_W::new(self, 4)
    }
    #[doc = "Bit 14 - Clear Bits RDV for RBUF0"]
    #[inline(always)]
    #[must_use]
    pub fn crdv0(&mut self) -> CRDV0_W<FMR_SPEC> {
        CRDV0_W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear Bit RDV for RBUF1"]
    #[inline(always)]
    #[must_use]
    pub fn crdv1(&mut self) -> CRDV1_W<FMR_SPEC> {
        CRDV1_W::new(self, 15)
    }
    #[doc = "Bit 16 - Set Interrupt Output SRx"]
    #[inline(always)]
    #[must_use]
    pub fn sio0(&mut self) -> SIO0_W<FMR_SPEC> {
        SIO0_W::new(self, 16)
    }
    #[doc = "Bit 17 - Set Interrupt Output SRx"]
    #[inline(always)]
    #[must_use]
    pub fn sio1(&mut self) -> SIO1_W<FMR_SPEC> {
        SIO1_W::new(self, 17)
    }
    #[doc = "Bit 18 - Set Interrupt Output SRx"]
    #[inline(always)]
    #[must_use]
    pub fn sio2(&mut self) -> SIO2_W<FMR_SPEC> {
        SIO2_W::new(self, 18)
    }
    #[doc = "Bit 19 - Set Interrupt Output SRx"]
    #[inline(always)]
    #[must_use]
    pub fn sio3(&mut self) -> SIO3_W<FMR_SPEC> {
        SIO3_W::new(self, 19)
    }
    #[doc = "Bit 20 - Set Interrupt Output SRx"]
    #[inline(always)]
    #[must_use]
    pub fn sio4(&mut self) -> SIO4_W<FMR_SPEC> {
        SIO4_W::new(self, 20)
    }
    #[doc = "Bit 21 - Set Interrupt Output SRx"]
    #[inline(always)]
    #[must_use]
    pub fn sio5(&mut self) -> SIO5_W<FMR_SPEC> {
        SIO5_W::new(self, 21)
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
#[doc = "Flag Modification Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMR_SPEC;
impl crate::RegisterSpec for FMR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fmr::W`](W) writer structure"]
impl crate::Writable for FMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMR to value 0"]
impl crate::Resettable for FMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
