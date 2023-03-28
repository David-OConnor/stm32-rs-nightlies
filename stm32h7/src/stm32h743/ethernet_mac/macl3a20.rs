#[doc = "Register `MACL3A20` reader"]
pub struct R(crate::R<MACL3A20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACL3A20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACL3A20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACL3A20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACL3A20` writer"]
pub struct W(crate::W<MACL3A20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACL3A20_SPEC>;
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
impl From<crate::W<MACL3A20_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACL3A20_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `L3A20` reader - Layer 3 Address 2 Field"]
pub type L3A20_R = crate::FieldReader<u32, u32>;
#[doc = "Field `L3A20` writer - Layer 3 Address 2 Field"]
pub type L3A20_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACL3A20_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Layer 3 Address 2 Field"]
    #[inline(always)]
    pub fn l3a20(&self) -> L3A20_R {
        L3A20_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Layer 3 Address 2 Field"]
    #[inline(always)]
    #[must_use]
    pub fn l3a20(&mut self) -> L3A20_W<0> {
        L3A20_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Layer3 Address 2 filter 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macl3a20](index.html) module"]
pub struct MACL3A20_SPEC;
impl crate::RegisterSpec for MACL3A20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macl3a20::R](R) reader structure"]
impl crate::Readable for MACL3A20_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macl3a20::W](W) writer structure"]
impl crate::Writable for MACL3A20_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACL3A20 to value 0"]
impl crate::Resettable for MACL3A20_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
