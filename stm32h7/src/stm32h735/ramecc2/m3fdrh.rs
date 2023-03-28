#[doc = "Register `M3FDRH` reader"]
pub struct R(crate::R<M3FDRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M3FDRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M3FDRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M3FDRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FDATAH` reader - Failing data high (64-bit memory)"]
pub type FDATAH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Failing data high (64-bit memory)"]
    #[inline(always)]
    pub fn fdatah(&self) -> FDATAH_R {
        FDATAH_R::new(self.bits)
    }
}
#[doc = "RAMECC monitor x failing data high register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m3fdrh](index.html) module"]
pub struct M3FDRH_SPEC;
impl crate::RegisterSpec for M3FDRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m3fdrh::R](R) reader structure"]
impl crate::Readable for M3FDRH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets M3FDRH to value 0"]
impl crate::Resettable for M3FDRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
