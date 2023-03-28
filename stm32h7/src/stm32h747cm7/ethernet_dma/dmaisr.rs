#[doc = "Register `DMAISR` reader"]
pub struct R(crate::R<DMAISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAISR` writer"]
pub struct W(crate::W<DMAISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAISR_SPEC>;
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
impl From<crate::W<DMAISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DC0IS` reader - DMA Channel Interrupt Status"]
pub type DC0IS_R = crate::BitReader<bool>;
#[doc = "Field `DC0IS` writer - DMA Channel Interrupt Status"]
pub type DC0IS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAISR_SPEC, bool, O>;
#[doc = "Field `MTLIS` reader - MTL Interrupt Status"]
pub type MTLIS_R = crate::BitReader<bool>;
#[doc = "Field `MTLIS` writer - MTL Interrupt Status"]
pub type MTLIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAISR_SPEC, bool, O>;
#[doc = "Field `MACIS` reader - MAC Interrupt Status"]
pub type MACIS_R = crate::BitReader<bool>;
#[doc = "Field `MACIS` writer - MAC Interrupt Status"]
pub type MACIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAISR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DMA Channel Interrupt Status"]
    #[inline(always)]
    pub fn dc0is(&self) -> DC0IS_R {
        DC0IS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - MTL Interrupt Status"]
    #[inline(always)]
    pub fn mtlis(&self) -> MTLIS_R {
        MTLIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MAC Interrupt Status"]
    #[inline(always)]
    pub fn macis(&self) -> MACIS_R {
        MACIS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn dc0is(&mut self) -> DC0IS_W<0> {
        DC0IS_W::new(self)
    }
    #[doc = "Bit 16 - MTL Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn mtlis(&mut self) -> MTLIS_W<16> {
        MTLIS_W::new(self)
    }
    #[doc = "Bit 17 - MAC Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn macis(&mut self) -> MACIS_W<17> {
        MACIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaisr](index.html) module"]
pub struct DMAISR_SPEC;
impl crate::RegisterSpec for DMAISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaisr::R](R) reader structure"]
impl crate::Readable for DMAISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaisr::W](W) writer structure"]
impl crate::Writable for DMAISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAISR to value 0"]
impl crate::Resettable for DMAISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
