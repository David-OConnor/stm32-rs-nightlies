#[doc = "Register `OTG_HCDMAB12` reader"]
pub struct R(crate::R<OTG_HCDMAB12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HCDMAB12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HCDMAB12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HCDMAB12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HCDMAB` reader - HCDMAB"]
pub type HCDMAB_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - HCDMAB"]
    #[inline(always)]
    pub fn hcdmab(&self) -> HCDMAB_R {
        HCDMAB_R::new(self.bits)
    }
}
#[doc = "OTG host channel-n DMA address buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdmab12](index.html) module"]
pub struct OTG_HCDMAB12_SPEC;
impl crate::RegisterSpec for OTG_HCDMAB12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hcdmab12::R](R) reader structure"]
impl crate::Readable for OTG_HCDMAB12_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTG_HCDMAB12 to value 0"]
impl crate::Resettable for OTG_HCDMAB12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
