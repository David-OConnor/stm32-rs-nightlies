#[doc = "Register `OTG_HCDMAB0` reader"]
pub struct R(crate::R<OTG_HCDMAB0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HCDMAB0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HCDMAB0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HCDMAB0_SPEC>) -> Self {
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
#[doc = "OTG host channel-n DMA address buffer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdmab0](index.html) module"]
pub struct OTG_HCDMAB0_SPEC;
impl crate::RegisterSpec for OTG_HCDMAB0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hcdmab0::R](R) reader structure"]
impl crate::Readable for OTG_HCDMAB0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTG_HCDMAB0 to value 0"]
impl crate::Resettable for OTG_HCDMAB0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
