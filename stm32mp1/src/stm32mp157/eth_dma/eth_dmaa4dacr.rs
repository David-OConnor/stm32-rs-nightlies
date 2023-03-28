#[doc = "Register `ETH_DMAA4DACR` reader"]
pub struct R(crate::R<ETH_DMAA4DACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAA4DACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAA4DACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAA4DACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMAA4DACR` writer"]
pub struct W(crate::W<ETH_DMAA4DACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAA4DACR_SPEC>;
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
impl From<crate::W<ETH_DMAA4DACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAA4DACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDWC` reader - TDWC"]
pub type TDWC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TDWC` writer - TDWC"]
pub type TDWC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAA4DACR_SPEC, u8, u8, 4, O>;
#[doc = "Field `TDWD` reader - TDWD"]
pub type TDWD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TDWD` writer - TDWD"]
pub type TDWD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAA4DACR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RDRC` reader - RDRC"]
pub type RDRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDRC` writer - RDRC"]
pub type RDRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAA4DACR_SPEC, u8, u8, 4, O>;
#[doc = "Field `RDP` reader - RDP"]
pub type RDP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDP` writer - RDP"]
pub type RDP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAA4DACR_SPEC, u8, u8, 3, O>;
#[doc = "Field `WRP` reader - WRP"]
pub type WRP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRP` writer - WRP"]
pub type WRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAA4DACR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:3 - TDWC"]
    #[inline(always)]
    pub fn tdwc(&self) -> TDWC_R {
        TDWC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - TDWD"]
    #[inline(always)]
    pub fn tdwd(&self) -> TDWD_R {
        TDWD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - RDRC"]
    #[inline(always)]
    pub fn rdrc(&self) -> RDRC_R {
        RDRC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - RDP"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - WRP"]
    #[inline(always)]
    pub fn wrp(&self) -> WRP_R {
        WRP_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TDWC"]
    #[inline(always)]
    #[must_use]
    pub fn tdwc(&mut self) -> TDWC_W<0> {
        TDWC_W::new(self)
    }
    #[doc = "Bits 4:5 - TDWD"]
    #[inline(always)]
    #[must_use]
    pub fn tdwd(&mut self) -> TDWD_W<4> {
        TDWD_W::new(self)
    }
    #[doc = "Bits 8:11 - RDRC"]
    #[inline(always)]
    #[must_use]
    pub fn rdrc(&mut self) -> RDRC_W<8> {
        RDRC_W::new(self)
    }
    #[doc = "Bits 16:18 - RDP"]
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<16> {
        RDP_W::new(self)
    }
    #[doc = "Bits 20:22 - WRP"]
    #[inline(always)]
    #[must_use]
    pub fn wrp(&mut self) -> WRP_W<20> {
        WRP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AXI4 descriptor ACE control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmaa4dacr](index.html) module"]
pub struct ETH_DMAA4DACR_SPEC;
impl crate::RegisterSpec for ETH_DMAA4DACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmaa4dacr::R](R) reader structure"]
impl crate::Readable for ETH_DMAA4DACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmaa4dacr::W](W) writer structure"]
impl crate::Writable for ETH_DMAA4DACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_DMAA4DACR to value 0"]
impl crate::Resettable for ETH_DMAA4DACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
