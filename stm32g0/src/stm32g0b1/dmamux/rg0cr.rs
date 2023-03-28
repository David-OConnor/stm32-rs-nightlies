#[doc = "Register `RG0CR` reader"]
pub struct R(crate::R<RG0CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RG0CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RG0CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RG0CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RG0CR` writer"]
pub struct W(crate::W<RG0CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RG0CR_SPEC>;
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
impl From<crate::W<RG0CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RG0CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIG_ID` reader - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator"]
pub type SIG_ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIG_ID` writer - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator"]
pub type SIG_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RG0CR_SPEC, u8, u8, 5, O>;
#[doc = "Field `OIE` reader - Trigger overrun interrupt enable"]
pub type OIE_R = crate::BitReader<bool>;
#[doc = "Field `OIE` writer - Trigger overrun interrupt enable"]
pub type OIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RG0CR_SPEC, bool, O>;
#[doc = "Field `GE` reader - DMA request generator channel x enable"]
pub type GE_R = crate::BitReader<bool>;
#[doc = "Field `GE` writer - DMA request generator channel x enable"]
pub type GE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RG0CR_SPEC, bool, O>;
#[doc = "Field `GPOL` reader - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input"]
pub type GPOL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPOL` writer - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input"]
pub type GPOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RG0CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `GNBREQ` reader - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field shall only be written when GE bit is disabled."]
pub type GNBREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GNBREQ` writer - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field shall only be written when GE bit is disabled."]
pub type GNBREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RG0CR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator"]
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA request generator channel x enable"]
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input"]
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field shall only be written when GE bit is disabled."]
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Signal identification Selects the DMA request trigger input used for the channel x of the DMA request generator"]
    #[inline(always)]
    #[must_use]
    pub fn sig_id(&mut self) -> SIG_ID_W<0> {
        SIG_ID_W::new(self)
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn oie(&mut self) -> OIE_W<8> {
        OIE_W::new(self)
    }
    #[doc = "Bit 16 - DMA request generator channel x enable"]
    #[inline(always)]
    #[must_use]
    pub fn ge(&mut self) -> GE_W<16> {
        GE_W::new(self)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity Defines the edge polarity of the selected trigger input"]
    #[inline(always)]
    #[must_use]
    pub fn gpol(&mut self) -> GPOL_W<17> {
        GPOL_W::new(self)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to be generated (minus 1) Defines the number of DMA requests to be generated after a trigger event. The actual number of generated DMA requests is GNBREQ +1. Note: This field shall only be written when GE bit is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn gnbreq(&mut self) -> GNBREQ_W<19> {
        GNBREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAMUX request generator channel x configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rg0cr](index.html) module"]
pub struct RG0CR_SPEC;
impl crate::RegisterSpec for RG0CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rg0cr::R](R) reader structure"]
impl crate::Readable for RG0CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rg0cr::W](W) writer structure"]
impl crate::Writable for RG0CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RG0CR to value 0"]
impl crate::Resettable for RG0CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
