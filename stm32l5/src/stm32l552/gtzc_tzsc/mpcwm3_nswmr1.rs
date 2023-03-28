#[doc = "Register `MPCWM3_NSWMR1` reader"]
pub struct R(crate::R<MPCWM3_NSWMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCWM3_NSWMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCWM3_NSWMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCWM3_NSWMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPCWM3_NSWMR1` writer"]
pub struct W(crate::W<MPCWM3_NSWMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCWM3_NSWMR1_SPEC>;
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
impl From<crate::W<MPCWM3_NSWMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCWM3_NSWMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NSWM2STRT` reader - NSWM2STRT"]
pub type NSWM2STRT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NSWM2STRT` writer - NSWM2STRT"]
pub type NSWM2STRT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MPCWM3_NSWMR1_SPEC, u16, u16, 11, O>;
#[doc = "Field `NSWM2LGTH` reader - NSWM2LGTH"]
pub type NSWM2LGTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NSWM2LGTH` writer - NSWM2LGTH"]
pub type NSWM2LGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MPCWM3_NSWMR1_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:10 - NSWM2STRT"]
    #[inline(always)]
    pub fn nswm2strt(&self) -> NSWM2STRT_R {
        NSWM2STRT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:27 - NSWM2LGTH"]
    #[inline(always)]
    pub fn nswm2lgth(&self) -> NSWM2LGTH_R {
        NSWM2LGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - NSWM2STRT"]
    #[inline(always)]
    #[must_use]
    pub fn nswm2strt(&mut self) -> NSWM2STRT_W<0> {
        NSWM2STRT_W::new(self)
    }
    #[doc = "Bits 16:27 - NSWM2LGTH"]
    #[inline(always)]
    #[must_use]
    pub fn nswm2lgth(&mut self) -> NSWM2LGTH_W<16> {
        NSWM2LGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZSC external memory non-secure watermark register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpcwm3_nswmr1](index.html) module"]
pub struct MPCWM3_NSWMR1_SPEC;
impl crate::RegisterSpec for MPCWM3_NSWMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpcwm3_nswmr1::R](R) reader structure"]
impl crate::Readable for MPCWM3_NSWMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpcwm3_nswmr1::W](W) writer structure"]
impl crate::Writable for MPCWM3_NSWMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPCWM3_NSWMR1 to value 0"]
impl crate::Resettable for MPCWM3_NSWMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
