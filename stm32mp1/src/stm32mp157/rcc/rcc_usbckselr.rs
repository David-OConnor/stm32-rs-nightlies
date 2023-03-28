#[doc = "Register `RCC_USBCKSELR` reader"]
pub struct R(crate::R<RCC_USBCKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_USBCKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_USBCKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_USBCKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_USBCKSELR` writer"]
pub struct W(crate::W<RCC_USBCKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_USBCKSELR_SPEC>;
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
impl From<crate::W<RCC_USBCKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_USBCKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBPHYSRC` reader - USBPHYSRC"]
pub type USBPHYSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBPHYSRC` writer - USBPHYSRC"]
pub type USBPHYSRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCC_USBCKSELR_SPEC, u8, u8, 2, O>;
#[doc = "Field `USBOSRC` reader - USBOSRC"]
pub type USBOSRC_R = crate::BitReader<bool>;
#[doc = "Field `USBOSRC` writer - USBOSRC"]
pub type USBOSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_USBCKSELR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - USBPHYSRC"]
    #[inline(always)]
    pub fn usbphysrc(&self) -> USBPHYSRC_R {
        USBPHYSRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - USBOSRC"]
    #[inline(always)]
    pub fn usbosrc(&self) -> USBOSRC_R {
        USBOSRC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - USBPHYSRC"]
    #[inline(always)]
    #[must_use]
    pub fn usbphysrc(&mut self) -> USBPHYSRC_W<0> {
        USBPHYSRC_W::new(self)
    }
    #[doc = "Bit 4 - USBOSRC"]
    #[inline(always)]
    #[must_use]
    pub fn usbosrc(&mut self) -> USBOSRC_W<4> {
        USBOSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the USBPHY PLL of the USB HOST and USB OTG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_usbckselr](index.html) module"]
pub struct RCC_USBCKSELR_SPEC;
impl crate::RegisterSpec for RCC_USBCKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_usbckselr::R](R) reader structure"]
impl crate::Readable for RCC_USBCKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_usbckselr::W](W) writer structure"]
impl crate::Writable for RCC_USBCKSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_USBCKSELR to value 0"]
impl crate::Resettable for RCC_USBCKSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
