#[doc = "Register `ETH_DMAC0RxIWTR` reader"]
pub struct R(crate::R<ETH_DMAC0RX_IWTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC0RX_IWTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC0RX_IWTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC0RX_IWTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMAC0RxIWTR` writer"]
pub struct W(crate::W<ETH_DMAC0RX_IWTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAC0RX_IWTR_SPEC>;
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
impl From<crate::W<ETH_DMAC0RX_IWTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAC0RX_IWTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RWT` reader - Receive Interrupt Watchdog Timer Count"]
pub type RWT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RWT` writer - Receive Interrupt Watchdog Timer Count"]
pub type RWT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAC0RX_IWTR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Receive Interrupt Watchdog Timer Count"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Interrupt Watchdog Timer Count"]
    #[inline(always)]
    #[must_use]
    pub fn rwt(&mut self) -> RWT_W<0> {
        RWT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Rx interrupt watchdog timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0rx_iwtr](index.html) module"]
pub struct ETH_DMAC0RX_IWTR_SPEC;
impl crate::RegisterSpec for ETH_DMAC0RX_IWTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmac0rx_iwtr::R](R) reader structure"]
impl crate::Readable for ETH_DMAC0RX_IWTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmac0rx_iwtr::W](W) writer structure"]
impl crate::Writable for ETH_DMAC0RX_IWTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETH_DMAC0RxIWTR to value 0"]
impl crate::Resettable for ETH_DMAC0RX_IWTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
