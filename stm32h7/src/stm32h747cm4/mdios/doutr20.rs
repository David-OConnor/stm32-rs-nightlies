#[doc = "Register `DOUTR20` reader"]
pub struct R(crate::R<DOUTR20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUTR20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUTR20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUTR20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOUTR20` writer"]
pub struct W(crate::W<DOUTR20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUTR20_SPEC>;
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
impl From<crate::W<DOUTR20_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUTR20_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT20` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT20_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DOUT20` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT20_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUTR20_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout20(&self) -> DOUT20_R {
        DOUT20_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout20(&mut self) -> DOUT20_W<0> {
        DOUT20_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIOS output data register 20\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr20](index.html) module"]
pub struct DOUTR20_SPEC;
impl crate::RegisterSpec for DOUTR20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doutr20::R](R) reader structure"]
impl crate::Readable for DOUTR20_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doutr20::W](W) writer structure"]
impl crate::Writable for DOUTR20_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOUTR20 to value 0"]
impl crate::Resettable for DOUTR20_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
