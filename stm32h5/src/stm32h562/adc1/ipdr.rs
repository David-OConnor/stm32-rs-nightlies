#[doc = "Register `IPDR` reader"]
pub struct R(crate::R<IPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ID` reader - Peripheral identifier These bits returns the ADC identifier. ID\\[31:0\\]
= 0x0011 0006: c7amba_aditf5_90_v1"]
pub type ID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral identifier These bits returns the ADC identifier. ID\\[31:0\\]
= 0x0011 0006: c7amba_aditf5_90_v1"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "ADC identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipdr](index.html) module"]
pub struct IPDR_SPEC;
impl crate::RegisterSpec for IPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipdr::R](R) reader structure"]
impl crate::Readable for IPDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IPDR to value 0x0011_0006"]
impl crate::Resettable for IPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0011_0006;
}
