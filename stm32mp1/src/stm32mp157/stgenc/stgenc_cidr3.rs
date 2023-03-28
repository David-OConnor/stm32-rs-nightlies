#[doc = "Register `STGENC_CIDR3` reader"]
pub struct R(crate::R<STGENC_CIDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENC_CIDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENC_CIDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENC_CIDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRMBL_3` reader - PRMBL_3"]
pub type PRMBL_3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PRMBL_3"]
    #[inline(always)]
    pub fn prmbl_3(&self) -> PRMBL_3_R {
        PRMBL_3_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "STGENC component ID3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_cidr3](index.html) module"]
pub struct STGENC_CIDR3_SPEC;
impl crate::RegisterSpec for STGENC_CIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenc_cidr3::R](R) reader structure"]
impl crate::Readable for STGENC_CIDR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STGENC_CIDR3 to value 0xb1"]
impl crate::Resettable for STGENC_CIDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0xb1;
}
