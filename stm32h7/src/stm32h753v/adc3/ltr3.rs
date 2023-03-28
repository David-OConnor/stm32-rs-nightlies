#[doc = "Register `LTR3` reader"]
pub struct R(crate::R<LTR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTR3` writer"]
pub struct W(crate::W<LTR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTR3_SPEC>;
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
impl From<crate::W<LTR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LTR3` reader - Analog watchdog 3 lower threshold"]
pub type LTR3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LTR3` writer - Analog watchdog 3 lower threshold"]
pub type LTR3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, LTR3_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:25 - Analog watchdog 3 lower threshold"]
    #[inline(always)]
    pub fn ltr3(&self) -> LTR3_R {
        LTR3_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - Analog watchdog 3 lower threshold"]
    #[inline(always)]
    #[must_use]
    pub fn ltr3(&mut self) -> LTR3_W<0> {
        LTR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC watchdog lower threshold register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltr3](index.html) module"]
pub struct LTR3_SPEC;
impl crate::RegisterSpec for LTR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltr3::R](R) reader structure"]
impl crate::Readable for LTR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltr3::W](W) writer structure"]
impl crate::Writable for LTR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LTR3 to value 0"]
impl crate::Resettable for LTR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
