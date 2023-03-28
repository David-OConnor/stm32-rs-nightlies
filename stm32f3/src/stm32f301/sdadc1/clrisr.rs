#[doc = "Register `CLRISR` reader"]
pub struct R(crate::R<CLRISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLRISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLRISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLRISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLRISR` writer"]
pub struct W(crate::W<CLRISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLRISR_SPEC>;
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
impl From<crate::W<CLRISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLRISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLREOCALF` reader - Clear the end of calibration flag"]
pub type CLREOCALF_R = crate::BitReader<bool>;
#[doc = "Field `CLREOCALF` writer - Clear the end of calibration flag"]
pub type CLREOCALF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRISR_SPEC, bool, O>;
#[doc = "Field `CLRJOVRF` reader - Clear the injected conversion overrun flag"]
pub type CLRJOVRF_R = crate::BitReader<bool>;
#[doc = "Field `CLRJOVRF` writer - Clear the injected conversion overrun flag"]
pub type CLRJOVRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRISR_SPEC, bool, O>;
#[doc = "Field `CLRROVRF` reader - Clear the regular conversion overrun flag"]
pub type CLRROVRF_R = crate::BitReader<bool>;
#[doc = "Field `CLRROVRF` writer - Clear the regular conversion overrun flag"]
pub type CLRROVRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRISR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Clear the end of calibration flag"]
    #[inline(always)]
    pub fn clreocalf(&self) -> CLREOCALF_R {
        CLREOCALF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Clear the injected conversion overrun flag"]
    #[inline(always)]
    pub fn clrjovrf(&self) -> CLRJOVRF_R {
        CLRJOVRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear the regular conversion overrun flag"]
    #[inline(always)]
    pub fn clrrovrf(&self) -> CLRROVRF_R {
        CLRROVRF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear the end of calibration flag"]
    #[inline(always)]
    #[must_use]
    pub fn clreocalf(&mut self) -> CLREOCALF_W<0> {
        CLREOCALF_W::new(self)
    }
    #[doc = "Bit 2 - Clear the injected conversion overrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrjovrf(&mut self) -> CLRJOVRF_W<2> {
        CLRJOVRF_W::new(self)
    }
    #[doc = "Bit 4 - Clear the regular conversion overrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn clrrovrf(&mut self) -> CLRROVRF_W<4> {
        CLRROVRF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt and status clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrisr](index.html) module"]
pub struct CLRISR_SPEC;
impl crate::RegisterSpec for CLRISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clrisr::R](R) reader structure"]
impl crate::Readable for CLRISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clrisr::W](W) writer structure"]
impl crate::Writable for CLRISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLRISR to value 0"]
impl crate::Resettable for CLRISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
