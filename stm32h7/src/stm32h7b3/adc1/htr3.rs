#[doc = "Register `HTR3` reader"]
pub struct R(crate::R<HTR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTR3` writer"]
pub struct W(crate::W<HTR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTR3_SPEC>;
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
impl From<crate::W<HTR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HTR3` reader - Analog watchdog 3 higher threshold"]
pub type HTR3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HTR3` writer - Analog watchdog 3 higher threshold"]
pub type HTR3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, HTR3_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:25 - Analog watchdog 3 higher threshold"]
    #[inline(always)]
    pub fn htr3(&self) -> HTR3_R {
        HTR3_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - Analog watchdog 3 higher threshold"]
    #[inline(always)]
    #[must_use]
    pub fn htr3(&mut self) -> HTR3_W<0> {
        HTR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC watchdog higher threshold register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htr3](index.html) module"]
pub struct HTR3_SPEC;
impl crate::RegisterSpec for HTR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htr3::R](R) reader structure"]
impl crate::Readable for HTR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htr3::W](W) writer structure"]
impl crate::Writable for HTR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HTR3 to value 0x03ff_ffff"]
impl crate::Resettable for HTR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff_ffff;
}
