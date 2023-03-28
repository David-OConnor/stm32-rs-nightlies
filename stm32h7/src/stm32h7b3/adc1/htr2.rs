#[doc = "Register `HTR2` reader"]
pub struct R(crate::R<HTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTR2` writer"]
pub struct W(crate::W<HTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTR2_SPEC>;
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
impl From<crate::W<HTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HTR2` reader - Analog watchdog 2 higher threshold"]
pub type HTR2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HTR2` writer - Analog watchdog 2 higher threshold"]
pub type HTR2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, HTR2_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:25 - Analog watchdog 2 higher threshold"]
    #[inline(always)]
    pub fn htr2(&self) -> HTR2_R {
        HTR2_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - Analog watchdog 2 higher threshold"]
    #[inline(always)]
    #[must_use]
    pub fn htr2(&mut self) -> HTR2_W<0> {
        HTR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC watchdog higher threshold register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htr2](index.html) module"]
pub struct HTR2_SPEC;
impl crate::RegisterSpec for HTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htr2::R](R) reader structure"]
impl crate::Readable for HTR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htr2::W](W) writer structure"]
impl crate::Writable for HTR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HTR2 to value 0x03ff_ffff"]
impl crate::Resettable for HTR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff_ffff;
}
