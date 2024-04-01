#[doc = "Register `GAHBCFG` reader"]
pub type R = crate::R<GahbcfgSpec>;
#[doc = "Register `GAHBCFG` writer"]
pub type W = crate::W<GahbcfgSpec>;
#[doc = "Global Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlblIntrMsk {
    #[doc = "0: Mask the interrupt assertion to the application."]
    Value1 = 0,
    #[doc = "1: Unmask the interrupt assertion to the application."]
    Value2 = 1,
}
impl From<GlblIntrMsk> for bool {
    #[inline(always)]
    fn from(variant: GlblIntrMsk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GlblIntrMsk` reader - Global Interrupt Mask"]
pub type GlblIntrMskR = crate::BitReader<GlblIntrMsk>;
impl GlblIntrMskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GlblIntrMsk {
        match self.bits {
            false => GlblIntrMsk::Value1,
            true => GlblIntrMsk::Value2,
        }
    }
    #[doc = "Mask the interrupt assertion to the application."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GlblIntrMsk::Value1
    }
    #[doc = "Unmask the interrupt assertion to the application."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GlblIntrMsk::Value2
    }
}
#[doc = "Field `GlblIntrMsk` writer - Global Interrupt Mask"]
pub type GlblIntrMskW<'a, REG> = crate::BitWriter<'a, REG, GlblIntrMsk>;
impl<'a, REG> GlblIntrMskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mask the interrupt assertion to the application."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(GlblIntrMsk::Value1)
    }
    #[doc = "Unmask the interrupt assertion to the application."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(GlblIntrMsk::Value2)
    }
}
#[doc = "Burst Length/Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HbstLen {
    #[doc = "0: Single"]
    Value1 = 0,
    #[doc = "1: INCR"]
    Value2 = 1,
    #[doc = "3: INCR4"]
    Value3 = 3,
    #[doc = "5: INCR8"]
    Value4 = 5,
    #[doc = "7: INCR16"]
    Value5 = 7,
}
impl From<HbstLen> for u8 {
    #[inline(always)]
    fn from(variant: HbstLen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HbstLen {
    type Ux = u8;
}
impl crate::IsEnum for HbstLen {}
#[doc = "Field `HBstLen` reader - Burst Length/Type"]
pub type HbstLenR = crate::FieldReader<HbstLen>;
impl HbstLenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HbstLen> {
        match self.bits {
            0 => Some(HbstLen::Value1),
            1 => Some(HbstLen::Value2),
            3 => Some(HbstLen::Value3),
            5 => Some(HbstLen::Value4),
            7 => Some(HbstLen::Value5),
            _ => None,
        }
    }
    #[doc = "Single"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HbstLen::Value1
    }
    #[doc = "INCR"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HbstLen::Value2
    }
    #[doc = "INCR4"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == HbstLen::Value3
    }
    #[doc = "INCR8"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == HbstLen::Value4
    }
    #[doc = "INCR16"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == HbstLen::Value5
    }
}
#[doc = "Field `HBstLen` writer - Burst Length/Type"]
pub type HbstLenW<'a, REG> = crate::FieldWriter<'a, REG, 4, HbstLen>;
impl<'a, REG> HbstLenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HbstLen::Value1)
    }
    #[doc = "INCR"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HbstLen::Value2)
    }
    #[doc = "INCR4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(HbstLen::Value3)
    }
    #[doc = "INCR8"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(HbstLen::Value4)
    }
    #[doc = "INCR16"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(HbstLen::Value5)
    }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaen {
    #[doc = "0: Core operates in Slave mode"]
    Value1 = 0,
    #[doc = "1: Core operates in a DMA mode"]
    Value2 = 1,
}
impl From<Dmaen> for bool {
    #[inline(always)]
    fn from(variant: Dmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEn` reader - DMA Enable"]
pub type DmaenR = crate::BitReader<Dmaen>;
impl DmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaen {
        match self.bits {
            false => Dmaen::Value1,
            true => Dmaen::Value2,
        }
    }
    #[doc = "Core operates in Slave mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Dmaen::Value1
    }
    #[doc = "Core operates in a DMA mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Dmaen::Value2
    }
}
#[doc = "Field `DMAEn` writer - DMA Enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG, Dmaen>;
impl<'a, REG> DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Core operates in Slave mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Value1)
    }
    #[doc = "Core operates in a DMA mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Value2)
    }
}
#[doc = "Non-Periodic TxFIFO Empty Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NptxFempLvl {
    #[doc = "0: DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is half empty"]
    Value1 = 0,
    #[doc = "1: DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is completely empty"]
    Value2 = 1,
}
impl From<NptxFempLvl> for bool {
    #[inline(always)]
    fn from(variant: NptxFempLvl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NPTxFEmpLvl` reader - Non-Periodic TxFIFO Empty Level"]
pub type NptxFempLvlR = crate::BitReader<NptxFempLvl>;
impl NptxFempLvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NptxFempLvl {
        match self.bits {
            false => NptxFempLvl::Value1,
            true => NptxFempLvl::Value2,
        }
    }
    #[doc = "DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is half empty"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NptxFempLvl::Value1
    }
    #[doc = "DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is completely empty"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NptxFempLvl::Value2
    }
}
#[doc = "Field `NPTxFEmpLvl` writer - Non-Periodic TxFIFO Empty Level"]
pub type NptxFempLvlW<'a, REG> = crate::BitWriter<'a, REG, NptxFempLvl>;
impl<'a, REG> NptxFempLvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is half empty"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(NptxFempLvl::Value1)
    }
    #[doc = "DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is completely empty"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(NptxFempLvl::Value2)
    }
}
#[doc = "Periodic TxFIFO Empty Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PtxFempLvl {
    #[doc = "0: GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is half empty"]
    Value1 = 0,
    #[doc = "1: GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is completely empty"]
    Value2 = 1,
}
impl From<PtxFempLvl> for bool {
    #[inline(always)]
    fn from(variant: PtxFempLvl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTxFEmpLvl` reader - Periodic TxFIFO Empty Level"]
pub type PtxFempLvlR = crate::BitReader<PtxFempLvl>;
impl PtxFempLvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PtxFempLvl {
        match self.bits {
            false => PtxFempLvl::Value1,
            true => PtxFempLvl::Value2,
        }
    }
    #[doc = "GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is half empty"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PtxFempLvl::Value1
    }
    #[doc = "GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is completely empty"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PtxFempLvl::Value2
    }
}
#[doc = "Field `PTxFEmpLvl` writer - Periodic TxFIFO Empty Level"]
pub type PtxFempLvlW<'a, REG> = crate::BitWriter<'a, REG, PtxFempLvl>;
impl<'a, REG> PtxFempLvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is half empty"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PtxFempLvl::Value1)
    }
    #[doc = "GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is completely empty"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PtxFempLvl::Value2)
    }
}
#[doc = "AHB Single Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahbsingle {
    #[doc = "0: The remaining data in a transfer is sent using INCR burst size. This is the default mode."]
    Value1 = 0,
    #[doc = "1: The remaining data in a transfer is sent using single burst size."]
    Value2 = 1,
}
impl From<Ahbsingle> for bool {
    #[inline(always)]
    fn from(variant: Ahbsingle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHBSingle` reader - AHB Single Support"]
pub type AhbsingleR = crate::BitReader<Ahbsingle>;
impl AhbsingleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ahbsingle {
        match self.bits {
            false => Ahbsingle::Value1,
            true => Ahbsingle::Value2,
        }
    }
    #[doc = "The remaining data in a transfer is sent using INCR burst size. This is the default mode."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == Ahbsingle::Value1
    }
    #[doc = "The remaining data in a transfer is sent using single burst size."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == Ahbsingle::Value2
    }
}
#[doc = "Field `AHBSingle` writer - AHB Single Support"]
pub type AhbsingleW<'a, REG> = crate::BitWriter<'a, REG, Ahbsingle>;
impl<'a, REG> AhbsingleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The remaining data in a transfer is sent using INCR burst size. This is the default mode."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbsingle::Value1)
    }
    #[doc = "The remaining data in a transfer is sent using single burst size."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbsingle::Value2)
    }
}
impl R {
    #[doc = "Bit 0 - Global Interrupt Mask"]
    #[inline(always)]
    pub fn glbl_intr_msk(&self) -> GlblIntrMskR {
        GlblIntrMskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Burst Length/Type"]
    #[inline(always)]
    pub fn hbst_len(&self) -> HbstLenR {
        HbstLenR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO Empty Level"]
    #[inline(always)]
    pub fn nptx_femp_lvl(&self) -> NptxFempLvlR {
        NptxFempLvlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Periodic TxFIFO Empty Level"]
    #[inline(always)]
    pub fn ptx_femp_lvl(&self) -> PtxFempLvlR {
        PtxFempLvlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 23 - AHB Single Support"]
    #[inline(always)]
    pub fn ahbsingle(&self) -> AhbsingleR {
        AhbsingleR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn glbl_intr_msk(&mut self) -> GlblIntrMskW<GahbcfgSpec> {
        GlblIntrMskW::new(self, 0)
    }
    #[doc = "Bits 1:4 - Burst Length/Type"]
    #[inline(always)]
    #[must_use]
    pub fn hbst_len(&mut self) -> HbstLenW<GahbcfgSpec> {
        HbstLenW::new(self, 1)
    }
    #[doc = "Bit 5 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<GahbcfgSpec> {
        DmaenW::new(self, 5)
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO Empty Level"]
    #[inline(always)]
    #[must_use]
    pub fn nptx_femp_lvl(&mut self) -> NptxFempLvlW<GahbcfgSpec> {
        NptxFempLvlW::new(self, 7)
    }
    #[doc = "Bit 8 - Periodic TxFIFO Empty Level"]
    #[inline(always)]
    #[must_use]
    pub fn ptx_femp_lvl(&mut self) -> PtxFempLvlW<GahbcfgSpec> {
        PtxFempLvlW::new(self, 8)
    }
    #[doc = "Bit 23 - AHB Single Support"]
    #[inline(always)]
    #[must_use]
    pub fn ahbsingle(&mut self) -> AhbsingleW<GahbcfgSpec> {
        AhbsingleW::new(self, 23)
    }
}
#[doc = "AHB Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GahbcfgSpec;
impl crate::RegisterSpec for GahbcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gahbcfg::R`](R) reader structure"]
impl crate::Readable for GahbcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gahbcfg::W`](W) writer structure"]
impl crate::Writable for GahbcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAHBCFG to value 0"]
impl crate::Resettable for GahbcfgSpec {
    const RESET_VALUE: u32 = 0;
}
