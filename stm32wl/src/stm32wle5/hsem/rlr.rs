#[doc = "Register `RLR%s` reader"]
pub struct R(crate::R<RLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PROCID` reader - Semaphore ProcessID"]
pub type PROCID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COREID` reader - Semaphore MASTERID"]
pub type COREID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCK` reader - Lock indication"]
pub type LOCK_R = crate::BitReader<LOCKR_A>;
#[doc = "Lock indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKR_A {
    #[doc = "0: Semaphore is free"]
    Free = 0,
    #[doc = "1: Semaphore is locked"]
    Locked = 1,
}
impl From<LOCKR_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKR_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKR_A {
        match self.bits {
            false => LOCKR_A::Free,
            true => LOCKR_A::Locked,
        }
    }
    #[doc = "Checks if the value of the field is `Free`"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == LOCKR_A::Free
    }
    #[doc = "Checks if the value of the field is `Locked`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKR_A::Locked
    }
}
impl R {
    #[doc = "Bits 0:7 - Semaphore ProcessID"]
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Semaphore MASTERID"]
    #[inline(always)]
    pub fn coreid(&self) -> COREID_R {
        COREID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Lock indication"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "HSEM Read lock register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlr](index.html) module"]
pub struct RLR_SPEC;
impl crate::RegisterSpec for RLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rlr::R](R) reader structure"]
impl crate::Readable for RLR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RLR%s to value 0"]
impl crate::Resettable for RLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
