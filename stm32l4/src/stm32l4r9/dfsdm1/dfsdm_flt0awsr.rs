#[doc = "Register `DFSDM_FLT0AWSR` reader"]
pub struct R(crate::R<DFSDM_FLT0AWSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT0AWSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT0AWSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT0AWSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AWLTF` reader - Analog watchdog low threshold flag"]
pub type AWLTF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWHTF` reader - Analog watchdog high threshold flag"]
pub type AWHTF_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Analog watchdog low threshold flag"]
    #[inline(always)]
    pub fn awltf(&self) -> AWLTF_R {
        AWLTF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Analog watchdog high threshold flag"]
    #[inline(always)]
    pub fn awhtf(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "analog watchdog status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt0awsr](index.html) module"]
pub struct DFSDM_FLT0AWSR_SPEC;
impl crate::RegisterSpec for DFSDM_FLT0AWSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_flt0awsr::R](R) reader structure"]
impl crate::Readable for DFSDM_FLT0AWSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFSDM_FLT0AWSR to value 0"]
impl crate::Resettable for DFSDM_FLT0AWSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
