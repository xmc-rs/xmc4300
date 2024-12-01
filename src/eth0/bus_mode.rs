#[doc = "Register `BUS_MODE` reader"]
pub type R = crate::R<BUS_MODE_SPEC>;
#[doc = "Register `BUS_MODE` writer"]
pub type W = crate::W<BUS_MODE_SPEC>;
#[doc = "Field `SWR` reader - Software Reset"]
pub type SWR_R = crate::BitReader;
#[doc = "Field `SWR` writer - Software Reset"]
pub type SWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DA` reader - DMA Arbitration Scheme"]
pub type DA_R = crate::BitReader;
#[doc = "Field `DA` writer - DMA Arbitration Scheme"]
pub type DA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSL` reader - Descriptor Skip Length"]
pub type DSL_R = crate::FieldReader;
#[doc = "Field `DSL` writer - Descriptor Skip Length"]
pub type DSL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ATDS` reader - Alternate Descriptor Size"]
pub type ATDS_R = crate::BitReader;
#[doc = "Field `ATDS` writer - Alternate Descriptor Size"]
pub type ATDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBL` reader - Programmable Burst Length"]
pub type PBL_R = crate::FieldReader;
#[doc = "Field `PBL` writer - Programmable Burst Length"]
pub type PBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PR` reader - Priority Ratio"]
pub type PR_R = crate::FieldReader;
#[doc = "Field `PR` writer - Priority Ratio"]
pub type PR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FB` reader - Fixed Burst"]
pub type FB_R = crate::BitReader;
#[doc = "Field `FB` writer - Fixed Burst"]
pub type FB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPBL` reader - Rx DMA PBL"]
pub type RPBL_R = crate::FieldReader;
#[doc = "Field `RPBL` writer - Rx DMA PBL"]
pub type RPBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `USP` reader - Use Seperate PBL"]
pub type USP_R = crate::BitReader;
#[doc = "Field `USP` writer - Use Seperate PBL"]
pub type USP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBLX8` reader - 8xPBL Mode"]
pub type PBLX8_R = crate::BitReader;
#[doc = "Field `PBLX8` writer - 8xPBL Mode"]
pub type PBLX8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AAL` reader - Address Aligned Beats"]
pub type AAL_R = crate::BitReader;
#[doc = "Field `AAL` writer - Address Aligned Beats"]
pub type AAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB` reader - Mixed Burst"]
pub type MB_R = crate::BitReader;
#[doc = "Field `MB` writer - Mixed Burst"]
pub type MB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPR` reader - Transmit Priority"]
pub type TXPR_R = crate::BitReader;
#[doc = "Field `TXPR` writer - Transmit Priority"]
pub type TXPR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRWG` reader - Channel Priority Weights"]
pub type PRWG_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Arbitration Scheme"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length"]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Alternate Descriptor Size"]
    #[inline(always)]
    pub fn atds(&self) -> ATDS_R {
        ATDS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Programmable Burst Length"]
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Priority Ratio"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Fixed Burst"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    pub fn rpbl(&self) -> RPBL_R {
        RPBL_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Use Seperate PBL"]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 8xPBL Mode"]
    #[inline(always)]
    pub fn pblx8(&self) -> PBLX8_R {
        PBLX8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Address Aligned Beats"]
    #[inline(always)]
    pub fn aal(&self) -> AAL_R {
        AAL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Mixed Burst"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit Priority"]
    #[inline(always)]
    pub fn txpr(&self) -> TXPR_R {
        TXPR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Channel Priority Weights"]
    #[inline(always)]
    pub fn prwg(&self) -> PRWG_R {
        PRWG_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W<BUS_MODE_SPEC> {
        SWR_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Arbitration Scheme"]
    #[inline(always)]
    pub fn da(&mut self) -> DA_W<BUS_MODE_SPEC> {
        DA_W::new(self, 1)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length"]
    #[inline(always)]
    pub fn dsl(&mut self) -> DSL_W<BUS_MODE_SPEC> {
        DSL_W::new(self, 2)
    }
    #[doc = "Bit 7 - Alternate Descriptor Size"]
    #[inline(always)]
    pub fn atds(&mut self) -> ATDS_W<BUS_MODE_SPEC> {
        ATDS_W::new(self, 7)
    }
    #[doc = "Bits 8:13 - Programmable Burst Length"]
    #[inline(always)]
    pub fn pbl(&mut self) -> PBL_W<BUS_MODE_SPEC> {
        PBL_W::new(self, 8)
    }
    #[doc = "Bits 14:15 - Priority Ratio"]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W<BUS_MODE_SPEC> {
        PR_W::new(self, 14)
    }
    #[doc = "Bit 16 - Fixed Burst"]
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W<BUS_MODE_SPEC> {
        FB_W::new(self, 16)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    pub fn rpbl(&mut self) -> RPBL_W<BUS_MODE_SPEC> {
        RPBL_W::new(self, 17)
    }
    #[doc = "Bit 23 - Use Seperate PBL"]
    #[inline(always)]
    pub fn usp(&mut self) -> USP_W<BUS_MODE_SPEC> {
        USP_W::new(self, 23)
    }
    #[doc = "Bit 24 - 8xPBL Mode"]
    #[inline(always)]
    pub fn pblx8(&mut self) -> PBLX8_W<BUS_MODE_SPEC> {
        PBLX8_W::new(self, 24)
    }
    #[doc = "Bit 25 - Address Aligned Beats"]
    #[inline(always)]
    pub fn aal(&mut self) -> AAL_W<BUS_MODE_SPEC> {
        AAL_W::new(self, 25)
    }
    #[doc = "Bit 26 - Mixed Burst"]
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W<BUS_MODE_SPEC> {
        MB_W::new(self, 26)
    }
    #[doc = "Bit 27 - Transmit Priority"]
    #[inline(always)]
    pub fn txpr(&mut self) -> TXPR_W<BUS_MODE_SPEC> {
        TXPR_W::new(self, 27)
    }
}
#[doc = "Bus Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUS_MODE_SPEC;
impl crate::RegisterSpec for BUS_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_mode::R`](R) reader structure"]
impl crate::Readable for BUS_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bus_mode::W`](W) writer structure"]
impl crate::Writable for BUS_MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUS_MODE to value 0x0002_0101"]
impl crate::Resettable for BUS_MODE_SPEC {
    const RESET_VALUE: u32 = 0x0002_0101;
}
