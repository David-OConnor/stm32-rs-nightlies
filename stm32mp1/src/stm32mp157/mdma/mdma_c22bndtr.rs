#[doc = "Register `MDMA_C22BNDTR` reader"]
pub struct R(crate::R<MDMA_C22BNDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMA_C22BNDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMA_C22BNDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMA_C22BNDTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDMA_C22BNDTR` writer"]
pub struct W(crate::W<MDMA_C22BNDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDMA_C22BNDTR_SPEC>;
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
impl From<crate::W<MDMA_C22BNDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDMA_C22BNDTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BNDT` reader - BNDT"]
pub type BNDT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BNDT` writer - BNDT"]
pub type BNDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMA_C22BNDTR_SPEC, u32, u32, 17, O>;
#[doc = "Field `BRSUM` reader - BRSUM"]
pub type BRSUM_R = crate::BitReader<bool>;
#[doc = "Field `BRSUM` writer - BRSUM"]
pub type BRSUM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C22BNDTR_SPEC, bool, O>;
#[doc = "Field `BRDUM` reader - BRDUM"]
pub type BRDUM_R = crate::BitReader<bool>;
#[doc = "Field `BRDUM` writer - BRDUM"]
pub type BRDUM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C22BNDTR_SPEC, bool, O>;
#[doc = "Field `BRC` reader - BRC"]
pub type BRC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BRC` writer - BRC"]
pub type BRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMA_C22BNDTR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:16 - BNDT"]
    #[inline(always)]
    pub fn bndt(&self) -> BNDT_R {
        BNDT_R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 18 - BRSUM"]
    #[inline(always)]
    pub fn brsum(&self) -> BRSUM_R {
        BRSUM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - BRDUM"]
    #[inline(always)]
    pub fn brdum(&self) -> BRDUM_R {
        BRDUM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:31 - BRC"]
    #[inline(always)]
    pub fn brc(&self) -> BRC_R {
        BRC_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:16 - BNDT"]
    #[inline(always)]
    #[must_use]
    pub fn bndt(&mut self) -> BNDT_W<0> {
        BNDT_W::new(self)
    }
    #[doc = "Bit 18 - BRSUM"]
    #[inline(always)]
    #[must_use]
    pub fn brsum(&mut self) -> BRSUM_W<18> {
        BRSUM_W::new(self)
    }
    #[doc = "Bit 19 - BRDUM"]
    #[inline(always)]
    #[must_use]
    pub fn brdum(&mut self) -> BRDUM_W<19> {
        BRDUM_W::new(self)
    }
    #[doc = "Bits 20:31 - BRC"]
    #[inline(always)]
    #[must_use]
    pub fn brc(&mut self) -> BRC_W<20> {
        BRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\]
+ 0x04).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c22bndtr](index.html) module"]
pub struct MDMA_C22BNDTR_SPEC;
impl crate::RegisterSpec for MDMA_C22BNDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdma_c22bndtr::R](R) reader structure"]
impl crate::Readable for MDMA_C22BNDTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdma_c22bndtr::W](W) writer structure"]
impl crate::Writable for MDMA_C22BNDTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDMA_C22BNDTR to value 0"]
impl crate::Resettable for MDMA_C22BNDTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
