#[doc = "Register `DOUTR7` reader"]
pub struct R(crate::R<DOUTR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUTR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUTR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUTR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOUTR7` writer"]
pub struct W(crate::W<DOUTR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUTR7_SPEC>;
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
impl From<crate::W<DOUTR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUTR7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT7` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DOUT7` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUTR7_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout7(&self) -> DOUT7_R {
        DOUT7_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout7(&mut self) -> DOUT7_W<0> {
        DOUT7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIOS output data register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr7](index.html) module"]
pub struct DOUTR7_SPEC;
impl crate::RegisterSpec for DOUTR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doutr7::R](R) reader structure"]
impl crate::Readable for DOUTR7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doutr7::W](W) writer structure"]
impl crate::Writable for DOUTR7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOUTR7 to value 0"]
impl crate::Resettable for DOUTR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
