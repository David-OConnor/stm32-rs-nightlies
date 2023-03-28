#[doc = "Register `AWD3CR` reader"]
pub struct R(crate::R<AWD3CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AWD3CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AWD3CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AWD3CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AWD3CR` writer"]
pub struct W(crate::W<AWD3CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AWD3CR_SPEC>;
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
impl From<crate::W<AWD3CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AWD3CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AWD3CH` reader - AWD3CH"]
pub type AWD3CH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AWD3CH` writer - AWD3CH"]
pub type AWD3CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AWD3CR_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 1:18 - AWD3CH"]
    #[inline(always)]
    pub fn awd3ch(&self) -> AWD3CH_R {
        AWD3CH_R::new((self.bits >> 1) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 1:18 - AWD3CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd3ch(&mut self) -> AWD3CH_W<1> {
        AWD3CH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Watchdog 3 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [awd3cr](index.html) module"]
pub struct AWD3CR_SPEC;
impl crate::RegisterSpec for AWD3CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [awd3cr::R](R) reader structure"]
impl crate::Readable for AWD3CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [awd3cr::W](W) writer structure"]
impl crate::Writable for AWD3CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AWD3CR to value 0"]
impl crate::Resettable for AWD3CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
