#[doc = "Register `DOUTR25` reader"]
pub struct R(crate::R<DOUTR25_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUTR25_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUTR25_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUTR25_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOUTR25` writer"]
pub struct W(crate::W<DOUTR25_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUTR25_SPEC>;
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
impl From<crate::W<DOUTR25_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUTR25_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT25` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT25_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DOUT25` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT25_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUTR25_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout25(&self) -> DOUT25_R {
        DOUT25_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout25(&mut self) -> DOUT25_W<0> {
        DOUT25_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIOS output data register 25\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr25](index.html) module"]
pub struct DOUTR25_SPEC;
impl crate::RegisterSpec for DOUTR25_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doutr25::R](R) reader structure"]
impl crate::Readable for DOUTR25_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doutr25::W](W) writer structure"]
impl crate::Writable for DOUTR25_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOUTR25 to value 0"]
impl crate::Resettable for DOUTR25_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
