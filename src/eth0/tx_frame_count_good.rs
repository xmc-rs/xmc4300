#[doc = "Register `TX_FRAME_COUNT_GOOD` reader"]
pub struct R(crate::R<TX_FRAME_COUNT_GOOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_FRAME_COUNT_GOOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_FRAME_COUNT_GOOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_FRAME_COUNT_GOOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXFRMG` reader - This field indicates the number of transmitted good frames, exclusive of preamble."]
pub struct TXFRMG_R(crate::FieldReader<u32, u32>);
impl TXFRMG_R {
    pub(crate) fn new(bits: u32) -> Self {
        TXFRMG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFRMG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - This field indicates the number of transmitted good frames, exclusive of preamble."]
    #[inline(always)]
    pub fn txfrmg(&self) -> TXFRMG_R {
        TXFRMG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Tx Frame Count Good Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_frame_count_good](index.html) module"]
pub struct TX_FRAME_COUNT_GOOD_SPEC;
impl crate::RegisterSpec for TX_FRAME_COUNT_GOOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_frame_count_good::R](R) reader structure"]
impl crate::Readable for TX_FRAME_COUNT_GOOD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TX_FRAME_COUNT_GOOD to value 0"]
impl crate::Resettable for TX_FRAME_COUNT_GOOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
