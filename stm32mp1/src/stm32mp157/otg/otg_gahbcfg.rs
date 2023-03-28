#[doc = "Register `OTG_GAHBCFG` reader"]
pub struct R(crate::R<OTG_GAHBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_GAHBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_GAHBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_GAHBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_GAHBCFG` writer"]
pub struct W(crate::W<OTG_GAHBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_GAHBCFG_SPEC>;
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
impl From<crate::W<OTG_GAHBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_GAHBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GINTMSK` reader - GINTMSK"]
pub type GINTMSK_R = crate::BitReader<bool>;
#[doc = "Field `GINTMSK` writer - GINTMSK"]
pub type GINTMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GAHBCFG_SPEC, bool, O>;
#[doc = "Field `HBSTLEN` reader - HBSTLEN"]
pub type HBSTLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HBSTLEN` writer - HBSTLEN"]
pub type HBSTLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_GAHBCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `DMAEN` reader - DMAEN"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMAEN"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GAHBCFG_SPEC, bool, O>;
#[doc = "Field `TXFELVL` reader - TXFELVL"]
pub type TXFELVL_R = crate::BitReader<bool>;
#[doc = "Field `TXFELVL` writer - TXFELVL"]
pub type TXFELVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GAHBCFG_SPEC, bool, O>;
#[doc = "Field `PTXFELVL` reader - PTXFELVL"]
pub type PTXFELVL_R = crate::BitReader<bool>;
#[doc = "Field `PTXFELVL` writer - PTXFELVL"]
pub type PTXFELVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_GAHBCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GINTMSK"]
    #[inline(always)]
    pub fn gintmsk(&self) -> GINTMSK_R {
        GINTMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - HBSTLEN"]
    #[inline(always)]
    pub fn hbstlen(&self) -> HBSTLEN_R {
        HBSTLEN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - TXFELVL"]
    #[inline(always)]
    pub fn txfelvl(&self) -> TXFELVL_R {
        TXFELVL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PTXFELVL"]
    #[inline(always)]
    pub fn ptxfelvl(&self) -> PTXFELVL_R {
        PTXFELVL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GINTMSK"]
    #[inline(always)]
    #[must_use]
    pub fn gintmsk(&mut self) -> GINTMSK_W<0> {
        GINTMSK_W::new(self)
    }
    #[doc = "Bits 1:4 - HBSTLEN"]
    #[inline(always)]
    #[must_use]
    pub fn hbstlen(&mut self) -> HBSTLEN_W<1> {
        HBSTLEN_W::new(self)
    }
    #[doc = "Bit 5 - DMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<5> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 7 - TXFELVL"]
    #[inline(always)]
    #[must_use]
    pub fn txfelvl(&mut self) -> TXFELVL_W<7> {
        TXFELVL_W::new(self)
    }
    #[doc = "Bit 8 - PTXFELVL"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfelvl(&mut self) -> PTXFELVL_W<8> {
        PTXFELVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_gahbcfg](index.html) module"]
pub struct OTG_GAHBCFG_SPEC;
impl crate::RegisterSpec for OTG_GAHBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_gahbcfg::R](R) reader structure"]
impl crate::Readable for OTG_GAHBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_gahbcfg::W](W) writer structure"]
impl crate::Writable for OTG_GAHBCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_GAHBCFG to value 0"]
impl crate::Resettable for OTG_GAHBCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
