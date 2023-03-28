#[doc = "Register `TZC_FAIL_CONTROL1` reader"]
pub struct R(crate::R<TZC_FAIL_CONTROL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_FAIL_CONTROL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_FAIL_CONTROL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_FAIL_CONTROL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRIVILEGE` reader - PRIVILEGE"]
pub type PRIVILEGE_R = crate::BitReader<bool>;
#[doc = "Field `NON_SECURE` reader - NON_SECURE"]
pub type NON_SECURE_R = crate::BitReader<bool>;
#[doc = "Field `DIRECTION` reader - DIRECTION"]
pub type DIRECTION_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 20 - PRIVILEGE"]
    #[inline(always)]
    pub fn privilege(&self) -> PRIVILEGE_R {
        PRIVILEGE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - NON_SECURE"]
    #[inline(always)]
    pub fn non_secure(&self) -> NON_SECURE_R {
        NON_SECURE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - DIRECTION"]
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Status information about the first access that failed a region permission check in the associated filter (0 to 1).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_fail_control1](index.html) module"]
pub struct TZC_FAIL_CONTROL1_SPEC;
impl crate::RegisterSpec for TZC_FAIL_CONTROL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_fail_control1::R](R) reader structure"]
impl crate::Readable for TZC_FAIL_CONTROL1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TZC_FAIL_CONTROL1 to value 0"]
impl crate::Resettable for TZC_FAIL_CONTROL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
