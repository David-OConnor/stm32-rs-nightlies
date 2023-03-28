#[doc = "Register `IPID` reader"]
pub struct R(crate::R<IPID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPID_SPEC>) -> Self {
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
#[doc = "UCPD IP ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipid](index.html) module"]
pub struct IPID_SPEC;
impl crate::RegisterSpec for IPID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipid::R](R) reader structure"]
impl crate::Readable for IPID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IPID to value 0x0015_0021"]
impl crate::Resettable for IPID_SPEC {
    const RESET_VALUE: Self::Ux = 0x0015_0021;
}
