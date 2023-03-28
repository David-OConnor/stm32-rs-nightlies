#[doc = "Register `DOUTR1` reader"]
pub struct R(crate::R<DOUTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOUTR1` writer"]
pub struct W(crate::W<DOUTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUTR1_SPEC>;
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
impl From<crate::W<DOUTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT1` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DOUT1` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUTR1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout1(&self) -> DOUT1_R {
        DOUT1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout1(&mut self) -> DOUT1_W<0> {
        DOUT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIOS output data register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr1](index.html) module"]
pub struct DOUTR1_SPEC;
impl crate::RegisterSpec for DOUTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doutr1::R](R) reader structure"]
impl crate::Readable for DOUTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doutr1::W](W) writer structure"]
impl crate::Writable for DOUTR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOUTR1 to value 0"]
impl crate::Resettable for DOUTR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
