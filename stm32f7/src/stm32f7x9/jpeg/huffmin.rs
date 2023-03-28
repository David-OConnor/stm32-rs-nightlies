#[doc = "Register `HUFFMIN%s` reader"]
pub struct R(crate::R<HUFFMIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HUFFMIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HUFFMIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HUFFMIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HUFFMIN%s` writer"]
pub struct W(crate::W<HUFFMIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HUFFMIN_SPEC>;
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
impl From<crate::W<HUFFMIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HUFFMIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HuffMin_RAM` reader - HuffMin RAM"]
pub type HUFF_MIN_RAM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HuffMin_RAM` writer - HuffMin RAM"]
pub type HUFF_MIN_RAM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HUFFMIN_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - HuffMin RAM"]
    #[inline(always)]
    pub fn huff_min_ram(&self) -> HUFF_MIN_RAM_R {
        HUFF_MIN_RAM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HuffMin RAM"]
    #[inline(always)]
    #[must_use]
    pub fn huff_min_ram(&mut self) -> HUFF_MIN_RAM_W<0> {
        HUFF_MIN_RAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG HuffMin tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffmin](index.html) module"]
pub struct HUFFMIN_SPEC;
impl crate::RegisterSpec for HUFFMIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [huffmin::R](R) reader structure"]
impl crate::Readable for HUFFMIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [huffmin::W](W) writer structure"]
impl crate::Writable for HUFFMIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HUFFMIN%s to value 0"]
impl crate::Resettable for HUFFMIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
