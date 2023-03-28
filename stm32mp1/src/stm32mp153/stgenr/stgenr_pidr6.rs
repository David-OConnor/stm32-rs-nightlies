#[doc = "Register `STGENR_PIDR6` reader"]
pub struct R(crate::R<STGENR_PIDR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENR_PIDR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENR_PIDR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENR_PIDR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PIDR6` reader - PIDR6"]
pub type PIDR6_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - PIDR6"]
    #[inline(always)]
    pub fn pidr6(&self) -> PIDR6_R {
        PIDR6_R::new(self.bits)
    }
}
#[doc = "STGENR peripheral ID6 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_pidr6](index.html) module"]
pub struct STGENR_PIDR6_SPEC;
impl crate::RegisterSpec for STGENR_PIDR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenr_pidr6::R](R) reader structure"]
impl crate::Readable for STGENR_PIDR6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STGENR_PIDR6 to value 0"]
impl crate::Resettable for STGENR_PIDR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
