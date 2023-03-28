#[doc = "Register `RCC_IDR` reader"]
pub struct R(crate::R<RCC_IDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_IDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_IDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_IDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "This register gives the unique identifier of the RCC\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_idr](index.html) module"]
pub struct RCC_IDR_SPEC;
impl crate::RegisterSpec for RCC_IDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_idr::R](R) reader structure"]
impl crate::Readable for RCC_IDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RCC_IDR to value 0x01"]
impl crate::Resettable for RCC_IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
