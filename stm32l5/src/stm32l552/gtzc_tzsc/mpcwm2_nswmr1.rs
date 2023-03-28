#[doc = "Register `MPCWM2_NSWMR1` reader"]
pub struct R(crate::R<MPCWM2_NSWMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCWM2_NSWMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCWM2_NSWMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCWM2_NSWMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCWM2_NSWMR1` writer"]
pub struct W(crate::W<MPCWM2_NSWMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCWM2_NSWMR1_SPEC>;
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
impl From<crate::W<MPCWM2_NSWMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCWM2_NSWMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NSWM1STRT` reader - NSWM1STRT"]
pub type NSWM1STRT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NSWM1STRT` writer - NSWM1STRT"]
pub type NSWM1STRT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MPCWM2_NSWMR1_SPEC, u16, u16, 11, O>;
#[doc = "Field `NSWM1LGTH` reader - NSWM1LGTH"]
pub type NSWM1LGTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NSWM1LGTH` writer - NSWM1LGTH"]
pub type NSWM1LGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MPCWM2_NSWMR1_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:10 - NSWM1STRT"]
    #[inline(always)]
    pub fn nswm1strt(&self) -> NSWM1STRT_R {
        NSWM1STRT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:27 - NSWM1LGTH"]
    #[inline(always)]
    pub fn nswm1lgth(&self) -> NSWM1LGTH_R {
        NSWM1LGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - NSWM1STRT"]
    #[inline(always)]
    #[must_use]
    pub fn nswm1strt(&mut self) -> NSWM1STRT_W<0> {
        NSWM1STRT_W::new(self)
    }
    #[doc = "Bits 16:27 - NSWM1LGTH"]
    #[inline(always)]
    #[must_use]
    pub fn nswm1lgth(&mut self) -> NSWM1LGTH_W<16> {
        NSWM1LGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZSC external memory non-secure watermark register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcwm2_nswmr1](index.html) module"]
pub struct MPCWM2_NSWMR1_SPEC;
impl crate::RegisterSpec for MPCWM2_NSWMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcwm2_nswmr1::R](R) reader structure"]
impl crate::Readable for MPCWM2_NSWMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcwm2_nswmr1::W](W) writer structure"]
impl crate::Writable for MPCWM2_NSWMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPCWM2_NSWMR1 to value 0"]
impl crate::Resettable for MPCWM2_NSWMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
