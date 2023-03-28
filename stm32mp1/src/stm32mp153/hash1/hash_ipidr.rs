#[doc = "Register `HASH_IPIDR` reader"]
pub struct R(crate::R<HASH_IPIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_IPIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_IPIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_IPIDR_SPEC>) -> Self {
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
#[doc = "HASH Identification\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_ipidr](index.html) module"]
pub struct HASH_IPIDR_SPEC;
impl crate::RegisterSpec for HASH_IPIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_ipidr::R](R) reader structure"]
impl crate::Readable for HASH_IPIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HASH_IPIDR to value 0x0017_0031"]
impl crate::Resettable for HASH_IPIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0017_0031;
}
