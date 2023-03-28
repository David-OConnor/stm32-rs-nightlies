#[doc = "Register `STGENR_CIDR2` reader"]
pub struct R(crate::R<STGENR_CIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENR_CIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENR_CIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENR_CIDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRMBL_2` reader - PRMBL_2"]
pub type PRMBL_2_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PRMBL_2"]
    #[inline(always)]
    pub fn prmbl_2(&self) -> PRMBL_2_R {
        PRMBL_2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "STGENR component ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_cidr2](index.html) module"]
pub struct STGENR_CIDR2_SPEC;
impl crate::RegisterSpec for STGENR_CIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenr_cidr2::R](R) reader structure"]
impl crate::Readable for STGENR_CIDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STGENR_CIDR2 to value 0x50"]
impl crate::Resettable for STGENR_CIDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x50;
}
