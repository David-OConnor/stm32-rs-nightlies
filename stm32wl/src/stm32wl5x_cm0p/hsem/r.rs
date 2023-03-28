#[doc = "Register `R%s` reader"]
pub struct R(crate::R<R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R%s` writer"]
pub struct W(crate::W<R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROCID` reader - Semaphore ProcessID"]
pub type PROCID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PROCID` writer - Semaphore ProcessID"]
pub type PROCID_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, R_SPEC, u8, u8, 8, O>;
#[doc = "Field `COREID` reader - COREID"]
pub type COREID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COREID` writer - COREID"]
pub type COREID_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, R_SPEC, u8, u8, 4, O>;
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
#[doc = "Lock indication\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKW_AW {
    #[doc = "0: Free semaphore"]
    Free = 0,
    #[doc = "1: Try to lock semaphore"]
    TryLock = 1,
}
impl From<LOCKW_AW> for bool {
    #[inline(always)]
    fn from(variant: LOCKW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` writer - Lock indication"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, R_SPEC, LOCKW_AW, O>;
impl<'a, const O: u8> LOCK_W<'a, O> {
    #[doc = "Free semaphore"]
    #[inline(always)]
    pub fn free(self) -> &'a mut W {
        self.variant(LOCKW_AW::Free)
    }
    #[doc = "Try to lock semaphore"]
    #[inline(always)]
    pub fn try_lock(self) -> &'a mut W {
        self.variant(LOCKW_AW::TryLock)
    }
}
impl R {
    #[doc = "Bits 0:7 - Semaphore ProcessID"]
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - COREID"]
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
impl W {
    #[doc = "Bits 0:7 - Semaphore ProcessID"]
    #[inline(always)]
    #[must_use]
    pub fn procid(&mut self) -> PROCID_W<0> {
        PROCID_W::new(self)
    }
    #[doc = "Bits 8:11 - COREID"]
    #[inline(always)]
    #[must_use]
    pub fn coreid(&mut self) -> COREID_W<8> {
        COREID_W::new(self)
    }
    #[doc = "Bit 31 - Lock indication"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSEM register HSEM_R%s HSEM_R31\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r](index.html) module"]
pub struct R_SPEC;
impl crate::RegisterSpec for R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r::R](R) reader structure"]
impl crate::Readable for R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r::W](W) writer structure"]
impl crate::Writable for R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets R%s to value 0"]
impl crate::Resettable for R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
