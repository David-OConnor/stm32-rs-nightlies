#[doc = "Register `TZC_PID2` reader"]
pub struct R(crate::R<TZC_PID2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_PID2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_PID2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_PID2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PER_ID_2` reader - PER_ID_2"]
pub type PER_ID_2_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PER_ID_2"]
    #[inline(always)]
    pub fn per_id_2(&self) -> PER_ID_2_R {
        PER_ID_2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Peripheral ID 2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_pid2](index.html) module"]
pub struct TZC_PID2_SPEC;
impl crate::RegisterSpec for TZC_PID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_pid2::R](R) reader structure"]
impl crate::Readable for TZC_PID2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TZC_PID2 to value 0x2b"]
impl crate::Resettable for TZC_PID2_SPEC {
    const RESET_VALUE: Self::Ux = 0x2b;
}
