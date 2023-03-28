#[doc = "Register `DOUTR24` reader"]
pub struct R(crate::R<DOUTR24_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUTR24_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUTR24_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUTR24_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOUTR24` writer"]
pub struct W(crate::W<DOUTR24_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUTR24_SPEC>;
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
impl From<crate::W<DOUTR24_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUTR24_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT24` reader - Output data sent to MDIO Master during read frames"]
pub type DOUT24_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DOUT24` writer - Output data sent to MDIO Master during read frames"]
pub type DOUT24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUTR24_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout24(&self) -> DOUT24_R {
        DOUT24_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    #[must_use]
    pub fn dout24(&mut self) -> DOUT24_W<0> {
        DOUT24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDIOS output data register 24\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutr24](index.html) module"]
pub struct DOUTR24_SPEC;
impl crate::RegisterSpec for DOUTR24_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doutr24::R](R) reader structure"]
impl crate::Readable for DOUTR24_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doutr24::W](W) writer structure"]
impl crate::Writable for DOUTR24_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOUTR24 to value 0"]
impl crate::Resettable for DOUTR24_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
