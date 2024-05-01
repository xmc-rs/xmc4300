#[doc = "Register `BYPCR` reader"]
pub type R = crate::R<BypcrSpec>;
#[doc = "Register `BYPCR` writer"]
pub type W = crate::W<BypcrSpec>;
#[doc = "Field `BWLE` reader - Bypass Word Length"]
pub type BwleR = crate::FieldReader;
#[doc = "Field `BWLE` writer - Bypass Word Length"]
pub type BwleW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Bypass Data Single Shot Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bdssm {
    #[doc = "0: The bypass data is still considered as valid after it has been loaded into TBUF. The loading of the data into TBUF does not clear BDV."]
    Value1 = 0,
    #[doc = "1: The bypass data is considered as invalid after it has been loaded into TBUF. The loading of the data into TBUF clears BDV."]
    Value2 = 1,
}
impl From<Bdssm> for bool {
    #[inline(always)]
    fn from(variant: Bdssm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BDSSM` reader - Bypass Data Single Shot Mode"]
pub type BdssmR = crate::BitReader<Bdssm>;
impl BdssmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bdssm {
        match self.bits {
            false => Bdssm::Value1,
            true => Bdssm::Value2,
        }
    }
    #[doc = "The bypass data is still considered as valid after it has been loaded into TBUF. The loading of the data into TBUF does not clear BDV."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bdssm::Value1
    }
    #[doc = "The bypass data is considered as invalid after it has been loaded into TBUF. The loading of the data into TBUF clears BDV."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bdssm::Value2
    }
}
#[doc = "Field `BDSSM` writer - Bypass Data Single Shot Mode"]
pub type BdssmW<'a, REG> = crate::BitWriter<'a, REG, Bdssm>;
impl<'a, REG> BdssmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The bypass data is still considered as valid after it has been loaded into TBUF. The loading of the data into TBUF does not clear BDV."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bdssm::Value1)
    }
    #[doc = "The bypass data is considered as invalid after it has been loaded into TBUF. The loading of the data into TBUF clears BDV."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bdssm::Value2)
    }
}
#[doc = "Bypass Data Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bden {
    #[doc = "0: The transfer of bypass data is disabled."]
    Value1 = 0,
    #[doc = "1: The transfer of bypass data to TBUF is possible. Bypass data will be transferred to TBUF according to its priority if BDV = 1."]
    Value2 = 1,
    #[doc = "2: Gated bypass data transfer is enabled. Bypass data will be transferred to TBUF according to its priority if BDV = 1 and while DX2S = 0."]
    Value3 = 2,
    #[doc = "3: Gated bypass data transfer is enabled. Bypass data will be transferred to TBUF according to its priority if BDV = 1 and while DX2S = 1."]
    Value4 = 3,
}
impl From<Bden> for u8 {
    #[inline(always)]
    fn from(variant: Bden) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bden {
    type Ux = u8;
}
impl crate::IsEnum for Bden {}
#[doc = "Field `BDEN` reader - Bypass Data Enable"]
pub type BdenR = crate::FieldReader<Bden>;
impl BdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bden {
        match self.bits {
            0 => Bden::Value1,
            1 => Bden::Value2,
            2 => Bden::Value3,
            3 => Bden::Value4,
            _ => unreachable!(),
        }
    }
    #[doc = "The transfer of bypass data is disabled."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bden::Value1
    }
    #[doc = "The transfer of bypass data to TBUF is possible. Bypass data will be transferred to TBUF according to its priority if BDV = 1."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bden::Value2
    }
    #[doc = "Gated bypass data transfer is enabled. Bypass data will be transferred to TBUF according to its priority if BDV = 1 and while DX2S = 0."]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == Bden::Value3
    }
    #[doc = "Gated bypass data transfer is enabled. Bypass data will be transferred to TBUF according to its priority if BDV = 1 and while DX2S = 1."]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == Bden::Value4
    }
}
#[doc = "Field `BDEN` writer - Bypass Data Enable"]
pub type BdenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bden, crate::Safe>;
impl<'a, REG> BdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The transfer of bypass data is disabled."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bden::Value1)
    }
    #[doc = "The transfer of bypass data to TBUF is possible. Bypass data will be transferred to TBUF according to its priority if BDV = 1."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bden::Value2)
    }
    #[doc = "Gated bypass data transfer is enabled. Bypass data will be transferred to TBUF according to its priority if BDV = 1 and while DX2S = 0."]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(Bden::Value3)
    }
    #[doc = "Gated bypass data transfer is enabled. Bypass data will be transferred to TBUF according to its priority if BDV = 1 and while DX2S = 1."]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(Bden::Value4)
    }
}
#[doc = "Bypass Data Valid Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bdvtr {
    #[doc = "0: Bit BDV is not influenced by DX2T."]
    Value1 = 0,
    #[doc = "1: Bit BDV is set if DX2T is active."]
    Value2 = 1,
}
impl From<Bdvtr> for bool {
    #[inline(always)]
    fn from(variant: Bdvtr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BDVTR` reader - Bypass Data Valid Trigger"]
pub type BdvtrR = crate::BitReader<Bdvtr>;
impl BdvtrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bdvtr {
        match self.bits {
            false => Bdvtr::Value1,
            true => Bdvtr::Value2,
        }
    }
    #[doc = "Bit BDV is not influenced by DX2T."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bdvtr::Value1
    }
    #[doc = "Bit BDV is set if DX2T is active."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bdvtr::Value2
    }
}
#[doc = "Field `BDVTR` writer - Bypass Data Valid Trigger"]
pub type BdvtrW<'a, REG> = crate::BitWriter<'a, REG, Bdvtr>;
impl<'a, REG> BdvtrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit BDV is not influenced by DX2T."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bdvtr::Value1)
    }
    #[doc = "Bit BDV is set if DX2T is active."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bdvtr::Value2)
    }
}
#[doc = "Bypass Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bprio {
    #[doc = "0: The transmit FIFO data has a higher priority than the bypass data."]
    Value1 = 0,
    #[doc = "1: The bypass data has a higher priority than the transmit FIFO data."]
    Value2 = 1,
}
impl From<Bprio> for bool {
    #[inline(always)]
    fn from(variant: Bprio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPRIO` reader - Bypass Priority"]
pub type BprioR = crate::BitReader<Bprio>;
impl BprioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bprio {
        match self.bits {
            false => Bprio::Value1,
            true => Bprio::Value2,
        }
    }
    #[doc = "The transmit FIFO data has a higher priority than the bypass data."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bprio::Value1
    }
    #[doc = "The bypass data has a higher priority than the transmit FIFO data."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bprio::Value2
    }
}
#[doc = "Field `BPRIO` writer - Bypass Priority"]
pub type BprioW<'a, REG> = crate::BitWriter<'a, REG, Bprio>;
impl<'a, REG> BprioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The transmit FIFO data has a higher priority than the bypass data."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Bprio::Value1)
    }
    #[doc = "The bypass data has a higher priority than the transmit FIFO data."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Bprio::Value2)
    }
}
#[doc = "Bypass Data Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bdv {
    #[doc = "0: The bypass data is not valid."]
    Value1 = 0,
    #[doc = "1: The bypass data is valid."]
    Value2 = 1,
}
impl From<Bdv> for bool {
    #[inline(always)]
    fn from(variant: Bdv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BDV` reader - Bypass Data Valid"]
pub type BdvR = crate::BitReader<Bdv>;
impl BdvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bdv {
        match self.bits {
            false => Bdv::Value1,
            true => Bdv::Value2,
        }
    }
    #[doc = "The bypass data is not valid."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Bdv::Value1
    }
    #[doc = "The bypass data is valid."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Bdv::Value2
    }
}
#[doc = "Field `BSELO` reader - Bypass Select Outputs"]
pub type BseloR = crate::FieldReader;
#[doc = "Field `BSELO` writer - Bypass Select Outputs"]
pub type BseloW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BHPC` reader - Bypass Hardware Port Control"]
pub type BhpcR = crate::FieldReader;
#[doc = "Field `BHPC` writer - Bypass Hardware Port Control"]
pub type BhpcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Bypass Word Length"]
    #[inline(always)]
    pub fn bwle(&self) -> BwleR {
        BwleR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Bypass Data Single Shot Mode"]
    #[inline(always)]
    pub fn bdssm(&self) -> BdssmR {
        BdssmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Bypass Data Enable"]
    #[inline(always)]
    pub fn bden(&self) -> BdenR {
        BdenR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Bypass Data Valid Trigger"]
    #[inline(always)]
    pub fn bdvtr(&self) -> BdvtrR {
        BdvtrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Bypass Priority"]
    #[inline(always)]
    pub fn bprio(&self) -> BprioR {
        BprioR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Bypass Data Valid"]
    #[inline(always)]
    pub fn bdv(&self) -> BdvR {
        BdvR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Bypass Select Outputs"]
    #[inline(always)]
    pub fn bselo(&self) -> BseloR {
        BseloR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - Bypass Hardware Port Control"]
    #[inline(always)]
    pub fn bhpc(&self) -> BhpcR {
        BhpcR::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bypass Word Length"]
    #[inline(always)]
    #[must_use]
    pub fn bwle(&mut self) -> BwleW<BypcrSpec> {
        BwleW::new(self, 0)
    }
    #[doc = "Bit 8 - Bypass Data Single Shot Mode"]
    #[inline(always)]
    #[must_use]
    pub fn bdssm(&mut self) -> BdssmW<BypcrSpec> {
        BdssmW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Bypass Data Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bden(&mut self) -> BdenW<BypcrSpec> {
        BdenW::new(self, 10)
    }
    #[doc = "Bit 12 - Bypass Data Valid Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn bdvtr(&mut self) -> BdvtrW<BypcrSpec> {
        BdvtrW::new(self, 12)
    }
    #[doc = "Bit 13 - Bypass Priority"]
    #[inline(always)]
    #[must_use]
    pub fn bprio(&mut self) -> BprioW<BypcrSpec> {
        BprioW::new(self, 13)
    }
    #[doc = "Bits 16:20 - Bypass Select Outputs"]
    #[inline(always)]
    #[must_use]
    pub fn bselo(&mut self) -> BseloW<BypcrSpec> {
        BseloW::new(self, 16)
    }
    #[doc = "Bits 21:23 - Bypass Hardware Port Control"]
    #[inline(always)]
    #[must_use]
    pub fn bhpc(&mut self) -> BhpcW<BypcrSpec> {
        BhpcW::new(self, 21)
    }
}
#[doc = "Bypass Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bypcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bypcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BypcrSpec;
impl crate::RegisterSpec for BypcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bypcr::R`](R) reader structure"]
impl crate::Readable for BypcrSpec {}
#[doc = "`write(|w| ..)` method takes [`bypcr::W`](W) writer structure"]
impl crate::Writable for BypcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BYPCR to value 0"]
impl crate::Resettable for BypcrSpec {
    const RESET_VALUE: u32 = 0;
}
