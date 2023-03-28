#[doc = "Register `ETH_MACHT1R` reader"]
pub struct R(crate::R<ETH_MACHT1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACHT1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACHT1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACHT1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACHT1R` writer"]
pub struct W(crate::W<ETH_MACHT1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACHT1R_SPEC>;
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
impl From<crate::W<ETH_MACHT1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACHT1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HT63T32` reader - MAC Hash Table Second 32 Bits This field contains the second 32 Bits \\[63:32\\]
of the Hash table."]
pub type HT63T32_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HT63T32` writer - MAC Hash Table Second 32 Bits This field contains the second 32 Bits \\[63:32\\]
of the Hash table."]
pub type HT63T32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MACHT1R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - MAC Hash Table Second 32 Bits This field contains the second 32 Bits \\[63:32\\]
of the Hash table."]
    #[inline(always)]
    pub fn ht63t32(&self) -> HT63T32_R {
        HT63T32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Hash Table Second 32 Bits This field contains the second 32 Bits \\[63:32\\]
of the Hash table."]
    #[inline(always)]
    #[must_use]
    pub fn ht63t32(&mut self) -> HT63T32_W<0> {
        HT63T32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hash Table 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macht1r](index.html) module"]
pub struct ETH_MACHT1R_SPEC;
impl crate::RegisterSpec for ETH_MACHT1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macht1r::R](R) reader structure"]
impl crate::Readable for ETH_MACHT1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macht1r::W](W) writer structure"]
impl crate::Writable for ETH_MACHT1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACHT1R to value 0"]
impl crate::Resettable for ETH_MACHT1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
