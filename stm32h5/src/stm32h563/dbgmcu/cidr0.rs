#[doc = "Register `CIDR0` reader"]
pub struct R(crate::R<CIDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PREAMBLE` reader - component identification bits \\[7:0\\]"]
pub type PREAMBLE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - component identification bits \\[7:0\\]"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DBGMCU CoreSight component identity register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr0](index.html) module"]
pub struct CIDR0_SPEC;
impl crate::RegisterSpec for CIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cidr0::R](R) reader structure"]
impl crate::Readable for CIDR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIDR0 to value 0x0d"]
impl crate::Resettable for CIDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d;
}
