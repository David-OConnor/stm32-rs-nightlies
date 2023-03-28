#[doc = "Register `NSSR` reader"]
pub struct R(crate::R<NSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NSSR` writer"]
pub struct W(crate::W<NSSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSSR_SPEC>;
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
impl From<crate::W<NSSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NSEOP` reader - NSEOP"]
pub type NSEOP_R = crate::BitReader<bool>;
#[doc = "Field `NSEOP` writer - NSEOP"]
pub type NSEOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSSR_SPEC, bool, O>;
#[doc = "Field `NSOPERR` reader - NSOPERR"]
pub type NSOPERR_R = crate::BitReader<bool>;
#[doc = "Field `NSOPERR` writer - NSOPERR"]
pub type NSOPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSSR_SPEC, bool, O>;
#[doc = "Field `NSPROGERR` reader - NSPROGERR"]
pub type NSPROGERR_R = crate::BitReader<bool>;
#[doc = "Field `NSPROGERR` writer - NSPROGERR"]
pub type NSPROGERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSSR_SPEC, bool, O>;
#[doc = "Field `NSWRPERR` reader - NSWRPERR"]
pub type NSWRPERR_R = crate::BitReader<bool>;
#[doc = "Field `NSWRPERR` writer - NSWRPERR"]
pub type NSWRPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSSR_SPEC, bool, O>;
#[doc = "Field `NSPGAERR` reader - NSPGAERR"]
pub type NSPGAERR_R = crate::BitReader<bool>;
#[doc = "Field `NSPGAERR` writer - NSPGAERR"]
pub type NSPGAERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSSR_SPEC, bool, O>;
#[doc = "Field `NSSIZERR` reader - NSSIZERR"]
pub type NSSIZERR_R = crate::BitReader<bool>;
#[doc = "Field `NSSIZERR` writer - NSSIZERR"]
pub type NSSIZERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSSR_SPEC, bool, O>;
#[doc = "Field `NSPGSERR` reader - NSPGSERR"]
pub type NSPGSERR_R = crate::BitReader<bool>;
#[doc = "Field `NSPGSERR` writer - NSPGSERR"]
pub type NSPGSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSSR_SPEC, bool, O>;
#[doc = "Field `OPTWERR` reader - OPTWERR"]
pub type OPTWERR_R = crate::BitReader<bool>;
#[doc = "Field `OPTWERR` writer - OPTWERR"]
pub type OPTWERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSSR_SPEC, bool, O>;
#[doc = "Field `OPTVERR` reader - OPTVERR"]
pub type OPTVERR_R = crate::BitReader<bool>;
#[doc = "Field `OPTVERR` writer - OPTVERR"]
pub type OPTVERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, NSSR_SPEC, bool, O>;
#[doc = "Field `NSBSY` reader - NSBusy"]
pub type NSBSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - NSEOP"]
    #[inline(always)]
    pub fn nseop(&self) -> NSEOP_R {
        NSEOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NSOPERR"]
    #[inline(always)]
    pub fn nsoperr(&self) -> NSOPERR_R {
        NSOPERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - NSPROGERR"]
    #[inline(always)]
    pub fn nsprogerr(&self) -> NSPROGERR_R {
        NSPROGERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NSWRPERR"]
    #[inline(always)]
    pub fn nswrperr(&self) -> NSWRPERR_R {
        NSWRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NSPGAERR"]
    #[inline(always)]
    pub fn nspgaerr(&self) -> NSPGAERR_R {
        NSPGAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NSSIZERR"]
    #[inline(always)]
    pub fn nssizerr(&self) -> NSSIZERR_R {
        NSSIZERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NSPGSERR"]
    #[inline(always)]
    pub fn nspgserr(&self) -> NSPGSERR_R {
        NSPGSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - OPTWERR"]
    #[inline(always)]
    pub fn optwerr(&self) -> OPTWERR_R {
        OPTWERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - OPTVERR"]
    #[inline(always)]
    pub fn optverr(&self) -> OPTVERR_R {
        OPTVERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NSBusy"]
    #[inline(always)]
    pub fn nsbsy(&self) -> NSBSY_R {
        NSBSY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NSEOP"]
    #[inline(always)]
    #[must_use]
    pub fn nseop(&mut self) -> NSEOP_W<0> {
        NSEOP_W::new(self)
    }
    #[doc = "Bit 1 - NSOPERR"]
    #[inline(always)]
    #[must_use]
    pub fn nsoperr(&mut self) -> NSOPERR_W<1> {
        NSOPERR_W::new(self)
    }
    #[doc = "Bit 3 - NSPROGERR"]
    #[inline(always)]
    #[must_use]
    pub fn nsprogerr(&mut self) -> NSPROGERR_W<3> {
        NSPROGERR_W::new(self)
    }
    #[doc = "Bit 4 - NSWRPERR"]
    #[inline(always)]
    #[must_use]
    pub fn nswrperr(&mut self) -> NSWRPERR_W<4> {
        NSWRPERR_W::new(self)
    }
    #[doc = "Bit 5 - NSPGAERR"]
    #[inline(always)]
    #[must_use]
    pub fn nspgaerr(&mut self) -> NSPGAERR_W<5> {
        NSPGAERR_W::new(self)
    }
    #[doc = "Bit 6 - NSSIZERR"]
    #[inline(always)]
    #[must_use]
    pub fn nssizerr(&mut self) -> NSSIZERR_W<6> {
        NSSIZERR_W::new(self)
    }
    #[doc = "Bit 7 - NSPGSERR"]
    #[inline(always)]
    #[must_use]
    pub fn nspgserr(&mut self) -> NSPGSERR_W<7> {
        NSPGSERR_W::new(self)
    }
    #[doc = "Bit 13 - OPTWERR"]
    #[inline(always)]
    #[must_use]
    pub fn optwerr(&mut self) -> OPTWERR_W<13> {
        OPTWERR_W::new(self)
    }
    #[doc = "Bit 15 - OPTVERR"]
    #[inline(always)]
    #[must_use]
    pub fn optverr(&mut self) -> OPTVERR_W<15> {
        OPTVERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nssr](index.html) module"]
pub struct NSSR_SPEC;
impl crate::RegisterSpec for NSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nssr::R](R) reader structure"]
impl crate::Readable for NSSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nssr::W](W) writer structure"]
impl crate::Writable for NSSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NSSR to value 0"]
impl crate::Resettable for NSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
