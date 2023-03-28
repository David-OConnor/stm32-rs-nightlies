#[doc = "Register `RCC_MP_APRSTSR` reader"]
pub struct R(crate::R<RCC_MP_APRSTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_APRSTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_APRSTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_APRSTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RSTTOV` reader - RSTTOV"]
pub type RSTTOV_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 8:14 - RSTTOV"]
    #[inline(always)]
    pub fn rsttov(&self) -> RSTTOV_R {
        RSTTOV_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
#[doc = "This register provides a status of the RDCTL. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_aprstsr](index.html) module"]
pub struct RCC_MP_APRSTSR_SPEC;
impl crate::RegisterSpec for RCC_MP_APRSTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_aprstsr::R](R) reader structure"]
impl crate::Readable for RCC_MP_APRSTSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RCC_MP_APRSTSR to value 0"]
impl crate::Resettable for RCC_MP_APRSTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
