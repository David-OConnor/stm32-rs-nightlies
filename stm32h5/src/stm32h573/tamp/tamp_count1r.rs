#[doc = "Register `TAMP_COUNT1R` reader"]
pub struct R(crate::R<TAMP_COUNT1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_COUNT1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_COUNT1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_COUNT1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT` reader - This register is read-only only and is incremented by one when a write access is done to this register. This register cannot roll-over and is frozen when reaching the maximum value."]
pub type COUNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is read-only only and is incremented by one when a write access is done to this register. This register cannot roll-over and is frozen when reaching the maximum value."]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
#[doc = "TAMP monotonic counter 1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_count1r](index.html) module"]
pub struct TAMP_COUNT1R_SPEC;
impl crate::RegisterSpec for TAMP_COUNT1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamp_count1r::R](R) reader structure"]
impl crate::Readable for TAMP_COUNT1R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TAMP_COUNT1R to value 0"]
impl crate::Resettable for TAMP_COUNT1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
