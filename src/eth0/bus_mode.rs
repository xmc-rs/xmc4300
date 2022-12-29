#[doc = "Register `BUS_MODE` reader"]
pub struct R(crate::R<BUS_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUS_MODE` writer"]
pub struct W(crate::W<BUS_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUS_MODE_SPEC>;
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
impl From<crate::W<BUS_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUS_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWR` reader - Software Reset"]
pub type SWR_R = crate::BitReader<bool>;
#[doc = "Field `SWR` writer - Software Reset"]
pub type SWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUS_MODE_SPEC, bool, O>;
#[doc = "Field `DA` reader - DMA Arbitration Scheme"]
pub type DA_R = crate::BitReader<bool>;
#[doc = "Field `DA` writer - DMA Arbitration Scheme"]
pub type DA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUS_MODE_SPEC, bool, O>;
#[doc = "Field `DSL` reader - Descriptor Skip Length"]
pub type DSL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSL` writer - Descriptor Skip Length"]
pub type DSL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUS_MODE_SPEC, u8, u8, 5, O>;
#[doc = "Field `ATDS` reader - Alternate Descriptor Size"]
pub type ATDS_R = crate::BitReader<bool>;
#[doc = "Field `ATDS` writer - Alternate Descriptor Size"]
pub type ATDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUS_MODE_SPEC, bool, O>;
#[doc = "Field `PBL` reader - Programmable Burst Length"]
pub type PBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PBL` writer - Programmable Burst Length"]
pub type PBL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUS_MODE_SPEC, u8, u8, 6, O>;
#[doc = "Field `PR` reader - Priority Ratio"]
pub type PR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PR` writer - Priority Ratio"]
pub type PR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUS_MODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `FB` reader - Fixed Burst"]
pub type FB_R = crate::BitReader<bool>;
#[doc = "Field `FB` writer - Fixed Burst"]
pub type FB_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUS_MODE_SPEC, bool, O>;
#[doc = "Field `RPBL` reader - Rx DMA PBL"]
pub type RPBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RPBL` writer - Rx DMA PBL"]
pub type RPBL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BUS_MODE_SPEC, u8, u8, 6, O>;
#[doc = "Field `USP` reader - Use Seperate PBL"]
pub type USP_R = crate::BitReader<bool>;
#[doc = "Field `USP` writer - Use Seperate PBL"]
pub type USP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUS_MODE_SPEC, bool, O>;
#[doc = "Field `PBLX8` reader - 8xPBL Mode"]
pub type PBLX8_R = crate::BitReader<bool>;
#[doc = "Field `PBLX8` writer - 8xPBL Mode"]
pub type PBLX8_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUS_MODE_SPEC, bool, O>;
#[doc = "Field `AAL` reader - Address Aligned Beats"]
pub type AAL_R = crate::BitReader<bool>;
#[doc = "Field `AAL` writer - Address Aligned Beats"]
pub type AAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUS_MODE_SPEC, bool, O>;
#[doc = "Field `MB` reader - Mixed Burst"]
pub type MB_R = crate::BitReader<bool>;
#[doc = "Field `MB` writer - Mixed Burst"]
pub type MB_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUS_MODE_SPEC, bool, O>;
#[doc = "Field `TXPR` reader - Transmit Priority"]
pub type TXPR_R = crate::BitReader<bool>;
#[doc = "Field `TXPR` writer - Transmit Priority"]
pub type TXPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BUS_MODE_SPEC, bool, O>;
#[doc = "Field `PRWG` reader - Channel Priority Weights"]
pub type PRWG_R = crate::FieldReader<u8, u8>;
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
    #[must_use]
    pub fn swr(&mut self) -> SWR_W<0> {
        SWR_W::new(self)
    }
    #[doc = "Bit 1 - DMA Arbitration Scheme"]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<1> {
        DA_W::new(self)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length"]
    #[inline(always)]
    #[must_use]
    pub fn dsl(&mut self) -> DSL_W<2> {
        DSL_W::new(self)
    }
    #[doc = "Bit 7 - Alternate Descriptor Size"]
    #[inline(always)]
    #[must_use]
    pub fn atds(&mut self) -> ATDS_W<7> {
        ATDS_W::new(self)
    }
    #[doc = "Bits 8:13 - Programmable Burst Length"]
    #[inline(always)]
    #[must_use]
    pub fn pbl(&mut self) -> PBL_W<8> {
        PBL_W::new(self)
    }
    #[doc = "Bits 14:15 - Priority Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<14> {
        PR_W::new(self)
    }
    #[doc = "Bit 16 - Fixed Burst"]
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FB_W<16> {
        FB_W::new(self)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    #[must_use]
    pub fn rpbl(&mut self) -> RPBL_W<17> {
        RPBL_W::new(self)
    }
    #[doc = "Bit 23 - Use Seperate PBL"]
    #[inline(always)]
    #[must_use]
    pub fn usp(&mut self) -> USP_W<23> {
        USP_W::new(self)
    }
    #[doc = "Bit 24 - 8xPBL Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pblx8(&mut self) -> PBLX8_W<24> {
        PBLX8_W::new(self)
    }
    #[doc = "Bit 25 - Address Aligned Beats"]
    #[inline(always)]
    #[must_use]
    pub fn aal(&mut self) -> AAL_W<25> {
        AAL_W::new(self)
    }
    #[doc = "Bit 26 - Mixed Burst"]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MB_W<26> {
        MB_W::new(self)
    }
    #[doc = "Bit 27 - Transmit Priority"]
    #[inline(always)]
    #[must_use]
    pub fn txpr(&mut self) -> TXPR_W<27> {
        TXPR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_mode](index.html) module"]
pub struct BUS_MODE_SPEC;
impl crate::RegisterSpec for BUS_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus_mode::R](R) reader structure"]
impl crate::Readable for BUS_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bus_mode::W](W) writer structure"]
impl crate::Writable for BUS_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUS_MODE to value 0x0002_0101"]
impl crate::Resettable for BUS_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0101;
}
