#[doc = "Register `FDCAN_RXF0A` reader"]
pub struct R(crate::R<FDCAN_RXF0A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_RXF0A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_RXF0A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_RXF0A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_RXF0A` writer"]
pub struct W(crate::W<FDCAN_RXF0A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_RXF0A_SPEC>;
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
impl From<crate::W<FDCAN_RXF0A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_RXF0A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F0AI` reader - Rx FIFO 0 Acknowledge Index"]
pub type F0AI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F0AI` writer - Rx FIFO 0 Acknowledge Index"]
pub type F0AI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_RXF0A_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Rx FIFO 0 Acknowledge Index"]
    #[inline(always)]
    pub fn f0ai(&self) -> F0AI_R {
        F0AI_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Rx FIFO 0 Acknowledge Index"]
    #[inline(always)]
    #[must_use]
    pub fn f0ai(&mut self) -> F0AI_W<0> {
        F0AI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Rx FIFO 0 Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxf0a](index.html) module"]
pub struct FDCAN_RXF0A_SPEC;
impl crate::RegisterSpec for FDCAN_RXF0A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_rxf0a::R](R) reader structure"]
impl crate::Readable for FDCAN_RXF0A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_rxf0a::W](W) writer structure"]
impl crate::Writable for FDCAN_RXF0A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_RXF0A to value 0"]
impl crate::Resettable for FDCAN_RXF0A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
