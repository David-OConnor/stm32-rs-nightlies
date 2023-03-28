#[doc = "Register `HUFFBASE%s` reader"]
pub struct R(crate::R<HUFFBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HUFFBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HUFFBASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HUFFBASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HUFFBASE%s` writer"]
pub struct W(crate::W<HUFFBASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HUFFBASE_SPEC>;
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
impl From<crate::W<HUFFBASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HUFFBASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HuffBase_RAM_0` reader - HuffBase RAM"]
pub type HUFF_BASE_RAM_0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HuffBase_RAM_0` writer - HuffBase RAM"]
pub type HUFF_BASE_RAM_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HUFFBASE_SPEC, u16, u16, 9, O>;
#[doc = "Field `HuffBase_RAM_1` reader - HuffBase RAM"]
pub type HUFF_BASE_RAM_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HuffBase_RAM_1` writer - HuffBase RAM"]
pub type HUFF_BASE_RAM_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HUFFBASE_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - HuffBase RAM"]
    #[inline(always)]
    pub fn huff_base_ram_0(&self) -> HUFF_BASE_RAM_0_R {
        HUFF_BASE_RAM_0_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - HuffBase RAM"]
    #[inline(always)]
    pub fn huff_base_ram_1(&self) -> HUFF_BASE_RAM_1_R {
        HUFF_BASE_RAM_1_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - HuffBase RAM"]
    #[inline(always)]
    #[must_use]
    pub fn huff_base_ram_0(&mut self) -> HUFF_BASE_RAM_0_W<0> {
        HUFF_BASE_RAM_0_W::new(self)
    }
    #[doc = "Bits 16:24 - HuffBase RAM"]
    #[inline(always)]
    #[must_use]
    pub fn huff_base_ram_1(&mut self) -> HUFF_BASE_RAM_1_W<16> {
        HUFF_BASE_RAM_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG HuffSymb tables\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [huffbase](index.html) module"]
pub struct HUFFBASE_SPEC;
impl crate::RegisterSpec for HUFFBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [huffbase::R](R) reader structure"]
impl crate::Readable for HUFFBASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [huffbase::W](W) writer structure"]
impl crate::Writable for HUFFBASE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HUFFBASE%s to value 0"]
impl crate::Resettable for HUFFBASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
