#[doc = "Register `FMR` writer"]
pub type W = crate::W<FmrSpec>;
#[doc = "Modify Transmit Data Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mtdv {
    #[doc = "0: No action."]
    Value1 = 0,
    #[doc = "1: Bit TDV is set, TE is unchanged."]
    Value2 = 1,
    #[doc = "2: Bits TDV and TE are cleared."]
    Value3 = 2,
}
impl From<Mtdv> for u8 {
    #[inline(always)]
    fn from(variant: Mtdv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mtdv {
    type Ux = u8;
}
impl crate::IsEnum for Mtdv {}
#[doc = "Field `MTDV` writer - Modify Transmit Data Valid"]
pub type MtdvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mtdv>;
impl<'a, REG> MtdvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Mtdv::Value1)
    }
    #[doc = "Bit TDV is set, TE is unchanged."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Mtdv::Value2)
    }
    #[doc = "Bits TDV and TE are cleared."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Mtdv::Value3)
    }
}
#[doc = "Activate Bit TVC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Atvc {
    #[doc = "0: No action."]
    Value1 = 0,
    #[doc = "1: Bit TCSR.TVC is set."]
    Value2 = 1,
}
impl From<Atvc> for bool {
    #[inline(always)]
    fn from(variant: Atvc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ATVC` writer - Activate Bit TVC"]
pub type AtvcW<'a, REG> = crate::BitWriter<'a, REG, Atvc>;
impl<'a, REG> AtvcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Atvc::Value1)
    }
    #[doc = "Bit TCSR.TVC is set."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Atvc::Value2)
    }
}
#[doc = "Clear Bits RDV for RBUF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crdv0 {
    #[doc = "0: No action."]
    Value1 = 0,
    #[doc = "1: Bits RBUF01SR.RDV00 and RBUF01SR.RDV10 are cleared."]
    Value2 = 1,
}
impl From<Crdv0> for bool {
    #[inline(always)]
    fn from(variant: Crdv0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRDV0` writer - Clear Bits RDV for RBUF0"]
pub type Crdv0W<'a, REG> = crate::BitWriter<'a, REG, Crdv0>;
impl<'a, REG> Crdv0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Crdv0::Value1)
    }
    #[doc = "Bits RBUF01SR.RDV00 and RBUF01SR.RDV10 are cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Crdv0::Value2)
    }
}
#[doc = "Clear Bit RDV for RBUF1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crdv1 {
    #[doc = "0: No action."]
    Value1 = 0,
    #[doc = "1: Bits RBUF01SR.RDV01 and RBUF01SR.RDV11 are cleared."]
    Value2 = 1,
}
impl From<Crdv1> for bool {
    #[inline(always)]
    fn from(variant: Crdv1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRDV1` writer - Clear Bit RDV for RBUF1"]
pub type Crdv1W<'a, REG> = crate::BitWriter<'a, REG, Crdv1>;
impl<'a, REG> Crdv1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Crdv1::Value1)
    }
    #[doc = "Bits RBUF01SR.RDV01 and RBUF01SR.RDV11 are cleared."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Crdv1::Value2)
    }
}
#[doc = "Set Interrupt Output SRx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sio0 {
    #[doc = "0: No action."]
    Value1 = 0,
    #[doc = "1: The service request output SRx is activated."]
    Value2 = 1,
}
impl From<Sio0> for bool {
    #[inline(always)]
    fn from(variant: Sio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIO0` writer - Set Interrupt Output SRx"]
pub type Sio0W<'a, REG> = crate::BitWriter<'a, REG, Sio0>;
impl<'a, REG> Sio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sio0::Value1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sio0::Value2)
    }
}
#[doc = "Set Interrupt Output SRx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sio1 {
    #[doc = "0: No action."]
    Value1 = 0,
    #[doc = "1: The service request output SRx is activated."]
    Value2 = 1,
}
impl From<Sio1> for bool {
    #[inline(always)]
    fn from(variant: Sio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIO1` writer - Set Interrupt Output SRx"]
pub type Sio1W<'a, REG> = crate::BitWriter<'a, REG, Sio1>;
impl<'a, REG> Sio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sio1::Value1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sio1::Value2)
    }
}
#[doc = "Set Interrupt Output SRx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sio2 {
    #[doc = "0: No action."]
    Value1 = 0,
    #[doc = "1: The service request output SRx is activated."]
    Value2 = 1,
}
impl From<Sio2> for bool {
    #[inline(always)]
    fn from(variant: Sio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIO2` writer - Set Interrupt Output SRx"]
pub type Sio2W<'a, REG> = crate::BitWriter<'a, REG, Sio2>;
impl<'a, REG> Sio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sio2::Value1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sio2::Value2)
    }
}
#[doc = "Set Interrupt Output SRx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sio3 {
    #[doc = "0: No action."]
    Value1 = 0,
    #[doc = "1: The service request output SRx is activated."]
    Value2 = 1,
}
impl From<Sio3> for bool {
    #[inline(always)]
    fn from(variant: Sio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIO3` writer - Set Interrupt Output SRx"]
pub type Sio3W<'a, REG> = crate::BitWriter<'a, REG, Sio3>;
impl<'a, REG> Sio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sio3::Value1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sio3::Value2)
    }
}
#[doc = "Set Interrupt Output SRx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sio4 {
    #[doc = "0: No action."]
    Value1 = 0,
    #[doc = "1: The service request output SRx is activated."]
    Value2 = 1,
}
impl From<Sio4> for bool {
    #[inline(always)]
    fn from(variant: Sio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIO4` writer - Set Interrupt Output SRx"]
pub type Sio4W<'a, REG> = crate::BitWriter<'a, REG, Sio4>;
impl<'a, REG> Sio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sio4::Value1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sio4::Value2)
    }
}
#[doc = "Set Interrupt Output SRx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sio5 {
    #[doc = "0: No action."]
    Value1 = 0,
    #[doc = "1: The service request output SRx is activated."]
    Value2 = 1,
}
impl From<Sio5> for bool {
    #[inline(always)]
    fn from(variant: Sio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIO5` writer - Set Interrupt Output SRx"]
pub type Sio5W<'a, REG> = crate::BitWriter<'a, REG, Sio5>;
impl<'a, REG> Sio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Sio5::Value1)
    }
    #[doc = "The service request output SRx is activated."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Sio5::Value2)
    }
}
impl W {
    #[doc = "Bits 0:1 - Modify Transmit Data Valid"]
    #[inline(always)]
    #[must_use]
    pub fn mtdv(&mut self) -> MtdvW<FmrSpec> {
        MtdvW::new(self, 0)
    }
    #[doc = "Bit 4 - Activate Bit TVC"]
    #[inline(always)]
    #[must_use]
    pub fn atvc(&mut self) -> AtvcW<FmrSpec> {
        AtvcW::new(self, 4)
    }
    #[doc = "Bit 14 - Clear Bits RDV for RBUF0"]
    #[inline(always)]
    #[must_use]
    pub fn crdv0(&mut self) -> Crdv0W<FmrSpec> {
        Crdv0W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear Bit RDV for RBUF1"]
    #[inline(always)]
    #[must_use]
    pub fn crdv1(&mut self) -> Crdv1W<FmrSpec> {
        Crdv1W::new(self, 15)
    }
    #[doc = "Bit 16 - Set Interrupt Output SRx"]
    #[inline(always)]
    #[must_use]
    pub fn sio0(&mut self) -> Sio0W<FmrSpec> {
        Sio0W::new(self, 16)
    }
    #[doc = "Bit 17 - Set Interrupt Output SRx"]
    #[inline(always)]
    #[must_use]
    pub fn sio1(&mut self) -> Sio1W<FmrSpec> {
        Sio1W::new(self, 17)
    }
    #[doc = "Bit 18 - Set Interrupt Output SRx"]
    #[inline(always)]
    #[must_use]
    pub fn sio2(&mut self) -> Sio2W<FmrSpec> {
        Sio2W::new(self, 18)
    }
    #[doc = "Bit 19 - Set Interrupt Output SRx"]
    #[inline(always)]
    #[must_use]
    pub fn sio3(&mut self) -> Sio3W<FmrSpec> {
        Sio3W::new(self, 19)
    }
    #[doc = "Bit 20 - Set Interrupt Output SRx"]
    #[inline(always)]
    #[must_use]
    pub fn sio4(&mut self) -> Sio4W<FmrSpec> {
        Sio4W::new(self, 20)
    }
    #[doc = "Bit 21 - Set Interrupt Output SRx"]
    #[inline(always)]
    #[must_use]
    pub fn sio5(&mut self) -> Sio5W<FmrSpec> {
        Sio5W::new(self, 21)
    }
}
#[doc = "Flag Modification Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmrSpec;
impl crate::RegisterSpec for FmrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fmr::W`](W) writer structure"]
impl crate::Writable for FmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMR to value 0"]
impl crate::Resettable for FmrSpec {
    const RESET_VALUE: u32 = 0;
}
