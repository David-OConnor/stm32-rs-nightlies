#[doc = "Register `LPTIM_CNT` reader"]
pub struct R(crate::R<LPTIM_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - Counter value When the LPTIM is running with an asynchronous clock, reading the LPTIM_CNT register may return unreliable values. So in this case it is necessary to perform two consecutive read accesses and verify that the two returned values are identical."]
pub type CNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter value When the LPTIM is running with an asynchronous clock, reading the LPTIM_CNT register may return unreliable values. So in this case it is necessary to perform two consecutive read accesses and verify that the two returned values are identical."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "LPTIM counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptim_cnt](index.html) module"]
pub struct LPTIM_CNT_SPEC;
impl crate::RegisterSpec for LPTIM_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptim_cnt::R](R) reader structure"]
impl crate::Readable for LPTIM_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LPTIM_CNT to value 0"]
impl crate::Resettable for LPTIM_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
