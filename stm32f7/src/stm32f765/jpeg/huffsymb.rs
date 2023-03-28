#[doc = "Register `HUFFSYMB%s` reader"]
pub struct R(crate::R<HUFFSYMB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HUFFSYMB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HUFFSYMB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HUFFSYMB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HUFFSYMB%s` writer"]
pub struct W(crate::W<HUFFSYMB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HUFFSYMB_SPEC>;
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
impl From<crate::W<HUFFSYMB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HUFFSYMB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HuffSymb_RAM` reader - DHTSymb RAM"]
pub type HUFF_SYMB_RAM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HuffSymb_RAM` writer - DHTSymb RAM"]
pub type HUFF_SYMB_RAM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HUFFSYMB_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - DHTSymb RAM"]
    #[inline(always)]
    pub fn huff_symb_ram(&self) -> HUFF_SYMB_RAM_R {
        HUFF_SYMB_RAM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DHTSymb RAM"]
    #[inline(always)]
    #[must_use]
    pub fn huff_symb_ram(&mut self) -> HUFF_SYMB_RAM_W<0> {
        HUFF_SYMB_RAM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG HUFFSYMB tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffsymb](index.html) module"]
pub struct HUFFSYMB_SPEC;
impl crate::RegisterSpec for HUFFSYMB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [huffsymb::R](R) reader structure"]
impl crate::Readable for HUFFSYMB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [huffsymb::W](W) writer structure"]
impl crate::Writable for HUFFSYMB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HUFFSYMB%s to value 0"]
impl crate::Resettable for HUFFSYMB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
