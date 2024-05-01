#[doc = "Register `BUS_MODE` reader"]
pub type R = crate::R<BusModeSpec>;
#[doc = "Register `BUS_MODE` writer"]
pub type W = crate::W<BusModeSpec>;
#[doc = "Field `SWR` reader - Software Reset"]
pub type SwrR = crate::BitReader;
#[doc = "Field `SWR` writer - Software Reset"]
pub type SwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DA` reader - DMA Arbitration Scheme"]
pub type DaR = crate::BitReader;
#[doc = "Field `DA` writer - DMA Arbitration Scheme"]
pub type DaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSL` reader - Descriptor Skip Length"]
pub type DslR = crate::FieldReader;
#[doc = "Field `DSL` writer - Descriptor Skip Length"]
pub type DslW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ATDS` reader - Alternate Descriptor Size"]
pub type AtdsR = crate::BitReader;
#[doc = "Field `ATDS` writer - Alternate Descriptor Size"]
pub type AtdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBL` reader - Programmable Burst Length"]
pub type PblR = crate::FieldReader;
#[doc = "Field `PBL` writer - Programmable Burst Length"]
pub type PblW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PR` reader - Priority Ratio"]
pub type PrR = crate::FieldReader;
#[doc = "Field `PR` writer - Priority Ratio"]
pub type PrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FB` reader - Fixed Burst"]
pub type FbR = crate::BitReader;
#[doc = "Field `FB` writer - Fixed Burst"]
pub type FbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPBL` reader - Rx DMA PBL"]
pub type RpblR = crate::FieldReader;
#[doc = "Field `RPBL` writer - Rx DMA PBL"]
pub type RpblW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `USP` reader - Use Seperate PBL"]
pub type UspR = crate::BitReader;
#[doc = "Field `USP` writer - Use Seperate PBL"]
pub type UspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBLX8` reader - 8xPBL Mode"]
pub type Pblx8R = crate::BitReader;
#[doc = "Field `PBLX8` writer - 8xPBL Mode"]
pub type Pblx8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AAL` reader - Address Aligned Beats"]
pub type AalR = crate::BitReader;
#[doc = "Field `AAL` writer - Address Aligned Beats"]
pub type AalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB` reader - Mixed Burst"]
pub type MbR = crate::BitReader;
#[doc = "Field `MB` writer - Mixed Burst"]
pub type MbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPR` reader - Transmit Priority"]
pub type TxprR = crate::BitReader;
#[doc = "Field `TXPR` writer - Transmit Priority"]
pub type TxprW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRWG` reader - Channel Priority Weights"]
pub type PrwgR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&self) -> SwrR {
        SwrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Arbitration Scheme"]
    #[inline(always)]
    pub fn da(&self) -> DaR {
        DaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length"]
    #[inline(always)]
    pub fn dsl(&self) -> DslR {
        DslR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Alternate Descriptor Size"]
    #[inline(always)]
    pub fn atds(&self) -> AtdsR {
        AtdsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Programmable Burst Length"]
    #[inline(always)]
    pub fn pbl(&self) -> PblR {
        PblR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Priority Ratio"]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Fixed Burst"]
    #[inline(always)]
    pub fn fb(&self) -> FbR {
        FbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    pub fn rpbl(&self) -> RpblR {
        RpblR::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Use Seperate PBL"]
    #[inline(always)]
    pub fn usp(&self) -> UspR {
        UspR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 8xPBL Mode"]
    #[inline(always)]
    pub fn pblx8(&self) -> Pblx8R {
        Pblx8R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Address Aligned Beats"]
    #[inline(always)]
    pub fn aal(&self) -> AalR {
        AalR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Mixed Burst"]
    #[inline(always)]
    pub fn mb(&self) -> MbR {
        MbR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit Priority"]
    #[inline(always)]
    pub fn txpr(&self) -> TxprR {
        TxprR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Channel Priority Weights"]
    #[inline(always)]
    pub fn prwg(&self) -> PrwgR {
        PrwgR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swr(&mut self) -> SwrW<BusModeSpec> {
        SwrW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Arbitration Scheme"]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DaW<BusModeSpec> {
        DaW::new(self, 1)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length"]
    #[inline(always)]
    #[must_use]
    pub fn dsl(&mut self) -> DslW<BusModeSpec> {
        DslW::new(self, 2)
    }
    #[doc = "Bit 7 - Alternate Descriptor Size"]
    #[inline(always)]
    #[must_use]
    pub fn atds(&mut self) -> AtdsW<BusModeSpec> {
        AtdsW::new(self, 7)
    }
    #[doc = "Bits 8:13 - Programmable Burst Length"]
    #[inline(always)]
    #[must_use]
    pub fn pbl(&mut self) -> PblW<BusModeSpec> {
        PblW::new(self, 8)
    }
    #[doc = "Bits 14:15 - Priority Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PrW<BusModeSpec> {
        PrW::new(self, 14)
    }
    #[doc = "Bit 16 - Fixed Burst"]
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FbW<BusModeSpec> {
        FbW::new(self, 16)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    #[must_use]
    pub fn rpbl(&mut self) -> RpblW<BusModeSpec> {
        RpblW::new(self, 17)
    }
    #[doc = "Bit 23 - Use Seperate PBL"]
    #[inline(always)]
    #[must_use]
    pub fn usp(&mut self) -> UspW<BusModeSpec> {
        UspW::new(self, 23)
    }
    #[doc = "Bit 24 - 8xPBL Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pblx8(&mut self) -> Pblx8W<BusModeSpec> {
        Pblx8W::new(self, 24)
    }
    #[doc = "Bit 25 - Address Aligned Beats"]
    #[inline(always)]
    #[must_use]
    pub fn aal(&mut self) -> AalW<BusModeSpec> {
        AalW::new(self, 25)
    }
    #[doc = "Bit 26 - Mixed Burst"]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MbW<BusModeSpec> {
        MbW::new(self, 26)
    }
    #[doc = "Bit 27 - Transmit Priority"]
    #[inline(always)]
    #[must_use]
    pub fn txpr(&mut self) -> TxprW<BusModeSpec> {
        TxprW::new(self, 27)
    }
}
#[doc = "Bus Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusModeSpec;
impl crate::RegisterSpec for BusModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_mode::R`](R) reader structure"]
impl crate::Readable for BusModeSpec {}
#[doc = "`write(|w| ..)` method takes [`bus_mode::W`](W) writer structure"]
impl crate::Writable for BusModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUS_MODE to value 0x0002_0101"]
impl crate::Resettable for BusModeSpec {
    const RESET_VALUE: u32 = 0x0002_0101;
}
