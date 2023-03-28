#[doc = "Register `NSBOOTR_CUR` reader"]
pub struct R(crate::R<NSBOOTR_CUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NSBOOTR_CUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NSBOOTR_CUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NSBOOTR_CUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NSBOOT_LOCK` reader - A field locking the values of SWAP_BANK, and NSBOOTADD settings."]
pub type NSBOOT_LOCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NSBOOTADD` reader - unique boot entry address These bits reflect the UBE address"]
pub type NSBOOTADD_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:7 - A field locking the values of SWAP_BANK, and NSBOOTADD settings."]
    #[inline(always)]
    pub fn nsboot_lock(&self) -> NSBOOT_LOCK_R {
        NSBOOT_LOCK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - unique boot entry address These bits reflect the UBE address"]
    #[inline(always)]
    pub fn nsbootadd(&self) -> NSBOOTADD_R {
        NSBOOTADD_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "FLASH non-secure unique boot entry register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsbootr_cur](index.html) module"]
pub struct NSBOOTR_CUR_SPEC;
impl crate::RegisterSpec for NSBOOTR_CUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nsbootr_cur::R](R) reader structure"]
impl crate::Readable for NSBOOTR_CUR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NSBOOTR_CUR to value 0"]
impl crate::Resettable for NSBOOTR_CUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
