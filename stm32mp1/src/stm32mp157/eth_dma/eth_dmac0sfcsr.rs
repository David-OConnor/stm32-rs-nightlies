#[doc = "Register `ETH_DMAC0SFCSR` reader"]
pub struct R(crate::R<ETH_DMAC0SFCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC0SFCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC0SFCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC0SFCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMAC0SFCSR` writer"]
pub struct W(crate::W<ETH_DMAC0SFCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAC0SFCSR_SPEC>;
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
impl From<crate::W<ETH_DMAC0SFCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAC0SFCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ESC` reader - ESC"]
pub type ESC_R = crate::BitReader<bool>;
#[doc = "Field `ESC` writer - ESC"]
pub type ESC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0SFCSR_SPEC, bool, O>;
#[doc = "Field `ASC` reader - ASC"]
pub type ASC_R = crate::BitReader<bool>;
#[doc = "Field `ASC` writer - ASC"]
pub type ASC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0SFCSR_SPEC, bool, O>;
#[doc = "Field `RSN` reader - RSN"]
pub type RSN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSN` writer - RSN"]
pub type RSN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAC0SFCSR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - ESC"]
    #[inline(always)]
    pub fn esc(&self) -> ESC_R {
        ESC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ASC"]
    #[inline(always)]
    pub fn asc(&self) -> ASC_R {
        ASC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:19 - RSN"]
    #[inline(always)]
    pub fn rsn(&self) -> RSN_R {
        RSN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ESC"]
    #[inline(always)]
    #[must_use]
    pub fn esc(&mut self) -> ESC_W<0> {
        ESC_W::new(self)
    }
    #[doc = "Bit 1 - ASC"]
    #[inline(always)]
    #[must_use]
    pub fn asc(&mut self) -> ASC_W<1> {
        ASC_W::new(self)
    }
    #[doc = "Bits 16:19 - RSN"]
    #[inline(always)]
    #[must_use]
    pub fn rsn(&mut self) -> RSN_W<16> {
        RSN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel i slot function control status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0sfcsr](index.html) module"]
pub struct ETH_DMAC0SFCSR_SPEC;
impl crate::RegisterSpec for ETH_DMAC0SFCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmac0sfcsr::R](R) reader structure"]
impl crate::Readable for ETH_DMAC0SFCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmac0sfcsr::W](W) writer structure"]
impl crate::Writable for ETH_DMAC0SFCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_DMAC0SFCSR to value 0"]
impl crate::Resettable for ETH_DMAC0SFCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
