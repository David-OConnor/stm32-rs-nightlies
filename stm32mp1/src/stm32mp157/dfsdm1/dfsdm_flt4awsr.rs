#[doc = "Register `DFSDM_FLT4AWSR` reader"]
pub struct R(crate::R<DFSDM_FLT4AWSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT4AWSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT4AWSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT4AWSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AWLTF` reader - AWLTF"]
pub type AWLTF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWHTF` reader - AWHTF"]
pub type AWHTF_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - AWLTF"]
    #[inline(always)]
    pub fn awltf(&self) -> AWLTF_R {
        AWLTF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - AWHTF"]
    #[inline(always)]
    pub fn awhtf(&self) -> AWHTF_R {
        AWHTF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "DFSDM filter 4 analog watchdog status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt4awsr](index.html) module"]
pub struct DFSDM_FLT4AWSR_SPEC;
impl crate::RegisterSpec for DFSDM_FLT4AWSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_flt4awsr::R](R) reader structure"]
impl crate::Readable for DFSDM_FLT4AWSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFSDM_FLT4AWSR to value 0"]
impl crate::Resettable for DFSDM_FLT4AWSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
