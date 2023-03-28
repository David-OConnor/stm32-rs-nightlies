#[doc = "Register `FDCAN_RXF1A` reader"]
pub struct R(crate::R<FDCAN_RXF1A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_RXF1A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_RXF1A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_RXF1A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_RXF1A` writer"]
pub struct W(crate::W<FDCAN_RXF1A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_RXF1A_SPEC>;
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
impl From<crate::W<FDCAN_RXF1A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_RXF1A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F1AI` reader - Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFO 1 get index RXF1S\\[F1GI\\]
to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\\[F1FL\\]."]
pub type F1AI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F1AI` writer - Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFO 1 get index RXF1S\\[F1GI\\]
to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\\[F1FL\\]."]
pub type F1AI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_RXF1A_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFO 1 get index RXF1S\\[F1GI\\]
to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\\[F1FL\\]."]
    #[inline(always)]
    pub fn f1ai(&self) -> F1AI_R {
        F1AI_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Rx FIFO 1 acknowledge index After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This sets the Rx FIFO 1 get index RXF1S\\[F1GI\\]
to F1AI + 1 and update the FIFO 1 Fill Level RXF1S\\[F1FL\\]."]
    #[inline(always)]
    #[must_use]
    pub fn f1ai(&mut self) -> F1AI_W<0> {
        F1AI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Rx FIFO 1 acknowledge register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxf1a](index.html) module"]
pub struct FDCAN_RXF1A_SPEC;
impl crate::RegisterSpec for FDCAN_RXF1A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_rxf1a::R](R) reader structure"]
impl crate::Readable for FDCAN_RXF1A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_rxf1a::W](W) writer structure"]
impl crate::Writable for FDCAN_RXF1A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_RXF1A to value 0"]
impl crate::Resettable for FDCAN_RXF1A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
