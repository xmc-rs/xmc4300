#[doc = "Register `GAHBCFG` reader"]
pub type R = crate::R<GAHBCFG_SPEC>;
#[doc = "Register `GAHBCFG` writer"]
pub type W = crate::W<GAHBCFG_SPEC>;
#[doc = "Field `GlblIntrMsk` reader - Global Interrupt Mask"]
pub type GLBL_INTR_MSK_R = crate::BitReader<GLBL_INTR_MSK_A>;
#[doc = "Global Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GLBL_INTR_MSK_A {
    #[doc = "0: Mask the interrupt assertion to the application."]
    VALUE1 = 0,
    #[doc = "1: Unmask the interrupt assertion to the application."]
    VALUE2 = 1,
}
impl From<GLBL_INTR_MSK_A> for bool {
    #[inline(always)]
    fn from(variant: GLBL_INTR_MSK_A) -> Self {
        variant as u8 != 0
    }
}
impl GLBL_INTR_MSK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GLBL_INTR_MSK_A {
        match self.bits {
            false => GLBL_INTR_MSK_A::VALUE1,
            true => GLBL_INTR_MSK_A::VALUE2,
        }
    }
    #[doc = "Mask the interrupt assertion to the application."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == GLBL_INTR_MSK_A::VALUE1
    }
    #[doc = "Unmask the interrupt assertion to the application."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == GLBL_INTR_MSK_A::VALUE2
    }
}
#[doc = "Field `GlblIntrMsk` writer - Global Interrupt Mask"]
pub type GLBL_INTR_MSK_W<'a, REG> = crate::BitWriter<'a, REG, GLBL_INTR_MSK_A>;
impl<'a, REG> GLBL_INTR_MSK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mask the interrupt assertion to the application."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(GLBL_INTR_MSK_A::VALUE1)
    }
    #[doc = "Unmask the interrupt assertion to the application."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(GLBL_INTR_MSK_A::VALUE2)
    }
}
#[doc = "Field `HBstLen` reader - Burst Length/Type"]
pub type HBST_LEN_R = crate::FieldReader<HBST_LEN_A>;
#[doc = "Burst Length/Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HBST_LEN_A {
    #[doc = "0: Single"]
    VALUE1 = 0,
    #[doc = "1: INCR"]
    VALUE2 = 1,
    #[doc = "3: INCR4"]
    VALUE3 = 3,
    #[doc = "5: INCR8"]
    VALUE4 = 5,
    #[doc = "7: INCR16"]
    VALUE5 = 7,
}
impl From<HBST_LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: HBST_LEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HBST_LEN_A {
    type Ux = u8;
}
impl HBST_LEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HBST_LEN_A> {
        match self.bits {
            0 => Some(HBST_LEN_A::VALUE1),
            1 => Some(HBST_LEN_A::VALUE2),
            3 => Some(HBST_LEN_A::VALUE3),
            5 => Some(HBST_LEN_A::VALUE4),
            7 => Some(HBST_LEN_A::VALUE5),
            _ => None,
        }
    }
    #[doc = "Single"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == HBST_LEN_A::VALUE1
    }
    #[doc = "INCR"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == HBST_LEN_A::VALUE2
    }
    #[doc = "INCR4"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == HBST_LEN_A::VALUE3
    }
    #[doc = "INCR8"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == HBST_LEN_A::VALUE4
    }
    #[doc = "INCR16"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == HBST_LEN_A::VALUE5
    }
}
#[doc = "Field `HBstLen` writer - Burst Length/Type"]
pub type HBST_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4, HBST_LEN_A>;
impl<'a, REG> HBST_LEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(HBST_LEN_A::VALUE1)
    }
    #[doc = "INCR"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(HBST_LEN_A::VALUE2)
    }
    #[doc = "INCR4"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut crate::W<REG> {
        self.variant(HBST_LEN_A::VALUE3)
    }
    #[doc = "INCR8"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut crate::W<REG> {
        self.variant(HBST_LEN_A::VALUE4)
    }
    #[doc = "INCR16"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut crate::W<REG> {
        self.variant(HBST_LEN_A::VALUE5)
    }
}
#[doc = "Field `DMAEn` reader - DMA Enable"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: Core operates in Slave mode"]
    VALUE1 = 0,
    #[doc = "1: Core operates in a DMA mode"]
    VALUE2 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::VALUE1,
            true => DMAEN_A::VALUE2,
        }
    }
    #[doc = "Core operates in Slave mode"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DMAEN_A::VALUE1
    }
    #[doc = "Core operates in a DMA mode"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == DMAEN_A::VALUE2
    }
}
#[doc = "Field `DMAEn` writer - DMA Enable"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN_A>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Core operates in Slave mode"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN_A::VALUE1)
    }
    #[doc = "Core operates in a DMA mode"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN_A::VALUE2)
    }
}
#[doc = "Field `NPTxFEmpLvl` reader - Non-Periodic TxFIFO Empty Level"]
pub type NPTX_FEMP_LVL_R = crate::BitReader<NPTX_FEMP_LVL_A>;
#[doc = "Non-Periodic TxFIFO Empty Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NPTX_FEMP_LVL_A {
    #[doc = "0: DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is half empty"]
    VALUE1 = 0,
    #[doc = "1: DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is completely empty"]
    VALUE2 = 1,
}
impl From<NPTX_FEMP_LVL_A> for bool {
    #[inline(always)]
    fn from(variant: NPTX_FEMP_LVL_A) -> Self {
        variant as u8 != 0
    }
}
impl NPTX_FEMP_LVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NPTX_FEMP_LVL_A {
        match self.bits {
            false => NPTX_FEMP_LVL_A::VALUE1,
            true => NPTX_FEMP_LVL_A::VALUE2,
        }
    }
    #[doc = "DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is half empty"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == NPTX_FEMP_LVL_A::VALUE1
    }
    #[doc = "DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is completely empty"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == NPTX_FEMP_LVL_A::VALUE2
    }
}
#[doc = "Field `NPTxFEmpLvl` writer - Non-Periodic TxFIFO Empty Level"]
pub type NPTX_FEMP_LVL_W<'a, REG> = crate::BitWriter<'a, REG, NPTX_FEMP_LVL_A>;
impl<'a, REG> NPTX_FEMP_LVL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is half empty"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(NPTX_FEMP_LVL_A::VALUE1)
    }
    #[doc = "DIEPINTx.TxFEmp interrupt indicates that the IN Endpoint TxFIFO is completely empty"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(NPTX_FEMP_LVL_A::VALUE2)
    }
}
#[doc = "Field `PTxFEmpLvl` reader - Periodic TxFIFO Empty Level"]
pub type PTX_FEMP_LVL_R = crate::BitReader<PTX_FEMP_LVL_A>;
#[doc = "Periodic TxFIFO Empty Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTX_FEMP_LVL_A {
    #[doc = "0: GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is half empty"]
    VALUE1 = 0,
    #[doc = "1: GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is completely empty"]
    VALUE2 = 1,
}
impl From<PTX_FEMP_LVL_A> for bool {
    #[inline(always)]
    fn from(variant: PTX_FEMP_LVL_A) -> Self {
        variant as u8 != 0
    }
}
impl PTX_FEMP_LVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PTX_FEMP_LVL_A {
        match self.bits {
            false => PTX_FEMP_LVL_A::VALUE1,
            true => PTX_FEMP_LVL_A::VALUE2,
        }
    }
    #[doc = "GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is half empty"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PTX_FEMP_LVL_A::VALUE1
    }
    #[doc = "GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is completely empty"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PTX_FEMP_LVL_A::VALUE2
    }
}
#[doc = "Field `PTxFEmpLvl` writer - Periodic TxFIFO Empty Level"]
pub type PTX_FEMP_LVL_W<'a, REG> = crate::BitWriter<'a, REG, PTX_FEMP_LVL_A>;
impl<'a, REG> PTX_FEMP_LVL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is half empty"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(PTX_FEMP_LVL_A::VALUE1)
    }
    #[doc = "GINTSTS.PTxFEmp interrupt indicates that the Periodic TxFIFO is completely empty"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(PTX_FEMP_LVL_A::VALUE2)
    }
}
#[doc = "Field `AHBSingle` reader - AHB Single Support"]
pub type AHBSINGLE_R = crate::BitReader<AHBSINGLE_A>;
#[doc = "AHB Single Support\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AHBSINGLE_A {
    #[doc = "0: The remaining data in a transfer is sent using INCR burst size. This is the default mode."]
    VALUE1 = 0,
    #[doc = "1: The remaining data in a transfer is sent using single burst size."]
    VALUE2 = 1,
}
impl From<AHBSINGLE_A> for bool {
    #[inline(always)]
    fn from(variant: AHBSINGLE_A) -> Self {
        variant as u8 != 0
    }
}
impl AHBSINGLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AHBSINGLE_A {
        match self.bits {
            false => AHBSINGLE_A::VALUE1,
            true => AHBSINGLE_A::VALUE2,
        }
    }
    #[doc = "The remaining data in a transfer is sent using INCR burst size. This is the default mode."]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == AHBSINGLE_A::VALUE1
    }
    #[doc = "The remaining data in a transfer is sent using single burst size."]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == AHBSINGLE_A::VALUE2
    }
}
#[doc = "Field `AHBSingle` writer - AHB Single Support"]
pub type AHBSINGLE_W<'a, REG> = crate::BitWriter<'a, REG, AHBSINGLE_A>;
impl<'a, REG> AHBSINGLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The remaining data in a transfer is sent using INCR burst size. This is the default mode."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut crate::W<REG> {
        self.variant(AHBSINGLE_A::VALUE1)
    }
    #[doc = "The remaining data in a transfer is sent using single burst size."]
    #[inline(always)]
    pub fn value2(self) -> &'a mut crate::W<REG> {
        self.variant(AHBSINGLE_A::VALUE2)
    }
}
impl R {
    #[doc = "Bit 0 - Global Interrupt Mask"]
    #[inline(always)]
    pub fn glbl_intr_msk(&self) -> GLBL_INTR_MSK_R {
        GLBL_INTR_MSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Burst Length/Type"]
    #[inline(always)]
    pub fn hbst_len(&self) -> HBST_LEN_R {
        HBST_LEN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO Empty Level"]
    #[inline(always)]
    pub fn nptx_femp_lvl(&self) -> NPTX_FEMP_LVL_R {
        NPTX_FEMP_LVL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Periodic TxFIFO Empty Level"]
    #[inline(always)]
    pub fn ptx_femp_lvl(&self) -> PTX_FEMP_LVL_R {
        PTX_FEMP_LVL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 23 - AHB Single Support"]
    #[inline(always)]
    pub fn ahbsingle(&self) -> AHBSINGLE_R {
        AHBSINGLE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn glbl_intr_msk(&mut self) -> GLBL_INTR_MSK_W<GAHBCFG_SPEC> {
        GLBL_INTR_MSK_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - Burst Length/Type"]
    #[inline(always)]
    #[must_use]
    pub fn hbst_len(&mut self) -> HBST_LEN_W<GAHBCFG_SPEC> {
        HBST_LEN_W::new(self, 1)
    }
    #[doc = "Bit 5 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<GAHBCFG_SPEC> {
        DMAEN_W::new(self, 5)
    }
    #[doc = "Bit 7 - Non-Periodic TxFIFO Empty Level"]
    #[inline(always)]
    #[must_use]
    pub fn nptx_femp_lvl(&mut self) -> NPTX_FEMP_LVL_W<GAHBCFG_SPEC> {
        NPTX_FEMP_LVL_W::new(self, 7)
    }
    #[doc = "Bit 8 - Periodic TxFIFO Empty Level"]
    #[inline(always)]
    #[must_use]
    pub fn ptx_femp_lvl(&mut self) -> PTX_FEMP_LVL_W<GAHBCFG_SPEC> {
        PTX_FEMP_LVL_W::new(self, 8)
    }
    #[doc = "Bit 23 - AHB Single Support"]
    #[inline(always)]
    #[must_use]
    pub fn ahbsingle(&mut self) -> AHBSINGLE_W<GAHBCFG_SPEC> {
        AHBSINGLE_W::new(self, 23)
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
#[doc = "AHB Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gahbcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gahbcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GAHBCFG_SPEC;
impl crate::RegisterSpec for GAHBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gahbcfg::R`](R) reader structure"]
impl crate::Readable for GAHBCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gahbcfg::W`](W) writer structure"]
impl crate::Writable for GAHBCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAHBCFG to value 0"]
impl crate::Resettable for GAHBCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
