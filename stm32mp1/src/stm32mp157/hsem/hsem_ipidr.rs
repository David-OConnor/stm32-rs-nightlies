#[doc = "Register `HSEM_IPIDR` reader"]
pub struct R(crate::R<HSEM_IPIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSEM_IPIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSEM_IPIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSEM_IPIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IPID` reader - IPID"]
pub type IPID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IPID"]
    #[inline(always)]
    pub fn ipid(&self) -> IPID_R {
        IPID_R::new(self.bits)
    }
}
#[doc = "HSEM IP identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_ipidr](index.html) module"]
pub struct HSEM_IPIDR_SPEC;
impl crate::RegisterSpec for HSEM_IPIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsem_ipidr::R](R) reader structure"]
impl crate::Readable for HSEM_IPIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HSEM_IPIDR to value 0x0010_0072"]
impl crate::Resettable for HSEM_IPIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0072;
}
