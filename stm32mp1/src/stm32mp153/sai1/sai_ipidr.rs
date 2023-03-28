#[doc = "Register `SAI_IPIDR` reader"]
pub struct R(crate::R<SAI_IPIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_IPIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_IPIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_IPIDR_SPEC>) -> Self {
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
#[doc = "SAI identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_ipidr](index.html) module"]
pub struct SAI_IPIDR_SPEC;
impl crate::RegisterSpec for SAI_IPIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_ipidr::R](R) reader structure"]
impl crate::Readable for SAI_IPIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAI_IPIDR to value 0x0013_0031"]
impl crate::Resettable for SAI_IPIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0013_0031;
}
