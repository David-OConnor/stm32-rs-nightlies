#[doc = "Register `ETH_MACRxFCR` reader"]
pub struct R(crate::R<ETH_MACRX_FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACRX_FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACRX_FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACRX_FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_MACRxFCR` writer"]
pub struct W(crate::W<ETH_MACRX_FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACRX_FCR_SPEC>;
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
impl From<crate::W<ETH_MACRX_FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACRX_FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFE` reader - RFE"]
pub type RFE_R = crate::BitReader<bool>;
#[doc = "Field `RFE` writer - RFE"]
pub type RFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACRX_FCR_SPEC, bool, O>;
#[doc = "Field `UP` reader - UP"]
pub type UP_R = crate::BitReader<bool>;
#[doc = "Field `UP` writer - UP"]
pub type UP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MACRX_FCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RFE"]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UP"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RFE"]
    #[inline(always)]
    #[must_use]
    pub fn rfe(&mut self) -> RFE_W<0> {
        RFE_W::new(self)
    }
    #[doc = "Bit 1 - UP"]
    #[inline(always)]
    #[must_use]
    pub fn up(&mut self) -> UP_W<1> {
        UP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Receive Flow Control register controls the pausing of MAC Transmit based on the received Pause packet.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_macrx_fcr](index.html) module"]
pub struct ETH_MACRX_FCR_SPEC;
impl crate::RegisterSpec for ETH_MACRX_FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_macrx_fcr::R](R) reader structure"]
impl crate::Readable for ETH_MACRX_FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_macrx_fcr::W](W) writer structure"]
impl crate::Writable for ETH_MACRX_FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_MACRxFCR to value 0"]
impl crate::Resettable for ETH_MACRX_FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
