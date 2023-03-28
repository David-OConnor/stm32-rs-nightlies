#[doc = "Register `OR1` reader"]
pub struct R(crate::R<OR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OR1` writer"]
pub struct W(crate::W<OR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR1_SPEC>;
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
impl From<crate::W<OR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSE32EN` reader - HSE Divided by 32 enable"]
pub type HSE32EN_R = crate::BitReader<bool>;
#[doc = "Field `HSE32EN` writer - HSE Divided by 32 enable"]
pub type HSE32EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - HSE Divided by 32 enable"]
    #[inline(always)]
    pub fn hse32en(&self) -> HSE32EN_R {
        HSE32EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSE Divided by 32 enable"]
    #[inline(always)]
    #[must_use]
    pub fn hse32en(&mut self) -> HSE32EN_W<0> {
        HSE32EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM option register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or1](index.html) module"]
pub struct OR1_SPEC;
impl crate::RegisterSpec for OR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or1::R](R) reader structure"]
impl crate::Readable for OR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [or1::W](W) writer structure"]
impl crate::Writable for OR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OR1 to value 0"]
impl crate::Resettable for OR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
