#[doc = "Register `DMAMUX_RG7CR` reader"]
pub struct R(crate::R<DMAMUX_RG7CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAMUX_RG7CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAMUX_RG7CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAMUX_RG7CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAMUX_RG7CR` writer"]
pub struct W(crate::W<DMAMUX_RG7CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAMUX_RG7CR_SPEC>;
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
impl From<crate::W<DMAMUX_RG7CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAMUX_RG7CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIG_ID` reader - SIG_ID"]
pub type SIG_ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIG_ID` writer - SIG_ID"]
pub type SIG_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAMUX_RG7CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `OIE` reader - OIE"]
pub type OIE_R = crate::BitReader<bool>;
#[doc = "Field `OIE` writer - OIE"]
pub type OIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_RG7CR_SPEC, bool, O>;
#[doc = "Field `GE` reader - GE"]
pub type GE_R = crate::BitReader<bool>;
#[doc = "Field `GE` writer - GE"]
pub type GE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMUX_RG7CR_SPEC, bool, O>;
#[doc = "Field `GPOL` reader - GPOL"]
pub type GPOL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPOL` writer - GPOL"]
pub type GPOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAMUX_RG7CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `GNBREQ` reader - GNBREQ"]
pub type GNBREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GNBREQ` writer - GNBREQ"]
pub type GNBREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAMUX_RG7CR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:2 - SIG_ID"]
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - OIE"]
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - GE"]
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - GPOL"]
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - GNBREQ"]
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SIG_ID"]
    #[inline(always)]
    #[must_use]
    pub fn sig_id(&mut self) -> SIG_ID_W<0> {
        SIG_ID_W::new(self)
    }
    #[doc = "Bit 8 - OIE"]
    #[inline(always)]
    #[must_use]
    pub fn oie(&mut self) -> OIE_W<8> {
        OIE_W::new(self)
    }
    #[doc = "Bit 16 - GE"]
    #[inline(always)]
    #[must_use]
    pub fn ge(&mut self) -> GE_W<16> {
        GE_W::new(self)
    }
    #[doc = "Bits 17:18 - GPOL"]
    #[inline(always)]
    #[must_use]
    pub fn gpol(&mut self) -> GPOL_W<17> {
        GPOL_W::new(self)
    }
    #[doc = "Bits 19:23 - GNBREQ"]
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
#[doc = "DMAMUX request generator channel 7 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_rg7cr](index.html) module"]
pub struct DMAMUX_RG7CR_SPEC;
impl crate::RegisterSpec for DMAMUX_RG7CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmamux_rg7cr::R](R) reader structure"]
impl crate::Readable for DMAMUX_RG7CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmamux_rg7cr::W](W) writer structure"]
impl crate::Writable for DMAMUX_RG7CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAMUX_RG7CR to value 0"]
impl crate::Resettable for DMAMUX_RG7CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
