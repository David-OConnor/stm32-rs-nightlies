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
pub type PROCID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, R_SPEC, u8, u8, 8, O>;
#[doc = "Field `MASTERID` reader - Semaphore MasterID"]
pub type MASTERID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASTERID` writer - Semaphore MasterID"]
pub type MASTERID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, R_SPEC, u8, u8, 8, O>;
#[doc = "Field `LOCK` reader - Lock indication"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - Lock indication"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, R_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Semaphore ProcessID"]
    #[inline(always)]
    pub fn procid(&self) -> PROCID_R {
        PROCID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Semaphore MasterID"]
    #[inline(always)]
    pub fn masterid(&self) -> MASTERID_R {
        MASTERID_R::new(((self.bits >> 8) & 0xff) as u8)
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
    #[doc = "Bits 8:15 - Semaphore MasterID"]
    #[inline(always)]
    #[must_use]
    pub fn masterid(&mut self) -> MASTERID_W<8> {
        MASTERID_W::new(self)
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
